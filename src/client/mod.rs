//! HTTP client for Linear's GraphQL API

use chrono::{Duration as ChronoDuration, Utc};
use rand::Rng;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use thiserror::Error;
use tokio::time::sleep;

/// Errors that can occur when using the Linear API client
#[derive(Debug, Error)]
pub enum ClientError {
    /// Authentication failed (exit code 2)
    #[error("Authentication failed: {0}")]
    Auth(String),

    /// Network error (exit code 3)
    #[error("Network error: {0}")]
    Network(String),

    /// GraphQL error from Linear (exit code 4)
    /// Contains the error message and optionally the raw GraphQL errors for debugging
    #[error("GraphQL error: {0}")]
    GraphQL(String, Option<Vec<GraphQLError>>),

    /// Rate limited (retriable - wait time ≤60s)
    #[error("Rate limited: {0}")]
    RateLimited(u64),

    /// Rate limited (too long to wait - >60s)
    #[error("{0}")]
    RateLimitedTooLong(String),

    /// Server error
    #[error("Server error: {0}")]
    Server(String),

    /// Other errors
    #[error("{0}")]
    Other(String),
}

impl ClientError {
    /// Get the exit code for this error
    pub fn exit_code(&self) -> u8 {
        match self {
            ClientError::Auth(_) => 2,
            ClientError::Network(_) => 3,
            ClientError::GraphQL(_, _) => 4,
            ClientError::RateLimited(_) => 1,
            ClientError::RateLimitedTooLong(_) => 1,
            ClientError::Server(_) => 1,
            ClientError::Other(_) => 1,
        }
    }

    /// Get a helpful hint for this error
    pub fn hint(&self) -> Option<&'static str> {
        match self {
            ClientError::Auth(_) => Some("Check that LINEARS_API_KEY is set correctly"),
            ClientError::Network(_) => Some("Check your network connection"),
            ClientError::RateLimited(_) | ClientError::RateLimitedTooLong(_) => {
                Some("The API is rate-limiting your requests. Wait and try again")
            }
            ClientError::Server(_) => Some("Linear is experiencing issues. Try again later"),
            _ => None,
        }
    }
}

/// Retry configuration for 5xx errors
const MAX_RETRIES: u32 = 10;
const BASE_DELAY_MS: u64 = 100;
const MAX_DELAY_MS: u64 = 30_000;

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
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GraphQLError {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locations: Option<Vec<ErrorLocation>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<serde_json::Value>,
}

/// Error location in query
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ErrorLocation {
    pub line: i32,
    pub column: i32,
}

impl Client {
    /// Create a new client with the given API key
    pub fn new(
        api_key: &str,
        endpoint: Option<&str>,
        timeout_secs: u64,
        workspace: Option<&str>,
    ) -> Result<Self, ClientError> {
        let mut headers = HeaderMap::new();
        // Linear API expects the API key directly without Bearer prefix
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(api_key)
                .map_err(|e| ClientError::Auth(format!("Invalid API key format: {}", e)))?,
        );
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        // Add workspace header if provided (for multi-workspace API keys)
        if let Some(ws) = workspace {
            headers.insert(
                "Linear-Workspace",
                HeaderValue::from_str(ws)
                    .map_err(|e| ClientError::Auth(format!("Invalid workspace value: {}", e)))?,
            );
        }

        let http = reqwest::Client::builder()
            .default_headers(headers)
            .timeout(Duration::from_secs(timeout_secs))
            .build()
            .map_err(|e| ClientError::Other(format!("Failed to create HTTP client: {}", e)))?;

        let endpoint = endpoint
            .unwrap_or("https://api.linear.app/graphql")
            .to_string();

        Ok(Self { http, endpoint })
    }

    /// Execute a GraphQL request with automatic retries for 5xx errors and rate limits
    pub async fn execute(&self, request: GraphQLRequest) -> Result<GraphQLResponse, ClientError> {
        let mut retries = 0;

        loop {
            let result = self.execute_once(&request).await;

            match result {
                Ok(response) => return Ok(response),
                Err(ClientError::Server(ref msg)) if retries < MAX_RETRIES => {
                    // Retry on server errors with exponential backoff + jitter
                    retries += 1;
                    let base_delay = BASE_DELAY_MS * 2u64.pow(retries - 1);
                    let jitter = rand::rng().random_range(0..=base_delay / 2);
                    let delay = (base_delay + jitter).min(MAX_DELAY_MS);

                    eprintln!(
                        "Server error ({}), retrying in {}ms (attempt {}/{})",
                        msg, delay, retries, MAX_RETRIES
                    );

                    sleep(Duration::from_millis(delay)).await;
                }
                Err(ClientError::RateLimited(secs)) => {
                    // Rate limited with short wait - retry after waiting
                    eprintln!("Rate limited. Waiting {} seconds before retrying...", secs);
                    sleep(Duration::from_secs(secs)).await;
                    // Continue loop without incrementing retries
                }
                Err(e) => {
                    // Non-retryable error or max retries reached
                    return Err(e);
                }
            }
        }
    }

    /// Execute a single GraphQL request (no retries)
    async fn execute_once(&self, request: &GraphQLRequest) -> Result<GraphQLResponse, ClientError> {
        let response = self
            .http
            .post(&self.endpoint)
            .json(request)
            .send()
            .await
            .map_err(|e| {
                if e.is_timeout() {
                    ClientError::Network(format!("Request timed out: {}", e))
                } else if e.is_connect() {
                    ClientError::Network(format!("Connection failed: {}", e))
                } else {
                    ClientError::Network(format!("Request failed: {}", e))
                }
            })?;

        let status = response.status();

        if status == reqwest::StatusCode::UNAUTHORIZED {
            return Err(ClientError::Auth("Invalid or missing API key".to_string()));
        }

        if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
            let retry_after = response
                .headers()
                .get("Retry-After")
                .and_then(|h| h.to_str().ok())
                .and_then(|s| s.parse::<u64>().ok());

            if let Some(secs) = retry_after {
                if secs <= 60 {
                    // Short wait - can be retried automatically
                    return Err(ClientError::RateLimited(secs));
                } else {
                    // Too long to wait - show reset time and exit
                    let reset_time = Utc::now() + ChronoDuration::seconds(secs as i64);
                    let formatted = reset_time.format("%H:%M:%S UTC");
                    return Err(ClientError::RateLimitedTooLong(format!(
                        "Rate limited. Too long to wait ({} seconds). Rate limit resets at {}",
                        secs, formatted
                    )));
                }
            } else {
                // No Retry-After header - assume short wait
                return Err(ClientError::RateLimited(5));
            }
        }

        if status.is_server_error() {
            return Err(ClientError::Server(format!("HTTP {}", status)));
        }

        let body: GraphQLResponse = response
            .json()
            .await
            .map_err(|e| ClientError::Other(format!("Failed to parse response: {}", e)))?;

        // Check for GraphQL errors in the response
        if let Some(errors) = body.errors.clone() {
            if !errors.is_empty() {
                let messages: Vec<&str> = errors.iter().map(|e| e.message.as_str()).collect();
                // Check if any error is authentication related
                for msg in &messages {
                    if msg.to_lowercase().contains("authentication")
                        || msg.to_lowercase().contains("unauthorized")
                        || msg.to_lowercase().contains("api key")
                    {
                        return Err(ClientError::Auth(msg.to_string()));
                    }
                }
                return Err(ClientError::GraphQL(messages.join("; "), Some(errors)));
            }
        }

        Ok(body)
    }
}
