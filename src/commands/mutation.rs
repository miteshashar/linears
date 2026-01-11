//! Mutation commands: create, update, delete, archive, unarchive, mutate

use anyhow::Result;

use crate::cli::{Cli, InputOptions, SetOptions, VarsOptions};
use crate::client::GraphQLRequest;
use crate::generated::{MutationOp, Resource};
use crate::mutation_builder::{
    build_archive_mutation, build_create_mutation, build_delete_mutation, build_mutation,
    build_unarchive_mutation, build_update_mutation,
};
use crate::progress::with_spinner;
use crate::render;
use crate::validate;

use super::create_client;

/// Create a new entity
pub async fn cmd_create(cli: &Cli, resource: Resource, input: InputOptions) -> Result<()> {
    // Create client
    let client = create_client(&cli.global)?;

    // Parse the input
    let input_value = validate::resolve_input(input.input.as_deref(), input.input_file.as_deref())?;

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

/// Update an existing entity
pub async fn cmd_update(cli: &Cli, resource: Resource, id: String, set: SetOptions) -> Result<()> {
    // Create client
    let client = create_client(&cli.global)?;

    // Parse the input
    let input_value = validate::resolve_input(set.set.as_deref(), set.set_file.as_deref())?;

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

/// Delete an entity
pub async fn cmd_delete(cli: &Cli, resource: Resource, id: String) -> Result<()> {
    // Create client
    let client = create_client(&cli.global)?;

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

/// Archive an entity
pub async fn cmd_archive(cli: &Cli, resource: Resource, id: String) -> Result<()> {
    // Create client
    let client = create_client(&cli.global)?;

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

/// Unarchive an entity
pub async fn cmd_unarchive(cli: &Cli, resource: Resource, id: String) -> Result<()> {
    // Create client
    let client = create_client(&cli.global)?;

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

/// Execute any mutation operation
pub async fn cmd_mutate(cli: &Cli, op: MutationOp, vars: VarsOptions) -> Result<()> {
    // Create client
    let client = create_client(&cli.global)?;

    // Parse variables
    let mut variables = validate::resolve_input(vars.vars.as_deref(), vars.vars_file.as_deref())
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
    let (query, _) = build_mutation(op, variables.clone());

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
