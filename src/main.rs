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
            let (exit_code, error_kind, error_message) =
                if let Some(client_err) = e.downcast_ref::<client::ClientError>() {
                    let kind = match client_err {
                        client::ClientError::Auth(_) => "auth",
                        client::ClientError::Network(_) => "network",
                        client::ClientError::GraphQL(_) => "graphql",
                        client::ClientError::RateLimited(_) => "rate_limited",
                        client::ClientError::RateLimitedTooLong(_) => "rate_limited",
                        client::ClientError::Server(_) => "server",
                        client::ClientError::Other(_) => "other",
                    };
                    (client_err.exit_code(), kind, format!("{}", client_err))
                } else {
                    (exit_codes::GENERAL_ERROR, "general", format!("{:#}", e))
                };

            // Render error based on output format
            match cli.global.output {
                cli::OutputFormat::Json => {
                    let error_output = serde_json::json!({
                        "error": {
                            "kind": error_kind,
                            "message": error_message,
                        }
                    });
                    eprintln!(
                        "{}",
                        if cli.global.pretty {
                            serde_json::to_string_pretty(&error_output).unwrap_or_default()
                        } else {
                            serde_json::to_string(&error_output).unwrap_or_default()
                        }
                    );
                }
                cli::OutputFormat::Yaml => {
                    let error_output = serde_json::json!({
                        "error": {
                            "kind": error_kind,
                            "message": error_message,
                        }
                    });
                    eprintln!("{}", serde_yaml::to_string(&error_output).unwrap_or_default());
                }
                _ => {
                    eprintln!("Error: {}", error_message);
                }
            }
            ExitCode::from(exit_code)
        }
    }
}

/// Format a JSON value for table display
/// Handles datetime fields with relative time formatting
fn format_value_for_table(value: Option<&serde_json::Value>, field_name: &str) -> String {
    use chrono::{DateTime, Utc};
    use chrono_humanize::HumanTime;

    match value {
        Some(serde_json::Value::String(s)) => {
            // Check if this looks like a datetime field and try to parse it
            let is_datetime_field = field_name.ends_with("At")
                || field_name.ends_with("_at")
                || field_name == "createdAt"
                || field_name == "updatedAt"
                || field_name == "archivedAt"
                || field_name == "startedAt"
                || field_name == "completedAt"
                || field_name == "canceledAt"
                || field_name == "dueDate";

            if is_datetime_field {
                // Try to parse as ISO 8601 datetime
                if let Ok(dt) = s.parse::<DateTime<Utc>>() {
                    let age = Utc::now().signed_duration_since(dt);
                    // Use relative for recent dates (< 7 days)
                    if age.num_days().abs() < 7 {
                        return HumanTime::from(dt).to_string();
                    } else {
                        return dt.format("%b %d, %Y").to_string();
                    }
                }
            }
            s.clone()
        }
        Some(serde_json::Value::Number(n)) => n.to_string(),
        Some(serde_json::Value::Bool(b)) => b.to_string(),
        Some(serde_json::Value::Null) => "-".to_string(),
        Some(serde_json::Value::Object(o)) => {
            // For nested objects, try to get a display field
            o.get("name")
                .or_else(|| o.get("key"))
                .or_else(|| o.get("id"))
                .and_then(|v| v.as_str())
                .map(String::from)
                .unwrap_or_else(|| "[object]".to_string())
        }
        Some(serde_json::Value::Array(_)) => "[array]".to_string(),
        None => "-".to_string(),
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
    use progress::with_spinner;
    use query_builder::build_list_query;

    // Get API key (already validated in main)
    let api_key = get_api_key().expect("API key already validated");

    // Create client
    let client = Client::new(
        &api_key,
        cli.global.endpoint.as_deref(),
        cli.global.timeout,
        cli.global.workspace.as_deref(),
    )?;

    let resource_name = resource.field_name();
    let plural_name = query_builder::plural_field_name(resource_name);

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

    // Render the response with proper envelope
    match cli.global.output {
        cli::OutputFormat::Json => {
            let output = serde_json::json!({
                "resource": resource_name,
                "operation": "list",
                "pageInfo": page_info,
                "nodes": nodes,
            });
            if cli.global.pretty {
                println!("{}", serde_json::to_string_pretty(&output)?);
            } else {
                println!("{}", serde_json::to_string(&output)?);
            }
        }
        cli::OutputFormat::Yaml => {
            let output = serde_json::json!({
                "resource": resource_name,
                "operation": "list",
                "pageInfo": page_info,
                "nodes": nodes,
            });
            println!("{}", serde_yaml::to_string(&output)?);
        }
        cli::OutputFormat::Ndjson => {
            if let Some(arr) = nodes.as_array() {
                for node in arr {
                    println!("{}", serde_json::to_string(node)?);
                }
            }
        }
        cli::OutputFormat::Table | cli::OutputFormat::Text => {
            // Render as table
            if let Some(arr) = nodes.as_array() {
                if arr.is_empty() {
                    println!("No results found");
                } else {
                    // Extract keys from first item for headers
                    if let Some(first) = arr.first() {
                        if let Some(obj) = first.as_object() {
                            let headers: Vec<&str> = obj.keys().map(|s| s.as_str()).collect();

                            // Print header row
                            let header_line: Vec<String> = headers.iter().map(|h| h.to_uppercase()).collect();
                            println!("{}", header_line.join("\t"));

                            // Print separator
                            println!("{}", headers.iter().map(|h| "-".repeat(h.len().max(10))).collect::<Vec<_>>().join("\t"));

                            // Print data rows
                            for item in arr {
                                if let Some(item_obj) = item.as_object() {
                                    let row: Vec<String> = headers.iter().map(|h| {
                                        format_value_for_table(item_obj.get(*h), h)
                                    }).collect();
                                    println!("{}", row.join("\t"));
                                }
                            }
                        }
                    }
                }
            }
        }
    }

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

    // Render the response with proper envelope
    match cli.global.output {
        cli::OutputFormat::Json => {
            let output = serde_json::json!({
                "resource": resource_name,
                "operation": "get",
                "entity": entity,
            });
            if cli.global.pretty {
                println!("{}", serde_json::to_string_pretty(&output)?);
            } else {
                println!("{}", serde_json::to_string(&output)?);
            }
        }
        cli::OutputFormat::Yaml => {
            let output = serde_json::json!({
                "resource": resource_name,
                "operation": "get",
                "entity": entity,
            });
            println!("{}", serde_yaml::to_string(&output)?);
        }
        cli::OutputFormat::Ndjson => {
            println!("{}", serde_json::to_string(&entity)?);
        }
        _ => {
            // For table/text, just print YAML for readability
            println!("{}", serde_yaml::to_string(&entity)?);
        }
    }

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
    let plural_name = query_builder::plural_field_name(resource_name);
    let resource_data = &data[&plural_name];
    let nodes = resource_data.get("nodes").cloned().unwrap_or_default();

    // Render the response with proper envelope
    match cli.global.output {
        cli::OutputFormat::Json => {
            let output = serde_json::json!({
                "resource": resource_name,
                "operation": "search",
                "strategy": strategy.as_str(),
                "nodes": nodes,
            });
            if cli.global.pretty {
                println!("{}", serde_json::to_string_pretty(&output)?);
            } else {
                println!("{}", serde_json::to_string(&output)?);
            }
        }
        cli::OutputFormat::Yaml => {
            let output = serde_json::json!({
                "resource": resource_name,
                "operation": "search",
                "strategy": strategy.as_str(),
                "nodes": nodes,
            });
            println!("{}", serde_yaml::to_string(&output)?);
        }
        cli::OutputFormat::Ndjson => {
            if let Some(arr) = nodes.as_array() {
                for node in arr {
                    println!("{}", serde_json::to_string(node)?);
                }
            }
        }
        _ => {
            // For table/text, just print JSON for now
            let output = serde_json::json!({
                "resource": resource_name,
                "operation": "search",
                "strategy": strategy.as_str(),
                "nodes": nodes,
            });
            println!("{}", serde_json::to_string_pretty(&output)?);
        }
    }

    Ok(())
}

async fn cmd_raw(cli: &Cli, query: String) -> Result<()> {
    use client::{Client, GraphQLRequest};
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

    // Read query from file if it looks like a file path
    let query_text = if std::path::Path::new(&query).exists() {
        std::fs::read_to_string(&query)?
    } else {
        query
    };

    if cli.global.verbose {
        eprintln!("Query: {}", query_text);
    }

    // Execute the query with spinner
    let request = GraphQLRequest {
        query: query_text,
        variables: None,
        operation_name: None,
    };

    let response = with_spinner("Executing query...", client.execute(request)).await?;

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

    // Render the response with proper envelope
    match cli.global.output {
        cli::OutputFormat::Json => {
            let output = serde_json::json!({
                "op": op_name,
                "operation": "mutate",
                "result": result,
            });
            if cli.global.pretty {
                println!("{}", serde_json::to_string_pretty(&output)?);
            } else {
                println!("{}", serde_json::to_string(&output)?);
            }
        }
        cli::OutputFormat::Yaml => {
            let output = serde_json::json!({
                "op": op_name,
                "operation": "mutate",
                "result": result,
            });
            println!("{}", serde_yaml::to_string(&output)?);
        }
        _ => {
            let output = serde_json::json!({
                "op": op_name,
                "operation": "mutate",
                "result": result,
            });
            println!("{}", serde_json::to_string_pretty(&output)?);
        }
    }

    Ok(())
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
            // TODO: Implement schema diff
            println!("Schema diff not yet implemented");
            println!("Run 'cargo xtask schema sync' to check for updates");
        }
    }

    Ok(())
}
