//! Query commands: list, get, search, raw

use anyhow::Result;

use crate::cli::{Cli, ListOptions, VarsOptions};
use crate::client::GraphQLRequest;
use crate::common::constants::pagination;
use crate::generated::{self, Resource};
use crate::progress::with_spinner;
use crate::query_builder::{build_get_query, build_list_query_with_filter, build_search_query};
use crate::render;
use crate::validate;

use super::create_client;

/// Resolve filter from any source (inline, file, or stdin) with proper error handling
fn resolve_filter(options: &ListOptions) -> Result<Option<serde_json::Value>> {
    if let Some(ref filter_str) = options.filter {
        if filter_str == "-" {
            // Read filter from stdin
            let content = validate::read_stdin()
                .map_err(|e| anyhow::anyhow!("Failed to read filter from stdin: {}", e))?;
            let value = validate::parse_input(&content)
                .map_err(|e| anyhow::anyhow!("Failed to parse filter from stdin: {}", e))?;
            Ok(Some(value))
        } else {
            // Parse inline filter
            let value = validate::parse_input(filter_str)
                .map_err(|e| anyhow::anyhow!("Failed to parse filter: {}", e))?;
            Ok(Some(value))
        }
    } else if let Some(ref path) = options.filter_file {
        // Read filter from file
        let content = validate::read_file(path)
            .map_err(|e| anyhow::anyhow!("Failed to read filter file '{}': {}", path, e))?;
        let value = validate::parse_input(&content)
            .map_err(|e| anyhow::anyhow!("Failed to parse filter file '{}': {}", path, e))?;
        Ok(Some(value))
    } else {
        Ok(None)
    }
}

/// List entities with pagination and filtering
pub async fn cmd_list(cli: &Cli, resource: Resource, options: ListOptions) -> Result<()> {
    // Resolve filter from any source with proper error handling
    let filter_value = resolve_filter(&options)?;

    // Validate filter keys if a filter is provided
    if let Some(ref filter) = filter_value {
        if let Err(errors) = generated::validate_filter_keys(resource, filter) {
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

    // Create client
    let client = create_client(&cli.global)?;

    let resource_name = resource.field_name();
    // Use schema-derived plural name (avoids naive pluralization bugs)
    let plural_name = resource.plural_name();

    // If --all is specified, auto-paginate
    let (nodes, page_info) = if options.all {

        let mut all_nodes: Vec<serde_json::Value> = Vec::new();
        let mut cursor: Option<String> = None;
        // This will always be assigned at least once in the loop before being returned
        #[allow(unused_assignments)]
        let mut final_page_info: Option<serde_json::Value> = None;

        loop {
            // Build query with current cursor
            let mut page_options = options.clone();
            page_options.first = Some(pagination::PAGE_SIZE);
            page_options.after = cursor.clone();
            page_options.all = false; // Prevent infinite recursion

            let (query, variables) = build_list_query_with_filter(resource, &page_options, filter_value.clone());

            if cli.global.verbose {
                eprintln!("Query: {}", query);
                eprintln!("Variables: {}", serde_json::to_string_pretty(&variables)?);
            }

            let request = GraphQLRequest {
                query,
                variables: Some(variables),
                operation_name: None,
            };

            let page_count = all_nodes.len() / pagination::PAGE_SIZE as usize + 1;
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
            if !has_next || end_cursor.is_none() || all_nodes.len() >= pagination::MAX_RECORDS {
                break;
            }

            cursor = end_cursor;
        }

        // Truncate to max if we exceeded
        if all_nodes.len() > pagination::MAX_RECORDS {
            all_nodes.truncate(pagination::MAX_RECORDS);
        }

        (serde_json::Value::Array(all_nodes), final_page_info)
    } else {
        // Single page fetch
        let (query, variables) = build_list_query_with_filter(resource, &options, filter_value);

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

/// Get a single entity by ID or key
pub async fn cmd_get(cli: &Cli, resource: Resource, id: String) -> Result<()> {
    // Detect ID type for verbose output (Linear API accepts both UUID and identifier)
    let id_type = validate::detect_id_type(&id);
    if cli.global.verbose {
        let type_str = match id_type {
            validate::IdType::Uuid => "UUID",
            validate::IdType::Identifier => "identifier (e.g., ENG-123)",
            validate::IdType::Unknown => "unknown format",
        };
        eprintln!("ID type: {} ({})", type_str, id);
    }

    // Create client
    let client = create_client(&cli.global)?;

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

/// Search for entities using smart search strategy
pub async fn cmd_search(cli: &Cli, resource: Resource, text: String) -> Result<()> {
    // Create client
    let client = create_client(&cli.global)?;

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

/// Execute arbitrary GraphQL queries
pub async fn cmd_raw(cli: &Cli, query: String, vars: VarsOptions) -> Result<()> {
    // Create client
    let client = create_client(&cli.global)?;

    // Read query from file if it looks like a file path
    let query_text = if std::path::Path::new(&query).exists() {
        std::fs::read_to_string(&query)?
    } else {
        query
    };

    // Parse variables (if any provided)
    // If user explicitly provided --vars or --vars-file, parsing errors should be reported
    // Only use empty {} when no variables are provided at all
    let mut variables = if vars.vars.is_some() || vars.vars_file.is_some() {
        validate::resolve_input(vars.vars.as_deref(), vars.vars_file.as_deref())
            .map_err(|e| anyhow::anyhow!("Failed to parse variables: {}", e))?
    } else {
        serde_json::json!({})
    };

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
