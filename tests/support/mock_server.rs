//! Mock server helpers for testing GraphQL API interactions

use httpmock::prelude::*;
use serde_json::Value;

/// Test API key used in mock server tests
pub const TEST_API_KEY: &str = "lin_api_test_key_12345";

/// Create a new mock server for Linear's GraphQL API
pub fn create_mock_server() -> MockServer {
    MockServer::start()
}

/// Set up a mock for a successful GraphQL query response
pub fn mock_graphql_success<'a>(server: &'a MockServer, response_data: Value) -> httpmock::Mock<'a> {
    server.mock(|when, then| {
        when.method(POST)
            .path("/graphql")
            .header("content-type", "application/json");
        then.status(200)
            .header("content-type", "application/json")
            .json_body(serde_json::json!({
                "data": response_data
            }));
    })
}

/// Set up a mock for a GraphQL error response
pub fn mock_graphql_error<'a>(server: &'a MockServer, error_message: &str) -> httpmock::Mock<'a> {
    server.mock(|when, then| {
        when.method(POST).path("/graphql");
        then.status(200)
            .header("content-type", "application/json")
            .json_body(serde_json::json!({
                "data": null,
                "errors": [{
                    "message": error_message,
                    "locations": [{"line": 1, "column": 1}],
                    "path": ["query"]
                }]
            }));
    })
}

/// Set up a mock for an authentication error (401)
pub fn mock_auth_error<'a>(server: &'a MockServer) -> httpmock::Mock<'a> {
    server.mock(|when, then| {
        when.method(POST).path("/graphql");
        then.status(401)
            .header("content-type", "application/json")
            .json_body(serde_json::json!({
                "error": "Unauthorized",
                "message": "Invalid or missing API key"
            }));
    })
}

/// Set up a mock for rate limiting (429)
pub fn mock_rate_limit<'a>(server: &'a MockServer, retry_after_secs: u64) -> httpmock::Mock<'a> {
    server.mock(|when, then| {
        when.method(POST).path("/graphql");
        then.status(429)
            .header("Retry-After", retry_after_secs.to_string())
            .header("content-type", "application/json")
            .json_body(serde_json::json!({
                "error": "Too Many Requests",
                "message": "Rate limit exceeded"
            }));
    })
}

/// Set up a mock for server error (5xx)
pub fn mock_server_error<'a>(server: &'a MockServer, status_code: u16) -> httpmock::Mock<'a> {
    server.mock(|when, then| {
        when.method(POST).path("/graphql");
        then.status(status_code)
            .header("content-type", "application/json")
            .json_body(serde_json::json!({
                "error": "Internal Server Error",
                "message": "Something went wrong"
            }));
    })
}

/// Create a mock for listing issues
pub fn mock_list_issues<'a>(server: &'a MockServer, issues: Vec<Value>) -> httpmock::Mock<'a> {
    mock_graphql_success(
        server,
        serde_json::json!({
            "issues": {
                "nodes": issues,
                "pageInfo": {
                    "hasNextPage": false,
                    "hasPreviousPage": false,
                    "startCursor": if !issues.is_empty() { Some("cursor-start") } else { None },
                    "endCursor": if !issues.is_empty() { Some("cursor-end") } else { None }
                }
            }
        }),
    )
}

/// Create a mock for getting a single issue
pub fn mock_get_issue<'a>(server: &'a MockServer, issue: Value) -> httpmock::Mock<'a> {
    mock_graphql_success(server, serde_json::json!({ "issue": issue }))
}

/// Create a mock for listing teams
pub fn mock_list_teams<'a>(server: &'a MockServer, teams: Vec<Value>) -> httpmock::Mock<'a> {
    mock_graphql_success(
        server,
        serde_json::json!({
            "teams": {
                "nodes": teams,
                "pageInfo": {
                    "hasNextPage": false,
                    "hasPreviousPage": false,
                    "startCursor": if !teams.is_empty() { Some("cursor-start") } else { None },
                    "endCursor": if !teams.is_empty() { Some("cursor-end") } else { None }
                }
            }
        }),
    )
}

/// Create a mock for listing users
pub fn mock_list_users<'a>(server: &'a MockServer, users: Vec<Value>) -> httpmock::Mock<'a> {
    mock_graphql_success(
        server,
        serde_json::json!({
            "users": {
                "nodes": users,
                "pageInfo": {
                    "hasNextPage": false,
                    "hasPreviousPage": false,
                    "startCursor": if !users.is_empty() { Some("cursor-start") } else { None },
                    "endCursor": if !users.is_empty() { Some("cursor-end") } else { None }
                }
            }
        }),
    )
}

/// Helper to run CLI with mock server
pub fn cli_with_mock_server(server: &MockServer) -> assert_cmd::Command {
    let mut cmd = assert_cmd::Command::cargo_bin("linears").unwrap();
    cmd.env("LINEARS_API_KEY", TEST_API_KEY);
    cmd.env("LINEARS_ENDPOINT", format!("{}/graphql", server.base_url()));
    cmd
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_server_creation() {
        let server = create_mock_server();
        assert!(!server.base_url().is_empty());
    }

    #[test]
    fn test_mock_graphql_success() {
        let server = create_mock_server();
        let _mock = mock_graphql_success(&server, serde_json::json!({"test": "data"}));

        let client = reqwest::blocking::Client::new();
        let response = client
            .post(format!("{}/graphql", server.base_url()))
            .header("content-type", "application/json")
            .json(&serde_json::json!({"query": "{ test }"}))
            .send()
            .unwrap();

        assert_eq!(response.status(), 200);
        let body: Value = response.json().unwrap();
        assert_eq!(body["data"]["test"], "data");
    }
}
