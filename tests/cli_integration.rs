//! CLI integration tests for linears
//!
//! These tests use httpmock to simulate the Linear GraphQL API
//! and verify end-to-end CLI behavior.

mod factories;
mod support;

use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

use factories::generated::{issue, team, user};
use support::mock_server::{
    cli_with_mock_server, create_mock_server, mock_auth_error, mock_list_issues, mock_list_teams,
};

/// Test that missing API key results in exit code 2
#[test]
fn test_missing_api_key() {
    let mut cmd = Command::cargo_bin("linears").unwrap();
    cmd.arg("list")
        .arg("issue")
        .env_remove("LINEARS_API_KEY")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains("LINEARS_API_KEY"));
}

/// Test that --help works without API key
#[test]
fn test_help_without_api_key() {
    let mut cmd = Command::cargo_bin("linears").unwrap();
    cmd.arg("--help")
        .env_remove("LINEARS_API_KEY")
        .assert()
        .success()
        .stdout(predicate::str::contains("Linear's GraphQL API"));
}

/// Test that --version shows schema info
#[test]
fn test_version_shows_schema() {
    let mut cmd = Command::cargo_bin("linears").unwrap();
    cmd.arg("--version")
        .env_remove("LINEARS_API_KEY")
        .assert()
        .success()
        .stdout(predicate::str::contains("linears"))
        .stdout(predicate::str::contains("schema:"));
}

/// Test that resources command lists available resources
#[test]
fn test_resources_command() {
    let mut cmd = Command::cargo_bin("linears").unwrap();
    cmd.arg("resources")
        .env_remove("LINEARS_API_KEY")
        .assert()
        .success()
        .stdout(predicate::str::contains("issue"))
        .stdout(predicate::str::contains("team"));
}

/// Test that ops command lists available operations
#[test]
fn test_ops_command() {
    let mut cmd = Command::cargo_bin("linears").unwrap();
    cmd.arg("ops")
        .env_remove("LINEARS_API_KEY")
        .assert()
        .success()
        .stdout(predicate::str::contains("issue"));
}

/// Test list issues with mock server
#[test]
fn test_list_issues_json() {
    let server = create_mock_server();
    let issues = vec![issue(1), issue(2), issue(3)];
    let _mock = mock_list_issues(&server, issues);

    cli_with_mock_server(&server)
        .args(["--out", "json", "list", "issue"])
        .assert()
        .success()
        .stdout(predicate::str::contains("ENG-1"))
        .stdout(predicate::str::contains("ENG-2"))
        .stdout(predicate::str::contains("ENG-3"));
}

/// Test list teams with mock server
#[test]
fn test_list_teams_json() {
    let server = create_mock_server();
    // team(0) = Engineering, team(1) = Design, etc.
    let teams = vec![team(0), team(1)];
    let _mock = mock_list_teams(&server, teams);

    cli_with_mock_server(&server)
        .args(["--out", "json", "list", "team"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Engineering"));
}

/// Test authentication error handling
#[test]
fn test_auth_error_handling() {
    let server = create_mock_server();
    let _mock = mock_auth_error(&server);

    cli_with_mock_server(&server)
        .args(["list", "issue"])
        .assert()
        .failure()
        .code(2);
}

/// Test empty results
#[test]
fn test_empty_results() {
    let server = create_mock_server();
    let _mock = mock_list_issues(&server, vec![]);

    cli_with_mock_server(&server)
        .args(["--out", "json", "list", "issue"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"nodes\":[]").or(predicate::str::contains("\"nodes\": []")));
}

/// Test invalid resource name
#[test]
fn test_invalid_resource() {
    let mut cmd = Command::cargo_bin("linears").unwrap();
    cmd.args(["list", "notaresource"])
        .env("LINEARS_API_KEY", "test")
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid value"));
}

/// Test that factories produce deterministic output
#[test]
fn test_factory_determinism() {
    let issue1 = issue(42);
    let issue2 = issue(42);
    assert_eq!(issue1, issue2);

    let team1 = team(1);
    let team2 = team(1);
    assert_eq!(team1, team2);

    let user1 = user(5);
    let user2 = user(5);
    assert_eq!(user1, user2);
}

// ============= Discovery Command Tests =============

/// Test schema info command works without API key
#[test]
fn test_schema_info_no_api_key() {
    let mut cmd = Command::cargo_bin("linears").unwrap();
    cmd.args(["schema", "info"])
        .env_remove("LINEARS_API_KEY")
        .assert()
        .success()
        .stdout(predicate::str::contains("Schema Information"))
        .stdout(predicate::str::contains("Commit"));
}

/// Test resources shows all key resources
#[test]
fn test_resources_contains_key_types() {
    let mut cmd = Command::cargo_bin("linears").unwrap();
    cmd.arg("resources")
        .env_remove("LINEARS_API_KEY")
        .assert()
        .success()
        .stdout(predicate::str::contains("issue"))
        .stdout(predicate::str::contains("team"))
        .stdout(predicate::str::contains("user"))
        .stdout(predicate::str::contains("project"))
        .stdout(predicate::str::contains("cycle"));
}

/// Test ops shows key mutation operations
#[test]
fn test_ops_contains_key_mutations() {
    let mut cmd = Command::cargo_bin("linears").unwrap();
    cmd.arg("ops")
        .env_remove("LINEARS_API_KEY")
        .assert()
        .success()
        .stdout(predicate::str::contains("issueCreate"))
        .stdout(predicate::str::contains("issueUpdate"))
        .stdout(predicate::str::contains("commentCreate"));
}

// ============= Help Command Tests =============

/// Test list --help shows all options
#[test]
fn test_list_help_shows_options() {
    let mut cmd = Command::cargo_bin("linears").unwrap();
    cmd.args(["list", "--help"])
        .env_remove("LINEARS_API_KEY")
        .assert()
        .success()
        .stdout(predicate::str::contains("--first"))
        .stdout(predicate::str::contains("--after"))
        .stdout(predicate::str::contains("--filter"))
        .stdout(predicate::str::contains("--all"));
}

/// Test get --help shows usage
#[test]
fn test_get_help_shows_usage() {
    let mut cmd = Command::cargo_bin("linears").unwrap();
    cmd.args(["get", "--help"])
        .env_remove("LINEARS_API_KEY")
        .assert()
        .success()
        .stdout(predicate::str::contains("RESOURCE"))
        .stdout(predicate::str::contains("ID"));
}

/// Test create --help shows input options
#[test]
fn test_create_help_shows_input() {
    let mut cmd = Command::cargo_bin("linears").unwrap();
    cmd.args(["create", "--help"])
        .env_remove("LINEARS_API_KEY")
        .assert()
        .success()
        .stdout(predicate::str::contains("--input"))
        .stdout(predicate::str::contains("--input-file"));
}

/// Test update --help shows set options
#[test]
fn test_update_help_shows_set() {
    let mut cmd = Command::cargo_bin("linears").unwrap();
    cmd.args(["update", "--help"])
        .env_remove("LINEARS_API_KEY")
        .assert()
        .success()
        .stdout(predicate::str::contains("--set"))
        .stdout(predicate::str::contains("--set-file"));
}

// ============= Output Format Tests =============

/// Test YAML output format
#[test]
fn test_yaml_output() {
    let server = create_mock_server();
    let _mock = mock_list_issues(&server, vec![issue(1)]);

    cli_with_mock_server(&server)
        .args(["--out", "yaml", "list", "issue"])
        .assert()
        .success()
        .stdout(predicate::str::contains("nodes:"))
        .stdout(predicate::str::contains("ENG-1"));
}

/// Test NDJSON output format
#[test]
fn test_ndjson_output() {
    let server = create_mock_server();
    let _mock = mock_list_issues(&server, vec![issue(1), issue(2)]);

    let output = cli_with_mock_server(&server)
        .args(["--out", "ndjson", "list", "issue"])
        .assert()
        .success();

    // NDJSON should have newline-separated JSON objects
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let lines: Vec<&str> = stdout.lines().collect();
    assert!(lines.len() >= 2, "NDJSON should have multiple lines");
}

/// Test table output (default)
#[test]
fn test_table_output_default() {
    let server = create_mock_server();
    let _mock = mock_list_issues(&server, vec![issue(1)]);

    cli_with_mock_server(&server)
        .args(["list", "issue"])
        .assert()
        .success();
}

// ============= Error Handling Tests =============

/// Test GraphQL error returns exit code 4
#[test]
fn test_graphql_error_exit_code() {
    let server = create_mock_server();
    let _mock = support::mock_server::mock_graphql_error(&server, "Invalid query syntax");

    cli_with_mock_server(&server)
        .args(["list", "issue"])
        .assert()
        .failure()
        .code(4);
}

/// Test 5xx server error gets retried (we can't fully test retries but can test failure)
#[test]
fn test_server_error_handling() {
    let server = create_mock_server();
    let _mock = support::mock_server::mock_server_error(&server, 500);

    // Server error should eventually fail (after retries exhaust)
    // In test we just verify it doesn't succeed
    cli_with_mock_server(&server)
        .args(["list", "issue"])
        .timeout(std::time::Duration::from_secs(5))
        .assert()
        .failure();
}

// ============= Global Flag Tests =============

/// Test --no-color flag is accepted
#[test]
fn test_no_color_flag() {
    let server = create_mock_server();
    let _mock = mock_list_issues(&server, vec![issue(1)]);

    cli_with_mock_server(&server)
        .args(["--no-color", "list", "issue"])
        .assert()
        .success();
}

/// Test --pretty flag with JSON
#[test]
fn test_pretty_json() {
    let server = create_mock_server();
    let _mock = mock_list_issues(&server, vec![issue(1)]);

    let output = cli_with_mock_server(&server)
        .args(["--out", "json", "--pretty", "list", "issue"])
        .assert()
        .success();

    // Pretty-printed JSON has newlines and indentation
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    assert!(stdout.contains("\n  "), "Pretty JSON should have indentation");
}

/// Test --verbose flag shows query
#[test]
fn test_verbose_flag() {
    let server = create_mock_server();
    let _mock = mock_list_issues(&server, vec![issue(1)]);

    cli_with_mock_server(&server)
        .args(["--verbose", "list", "issue"])
        .assert()
        .success()
        .stderr(predicate::str::contains("Query:"));
}

// ============= Pagination Tests =============

/// Test --first limits results
#[test]
fn test_first_pagination() {
    let server = create_mock_server();
    let _mock = mock_list_issues(&server, vec![issue(1), issue(2)]);

    cli_with_mock_server(&server)
        .args(["--out", "json", "list", "issue", "--first", "5"])
        .assert()
        .success();
}

// ============= Invalid Input Tests =============

/// Test invalid output format
#[test]
fn test_invalid_output_format() {
    let mut cmd = Command::cargo_bin("linears").unwrap();
    cmd.args(["--out", "invalid", "resources"])
        .env("LINEARS_API_KEY", "test")
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid value"));
}

/// Test invalid mutation operation
#[test]
fn test_invalid_mutation_op() {
    let mut cmd = Command::cargo_bin("linears").unwrap();
    cmd.args(["mutate", "notAnOperation"])
        .env("LINEARS_API_KEY", "test")
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid value"));
}
