//! Query construction for list, get, and search operations

use crate::cli::ListOptions;
use crate::generated::Resource;

/// Build a list query for a resource
pub fn build_list_query(resource: Resource, options: &ListOptions) -> (String, serde_json::Value) {
    use crate::validate;

    let field_name = resource.field_name();
    let plural_name = plural_field_name(field_name);

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

    // Build query with appropriate parameters
    let query = if has_filter {
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

    let variables = serde_json::json!({
        "first": options.first,
        "after": options.after,
        "last": options.last,
        "before": options.before,
        "filter": filter_value,
        "includeArchived": include_archived,
    });

    (query, variables)
}

/// Get the fields to select for a resource type with preset
fn get_resource_fields_for_preset(resource: Resource, preset: crate::cli::Preset) -> &'static str {
    use crate::cli::Preset;

    match preset {
        Preset::Minimal => match resource {
            Resource::Issue => "id identifier title",
            Resource::Team => "id name key",
            Resource::User => "id name",
            Resource::Project => "id name",
            Resource::Cycle => "id name number",
            Resource::IssueLabel => "id name",
            Resource::Comment => "id body",
            _ => "id",
        },
        Preset::Default => match resource {
            Resource::Issue => "id title identifier priority createdAt state { name }",
            Resource::Team | Resource::Teams => "id name key description",
            Resource::User | Resource::Users => "id name email active",
            Resource::Project | Resource::Projects => "id name state startDate targetDate",
            Resource::Cycle | Resource::Cycles => "id name number startsAt endsAt",
            Resource::IssueLabel | Resource::IssueLabels => "id name color",
            Resource::Comment | Resource::Comments => "id body createdAt",
            Resource::WorkflowState | Resource::WorkflowStates => "id name color type",
            Resource::Attachment | Resource::Attachments => "id title url",
            Resource::Document | Resource::Documents => "id title createdAt",
            Resource::Roadmap | Resource::Roadmaps => "id name",
            Resource::Initiative | Resource::Initiatives => "id name",
            Resource::Integration | Resource::Integrations => "id service",
            Resource::Notification | Resource::Notifications => "id type createdAt",
            Resource::Webhook | Resource::Webhooks => "id url enabled",
            Resource::Viewer => "id name email",
            Resource::Organization => "id name urlKey",
            _ => "id",
        },
        Preset::Wide => match resource {
            Resource::Issue => "id title description identifier priority createdAt updatedAt state { name color } assignee { name email } creator { name } team { name key }",
            Resource::Team | Resource::Teams => "id name key description createdAt organization { name }",
            Resource::User | Resource::Users => "id name email displayName active admin createdAt",
            Resource::Project | Resource::Projects => "id name description state startDate targetDate completedAt createdAt",
            Resource::Cycle | Resource::Cycles => "id name number description startsAt endsAt completedAt",
            Resource::IssueLabel | Resource::IssueLabels => "id name color description createdAt",
            Resource::Comment | Resource::Comments => "id body createdAt updatedAt user { name }",
            Resource::WorkflowState | Resource::WorkflowStates => "id name color type position createdAt",
            Resource::Attachment | Resource::Attachments => "id title url createdAt",
            Resource::Document | Resource::Documents => "id title content createdAt updatedAt",
            Resource::Roadmap | Resource::Roadmaps => "id name description createdAt",
            Resource::Initiative | Resource::Initiatives => "id name description createdAt",
            Resource::Integration | Resource::Integrations => "id service createdAt",
            Resource::Notification | Resource::Notifications => "id type createdAt readAt",
            Resource::Webhook | Resource::Webhooks => "id url enabled createdAt",
            Resource::Viewer => "id name email displayName",
            Resource::Organization => "id name urlKey createdAt",
            _ => "id",
        },
    }
}

/// Get the default fields to select for a resource type
fn get_resource_fields(resource: Resource) -> &'static str {
    get_resource_fields_for_preset(resource, crate::cli::Preset::Default)
}

/// Build a get query for a single entity
pub fn build_get_query(resource: Resource, id: &str) -> (String, serde_json::Value) {
    let field_name = resource.field_name();
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

/// Get the fields to select for a single entity (more detailed than list)
fn get_entity_fields(resource: Resource) -> &'static str {
    match resource {
        Resource::Issue => "id title description identifier priority createdAt updatedAt state { name } assignee { name } creator { name } team { name key }",
        Resource::Team | Resource::Teams => "id name key description createdAt",
        Resource::User | Resource::Users => "id name email active admin createdAt",
        Resource::Project | Resource::Projects => "id name description state startDate targetDate createdAt",
        Resource::Cycle | Resource::Cycles => "id name number startsAt endsAt completedAt",
        Resource::IssueLabel | Resource::IssueLabels => "id name color description createdAt",
        Resource::Comment | Resource::Comments => "id body createdAt updatedAt user { name }",
        Resource::WorkflowState | Resource::WorkflowStates => "id name color type position",
        Resource::Attachment | Resource::Attachments => "id title url createdAt",
        Resource::Document | Resource::Documents => "id title content createdAt updatedAt",
        Resource::Roadmap | Resource::Roadmaps => "id name description createdAt",
        Resource::Initiative | Resource::Initiatives => "id name description createdAt",
        Resource::Integration | Resource::Integrations => "id service createdAt",
        Resource::Notification | Resource::Notifications => "id type createdAt readAt",
        Resource::Webhook | Resource::Webhooks => "id url enabled createdAt",
        Resource::Viewer => "id name email",
        Resource::Organization => "id name urlKey createdAt",
        _ => "id",
    }
}

/// Build a search query
pub fn build_search_query(
    resource: Resource,
    text: &str,
) -> (String, serde_json::Value, SearchStrategy) {
    let field_name = resource.field_name();
    let plural_name = plural_field_name(field_name);
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

/// Get the filter for searching a resource
fn get_search_filter(resource: Resource, text: &str) -> serde_json::Value {
    match resource {
        Resource::Issue => serde_json::json!({
            "or": [
                { "title": { "containsIgnoreCase": text } },
                { "description": { "containsIgnoreCase": text } },
            ]
        }),
        Resource::Team => serde_json::json!({
            "or": [
                { "name": { "containsIgnoreCase": text } },
                { "key": { "containsIgnoreCase": text } },
            ]
        }),
        Resource::User => serde_json::json!({
            "or": [
                { "name": { "containsIgnoreCase": text } },
                { "email": { "containsIgnoreCase": text } },
            ]
        }),
        Resource::Project => serde_json::json!({
            "name": { "containsIgnoreCase": text }
        }),
        Resource::IssueLabel => serde_json::json!({
            "name": { "containsIgnoreCase": text }
        }),
        _ => serde_json::json!({
            "name": { "containsIgnoreCase": text }
        }),
    }
}

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

/// Convert a singular field name to its plural form for GraphQL queries
pub fn plural_field_name(field: &str) -> String {
    // Simple pluralization - real impl would be more sophisticated
    if field.ends_with('s') {
        format!("{}es", field)
    } else if field.ends_with('y') {
        format!("{}ies", &field[..field.len() - 1])
    } else {
        format!("{}s", field)
    }
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
fn get_default_relation_fields(relation: &str) -> &'static str {
    match relation {
        "team" => "id name key",
        "assignee" | "creator" | "user" => "id name email",
        "state" => "id name color type",
        "project" => "id name",
        "cycle" => "id name number",
        "parent" => "id identifier title",
        "labels" => "nodes { id name color }",
        "comments" => "nodes { id body }",
        "attachments" => "nodes { id title url }",
        "subscribers" => "nodes { id name }",
        "children" => "nodes { id identifier title }",
        "organization" => "id name urlKey",
        _ => "id",
    }
}
