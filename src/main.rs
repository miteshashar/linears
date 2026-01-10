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
mod generated;
mod mutation_builder;
mod query_builder;
mod render;
mod validate;

use cli::{Cli, Commands};

#[tokio::main]
async fn main() -> Result<()> {
    // Load .env if present
    let _ = dotenvy::dotenv();

    let cli = Cli::parse();

    // Handle commands
    match cli.command {
        Commands::Resources => {
            cmd_resources(&cli)?;
        }
        Commands::Ops => {
            cmd_ops(&cli)?;
        }
        Commands::List { resource, options } => {
            cmd_list(&cli, resource, options).await?;
        }
        Commands::Get { resource, id } => {
            cmd_get(&cli, resource, id).await?;
        }
        Commands::Search { resource, text } => {
            cmd_search(&cli, resource, text).await?;
        }
        Commands::Raw { query } => {
            cmd_raw(&cli, query).await?;
        }
        Commands::Create { resource, input } => {
            cmd_create(&cli, resource, input).await?;
        }
        Commands::Update { resource, id, set } => {
            cmd_update(&cli, resource, id, set).await?;
        }
        Commands::Delete { resource, id } => {
            cmd_delete(&cli, resource, id).await?;
        }
        Commands::Archive { resource, id } => {
            cmd_archive(&cli, resource, id).await?;
        }
        Commands::Unarchive { resource, id } => {
            cmd_unarchive(&cli, resource, id).await?;
        }
        Commands::Mutate { op, vars } => {
            cmd_mutate(&cli, op, vars).await?;
        }
        Commands::Schema { action } => {
            cmd_schema(&cli, action)?;
        }
    }

    Ok(())
}

fn cmd_resources(cli: &Cli) -> Result<()> {
    use generated::Resource;

    let resources = Resource::all();

    match cli.global.output {
        cli::OutputFormat::Json => {
            let output = serde_json::json!({
                "resources": resources.iter().map(|r| r.field_name()).collect::<Vec<_>>(),
                "count": resources.len()
            });
            if cli.global.pretty {
                println!("{}", serde_json::to_string_pretty(&output)?);
            } else {
                println!("{}", serde_json::to_string(&output)?);
            }
        }
        cli::OutputFormat::Yaml => {
            let output = serde_json::json!({
                "resources": resources.iter().map(|r| r.field_name()).collect::<Vec<_>>(),
                "count": resources.len()
            });
            println!("{}", serde_yaml::to_string(&output)?);
        }
        _ => {
            println!("Available Resources ({}):", resources.len());
            println!();
            for resource in resources {
                println!("  {}", resource.field_name());
            }
        }
    }

    Ok(())
}

fn cmd_ops(cli: &Cli) -> Result<()> {
    use generated::MutationOp;

    let ops = MutationOp::all();

    match cli.global.output {
        cli::OutputFormat::Json => {
            let output = serde_json::json!({
                "operations": ops.iter().map(|o| o.operation_name()).collect::<Vec<_>>(),
                "count": ops.len()
            });
            if cli.global.pretty {
                println!("{}", serde_json::to_string_pretty(&output)?);
            } else {
                println!("{}", serde_json::to_string(&output)?);
            }
        }
        cli::OutputFormat::Yaml => {
            let output = serde_json::json!({
                "operations": ops.iter().map(|o| o.operation_name()).collect::<Vec<_>>(),
                "count": ops.len()
            });
            println!("{}", serde_yaml::to_string(&output)?);
        }
        _ => {
            println!("Available Mutation Operations ({}):", ops.len());
            println!();
            for op in ops {
                println!("  {}", op.operation_name());
            }
        }
    }

    Ok(())
}

async fn cmd_list(
    _cli: &Cli,
    _resource: generated::Resource,
    _options: cli::ListOptions,
) -> Result<()> {
    // TODO: Implement list command
    anyhow::bail!("List command not yet implemented")
}

async fn cmd_get(_cli: &Cli, _resource: generated::Resource, _id: String) -> Result<()> {
    // TODO: Implement get command
    anyhow::bail!("Get command not yet implemented")
}

async fn cmd_search(_cli: &Cli, _resource: generated::Resource, _text: String) -> Result<()> {
    // TODO: Implement search command
    anyhow::bail!("Search command not yet implemented")
}

async fn cmd_raw(_cli: &Cli, _query: String) -> Result<()> {
    // TODO: Implement raw command
    anyhow::bail!("Raw command not yet implemented")
}

async fn cmd_create(
    _cli: &Cli,
    _resource: generated::Resource,
    _input: cli::InputOptions,
) -> Result<()> {
    // TODO: Implement create command
    anyhow::bail!("Create command not yet implemented")
}

async fn cmd_update(
    _cli: &Cli,
    _resource: generated::Resource,
    _id: String,
    _set: cli::SetOptions,
) -> Result<()> {
    // TODO: Implement update command
    anyhow::bail!("Update command not yet implemented")
}

async fn cmd_delete(_cli: &Cli, _resource: generated::Resource, _id: String) -> Result<()> {
    // TODO: Implement delete command
    anyhow::bail!("Delete command not yet implemented")
}

async fn cmd_archive(_cli: &Cli, _resource: generated::Resource, _id: String) -> Result<()> {
    // TODO: Implement archive command
    anyhow::bail!("Archive command not yet implemented")
}

async fn cmd_unarchive(_cli: &Cli, _resource: generated::Resource, _id: String) -> Result<()> {
    // TODO: Implement unarchive command
    anyhow::bail!("Unarchive command not yet implemented")
}

async fn cmd_mutate(
    _cli: &Cli,
    _op: generated::MutationOp,
    _vars: cli::VarsOptions,
) -> Result<()> {
    // TODO: Implement mutate command
    anyhow::bail!("Mutate command not yet implemented")
}

fn cmd_schema(_cli: &Cli, action: cli::SchemaAction) -> Result<()> {
    match action {
        cli::SchemaAction::Info => {
            let meta_path =
                std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("schemas/linear/schema.meta.json");

            if meta_path.exists() {
                let content = std::fs::read_to_string(&meta_path)?;
                let meta: serde_json::Value = serde_json::from_str(&content)?;
                println!("Schema Information:");
                println!(
                    "  Source: {}",
                    meta["source"].as_str().unwrap_or("unknown")
                );
                println!(
                    "  Synced At: {}",
                    meta["syncedAt"].as_str().unwrap_or("unknown")
                );
            } else {
                println!("No schema metadata found.");
                println!("Run 'cargo xtask schema sync' to sync the schema.");
            }
        }
        cli::SchemaAction::Diff => {
            // TODO: Implement schema diff
            println!("Schema diff not yet implemented");
            println!("Run 'cargo xtask schema sync' to check for updates");
        }
        cli::SchemaAction::Sync => {
            println!("Schema sync requires maintainer access.");
            println!("Run: cargo xtask schema sync");
        }
    }

    Ok(())
}
