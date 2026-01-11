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
mod generated;
mod mutation_builder;
mod query_builder;
mod render;
mod validate;

use cli::{Cli, Commands};

/// Exit codes for the CLI
/// - 0: Success
/// - 1: General/unknown error
/// - 2: Auth error (missing/invalid LINEAR_API_KEY)
/// - 3: Network error (connection failed, timeout)
/// - 4: GraphQL error (valid request, Linear returned errors)
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
        // These commands don't require API access
        Commands::Resources | Commands::Ops | Commands::Schema { .. } => false,
        // All other commands require API access
        _ => true,
    }
}

/// Get the API key from environment, returning error message if missing or empty
fn get_api_key() -> Result<String, String> {
    match std::env::var("LINEAR_API_KEY") {
        Ok(key) if key.trim().is_empty() => Err(
            "LINEAR_API_KEY environment variable is empty.\n\
             \n\
             To use this command, set your Linear API key:\n\
             \n\
               export LINEAR_API_KEY='lin_api_...'\n\
             \n\
             Get your API key from: https://linear.app/settings/api"
                .to_string(),
        ),
        Ok(key) => Ok(key),
        Err(_) => Err(
            "Missing LINEAR_API_KEY environment variable.\n\
             \n\
             To use this command, set your Linear API key:\n\
             \n\
               export LINEAR_API_KEY='lin_api_...'\n\
             \n\
             Get your API key from: https://linear.app/settings/api"
                .to_string(),
        ),
    }
}

#[tokio::main]
async fn main() -> ExitCode {
    // Load .env if present
    let _ = dotenvy::dotenv();

    let cli = Cli::parse();

    // Check for API key if command requires it
    if command_requires_api(&cli.command) {
        if let Err(msg) = get_api_key() {
            eprintln!("Error: {}", msg);
            return ExitCode::from(exit_codes::AUTH_ERROR);
        }
    }

    // Handle commands
    let result: anyhow::Result<()> = match &cli.command {
        Commands::Resources => cmd_resources(&cli),
        Commands::Ops => cmd_ops(&cli),
        Commands::List { resource, options } => cmd_list(&cli, resource.clone(), options.clone()).await,
        Commands::Get { resource, id } => cmd_get(&cli, resource.clone(), id.clone()).await,
        Commands::Search { resource, text } => cmd_search(&cli, resource.clone(), text.clone()).await,
        Commands::Raw { query } => cmd_raw(&cli, query.clone()).await,
        Commands::Create { resource, input } => cmd_create(&cli, resource.clone(), input.clone()).await,
        Commands::Update { resource, id, set } => cmd_update(&cli, resource.clone(), id.clone(), set.clone()).await,
        Commands::Delete { resource, id } => cmd_delete(&cli, resource.clone(), id.clone()).await,
        Commands::Archive { resource, id } => cmd_archive(&cli, resource.clone(), id.clone()).await,
        Commands::Unarchive { resource, id } => cmd_unarchive(&cli, resource.clone(), id.clone()).await,
        Commands::Mutate { op, vars } => cmd_mutate(&cli, op.clone(), vars.clone()).await,
        Commands::Schema { action } => cmd_schema(&cli, action.clone()),
    };

    match result {
        Ok(()) => ExitCode::from(exit_codes::SUCCESS),
        Err(e) => {
            // Check if the error is a ClientError to get the right exit code
            let exit_code = if let Some(client_err) = e.downcast_ref::<client::ClientError>() {
                eprintln!("Error: {}", client_err);
                client_err.exit_code()
            } else {
                eprintln!("Error: {:#}", e);
                exit_codes::GENERAL_ERROR
            };
            ExitCode::from(exit_code)
        }
    }
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
    cli: &Cli,
    resource: generated::Resource,
    options: cli::ListOptions,
) -> Result<()> {
    use client::{Client, GraphQLRequest};
    use query_builder::build_list_query;

    // Get API key (already validated in main)
    let api_key = get_api_key().expect("API key already validated");

    // Create client
    let client = Client::new(
        &api_key,
        cli.global.endpoint.as_deref(),
        cli.global.timeout,
    )?;

    // Build the query
    let (query, variables) = build_list_query(resource, &options);

    if cli.global.verbose {
        eprintln!("Query: {}", query);
        eprintln!("Variables: {}", serde_json::to_string_pretty(&variables)?);
    }

    // Execute the query
    let request = GraphQLRequest {
        query,
        variables: Some(variables),
        operation_name: None,
    };

    let response = client.execute(request).await?;

    // Render the response
    match cli.global.output {
        cli::OutputFormat::Json => {
            if cli.global.pretty {
                println!("{}", serde_json::to_string_pretty(&response.data)?);
            } else {
                println!("{}", serde_json::to_string(&response.data)?);
            }
        }
        cli::OutputFormat::Yaml => {
            println!("{}", serde_yaml::to_string(&response.data)?);
        }
        _ => {
            // For table/text/ndjson, just print JSON for now
            // TODO: Implement proper table rendering
            println!("{}", serde_json::to_string_pretty(&response.data)?);
        }
    }

    Ok(())
}

async fn cmd_get(cli: &Cli, resource: generated::Resource, id: String) -> Result<()> {
    use client::{Client, GraphQLRequest};
    use query_builder::build_get_query;

    // Get API key (already validated in main)
    let api_key = get_api_key().expect("API key already validated");

    // Create client
    let client = Client::new(
        &api_key,
        cli.global.endpoint.as_deref(),
        cli.global.timeout,
    )?;

    // Build the query
    let (query, variables) = build_get_query(resource, &id);

    if cli.global.verbose {
        eprintln!("Query: {}", query);
        eprintln!("Variables: {}", serde_json::to_string_pretty(&variables)?);
    }

    // Execute the query
    let request = GraphQLRequest {
        query,
        variables: Some(variables),
        operation_name: None,
    };

    let response = client.execute(request).await?;

    // Render the response
    match cli.global.output {
        cli::OutputFormat::Json => {
            if cli.global.pretty {
                println!("{}", serde_json::to_string_pretty(&response.data)?);
            } else {
                println!("{}", serde_json::to_string(&response.data)?);
            }
        }
        cli::OutputFormat::Yaml => {
            println!("{}", serde_yaml::to_string(&response.data)?);
        }
        _ => {
            // For table/text/ndjson, just print JSON for now
            println!("{}", serde_json::to_string_pretty(&response.data)?);
        }
    }

    Ok(())
}

async fn cmd_search(cli: &Cli, resource: generated::Resource, text: String) -> Result<()> {
    use client::{Client, GraphQLRequest};
    use query_builder::build_search_query;

    // Get API key (already validated in main)
    let api_key = get_api_key().expect("API key already validated");

    // Create client
    let client = Client::new(
        &api_key,
        cli.global.endpoint.as_deref(),
        cli.global.timeout,
    )?;

    // Build the search query
    let (query, variables, strategy) = build_search_query(resource, &text);

    if cli.global.verbose {
        eprintln!("Search strategy: {:?}", strategy);
        eprintln!("Query: {}", query);
        eprintln!("Variables: {}", serde_json::to_string_pretty(&variables)?);
    }

    // Execute the query
    let request = GraphQLRequest {
        query,
        variables: Some(variables),
        operation_name: None,
    };

    let response = client.execute(request).await?;

    // Render the response
    match cli.global.output {
        cli::OutputFormat::Json => {
            if cli.global.pretty {
                println!("{}", serde_json::to_string_pretty(&response.data)?);
            } else {
                println!("{}", serde_json::to_string(&response.data)?);
            }
        }
        cli::OutputFormat::Yaml => {
            println!("{}", serde_yaml::to_string(&response.data)?);
        }
        _ => {
            // For table/text/ndjson, just print JSON for now
            println!("{}", serde_json::to_string_pretty(&response.data)?);
        }
    }

    Ok(())
}

async fn cmd_raw(_cli: &Cli, _query: String) -> Result<()> {
    // TODO: Implement raw command
    anyhow::bail!("Raw command not yet implemented")
}

async fn cmd_create(
    cli: &Cli,
    resource: generated::Resource,
    input: cli::InputOptions,
) -> Result<()> {
    use client::{Client, GraphQLRequest};
    use mutation_builder::build_create_mutation;
    use validate::resolve_input;

    // Get API key (already validated in main)
    let api_key = get_api_key().expect("API key already validated");

    // Create client
    let client = Client::new(
        &api_key,
        cli.global.endpoint.as_deref(),
        cli.global.timeout,
    )?;

    // Parse the input
    let input_value = resolve_input(input.input.as_deref(), input.input_file.as_deref())?;

    // Build the mutation
    let (query, variables) = build_create_mutation(resource.field_name(), input_value);

    if cli.global.verbose {
        eprintln!("Query: {}", query);
        eprintln!("Variables: {}", serde_json::to_string_pretty(&variables)?);
    }

    // Execute the mutation
    let request = GraphQLRequest {
        query,
        variables: Some(variables),
        operation_name: None,
    };

    let response = client.execute(request).await?;

    // Render the response
    match cli.global.output {
        cli::OutputFormat::Json => {
            if cli.global.pretty {
                println!("{}", serde_json::to_string_pretty(&response.data)?);
            } else {
                println!("{}", serde_json::to_string(&response.data)?);
            }
        }
        cli::OutputFormat::Yaml => {
            println!("{}", serde_yaml::to_string(&response.data)?);
        }
        _ => {
            println!("{}", serde_json::to_string_pretty(&response.data)?);
        }
    }

    Ok(())
}

async fn cmd_update(
    cli: &Cli,
    resource: generated::Resource,
    id: String,
    set: cli::SetOptions,
) -> Result<()> {
    use client::{Client, GraphQLRequest};
    use mutation_builder::build_update_mutation;
    use validate::resolve_input;

    // Get API key (already validated in main)
    let api_key = get_api_key().expect("API key already validated");

    // Create client
    let client = Client::new(
        &api_key,
        cli.global.endpoint.as_deref(),
        cli.global.timeout,
    )?;

    // Parse the input
    let input_value = resolve_input(set.set.as_deref(), set.set_file.as_deref())?;

    // Build the mutation
    let (query, variables) = build_update_mutation(resource.field_name(), &id, input_value);

    if cli.global.verbose {
        eprintln!("Query: {}", query);
        eprintln!("Variables: {}", serde_json::to_string_pretty(&variables)?);
    }

    // Execute the mutation
    let request = GraphQLRequest {
        query,
        variables: Some(variables),
        operation_name: None,
    };

    let response = client.execute(request).await?;

    // Render the response
    match cli.global.output {
        cli::OutputFormat::Json => {
            if cli.global.pretty {
                println!("{}", serde_json::to_string_pretty(&response.data)?);
            } else {
                println!("{}", serde_json::to_string(&response.data)?);
            }
        }
        cli::OutputFormat::Yaml => {
            println!("{}", serde_yaml::to_string(&response.data)?);
        }
        _ => {
            println!("{}", serde_json::to_string_pretty(&response.data)?);
        }
    }

    Ok(())
}

async fn cmd_delete(cli: &Cli, resource: generated::Resource, id: String) -> Result<()> {
    use client::{Client, GraphQLRequest};
    use mutation_builder::build_delete_mutation;

    // Get API key (already validated in main)
    let api_key = get_api_key().expect("API key already validated");

    // Create client
    let client = Client::new(
        &api_key,
        cli.global.endpoint.as_deref(),
        cli.global.timeout,
    )?;

    // Build the mutation
    let (query, variables) = build_delete_mutation(resource.field_name(), &id);

    if cli.global.verbose {
        eprintln!("Query: {}", query);
        eprintln!("Variables: {}", serde_json::to_string_pretty(&variables)?);
    }

    // Execute the mutation
    let request = GraphQLRequest {
        query,
        variables: Some(variables),
        operation_name: None,
    };

    let response = client.execute(request).await?;

    // Render the response
    match cli.global.output {
        cli::OutputFormat::Json => {
            if cli.global.pretty {
                println!("{}", serde_json::to_string_pretty(&response.data)?);
            } else {
                println!("{}", serde_json::to_string(&response.data)?);
            }
        }
        cli::OutputFormat::Yaml => {
            println!("{}", serde_yaml::to_string(&response.data)?);
        }
        _ => {
            println!("{}", serde_json::to_string_pretty(&response.data)?);
        }
    }

    Ok(())
}

async fn cmd_archive(cli: &Cli, resource: generated::Resource, id: String) -> Result<()> {
    use client::{Client, GraphQLRequest};
    use mutation_builder::build_archive_mutation;

    // Get API key (already validated in main)
    let api_key = get_api_key().expect("API key already validated");

    // Create client
    let client = Client::new(
        &api_key,
        cli.global.endpoint.as_deref(),
        cli.global.timeout,
    )?;

    // Build the mutation
    let (query, variables) = build_archive_mutation(resource.field_name(), &id);

    if cli.global.verbose {
        eprintln!("Query: {}", query);
        eprintln!("Variables: {}", serde_json::to_string_pretty(&variables)?);
    }

    // Execute the mutation
    let request = GraphQLRequest {
        query,
        variables: Some(variables),
        operation_name: None,
    };

    let response = client.execute(request).await?;

    // Render the response
    match cli.global.output {
        cli::OutputFormat::Json => {
            if cli.global.pretty {
                println!("{}", serde_json::to_string_pretty(&response.data)?);
            } else {
                println!("{}", serde_json::to_string(&response.data)?);
            }
        }
        cli::OutputFormat::Yaml => {
            println!("{}", serde_yaml::to_string(&response.data)?);
        }
        _ => {
            println!("{}", serde_json::to_string_pretty(&response.data)?);
        }
    }

    Ok(())
}

async fn cmd_unarchive(cli: &Cli, resource: generated::Resource, id: String) -> Result<()> {
    use client::{Client, GraphQLRequest};
    use mutation_builder::build_unarchive_mutation;

    // Get API key (already validated in main)
    let api_key = get_api_key().expect("API key already validated");

    // Create client
    let client = Client::new(
        &api_key,
        cli.global.endpoint.as_deref(),
        cli.global.timeout,
    )?;

    // Build the mutation
    let (query, variables) = build_unarchive_mutation(resource.field_name(), &id);

    if cli.global.verbose {
        eprintln!("Query: {}", query);
        eprintln!("Variables: {}", serde_json::to_string_pretty(&variables)?);
    }

    // Execute the mutation
    let request = GraphQLRequest {
        query,
        variables: Some(variables),
        operation_name: None,
    };

    let response = client.execute(request).await?;

    // Render the response
    match cli.global.output {
        cli::OutputFormat::Json => {
            if cli.global.pretty {
                println!("{}", serde_json::to_string_pretty(&response.data)?);
            } else {
                println!("{}", serde_json::to_string(&response.data)?);
            }
        }
        cli::OutputFormat::Yaml => {
            println!("{}", serde_yaml::to_string(&response.data)?);
        }
        _ => {
            println!("{}", serde_json::to_string_pretty(&response.data)?);
        }
    }

    Ok(())
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
