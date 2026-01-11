//! linears - A CLI for Linear's GraphQL API
//!
//! Provides complete coverage of Linear's API surface area with:
//! - Query commands: list, get, search, raw
//! - Mutation commands: create, update, delete, archive, mutate
//! - Discovery commands: resources, ops
//! - Schema commands: info, diff, sync

use std::process::ExitCode;

use anyhow::Result;
use clap::Parser;

mod cli;
mod client;
mod commands;
mod generated;
mod mutation_builder;
mod progress;
mod query_builder;
mod render;
mod schema_diff;
mod validate;

use cli::{Cli, Commands};
use commands::{get_api_key, *};

/// Exit codes for the CLI
mod exit_codes {
    pub const SUCCESS: u8 = 0;
    pub const GENERAL_ERROR: u8 = 1;
    pub const AUTH_ERROR: u8 = 2;
    #[allow(dead_code)]
    pub const NETWORK_ERROR: u8 = 3;
    #[allow(dead_code)]
    pub const GRAPHQL_ERROR: u8 = 4;
}

/// Check if the command requires API access
fn command_requires_api(cmd: &Commands) -> bool {
    match cmd {
        Commands::Resources | Commands::Ops | Commands::Schema { .. } => false,
        _ => true,
    }
}

#[tokio::main]
async fn main() -> ExitCode {
    let _ = dotenvy::dotenv();
    let cli = Cli::parse();

    progress::set_no_color(cli.global.no_color);

    // Check for API key if command requires it
    if command_requires_api(&cli.command) {
        if let Err(msg) = get_api_key() {
            eprintln!("Error: {}", msg);
            return ExitCode::from(exit_codes::AUTH_ERROR);
        }
    }

    // Dispatch to command handlers
    let result: Result<()> = match &cli.command {
        Commands::Resources => cmd_resources(&cli),
        Commands::Ops => cmd_ops(&cli),
        Commands::List { resource, options } => {
            cmd_list(&cli, resource.clone(), options.clone()).await
        }
        Commands::Get { resource, id } => cmd_get(&cli, resource.clone(), id.clone()).await,
        Commands::Search { resource, text } => {
            cmd_search(&cli, resource.clone(), text.clone()).await
        }
        Commands::Raw { query, vars } => cmd_raw(&cli, query.clone(), vars.clone()).await,
        Commands::Create { resource, input } => {
            cmd_create(&cli, resource.clone(), input.clone()).await
        }
        Commands::Update { resource, id, set } => {
            cmd_update(&cli, resource.clone(), id.clone(), set.clone()).await
        }
        Commands::Delete { resource, id } => cmd_delete(&cli, resource.clone(), id.clone()).await,
        Commands::Archive { resource, id } => {
            cmd_archive(&cli, resource.clone(), id.clone()).await
        }
        Commands::Unarchive { resource, id } => {
            cmd_unarchive(&cli, resource.clone(), id.clone()).await
        }
        Commands::Mutate { op, vars } => cmd_mutate(&cli, op.clone(), vars.clone()).await,
        Commands::Schema { action } => cmd_schema(&cli, action.clone()).await,
    };

    // Handle result
    match result {
        Ok(()) => ExitCode::from(exit_codes::SUCCESS),
        Err(e) => {
            let (exit_code, kind, message, hint, graphql_errors, details) =
                if let Some(err) = e.downcast_ref::<client::ClientError>() {
                    let kind = match err {
                        client::ClientError::Auth(_) => "auth",
                        client::ClientError::Network(_) => "network",
                        client::ClientError::GraphQL(_, _) => "graphql",
                        client::ClientError::RateLimited(_) => "rate_limited",
                        client::ClientError::RateLimitedTooLong(_) => "rate_limited",
                        client::ClientError::Server(_) => "server",
                        client::ClientError::Other(_) => "other",
                    };
                    let (gql_errors, details) = match err {
                        client::ClientError::GraphQL(_, Some(errors)) => {
                            (Some(errors.clone()), errors.first().and_then(|e| e.extensions.clone()))
                        }
                        _ => (None, None),
                    };
                    (err.exit_code(), kind, format!("{}", err), err.hint(), gql_errors, details)
                } else {
                    (exit_codes::GENERAL_ERROR, "general", format!("{:#}", e), None, None, None)
                };

            let error_info = render::ErrorInfo {
                kind,
                message: &message,
                hint: hint.as_deref(),
                details: details.as_ref(),
                graphql_errors: graphql_errors.as_ref(),
            };
            eprintln!("{}", render::render_error(cli.global.output, &error_info, cli.global.pretty));
            ExitCode::from(exit_code)
        }
    }
}
