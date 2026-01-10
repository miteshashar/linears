//! HTTP client for Linear's GraphQL API

use anyhow::{Context, Result};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// GraphQL client for Linear API
pub struct Client {
    http: reqwest::Client,
    endpoint: String,
}

/// GraphQL request body
#[derive(Debug, Serialize)]
pub struct GraphQLRequest {
    pub query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_name: Option<String>,
}

/// GraphQL response
#[derive(Debug, Deserialize)]
pub struct GraphQLResponse {
    pub data: Option<serde_json::Value>,
    pub errors: Option<Vec<GraphQLError>>,
}

/// GraphQL error
#[derive(Debug, Deserialize)]
pub struct GraphQLError {
    pub message: String,
    pub locations: Option<Vec<ErrorLocation>>,
    pub path: Option<Vec<serde_json::Value>>,
    pub extensions: Option<serde_json::Value>,
}

/// Error location in query
#[derive(Debug, Deserialize)]
pub struct ErrorLocation {
    pub line: i32,
    pub column: i32,
}

impl Client {
    /// Create a new client with the given API key
    pub fn new(api_key: &str, endpoint: Option<&str>, timeout_secs: u64) -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", api_key))
                .context("Invalid API key format")?,
        );
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let http = reqwest::Client::builder()
            .default_headers(headers)
            .timeout(Duration::from_secs(timeout_secs))
            .build()
            .context("Failed to create HTTP client")?;

        let endpoint = endpoint
            .unwrap_or("https://api.linear.app/graphql")
            .to_string();

        Ok(Self { http, endpoint })
    }

    /// Execute a GraphQL request
    pub async fn execute(&self, request: GraphQLRequest) -> Result<GraphQLResponse> {
        let response = self
            .http
            .post(&self.endpoint)
            .json(&request)
            .send()
            .await
            .context("Network request failed")?;

        let status = response.status();

        if status == reqwest::StatusCode::UNAUTHORIZED {
            anyhow::bail!("Authentication failed: Invalid or missing API key");
        }

        if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
            let retry_after = response
                .headers()
                .get("Retry-After")
                .and_then(|h| h.to_str().ok())
                .and_then(|s| s.parse::<u64>().ok());

            if let Some(secs) = retry_after {
                anyhow::bail!(
                    "Rate limited. Retry after {} seconds",
                    secs
                );
            } else {
                anyhow::bail!("Rate limited. Please wait and retry.");
            }
        }

        if status.is_server_error() {
            anyhow::bail!("Server error: {}", status);
        }

        let body: GraphQLResponse = response
            .json()
            .await
            .context("Failed to parse response")?;

        Ok(body)
    }
}
