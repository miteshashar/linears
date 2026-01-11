//! CLI command definitions using clap

use clap::{Args, Parser, Subcommand, ValueEnum};

use crate::common::constants::{client as client_const, env as env_const};
use crate::common::FieldsetPreset;
use crate::generated::{MutationOp, OrderBy, Resource};

use std::sync::OnceLock;

/// Schema metadata embedded at compile time
const SCHEMA_META: &str = include_str!("../../schemas/linear/schema.meta.json");

/// Static storage for the computed version string
static VERSION: OnceLock<String> = OnceLock::new();

/// Get version string with schema info
/// Uses OnceLock for idiomatic lazy initialization without memory leaks
fn get_version() -> &'static str {
    VERSION.get_or_init(|| {
        let pkg_version = env!("CARGO_PKG_VERSION");

        // Try to parse schema metadata for enhanced version info
        if let Ok(meta) = serde_json::from_str::<serde_json::Value>(SCHEMA_META) {
            let commit = meta["commit"]
                .as_str()
                .map(|c| if c.len() >= 7 { &c[..7] } else { c })
                .unwrap_or("unknown");
            let commit_date = meta["commitDate"]
                .as_str()
                .and_then(|d| d.split('T').next())
                .unwrap_or("unknown");
            format!("{} (schema: {}, {})", pkg_version, commit, commit_date)
        } else {
            pkg_version.to_string()
        }
    })
}

/// A CLI for Linear's GraphQL API
#[derive(Parser)]
#[command(name = "linears")]
#[command(version = get_version())]
#[command(about = "Complete CLI coverage of Linear's GraphQL API")]
#[command(long_about = None)]
pub struct Cli {
    #[command(flatten)]
    pub global: GlobalOptions,

    #[command(subcommand)]
    pub command: Commands,
}

/// Global options available to all commands
#[derive(Args)]
pub struct GlobalOptions {
    /// Output format
    #[arg(long = "out", value_enum, default_value = "table", env = env_const::OUTPUT)]
    pub output: OutputFormat,

    /// Pretty-print JSON/YAML output
    #[arg(long)]
    pub pretty: bool,

    /// Disable colored output
    #[arg(long)]
    pub no_color: bool,

    /// Show GraphQL query being sent
    #[arg(short, long)]
    pub verbose: bool,

    /// Override API endpoint
    #[arg(long, env = env_const::ENDPOINT)]
    pub endpoint: Option<String>,

    /// Request timeout in seconds
    #[arg(long, default_value_t = client_const::DEFAULT_TIMEOUT_SECS)]
    pub timeout: u64,

    /// Workspace slug or ID (for multi-workspace API keys)
    #[arg(long, env = env_const::WORKSPACE)]
    pub workspace: Option<String>,
}

/// Output format options
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum OutputFormat {
    /// JSON output
    Json,
    /// YAML output
    Yaml,
    /// Table output (default)
    Table,
    /// Human-readable text for single entities
    Text,
    /// Newline-delimited JSON (one object per line)
    Ndjson,
}

/// Available commands
#[derive(Subcommand)]
pub enum Commands {
    /// List all available query resources
    Resources,

    /// List all available mutation operations
    Ops,

    /// List entities with pagination and filtering
    List {
        /// The resource type to list
        resource: Resource,

        #[command(flatten)]
        options: ListOptions,
    },

    /// Get a single entity by ID or identifier
    Get {
        /// The resource type
        resource: Resource,

        /// Entity ID (UUID) or identifier (e.g., ENG-123)
        id: String,
    },

    /// Search entities with smart search strategy
    Search {
        /// The resource type to search
        resource: Resource,

        /// Search text
        text: String,
    },

    /// Execute arbitrary GraphQL query
    Raw {
        /// GraphQL query (inline or file path)
        #[arg(long)]
        query: String,

        #[command(flatten)]
        vars: VarsOptions,
    },

    /// Create a new entity
    Create {
        /// The resource type to create
        resource: Resource,

        #[command(flatten)]
        input: InputOptions,
    },

    /// Update an existing entity
    Update {
        /// The resource type to update
        resource: Resource,

        /// Entity ID or identifier
        id: String,

        #[command(flatten)]
        set: SetOptions,
    },

    /// Delete an entity
    Delete {
        /// The resource type to delete
        resource: Resource,

        /// Entity ID or identifier
        id: String,
    },

    /// Archive an entity
    Archive {
        /// The resource type to archive
        resource: Resource,

        /// Entity ID or identifier
        id: String,
    },

    /// Unarchive an entity
    Unarchive {
        /// The resource type to unarchive
        resource: Resource,

        /// Entity ID or identifier
        id: String,
    },

    /// Execute any mutation operation
    Mutate {
        /// The mutation operation to execute
        op: MutationOp,

        #[command(flatten)]
        vars: VarsOptions,
    },

    /// Schema management commands
    Schema {
        #[command(subcommand)]
        action: SchemaAction,
    },
}

/// List command options
#[derive(Args, Clone)]
pub struct ListOptions {
    /// Limit results (forward pagination)
    #[arg(long, default_value = "20")]
    pub first: Option<i32>,

    /// Pagination cursor (forward)
    #[arg(long)]
    pub after: Option<String>,

    /// Limit results (backward pagination)
    #[arg(long)]
    pub last: Option<i32>,

    /// Pagination cursor (backward)
    #[arg(long)]
    pub before: Option<String>,

    /// Auto-paginate all results (max 1000)
    #[arg(long)]
    pub all: bool,

    /// Include archived entities
    #[arg(long)]
    pub include_archived: bool,

    /// Sort order (createdAt or updatedAt)
    #[arg(long, value_enum)]
    pub order_by: Option<OrderBy>,

    /// Inline filter expression (JSON or YAML)
    #[arg(long)]
    pub filter: Option<String>,

    /// Filter from file
    #[arg(long)]
    pub filter_file: Option<String>,

    /// Field selection preset
    #[arg(long, value_enum, default_value = "default")]
    pub preset: FieldsetPreset,

    /// Comma-separated scalar fields to select
    #[arg(long)]
    pub select: Option<String>,

    /// Relation expansion (relation[:fields])
    #[arg(long)]
    pub expand: Option<Vec<String>>,
}


/// Input options for create command
#[derive(Args, Clone)]
pub struct InputOptions {
    /// Inline input (JSON or YAML, use '-' for stdin)
    #[arg(long)]
    pub input: Option<String>,

    /// Input from file
    #[arg(long)]
    pub input_file: Option<String>,
}

/// Set options for update command
#[derive(Args, Clone)]
pub struct SetOptions {
    /// Inline update data (JSON or YAML, use '-' for stdin)
    #[arg(long)]
    pub set: Option<String>,

    /// Update data from file
    #[arg(long)]
    pub set_file: Option<String>,
}

/// Variables options for mutate command
#[derive(Args, Clone)]
pub struct VarsOptions {
    /// Inline variables (JSON or YAML, use '-' for stdin)
    #[arg(long)]
    pub vars: Option<String>,

    /// Variables from file
    #[arg(long)]
    pub vars_file: Option<String>,

    /// Individual variable override (key=value)
    #[arg(long = "var")]
    pub var: Option<Vec<String>>,
}

/// Schema management actions
#[derive(Subcommand, Clone)]
pub enum SchemaAction {
    /// Display schema version information
    Info,
    /// Show diff between local and upstream schema
    Diff,
}
