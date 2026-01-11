//! Query construction for list, get, and search operations

use crate::cli::ListOptions;
use crate::generated::Resource;

/// Build a list query for a resource
pub fn build_list_query(resource: Resource, options: &ListOptions) -> (String, serde_json::Value) {
    let field_name = resource.field_name();
    let plural_name = plural_field_name(field_name);

    // Get fields for this specific resource type
    let node_fields = get_resource_fields(resource);

    let query = format!(
        r#"query List{resource}($first: Int, $after: String, $last: Int, $before: String) {{
  {field}(first: $first, after: $after, last: $last, before: $before) {{
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
    );

    let variables = serde_json::json!({
        "first": options.first,
        "after": options.after,
        "last": options.last,
        "before": options.before,
    });

    (query, variables)
}

/// Get the fields to select for a resource type
fn get_resource_fields(resource: Resource) -> &'static str {
    match resource {
        Resource::Issue => "id title identifier priority createdAt state { name }",
        Resource::Team => "id name key description",
        Resource::User => "id name email active",
        Resource::Project => "id name state startDate targetDate",
        Resource::Cycle => "id name number startsAt endsAt",
        Resource::IssueLabel => "id name color",
        Resource::Comment => "id body createdAt",
        Resource::Workflow => "id name",
        Resource::WorkflowState => "id name color type",
        Resource::Attachment => "id title url",
        Resource::Document => "id title createdAt",
        Resource::Roadmap => "id name",
        Resource::Initiative => "id name",
        Resource::Integration => "id service",
        Resource::Notification => "id type createdAt",
        Resource::Webhook => "id url enabled",
        Resource::ApiKey => "id label createdAt",
        Resource::Viewer => "id name email",
        Resource::Organization => "id name urlKey",
    }
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
        Resource::Team => "id name key description createdAt",
        Resource::User => "id name email active admin createdAt",
        Resource::Project => "id name description state startDate targetDate createdAt",
        Resource::Cycle => "id name number startsAt endsAt completedAt",
        Resource::IssueLabel => "id name color description createdAt",
        Resource::Comment => "id body createdAt updatedAt user { name }",
        Resource::Workflow => "id name createdAt",
        Resource::WorkflowState => "id name color type position",
        Resource::Attachment => "id title url createdAt",
        Resource::Document => "id title content createdAt updatedAt",
        Resource::Roadmap => "id name description createdAt",
        Resource::Initiative => "id name description createdAt",
        Resource::Integration => "id service createdAt",
        Resource::Notification => "id type createdAt readAt",
        Resource::Webhook => "id url enabled createdAt",
        Resource::ApiKey => "id label createdAt",
        Resource::Viewer => "id name email",
        Resource::Organization => "id name urlKey createdAt",
    }
}

/// Build a search query
pub fn build_search_query(
    resource: Resource,
    text: &str,
) -> (String, serde_json::Value, SearchStrategy) {
    let field_name = resource.field_name();

    // Determine search strategy based on resource
    let strategy = match resource {
        // Resources with native search
        _ => SearchStrategy::FilterHeuristic,
    };

    let query = format!(
        r#"query Search{resource}($filter: {resource}Filter) {{
  {field}(filter: $filter) {{
    nodes {{
      id
      ... on Issue {{ title identifier state {{ name }} }}
      ... on Team {{ name key }}
      ... on User {{ name email }}
      ... on Project {{ name state }}
    }}
  }}
}}"#,
        resource = to_pascal_case(field_name),
        field = plural_field_name(field_name),
    );

    // Build OR filter for common text fields
    let filter = serde_json::json!({
        "or": [
            { "name": { "containsIgnoreCase": text } },
            { "title": { "containsIgnoreCase": text } },
        ]
    });

    let variables = serde_json::json!({
        "filter": filter,
    });

    (query, variables, strategy)
}

/// Search strategy used
#[derive(Debug, Clone, Copy)]
pub enum SearchStrategy {
    /// Used native search query field
    Native,
    /// Used filter OR-heuristic
    FilterHeuristic,
}

impl SearchStrategy {
    pub fn as_str(&self) -> &'static str {
        match self {
            SearchStrategy::Native => "native",
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

fn plural_field_name(field: &str) -> String {
    // Simple pluralization - real impl would be more sophisticated
    if field.ends_with('s') {
        format!("{}es", field)
    } else if field.ends_with('y') {
        format!("{}ies", &field[..field.len() - 1])
    } else {
        format!("{}s", field)
    }
}
