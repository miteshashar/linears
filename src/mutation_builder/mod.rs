//! Mutation construction for create, update, delete, and archive operations

use crate::generated::{get_mutation_result_fields, MutationOp};

/// Build a mutation query for any operation
pub fn build_mutation(
    op: MutationOp,
    variables: serde_json::Value,
) -> (String, serde_json::Value) {
    let op_name = op.operation_name();

    // Extract resource name from operation (e.g., issueCreate -> issue)
    let resource_name = extract_resource_name(op_name);
    let entity_fields = get_mutation_result_fields(&resource_name);

    // Determine if this is a delete/archive operation (no entity returned)
    let is_status_only = op_name.ends_with("Delete") || op_name.ends_with("Archive") || op_name.ends_with("Unarchive");

    let query = if is_status_only {
        // Delete/archive operations just return success
        format!(
            r#"mutation {op}($id: String!) {{
  {op_camel}(id: $id) {{
    success
  }}
}}"#,
            op = to_pascal_case(op_name),
            op_camel = op_name,
        )
    } else {
        // Create/update operations return the entity
        format!(
            r#"mutation {op}($input: {op}Input!) {{
  {op_camel}(input: $input) {{
    success
    {resource} {{
      {entity_fields}
    }}
  }}
}}"#,
            op = to_pascal_case(op_name),
            op_camel = op_name,
            resource = resource_name,
            entity_fields = entity_fields,
        )
    };

    (query, variables)
}

/// Extract resource name from operation name (e.g., issueCreate -> issue)
fn extract_resource_name(op_name: &str) -> String {
    // Common suffixes to strip
    let suffixes = ["Create", "Update", "Delete", "Archive", "Unarchive"];

    for suffix in suffixes {
        if let Some(prefix) = op_name.strip_suffix(suffix) {
            return prefix.to_string();
        }
    }

    // Fallback: just use the op_name
    op_name.to_string()
}

/// Build a create mutation for a resource
pub fn build_create_mutation(
    resource_name: &str,
    input: serde_json::Value,
) -> (String, serde_json::Value) {
    let op_name = format!("{}Create", resource_name);
    let entity_fields = get_mutation_result_fields(resource_name);

    let query = format!(
        r#"mutation {op}($input: {resource}CreateInput!) {{
  {op_camel}(input: $input) {{
    success
    {resource_lower} {{
      {entity_fields}
    }}
  }}
}}"#,
        op = to_pascal_case(&op_name),
        op_camel = to_camel_case(&op_name),
        resource = to_pascal_case(resource_name),
        resource_lower = resource_name,
        entity_fields = entity_fields,
    );

    let variables = serde_json::json!({
        "input": input,
    });

    (query, variables)
}

/// Build an update mutation for a resource
pub fn build_update_mutation(
    resource_name: &str,
    id: &str,
    input: serde_json::Value,
) -> (String, serde_json::Value) {
    let op_name = format!("{}Update", resource_name);
    let entity_fields = get_mutation_result_fields(resource_name);

    let query = format!(
        r#"mutation {op}($id: String!, $input: {resource}UpdateInput!) {{
  {op_camel}(id: $id, input: $input) {{
    success
    {resource_lower} {{
      {entity_fields}
    }}
  }}
}}"#,
        op = to_pascal_case(&op_name),
        op_camel = to_camel_case(&op_name),
        resource = to_pascal_case(resource_name),
        resource_lower = resource_name,
        entity_fields = entity_fields,
    );

    let variables = serde_json::json!({
        "id": id,
        "input": input,
    });

    (query, variables)
}

/// Build a delete mutation for a resource
pub fn build_delete_mutation(resource_name: &str, id: &str) -> (String, serde_json::Value) {
    let op_name = format!("{}Delete", resource_name);

    let query = format!(
        r#"mutation {op}($id: String!) {{
  {op_camel}(id: $id) {{
    success
  }}
}}"#,
        op = to_pascal_case(&op_name),
        op_camel = to_camel_case(&op_name),
    );

    let variables = serde_json::json!({
        "id": id,
    });

    (query, variables)
}

/// Build an archive mutation for a resource
pub fn build_archive_mutation(resource_name: &str, id: &str) -> (String, serde_json::Value) {
    let op_name = format!("{}Archive", resource_name);

    let query = format!(
        r#"mutation {op}($id: String!) {{
  {op_camel}(id: $id) {{
    success
  }}
}}"#,
        op = to_pascal_case(&op_name),
        op_camel = to_camel_case(&op_name),
    );

    let variables = serde_json::json!({
        "id": id,
    });

    (query, variables)
}

/// Build an unarchive mutation for a resource
pub fn build_unarchive_mutation(resource_name: &str, id: &str) -> (String, serde_json::Value) {
    let op_name = format!("{}Unarchive", resource_name);

    let query = format!(
        r#"mutation {op}($id: String!) {{
  {op_camel}(id: $id) {{
    success
  }}
}}"#,
        op = to_pascal_case(&op_name),
        op_camel = to_camel_case(&op_name),
    );

    let variables = serde_json::json!({
        "id": id,
    });

    (query, variables)
}

fn to_pascal_case(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;

    for c in s.chars() {
        if c == '_' || c == '-' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(c.to_ascii_uppercase());
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }

    result
}

fn to_camel_case(s: &str) -> String {
    let pascal = to_pascal_case(s);
    let mut chars = pascal.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_lowercase().collect::<String>() + chars.as_str(),
    }
}

// Note: get_mutation_result_fields is now imported from crate::generated::mutation_registry
// as get_mutation_result_fields
