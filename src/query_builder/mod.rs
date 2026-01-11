//! Query construction for list, get, and search operations

use crate::cli::ListOptions;
use crate::generated::{
    get_entity_fields, get_preset_fields, get_relation_fields, get_search_filter, FieldPreset,
    Resource,
};

/// Build a list query for a resource
pub fn build_list_query(resource: Resource, options: &ListOptions) -> (String, serde_json::Value) {
    use crate::validate;

    let field_name = resource.field_name();
    // Use schema-derived plural name (avoids naive pluralization bugs)
    let plural_name = resource.plural_name();

    // Get fields - use --select if provided, otherwise use preset
    let mut node_fields: String = if let Some(ref select) = options.select {
        // User specified exact fields with --select
        select.replace(',', " ")
    } else {
        // Use preset-based fields
        get_resource_fields_for_preset(resource, options.preset).to_string()
    };

    // Handle --expand option for relation expansion
    if let Some(ref expands) = options.expand {
        for expand in expands {
            let expand_fields = parse_expand_spec(expand);
            node_fields.push_str(&expand_fields);
        }
    }

    // Check if filter is provided
    let has_filter = options.filter.is_some() || options.filter_file.is_some();

    // Check if orderBy is provided
    let has_order_by = options.order_by.is_some();

    // Build query with appropriate parameters
    let query = if has_filter {
        if has_order_by {
            format!(
                r#"query List{resource}($first: Int, $after: String, $last: Int, $before: String, $filter: {resource}Filter, $includeArchived: Boolean, $orderBy: PaginationOrderBy) {{
  {field}(first: $first, after: $after, last: $last, before: $before, filter: $filter, includeArchived: $includeArchived, orderBy: $orderBy) {{
    pageInfo {{
      hasNextPage
      hasPreviousPage
      startCursor
      endCursor
    }}
    nodes {{
      {node_fields}
    }}
  }}
}}"#,
                resource = to_pascal_case(field_name),
                field = plural_name,
                node_fields = node_fields,
            )
        } else {
            format!(
                r#"query List{resource}($first: Int, $after: String, $last: Int, $before: String, $filter: {resource}Filter, $includeArchived: Boolean) {{
  {field}(first: $first, after: $after, last: $last, before: $before, filter: $filter, includeArchived: $includeArchived) {{
    pageInfo {{
      hasNextPage
      hasPreviousPage
      startCursor
      endCursor
    }}
    nodes {{
      {node_fields}
    }}
  }}
}}"#,
                resource = to_pascal_case(field_name),
                field = plural_name,
                node_fields = node_fields,
            )
        }
    } else if has_order_by {
        format!(
            r#"query List{resource}($first: Int, $after: String, $last: Int, $before: String, $includeArchived: Boolean, $orderBy: PaginationOrderBy) {{
  {field}(first: $first, after: $after, last: $last, before: $before, includeArchived: $includeArchived, orderBy: $orderBy) {{
    pageInfo {{
      hasNextPage
      hasPreviousPage
      startCursor
      endCursor
    }}
    nodes {{
      {node_fields}
    }}
  }}
}}"#,
            resource = to_pascal_case(field_name),
            field = plural_name,
            node_fields = node_fields,
        )
    } else {
        format!(
            r#"query List{resource}($first: Int, $after: String, $last: Int, $before: String, $includeArchived: Boolean) {{
  {field}(first: $first, after: $after, last: $last, before: $before, includeArchived: $includeArchived) {{
    pageInfo {{
      hasNextPage
      hasPreviousPage
      startCursor
      endCursor
    }}
    nodes {{
      {node_fields}
    }}
  }}
}}"#,
            resource = to_pascal_case(field_name),
            field = plural_name,
            node_fields = node_fields,
        )
    };

    // Parse filter if provided (support stdin with '-')
    let filter_value: Option<serde_json::Value> = if let Some(ref filter) = options.filter {
        if filter == "-" {
            // Read filter from stdin
            validate::read_stdin()
                .ok()
                .and_then(|content| validate::parse_input(&content).ok())
        } else {
            validate::parse_input(filter).ok()
        }
    } else if let Some(ref path) = options.filter_file {
        validate::read_file(path)
            .ok()
            .and_then(|content| validate::parse_input(&content).ok())
    } else {
        None
    };

    // Only include includeArchived if it's true
    let include_archived: Option<bool> = if options.include_archived {
        Some(true)
    } else {
        None
    };

    // Convert OrderBy to GraphQL enum value
    let order_by_value: Option<&str> = options.order_by.as_ref().map(|o| o.as_graphql_value());

    let variables = serde_json::json!({
        "first": options.first,
        "after": options.after,
        "last": options.last,
        "before": options.before,
        "filter": filter_value,
        "includeArchived": include_archived,
        "orderBy": order_by_value,
    });

    (query, variables)
}

/// Get the fields to select for a resource type with preset
/// Delegates to generated registry
fn get_resource_fields_for_preset(resource: Resource, preset: crate::cli::Preset) -> &'static str {
    // Convert cli::Preset to generated::FieldPreset
    let field_preset = match preset {
        crate::cli::Preset::Minimal => FieldPreset::Minimal,
        crate::cli::Preset::Default => FieldPreset::Default,
        crate::cli::Preset::Wide => FieldPreset::Wide,
    };
    get_preset_fields(resource, field_preset)
}

/// Get the default fields to select for a resource type
fn get_resource_fields(resource: Resource) -> &'static str {
    get_resource_fields_for_preset(resource, crate::cli::Preset::Default)
}

/// Build a get query for a single entity
pub fn build_get_query(resource: Resource, id: &str) -> (String, serde_json::Value) {
    let field_name = resource.field_name();
    // Use generated registry for entity fields (wide preset)
    let entity_fields = get_entity_fields(resource);

    let query = format!(
        r#"query Get{resource}($id: String!) {{
  {field}(id: $id) {{
    {entity_fields}
  }}
}}"#,
        resource = to_pascal_case(field_name),
        field = field_name,
        entity_fields = entity_fields,
    );

    let variables = serde_json::json!({
        "id": id,
    });

    (query, variables)
}

/// Build a search query
pub fn build_search_query(
    resource: Resource,
    text: &str,
) -> (String, serde_json::Value, SearchStrategy) {
    let field_name = resource.field_name();
    // Use schema-derived plural name (avoids naive pluralization bugs)
    let plural_name = resource.plural_name();
    let node_fields = get_resource_fields(resource);

    // Use filter-based search for all resources
    // Note: issueSearch was deprecated, so we use filter approach universally
    let filter = get_search_filter(resource, text);
    let query = format!(
        r#"query Search{resource}($filter: {resource}Filter, $first: Int) {{
  {field}(filter: $filter, first: $first) {{
    nodes {{
      {node_fields}
    }}
  }}
}}"#,
        resource = to_pascal_case(field_name),
        field = plural_name,
        node_fields = node_fields,
    );
    let variables = serde_json::json!({
        "filter": filter,
        "first": 20,
    });

    (query, variables, SearchStrategy::FilterHeuristic)
}

// Note: get_search_filter is now imported from crate::generated::search_plan

/// Search strategy used
#[derive(Debug, Clone, Copy)]
pub enum SearchStrategy {
    /// Used filter OR-heuristic
    FilterHeuristic,
}

impl SearchStrategy {
    #[allow(dead_code)]
    pub fn as_str(&self) -> &'static str {
        match self {
            SearchStrategy::FilterHeuristic => "filter_heuristic",
        }
    }
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

/// Parse an --expand spec like "team" or "team:name,key"
/// Returns a GraphQL field selection string
fn parse_expand_spec(spec: &str) -> String {
    if let Some((relation, fields)) = spec.split_once(':') {
        // Explicit fields: --expand team:name,key
        let field_list = fields.replace(',', " ");
        format!(" {} {{ {} }}", relation, field_list)
    } else {
        // Default fields for the relation: --expand team
        let default_fields = get_default_relation_fields(spec);
        format!(" {} {{ {} }}", spec, default_fields)
    }
}

/// Get default fields for a relation expansion
/// Delegates to generated registry
fn get_default_relation_fields(relation: &str) -> &'static str {
    get_relation_fields(relation)
}
