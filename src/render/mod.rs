//! Output rendering for different formats

use crate::cli::OutputFormat;
use chrono::{DateTime, Utc};
use chrono_humanize::HumanTime;
use serde::Serialize;
use tabled::{Table, Tabled};

/// Render query list results
pub fn render_list<T: Serialize + Tabled>(
    format: OutputFormat,
    resource: &str,
    nodes: &[T],
    page_info: Option<&PageInfo>,
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
        OutputFormat::Ndjson => {
            nodes
                .iter()
                .filter_map(|n| serde_json::to_string(n).ok())
                .collect::<Vec<_>>()
                .join("\n")
        }
        OutputFormat::Table => {
            if nodes.is_empty() {
                "No results found".to_string()
            } else {
                Table::new(nodes).to_string()
            }
        }
        OutputFormat::Text => {
            // For list, fall back to table
            if nodes.is_empty() {
                "No results found".to_string()
            } else {
                Table::new(nodes).to_string()
            }
        }
    }
}

/// Render single entity result
pub fn render_entity<T: Serialize>(
    format: OutputFormat,
    resource: &str,
    entity: &T,
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
        OutputFormat::Ndjson => {
            serde_json::to_string(entity).unwrap_or_default()
        }
        OutputFormat::Table | OutputFormat::Text => {
            // Pretty-print the entity for text output
            serde_yaml::to_string(entity).unwrap_or_default()
        }
    }
}

/// Render mutation result
pub fn render_mutation<T: Serialize>(
    format: OutputFormat,
    op: &str,
    result: &T,
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
            format!("Success: {}", op)
        }
    }
}

/// Render search results
pub fn render_search<T: Serialize + Tabled>(
    format: OutputFormat,
    resource: &str,
    strategy: &str,
    nodes: &[T],
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
        OutputFormat::Ndjson => {
            nodes
                .iter()
                .filter_map(|n| serde_json::to_string(n).ok())
                .collect::<Vec<_>>()
                .join("\n")
        }
        OutputFormat::Table | OutputFormat::Text => {
            if nodes.is_empty() {
                "No results found".to_string()
            } else {
                Table::new(nodes).to_string()
            }
        }
    }
}

/// Render error
pub fn render_error(
    format: OutputFormat,
    kind: &str,
    message: &str,
    hint: Option<&str>,
    graphql_errors: Option<&[serde_json::Value]>,
    pretty: bool,
) -> String {
    match format {
        OutputFormat::Json => {
            let mut error = serde_json::json!({
                "kind": kind,
                "message": message,
            });
            if let Some(h) = hint {
                error["hint"] = serde_json::json!(h);
            }
            let mut output = serde_json::json!({
                "error": error,
            });
            if let Some(gql_errors) = graphql_errors {
                output["graphqlErrors"] = serde_json::json!(gql_errors);
            }
            if pretty {
                serde_json::to_string_pretty(&output).unwrap_or_default()
            } else {
                serde_json::to_string(&output).unwrap_or_default()
            }
        }
        OutputFormat::Yaml => {
            let mut error = serde_json::json!({
                "kind": kind,
                "message": message,
            });
            if let Some(h) = hint {
                error["hint"] = serde_json::json!(h);
            }
            let mut output = serde_json::json!({
                "error": error,
            });
            if let Some(gql_errors) = graphql_errors {
                output["graphqlErrors"] = serde_json::json!(gql_errors);
            }
            serde_yaml::to_string(&output).unwrap_or_default()
        }
        _ => {
            let mut msg = format!("Error: {}", message);
            if let Some(h) = hint {
                msg.push_str(&format!("\nHint: {}", h));
            }
            msg
        }
    }
}

/// Pagination info
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PageInfo {
    pub has_next_page: bool,
    pub has_previous_page: bool,
    pub start_cursor: Option<String>,
    pub end_cursor: Option<String>,
}

/// Format a datetime for display
pub fn format_datetime(dt: &DateTime<Utc>, relative: bool) -> String {
    if relative {
        HumanTime::from(*dt).to_string()
    } else {
        dt.format("%Y-%m-%d %H:%M").to_string()
    }
}

/// Format a datetime as relative ("2 hours ago") or absolute based on age
pub fn format_datetime_smart(dt: &DateTime<Utc>) -> String {
    let age = Utc::now().signed_duration_since(*dt);

    // Use relative for recent dates (< 7 days)
    if age.num_days() < 7 {
        HumanTime::from(*dt).to_string()
    } else {
        dt.format("%b %d, %Y").to_string()
    }
}
