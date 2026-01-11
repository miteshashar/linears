//! Command handlers for the CLI
//!
//! This module organizes command handlers into logical groups:
//! - query: list, get, search, raw
//! - mutation: create, update, delete, archive, unarchive, mutate
//! - discovery: resources, ops
//! - schema: info, diff

pub mod discovery;
pub mod mutation;
pub mod query;
pub mod schema;

use anyhow::Result;

use crate::cli::GlobalOptions;
use crate::client::Client;
use crate::common::constants::env;

// Re-export all command handlers for easy access
pub use discovery::{cmd_ops, cmd_resources};
pub use mutation::{cmd_archive, cmd_create, cmd_delete, cmd_mutate, cmd_unarchive, cmd_update};
pub use query::{cmd_get, cmd_list, cmd_raw, cmd_search};
pub use schema::cmd_schema;

/// Get the API key from environment, returning error message if missing or empty
pub fn get_api_key() -> Result<String, String> {
    match std::env::var(env::API_KEY) {
        Ok(key) if key.trim().is_empty() => Err(format!(
            "{} environment variable is empty.\n\
             \n\
             To use this command, set your Linear API key:\n\
             \n\
               export {}='lin_api_...'\n\
             \n\
             Get your API key from: https://linear.app/settings/api",
            env::API_KEY,
            env::API_KEY
        )),
        Ok(key) => Ok(key),
        Err(_) => Err(format!(
            "Missing {} environment variable.\n\
             \n\
             To use this command, set your Linear API key:\n\
             \n\
               export {}='lin_api_...'\n\
             \n\
             Get your API key from: https://linear.app/settings/api",
            env::API_KEY,
            env::API_KEY
        )),
    }
}

/// Create a client using the CLI's global options
pub fn create_client(global: &GlobalOptions) -> Result<Client> {
    let api_key = get_api_key().expect("API key already validated");
    Ok(Client::new(
        &api_key,
        global.endpoint.as_deref(),
        global.timeout,
        global.workspace.as_deref(),
    )?)
}
