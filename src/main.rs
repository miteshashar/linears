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
mod progress;
mod query_builder;
mod render;
mod schema_diff;
mod validate;

use cli::{Cli, Commands};

/// Exit codes for the CLI
/// - 0: Success
/// - 1: General/unknown error
/// - 2: Auth error (missing/invalid LINEARS_API_KEY)
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
    match std::env::var("LINEARS_API_KEY") {
        Ok(key) if key.trim().is_empty() => Err(
            "LINEARS_API_KEY environment variable is empty.\n\
             \n\
             To use this command, set your Linear API key:\n\
             \n\
               export LINEARS_API_KEY='lin_api_...'\n\
             \n\
             Get your API key from: https://linear.app/settings/api"
                .to_string(),
        ),
        Ok(key) => Ok(key),
        Err(_) => Err(
            "Missing LINEARS_API_KEY environment variable.\n\
             \n\
             To use this command, set your Linear API key:\n\
             \n\
               export LINEARS_API_KEY='lin_api_...'\n\
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

    // Set up color configuration
    progress::set_no_color(cli.global.no_color);

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
        Commands::Raw { query, vars } => cmd_raw(&cli, query.clone(), vars.clone()).await,
        Commands::Create { resource, input } => cmd_create(&cli, resource.clone(), input.clone()).await,
        Commands::Update { resource, id, set } => cmd_update(&cli, resource.clone(), id.clone(), set.clone()).await,
        Commands::Delete { resource, id } => cmd_delete(&cli, resource.clone(), id.clone()).await,
        Commands::Archive { resource, id } => cmd_archive(&cli, resource.clone(), id.clone()).await,
        Commands::Unarchive { resource, id } => cmd_unarchive(&cli, resource.clone(), id.clone()).await,
        Commands::Mutate { op, vars } => cmd_mutate(&cli, op.clone(), vars.clone()).await,
        Commands::Schema { action } => cmd_schema(&cli, action.clone()).await,
    };

    match result {
        Ok(()) => ExitCode::from(exit_codes::SUCCESS),
        Err(e) => {
            // Check if the error is a ClientError to get the right exit code
            let (exit_code, error_kind, error_message, hint, graphql_errors, details) =
                if let Some(client_err) = e.downcast_ref::<client::ClientError>() {
                    let kind = match client_err {
                        client::ClientError::Auth(_) => "auth",
                        client::ClientError::Network(_) => "network",
                        client::ClientError::GraphQL(_, _) => "graphql",
                        client::ClientError::RateLimited(_) => "rate_limited",
                        client::ClientError::RateLimitedTooLong(_) => "rate_limited",
                        client::ClientError::Server(_) => "server",
                        client::ClientError::Other(_) => "other",
                    };
                    let hint = client_err.hint();

                    // Extract GraphQL errors and details if present
                    let (graphql_errors, details) = match client_err {
                        client::ClientError::GraphQL(_, Some(errors)) => {
                            // Extract details from first error's extensions if present
                            let details = errors
                                .first()
                                .and_then(|e| e.extensions.clone());
                            (Some(errors.clone()), details)
                        }
                        _ => (None, None),
                    };

                    (client_err.exit_code(), kind, format!("{}", client_err), hint, graphql_errors, details)
                } else {
                    (exit_codes::GENERAL_ERROR, "general", format!("{:#}", e), None, None, None)
                };

            // Render error using render module
            let error_info = render::ErrorInfo {
                kind: error_kind,
                message: &error_message,
                hint: hint.as_deref(),
                details: details.as_ref(),
                graphql_errors: graphql_errors.as_ref(),
            };
            eprintln!(
                "{}",
                render::render_error(cli.global.output, &error_info, cli.global.pretty)
            );
            ExitCode::from(exit_code)
        }
    }
}

fn cmd_resources(cli: &Cli) -> Result<()> {
    use generated::Resource;

    let resources = Resource::all();
    let resource_names: Vec<&str> = resources.iter().map(|r| r.field_name()).collect();

    println!(
        "{}",
        render::render_resources(cli.global.output, &resource_names, cli.global.pretty)
    );

    Ok(())
}

fn cmd_ops(cli: &Cli) -> Result<()> {
    use generated::MutationOp;

    let ops = MutationOp::all();
    let op_names: Vec<&str> = ops.iter().map(|o| o.operation_name()).collect();

    println!(
        "{}",
        render::render_ops(cli.global.output, &op_names, cli.global.pretty)
    );

    Ok(())
}

async fn cmd_list(
    cli: &Cli,
    resource: generated::Resource,
    options: cli::ListOptions,
) -> Result<()> {
    use client::{Client, GraphQLRequest};
    use progress::with_spinner;
    use query_builder::build_list_query;

    // Get API key (already validated in main)
    let api_key = get_api_key().expect("API key already validated");

    // Validate filter keys if a filter is provided
    if let Some(ref filter_str) = options.filter {
        if filter_str != "-" {
            // Parse and validate the filter
            if let Ok(filter_value) = validate::parse_input(filter_str) {
                if let Err(errors) = generated::validate_filter_keys(resource, &filter_value) {
                    let resource_name = resource.field_name();
                    for (key, suggestion) in errors {
                        let suggestion_msg = suggestion
                            .map(|s| format!(". Did you mean: {}?", s))
                            .unwrap_or_default();
                        eprintln!(
                            "error: Unknown filter key '{}' for {}{}",
                            key, resource_name, suggestion_msg
                        );
                    }
                    anyhow::bail!("Invalid filter keys");
                }
            }
        }
    }

    // Create client
    let client = Client::new(
        &api_key,
        cli.global.endpoint.as_deref(),
        cli.global.timeout,
        cli.global.workspace.as_deref(),
    )?;

    let resource_name = resource.field_name();
    // Use schema-derived plural name (avoids naive pluralization bugs)
    let plural_name = resource.plural_name();

    // If --all is specified, auto-paginate
    let (nodes, page_info) = if options.all {
        const MAX_RECORDS: usize = 1000;
        const PAGE_SIZE: i32 = 50;

        let mut all_nodes: Vec<serde_json::Value> = Vec::new();
        let mut cursor: Option<String> = None;
        let mut final_page_info: Option<serde_json::Value> = None;

        loop {
            // Build query with current cursor
            let mut page_options = options.clone();
            page_options.first = Some(PAGE_SIZE);
            page_options.after = cursor.clone();
            page_options.all = false; // Prevent infinite recursion

            let (query, variables) = build_list_query(resource, &page_options);

            if cli.global.verbose {
                eprintln!("Query: {}", query);
                eprintln!("Variables: {}", serde_json::to_string_pretty(&variables)?);
            }

            let request = GraphQLRequest {
                query,
                variables: Some(variables),
                operation_name: None,
            };

            let page_count = all_nodes.len() / PAGE_SIZE as usize + 1;
            let response = with_spinner(
                &format!("Fetching {} (page {})...", resource_name, page_count),
                client.execute(request),
            )
            .await?;

            let data = response.data.unwrap_or_default();
            let resource_data = &data[&plural_name];

            // Extract nodes from this page
            if let Some(nodes_arr) = resource_data.get("nodes").and_then(|n| n.as_array()) {
                all_nodes.extend(nodes_arr.iter().cloned());
            }

            // Check pagination info
            let has_next = resource_data
                .get("pageInfo")
                .and_then(|p| p.get("hasNextPage"))
                .and_then(|h| h.as_bool())
                .unwrap_or(false);

            let end_cursor = resource_data
                .get("pageInfo")
                .and_then(|p| p.get("endCursor"))
                .and_then(|c| c.as_str())
                .map(String::from);

            final_page_info = resource_data.get("pageInfo").cloned();

            // Stop if no more pages or max records reached
            if !has_next || end_cursor.is_none() || all_nodes.len() >= MAX_RECORDS {
                break;
            }

            cursor = end_cursor;
        }

        // Truncate to max if we exceeded
        if all_nodes.len() > MAX_RECORDS {
            all_nodes.truncate(MAX_RECORDS);
        }

        (serde_json::Value::Array(all_nodes), final_page_info)
    } else {
        // Single page fetch
        let (query, variables) = build_list_query(resource, &options);

        if cli.global.verbose {
            eprintln!("Query: {}", query);
            eprintln!("Variables: {}", serde_json::to_string_pretty(&variables)?);
        }

        let request = GraphQLRequest {
            query,
            variables: Some(variables),
            operation_name: None,
        };

        let response = with_spinner(
            &format!("Fetching {}...", resource_name),
            client.execute(request),
        )
        .await?;

        let data = response.data.unwrap_or_default();
        let resource_data = &data[&plural_name];
        let nodes = resource_data.get("nodes").cloned().unwrap_or_default();
        let page_info = resource_data.get("pageInfo").cloned();

        (nodes, page_info)
    };

    // Render the response using render module
    println!(
        "{}",
        render::render_list_json(
            cli.global.output,
            resource_name,
            &nodes,
            page_info.as_ref(),
            cli.global.pretty
        )
    );

    Ok(())
}

async fn cmd_get(cli: &Cli, resource: generated::Resource, id: String) -> Result<()> {
    use client::{Client, GraphQLRequest};
    use progress::with_spinner;
    use query_builder::build_get_query;

    // Get API key (already validated in main)
    let api_key = get_api_key().expect("API key already validated");

    // Create client
    let client = Client::new(
        &api_key,
        cli.global.endpoint.as_deref(),
        cli.global.timeout,
        cli.global.workspace.as_deref(),
    )?;

    // Build the query
    let (query, variables) = build_get_query(resource, &id);

    if cli.global.verbose {
        eprintln!("Query: {}", query);
        eprintln!("Variables: {}", serde_json::to_string_pretty(&variables)?);
    }

    // Execute the query with spinner
    let request = GraphQLRequest {
        query,
        variables: Some(variables),
        operation_name: None,
    };

    let response = with_spinner(
        &format!("Fetching {}...", resource.field_name()),
        client.execute(request),
    )
    .await?;

    // Extract entity from response
    let data = response.data.unwrap_or_default();
    let resource_name = resource.field_name();
    let entity = data.get(resource_name).cloned().unwrap_or_default();

    // Render the response using render module
    println!(
        "{}",
        render::render_entity_json(cli.global.output, resource_name, &entity, cli.global.pretty)
    );

    Ok(())
}

async fn cmd_search(cli: &Cli, resource: generated::Resource, text: String) -> Result<()> {
    use client::{Client, GraphQLRequest};
    use progress::with_spinner;
    use query_builder::build_search_query;

    // Get API key (already validated in main)
    let api_key = get_api_key().expect("API key already validated");

    // Create client
    let client = Client::new(
        &api_key,
        cli.global.endpoint.as_deref(),
        cli.global.timeout,
        cli.global.workspace.as_deref(),
    )?;

    // Build the search query
    let (query, variables, strategy) = build_search_query(resource, &text);

    if cli.global.verbose {
        eprintln!("Search strategy: {:?}", strategy);
        eprintln!("Query: {}", query);
        eprintln!("Variables: {}", serde_json::to_string_pretty(&variables)?);
    }

    // Execute the query with spinner
    let request = GraphQLRequest {
        query,
        variables: Some(variables),
        operation_name: None,
    };

    let response = with_spinner(
        &format!("Searching {}...", resource.field_name()),
        client.execute(request),
    )
    .await?;

    // Extract nodes from response
    let data = response.data.unwrap_or_default();
    let resource_name = resource.field_name();
    // Use schema-derived plural name (avoids naive pluralization bugs)
    let plural_name = resource.plural_name();
    let resource_data = &data[plural_name];
    let nodes = resource_data.get("nodes").cloned().unwrap_or_default();

    // Render the response using render module
    println!(
        "{}",
        render::render_search_json(
            cli.global.output,
            resource_name,
            strategy.as_str(),
            &nodes,
            cli.global.pretty
        )
    );

    Ok(())
}

async fn cmd_raw(cli: &Cli, query: String, vars: cli::VarsOptions) -> Result<()> {
    use client::{Client, GraphQLRequest};
    use progress::with_spinner;
    use validate::resolve_input;

    // Get API key (already validated in main)
    let api_key = get_api_key().expect("API key already validated");

    // Create client
    let client = Client::new(
        &api_key,
        cli.global.endpoint.as_deref(),
        cli.global.timeout,
        cli.global.workspace.as_deref(),
    )?;

    // Read query from file if it looks like a file path
    let query_text = if std::path::Path::new(&query).exists() {
        std::fs::read_to_string(&query)?
    } else {
        query
    };

    // Parse variables (if any provided)
    let mut variables = resolve_input(vars.vars.as_deref(), vars.vars_file.as_deref())
        .unwrap_or_else(|_| serde_json::json!({}));

    // Apply individual variable overrides
    if let Some(var_overrides) = vars.var {
        if let Some(obj) = variables.as_object_mut() {
            for override_str in var_overrides {
                if let Some((key, value)) = override_str.split_once('=') {
                    // Try to parse as JSON, fallback to string
                    let parsed_value = serde_json::from_str(value)
                        .unwrap_or_else(|_| serde_json::Value::String(value.to_string()));
                    obj.insert(key.to_string(), parsed_value);
                }
            }
        }
    }

    // Only pass variables if they're not empty
    let variables_for_request = if variables.as_object().map(|o| o.is_empty()).unwrap_or(true) {
        None
    } else {
        Some(variables.clone())
    };

    if cli.global.verbose {
        eprintln!("Query: {}", query_text);
        if variables_for_request.is_some() {
            eprintln!("Variables: {}", serde_json::to_string_pretty(&variables)?);
        }
    }

    // Execute the query with spinner
    let request = GraphQLRequest {
        query: query_text,
        variables: variables_for_request,
        operation_name: None,
    };

    let response = with_spinner("Executing query...", client.execute(request)).await?;

    // Render the response using render module
    println!(
        "{}",
        render::render_raw(cli.global.output, &response.data, cli.global.pretty)
    );

    Ok(())
}

async fn cmd_create(
    cli: &Cli,
    resource: generated::Resource,
    input: cli::InputOptions,
) -> Result<()> {
    use client::{Client, GraphQLRequest};
    use mutation_builder::build_create_mutation;
    use progress::with_spinner;
    use validate::resolve_input;

    // Get API key (already validated in main)
    let api_key = get_api_key().expect("API key already validated");

    // Create client
    let client = Client::new(
        &api_key,
        cli.global.endpoint.as_deref(),
        cli.global.timeout,
        cli.global.workspace.as_deref(),
    )?;

    // Parse the input
    let input_value = resolve_input(input.input.as_deref(), input.input_file.as_deref())?;

    // Build the mutation
    let (query, variables) = build_create_mutation(resource.field_name(), input_value);

    if cli.global.verbose {
        eprintln!("Query: {}", query);
        eprintln!("Variables: {}", serde_json::to_string_pretty(&variables)?);
    }

    // Execute the mutation with spinner
    let request = GraphQLRequest {
        query,
        variables: Some(variables),
        operation_name: None,
    };

    let response = with_spinner(
        &format!("Creating {}...", resource.field_name()),
        client.execute(request),
    )
    .await?;

    // Render the response using render module
    println!(
        "{}",
        render::render_helper_mutation(cli.global.output, &response.data, cli.global.pretty)
    );

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
    use progress::with_spinner;
    use validate::resolve_input;

    // Get API key (already validated in main)
    let api_key = get_api_key().expect("API key already validated");

    // Create client
    let client = Client::new(
        &api_key,
        cli.global.endpoint.as_deref(),
        cli.global.timeout,
        cli.global.workspace.as_deref(),
    )?;

    // Parse the input
    let input_value = resolve_input(set.set.as_deref(), set.set_file.as_deref())?;

    // Build the mutation
    let (query, variables) = build_update_mutation(resource.field_name(), &id, input_value);

    if cli.global.verbose {
        eprintln!("Query: {}", query);
        eprintln!("Variables: {}", serde_json::to_string_pretty(&variables)?);
    }

    // Execute the mutation with spinner
    let request = GraphQLRequest {
        query,
        variables: Some(variables),
        operation_name: None,
    };

    let response = with_spinner(
        &format!("Updating {}...", resource.field_name()),
        client.execute(request),
    )
    .await?;

    // Render the response using render module
    println!(
        "{}",
        render::render_helper_mutation(cli.global.output, &response.data, cli.global.pretty)
    );

    Ok(())
}

async fn cmd_delete(cli: &Cli, resource: generated::Resource, id: String) -> Result<()> {
    use client::{Client, GraphQLRequest};
    use mutation_builder::build_delete_mutation;
    use progress::with_spinner;

    // Get API key (already validated in main)
    let api_key = get_api_key().expect("API key already validated");

    // Create client
    let client = Client::new(
        &api_key,
        cli.global.endpoint.as_deref(),
        cli.global.timeout,
        cli.global.workspace.as_deref(),
    )?;

    // Build the mutation
    let (query, variables) = build_delete_mutation(resource.field_name(), &id);

    if cli.global.verbose {
        eprintln!("Query: {}", query);
        eprintln!("Variables: {}", serde_json::to_string_pretty(&variables)?);
    }

    // Execute the mutation with spinner
    let request = GraphQLRequest {
        query,
        variables: Some(variables),
        operation_name: None,
    };

    let response = with_spinner(
        &format!("Deleting {}...", resource.field_name()),
        client.execute(request),
    )
    .await?;

    // Render the response using render module
    println!(
        "{}",
        render::render_helper_mutation(cli.global.output, &response.data, cli.global.pretty)
    );

    Ok(())
}

async fn cmd_archive(cli: &Cli, resource: generated::Resource, id: String) -> Result<()> {
    use client::{Client, GraphQLRequest};
    use mutation_builder::build_archive_mutation;
    use progress::with_spinner;

    // Get API key (already validated in main)
    let api_key = get_api_key().expect("API key already validated");

    // Create client
    let client = Client::new(
        &api_key,
        cli.global.endpoint.as_deref(),
        cli.global.timeout,
        cli.global.workspace.as_deref(),
    )?;

    // Build the mutation
    let (query, variables) = build_archive_mutation(resource.field_name(), &id);

    if cli.global.verbose {
        eprintln!("Query: {}", query);
        eprintln!("Variables: {}", serde_json::to_string_pretty(&variables)?);
    }

    // Execute the mutation with spinner
    let request = GraphQLRequest {
        query,
        variables: Some(variables),
        operation_name: None,
    };

    let response = with_spinner(
        &format!("Archiving {}...", resource.field_name()),
        client.execute(request),
    )
    .await?;

    // Render the response using render module
    println!(
        "{}",
        render::render_helper_mutation(cli.global.output, &response.data, cli.global.pretty)
    );

    Ok(())
}

async fn cmd_unarchive(cli: &Cli, resource: generated::Resource, id: String) -> Result<()> {
    use client::{Client, GraphQLRequest};
    use mutation_builder::build_unarchive_mutation;
    use progress::with_spinner;

    // Get API key (already validated in main)
    let api_key = get_api_key().expect("API key already validated");

    // Create client
    let client = Client::new(
        &api_key,
        cli.global.endpoint.as_deref(),
        cli.global.timeout,
        cli.global.workspace.as_deref(),
    )?;

    // Build the mutation
    let (query, variables) = build_unarchive_mutation(resource.field_name(), &id);

    if cli.global.verbose {
        eprintln!("Query: {}", query);
        eprintln!("Variables: {}", serde_json::to_string_pretty(&variables)?);
    }

    // Execute the mutation with spinner
    let request = GraphQLRequest {
        query,
        variables: Some(variables),
        operation_name: None,
    };

    let response = with_spinner(
        &format!("Unarchiving {}...", resource.field_name()),
        client.execute(request),
    )
    .await?;

    // Render the response using render module
    println!(
        "{}",
        render::render_helper_mutation(cli.global.output, &response.data, cli.global.pretty)
    );

    Ok(())
}

async fn cmd_mutate(
    cli: &Cli,
    op: generated::MutationOp,
    vars: cli::VarsOptions,
) -> Result<()> {
    use client::{Client, GraphQLRequest};
    use mutation_builder::build_mutation;
    use progress::with_spinner;
    use validate::resolve_input;

    // Get API key (already validated in main)
    let api_key = get_api_key().expect("API key already validated");

    // Create client
    let client = Client::new(
        &api_key,
        cli.global.endpoint.as_deref(),
        cli.global.timeout,
        cli.global.workspace.as_deref(),
    )?;

    // Parse variables
    let mut variables = resolve_input(vars.vars.as_deref(), vars.vars_file.as_deref())
        .unwrap_or_else(|_| serde_json::json!({}));

    // Apply individual variable overrides
    if let Some(var_overrides) = vars.var {
        if let Some(obj) = variables.as_object_mut() {
            for override_str in var_overrides {
                if let Some((key, value)) = override_str.split_once('=') {
                    // Try to parse as JSON, fallback to string
                    let parsed_value = serde_json::from_str(value)
                        .unwrap_or_else(|_| serde_json::Value::String(value.to_string()));
                    obj.insert(key.to_string(), parsed_value);
                }
            }
        }
    }

    // Build the mutation
    let (query, _) = build_mutation(op.clone(), variables.clone());

    if cli.global.verbose {
        eprintln!("Query: {}", query);
        eprintln!("Variables: {}", serde_json::to_string_pretty(&variables)?);
    }

    // Execute the mutation with spinner
    let request = GraphQLRequest {
        query,
        variables: Some(variables),
        operation_name: None,
    };

    let response = with_spinner(
        &format!("Executing {}...", op.operation_name()),
        client.execute(request),
    )
    .await?;

    // Extract mutation result
    let data = response.data.unwrap_or_default();
    let op_name = op.operation_name();
    let result = data.get(op_name).cloned().unwrap_or_default();

    // Render the response using render module
    println!(
        "{}",
        render::render_mutation_json(cli.global.output, op_name, &result, cli.global.pretty)
    );

    Ok(())
}

async fn cmd_schema(_cli: &Cli, action: cli::SchemaAction) -> Result<()> {
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
                    "  Commit: {}",
                    meta["commit"].as_str().unwrap_or("unknown")
                );
                println!(
                    "  Synced At: {}",
                    meta["syncedAt"].as_str().unwrap_or("unknown")
                );
                // Build GitHub permalink if commit is available
                if let (Some(source), Some(commit)) = (meta["source"].as_str(), meta["commit"].as_str()) {
                    if source.contains("github.com") {
                        println!("  Permalink: {}/tree/{}", source, commit);
                    }
                }
            } else {
                println!("No schema metadata found.");
                println!("Run 'cargo xtask schema sync' to sync the schema.");
            }
        }
        cli::SchemaAction::Diff => {
            use std::path::PathBuf;

            // Path to local schema
            let schema_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("schemas")
                .join("linear")
                .join("schema.graphql");

            if !schema_path.exists() {
                println!("No local schema found.");
                println!("Run 'cargo xtask schema sync' to sync the schema first.");
                return Ok(());
            }

            println!("Fetching upstream schema from Linear SDK...");

            // Fetch upstream schema
            let upstream_content = schema_diff::fetch_upstream_schema().await?;

            // Read local schema
            let local_content = schema_diff::read_local_schema(&schema_path)?;

            println!("Comparing schemas...\n");

            // Compare schemas
            let diff = schema_diff::diff_schemas(&local_content, &upstream_content)?;

            // Print the diff
            println!("{}", diff.format());

            if !diff.is_empty() {
                println!("Run 'cargo xtask schema sync' to update the local schema.");
            }
        }
    }

    Ok(())
}
