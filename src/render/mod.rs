//! Output rendering for different formats
//!
//! This module provides centralized output formatting for all CLI commands.
//! All command handlers should use these functions to ensure consistent output.

use crate::cli::OutputFormat;
use chrono::{DateTime, Utc};
use chrono_humanize::HumanTime;
use serde::Serialize;

// ============================================================================
// Query Rendering
// ============================================================================

/// Render list query results with proper envelope
pub fn render_list_json(
    format: OutputFormat,
    resource: &str,
    nodes: &serde_json::Value,
    page_info: Option<&serde_json::Value>,
    pretty: bool,
) -> String {
    match format {
        OutputFormat::Json => {
            let output = serde_json::json!({
                "resource": resource,
                "operation": "list",
                "pageInfo": page_info,
                "nodes": nodes,
            });
            if pretty {
                serde_json::to_string_pretty(&output).unwrap_or_default()
            } else {
                serde_json::to_string(&output).unwrap_or_default()
            }
        }
        OutputFormat::Yaml => {
            let output = serde_json::json!({
                "resource": resource,
                "operation": "list",
                "pageInfo": page_info,
                "nodes": nodes,
            });
            serde_yaml::to_string(&output).unwrap_or_default()
        }
        OutputFormat::Ndjson => render_ndjson(nodes),
        OutputFormat::Table | OutputFormat::Text => render_table(nodes),
    }
}

/// Render single entity (get) result with proper envelope
pub fn render_entity_json(
    format: OutputFormat,
    resource: &str,
    entity: &serde_json::Value,
    pretty: bool,
) -> String {
    match format {
        OutputFormat::Json => {
            let output = serde_json::json!({
                "resource": resource,
                "operation": "get",
                "entity": entity,
            });
            if pretty {
                serde_json::to_string_pretty(&output).unwrap_or_default()
            } else {
                serde_json::to_string(&output).unwrap_or_default()
            }
        }
        OutputFormat::Yaml => {
            let output = serde_json::json!({
                "resource": resource,
                "operation": "get",
                "entity": entity,
            });
            serde_yaml::to_string(&output).unwrap_or_default()
        }
        OutputFormat::Ndjson => serde_json::to_string(entity).unwrap_or_default(),
        OutputFormat::Table | OutputFormat::Text => {
            // For single entity, YAML is more readable
            serde_yaml::to_string(entity).unwrap_or_default()
        }
    }
}

/// Render search results with proper envelope
pub fn render_search_json(
    format: OutputFormat,
    resource: &str,
    strategy: &str,
    nodes: &serde_json::Value,
    pretty: bool,
) -> String {
    match format {
        OutputFormat::Json => {
            let output = serde_json::json!({
                "resource": resource,
                "operation": "search",
                "strategy": strategy,
                "nodes": nodes,
            });
            if pretty {
                serde_json::to_string_pretty(&output).unwrap_or_default()
            } else {
                serde_json::to_string(&output).unwrap_or_default()
            }
        }
        OutputFormat::Yaml => {
            let output = serde_json::json!({
                "resource": resource,
                "operation": "search",
                "strategy": strategy,
                "nodes": nodes,
            });
            serde_yaml::to_string(&output).unwrap_or_default()
        }
        OutputFormat::Ndjson => render_ndjson(nodes),
        OutputFormat::Table | OutputFormat::Text => {
            // Search uses pretty-printed JSON for table/text
            let output = serde_json::json!({
                "resource": resource,
                "operation": "search",
                "strategy": strategy,
                "nodes": nodes,
            });
            serde_json::to_string_pretty(&output).unwrap_or_default()
        }
    }
}

/// Render raw query results (no envelope)
pub fn render_raw(
    format: OutputFormat,
    data: &Option<serde_json::Value>,
    pretty: bool,
) -> String {
    match format {
        OutputFormat::Json => {
            if pretty {
                serde_json::to_string_pretty(data).unwrap_or_default()
            } else {
                serde_json::to_string(data).unwrap_or_default()
            }
        }
        OutputFormat::Yaml => serde_yaml::to_string(data).unwrap_or_default(),
        _ => serde_json::to_string_pretty(data).unwrap_or_default(),
    }
}

// ============================================================================
// Mutation Rendering
// ============================================================================

/// Render mutation result with proper envelope
pub fn render_mutation_json(
    format: OutputFormat,
    op: &str,
    result: &serde_json::Value,
    pretty: bool,
) -> String {
    match format {
        OutputFormat::Json => {
            let output = serde_json::json!({
                "op": op,
                "operation": "mutate",
                "result": result,
            });
            if pretty {
                serde_json::to_string_pretty(&output).unwrap_or_default()
            } else {
                serde_json::to_string(&output).unwrap_or_default()
            }
        }
        OutputFormat::Yaml => {
            let output = serde_json::json!({
                "op": op,
                "operation": "mutate",
                "result": result,
            });
            serde_yaml::to_string(&output).unwrap_or_default()
        }
        _ => {
            let output = serde_json::json!({
                "op": op,
                "operation": "mutate",
                "result": result,
            });
            serde_json::to_string_pretty(&output).unwrap_or_default()
        }
    }
}

/// Render helper mutation result (create/update/delete/archive)
/// These don't have the standard envelope, just the data
pub fn render_helper_mutation(
    format: OutputFormat,
    data: &Option<serde_json::Value>,
    pretty: bool,
) -> String {
    match format {
        OutputFormat::Json => {
            if pretty {
                serde_json::to_string_pretty(data).unwrap_or_default()
            } else {
                serde_json::to_string(data).unwrap_or_default()
            }
        }
        OutputFormat::Yaml => serde_yaml::to_string(data).unwrap_or_default(),
        _ => serde_json::to_string_pretty(data).unwrap_or_default(),
    }
}

// ============================================================================
// Discovery Rendering
// ============================================================================

/// Render resources list
pub fn render_resources(
    format: OutputFormat,
    resources: &[&str],
    pretty: bool,
) -> String {
    match format {
        OutputFormat::Json => {
            let output = serde_json::json!({
                "resources": resources,
                "count": resources.len()
            });
            if pretty {
                serde_json::to_string_pretty(&output).unwrap_or_default()
            } else {
                serde_json::to_string(&output).unwrap_or_default()
            }
        }
        OutputFormat::Yaml => {
            let output = serde_json::json!({
                "resources": resources,
                "count": resources.len()
            });
            serde_yaml::to_string(&output).unwrap_or_default()
        }
        _ => {
            let mut lines = vec![format!("Available Resources ({}):", resources.len()), String::new()];
            for r in resources {
                lines.push(format!("  {}", r));
            }
            lines.join("\n")
        }
    }
}

/// Render operations list
pub fn render_ops(
    format: OutputFormat,
    operations: &[&str],
    pretty: bool,
) -> String {
    match format {
        OutputFormat::Json => {
            let output = serde_json::json!({
                "operations": operations,
                "count": operations.len()
            });
            if pretty {
                serde_json::to_string_pretty(&output).unwrap_or_default()
            } else {
                serde_json::to_string(&output).unwrap_or_default()
            }
        }
        OutputFormat::Yaml => {
            let output = serde_json::json!({
                "operations": operations,
                "count": operations.len()
            });
            serde_yaml::to_string(&output).unwrap_or_default()
        }
        _ => {
            let mut lines = vec![format!("Available Mutation Operations ({}):", operations.len()), String::new()];
            for op in operations {
                lines.push(format!("  {}", op));
            }
            lines.join("\n")
        }
    }
}

// ============================================================================
// Error Rendering
// ============================================================================

/// Error info for rendering
pub struct ErrorInfo<'a> {
    pub kind: &'a str,
    pub message: &'a str,
    pub hint: Option<&'a str>,
    pub details: Option<&'a serde_json::Value>,
    pub graphql_errors: Option<&'a Vec<crate::client::GraphQLError>>,
}

/// Render error with proper envelope (per PRD spec)
pub fn render_error(
    format: OutputFormat,
    info: &ErrorInfo<'_>,
    pretty: bool,
) -> String {
    match format {
        OutputFormat::Json => {
            let mut error_obj = serde_json::json!({
                "kind": info.kind,
                "message": info.message,
            });
            if let Some(h) = info.hint {
                error_obj["hint"] = serde_json::json!(h);
            }
            if let Some(d) = info.details {
                error_obj["details"] = d.clone();
            }
            let mut output = serde_json::json!({ "error": error_obj });
            if let Some(errors) = info.graphql_errors {
                output["graphqlErrors"] = serde_json::to_value(errors).unwrap_or_default();
            }
            if pretty {
                serde_json::to_string_pretty(&output).unwrap_or_default()
            } else {
                serde_json::to_string(&output).unwrap_or_default()
            }
        }
        OutputFormat::Yaml => {
            let mut error_obj = serde_json::json!({
                "kind": info.kind,
                "message": info.message,
            });
            if let Some(h) = info.hint {
                error_obj["hint"] = serde_json::json!(h);
            }
            if let Some(d) = info.details {
                error_obj["details"] = d.clone();
            }
            let mut output = serde_json::json!({ "error": error_obj });
            if let Some(errors) = info.graphql_errors {
                output["graphqlErrors"] = serde_json::to_value(errors).unwrap_or_default();
            }
            serde_yaml::to_string(&output).unwrap_or_default()
        }
        _ => {
            let mut msg = format!("Error: {}", info.message);
            if let Some(h) = info.hint {
                msg.push_str(&format!("\nHint: {}", h));
            }
            msg
        }
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Render array as NDJSON (one JSON object per line)
fn render_ndjson(nodes: &serde_json::Value) -> String {
    if let Some(arr) = nodes.as_array() {
        arr.iter()
            .filter_map(|n| serde_json::to_string(n).ok())
            .collect::<Vec<_>>()
            .join("\n")
    } else {
        String::new()
    }
}

/// Render array as table with smart datetime formatting
fn render_table(nodes: &serde_json::Value) -> String {
    if let Some(arr) = nodes.as_array() {
        if arr.is_empty() {
            return "No results found".to_string();
        }

        // Extract keys from first item for headers
        if let Some(first) = arr.first() {
            if let Some(obj) = first.as_object() {
                let headers: Vec<&str> = obj.keys().map(|s| s.as_str()).collect();

                // Print header row
                let header_line: Vec<String> = headers.iter().map(|h| h.to_uppercase()).collect();
                let mut output = header_line.join("\t");
                output.push('\n');

                // Print separator
                output.push_str(
                    &headers
                        .iter()
                        .map(|h| "-".repeat(h.len().max(10)))
                        .collect::<Vec<_>>()
                        .join("\t"),
                );
                output.push('\n');

                // Print data rows
                for item in arr {
                    if let Some(item_obj) = item.as_object() {
                        let row: Vec<String> = headers
                            .iter()
                            .map(|h| format_value_for_table(item_obj.get(*h), h))
                            .collect();
                        output.push_str(&row.join("\t"));
                        output.push('\n');
                    }
                }

                // Remove trailing newline
                output.pop();
                return output;
            }
        }
    }
    "No results found".to_string()
}

/// Format a JSON value for table display
/// Handles datetime fields with relative time formatting
pub fn format_value_for_table(value: Option<&serde_json::Value>, field_name: &str) -> String {
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

/// Format a datetime for display
#[allow(dead_code)]
pub fn format_datetime(dt: &DateTime<Utc>, relative: bool) -> String {
    if relative {
        HumanTime::from(*dt).to_string()
    } else {
        dt.format("%Y-%m-%d %H:%M").to_string()
    }
}

/// Format a datetime as relative ("2 hours ago") or absolute based on age
#[allow(dead_code)]
pub fn format_datetime_smart(dt: &DateTime<Utc>) -> String {
    let age = Utc::now().signed_duration_since(*dt);

    // Use relative for recent dates (< 7 days)
    if age.num_days() < 7 {
        HumanTime::from(*dt).to_string()
    } else {
        dt.format("%b %d, %Y").to_string()
    }
}

// ============================================================================
// Pagination Info
// ============================================================================

/// Pagination info (for typed usage if needed)
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PageInfo {
    pub has_next_page: bool,
    pub has_previous_page: bool,
    pub start_cursor: Option<String>,
    pub end_cursor: Option<String>,
}
