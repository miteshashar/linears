//! linears - A CLI for Linear's GraphQL API
//!
//! Provides complete coverage of Linear's API surface area with:
//! - Query commands: list, get, search, raw
//! - Mutation commands: create, update, delete, archive, mutate
//! - Discovery commands: resources, ops
//! - Schema commands: info, diff, sync

use anyhow::Result;
use clap::Parser;

mod cli;
mod client;
mod commands;
mod common;
mod generated;
mod mutation_builder;
mod progress;
mod query_builder;
mod render;
mod schema_diff;
mod validate;

use cli::{Cli, Commands};
use commands::{get_api_key, *};
use common::ExitCode;

/// Check if the command requires API access
fn command_requires_api(cmd: &Commands) -> bool {
    !matches!(cmd, Commands::Resources | Commands::Ops | Commands::Schema { .. })
}

#[tokio::main]
async fn main() -> std::process::ExitCode {
    let _ = dotenvy::dotenv();
    let cli = Cli::parse();

    progress::set_no_color(cli.global.no_color);

    // Check for API key if command requires it
    if command_requires_api(&cli.command) {
        if let Err(msg) = get_api_key() {
            eprintln!("Error: {}", msg);
            return ExitCode::AuthError.into();
        }
    }

    // Dispatch to command handlers
    let result: Result<()> = match &cli.command {
        Commands::Resources => cmd_resources(&cli),
        Commands::Ops => cmd_ops(&cli),
        Commands::List { resource, options } => {
            cmd_list(&cli, *resource, options.clone()).await
        }
        Commands::Get { resource, id } => cmd_get(&cli, *resource, id.clone()).await,
        Commands::Search { resource, text } => {
            cmd_search(&cli, *resource, text.clone()).await
        }
        Commands::Raw { query, vars } => cmd_raw(&cli, query.clone(), vars.clone()).await,
        Commands::Create { resource, input } => {
            cmd_create(&cli, *resource, input.clone()).await
        }
        Commands::Update { resource, id, set } => {
            cmd_update(&cli, *resource, id.clone(), set.clone()).await
        }
        Commands::Delete { resource, id } => cmd_delete(&cli, *resource, id.clone()).await,
        Commands::Archive { resource, id } => {
            cmd_archive(&cli, *resource, id.clone()).await
        }
        Commands::Unarchive { resource, id } => {
            cmd_unarchive(&cli, *resource, id.clone()).await
        }
        Commands::Mutate { op, vars } => cmd_mutate(&cli, *op, vars.clone()).await,
        Commands::Schema { action } => cmd_schema(&cli, action.clone()).await,
    };

    // Handle result
    match result {
        Ok(()) => ExitCode::Success.into(),
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
                    (ExitCode::GeneralError, "general", format!("{:#}", e), None, None, None)
                };

            let error_info = render::ErrorInfo {
                kind,
                message: &message,
                hint: hint.as_deref(),
                details: details.as_ref(),
                graphql_errors: graphql_errors.as_ref(),
            };
            eprintln!("{}", render::render_error(cli.global.output, &error_info, cli.global.pretty));
            exit_code.into()
        }
    }
}
