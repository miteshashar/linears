//! Generated order_by enum - DO NOT EDIT
//! Run `cargo xtask codegen` to regenerate

use clap::ValueEnum;
use std::fmt;

/// Pagination order by field
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum OrderBy {
    /// Order by createdAt
    CreatedAt,
    /// Order by updatedAt
    UpdatedAt,
}

impl OrderBy {
    /// Get the GraphQL value for this order by field
    pub fn as_graphql_value(&self) -> &'static str {
        match self {
            OrderBy::CreatedAt => "createdAt",
            OrderBy::UpdatedAt => "updatedAt",
        }
    }
}

impl fmt::Display for OrderBy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_graphql_value())
    }
}
