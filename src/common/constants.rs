//! Application constants - single source of truth

/// Environment variable names
pub mod env {
    pub const API_KEY: &str = "LINEARS_API_KEY";
    pub const OUTPUT: &str = "LINEARS_OUTPUT";
    pub const ENDPOINT: &str = "LINEARS_ENDPOINT";
    pub const WORKSPACE: &str = "LINEARS_WORKSPACE";
}

/// Pagination defaults
pub mod pagination {
    /// Maximum records for --all flag
    pub const MAX_RECORDS: usize = 1000;
    /// Page size for auto-pagination
    pub const PAGE_SIZE: i32 = 50;
}

/// Client configuration
pub mod client {
    /// Maximum retry attempts for 5xx errors
    pub const MAX_RETRIES: u32 = 10;
    /// Base delay for exponential backoff (ms)
    pub const BASE_DELAY_MS: u64 = 100;
    /// Maximum delay cap (ms)
    pub const MAX_DELAY_MS: u64 = 30_000;
    /// Rate limit auto-retry threshold (seconds)
    pub const RATE_LIMIT_AUTO_RETRY_THRESHOLD_SECS: u64 = 60;
    /// Default request timeout (seconds)
    pub const DEFAULT_TIMEOUT_SECS: u64 = 30;
}

/// Display formatting
pub mod display {
    /// Days threshold for relative vs absolute datetime display
    pub const RELATIVE_TIME_THRESHOLD_DAYS: i64 = 7;
}
