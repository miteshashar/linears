//! Generated search plans - DO NOT EDIT
//! Run `cargo xtask codegen` to regenerate

use super::Resource;

/// Get the search filter for a resource with the given search text.
/// Returns a JSON value suitable for use as a filter variable in GraphQL queries.
/// The filter uses OR logic across text-searchable fields.
pub fn get_search_filter(resource: Resource, text: &str) -> serde_json::Value {
    match resource {
        Resource::AgentActivity => serde_json::json!({
            "or": [
                { "agentSessionId": { "containsIgnoreCase": text } },
                { "type": { "containsIgnoreCase": text } },
            ]
        }),
        Resource::Attachment => serde_json::json!({
            "or": [
                { "title": { "containsIgnoreCase": text } },
                { "url": { "containsIgnoreCase": text } },
                { "subtitle": { "containsIgnoreCase": text } },
            ]
        }),
        Resource::Comment => serde_json::json!({ "body": { "containsIgnoreCase": text } }),
        Resource::CustomView => serde_json::json!({
            "or": [
                { "name": { "containsIgnoreCase": text } },
                { "modelName": { "containsIgnoreCase": text } },
            ]
        }),
        Resource::Customer => serde_json::json!({
            "or": [
                { "name": { "containsIgnoreCase": text } },
                { "slackChannelId": { "containsIgnoreCase": text } },
            ]
        }),
        Resource::CustomerStatus => serde_json::json!({
            "or": [
                { "name": { "containsIgnoreCase": text } },
                { "description": { "containsIgnoreCase": text } },
                { "color": { "containsIgnoreCase": text } },
            ]
        }),
        Resource::CustomerTier => serde_json::json!({
            "or": [
                { "description": { "containsIgnoreCase": text } },
                { "color": { "containsIgnoreCase": text } },
                { "displayName": { "containsIgnoreCase": text } },
            ]
        }),
        Resource::Cycle => serde_json::json!({ "name": { "containsIgnoreCase": text } }),
        Resource::Document => serde_json::json!({
            "or": [
                { "title": { "containsIgnoreCase": text } },
                { "slugId": { "containsIgnoreCase": text } },
            ]
        }),
        Resource::Initiative => serde_json::json!({
            "or": [
                { "name": { "containsIgnoreCase": text } },
                { "activityType": { "containsIgnoreCase": text } },
                { "health": { "containsIgnoreCase": text } },
            ]
        }),
        Resource::Issue => serde_json::json!({
            "or": [
                { "title": { "containsIgnoreCase": text } },
                { "description": { "containsIgnoreCase": text } },
            ]
        }),
        Resource::IssueLabel => serde_json::json!({ "name": { "containsIgnoreCase": text } }),
        Resource::Notification => serde_json::json!({ "type": { "containsIgnoreCase": text } }),
        Resource::Project => serde_json::json!({
            "or": [
                { "name": { "containsIgnoreCase": text } },
                { "activityType": { "containsIgnoreCase": text } },
                { "health": { "containsIgnoreCase": text } },
            ]
        }),
        Resource::ProjectLabel => serde_json::json!({ "name": { "containsIgnoreCase": text } }),
        Resource::ProjectMilestone => serde_json::json!({ "name": { "containsIgnoreCase": text } }),
        Resource::ProjectStatus => serde_json::json!({
            "or": [
                { "name": { "containsIgnoreCase": text } },
                { "description": { "containsIgnoreCase": text } },
                { "type": { "containsIgnoreCase": text } },
            ]
        }),
        Resource::ProjectUpdates => serde_json::json!({ "health": { "containsIgnoreCase": text } }),
        Resource::Roadmap => serde_json::json!({
            "or": [
                { "name": { "containsIgnoreCase": text } },
                { "slugId": { "containsIgnoreCase": text } },
            ]
        }),
        Resource::Team => serde_json::json!({
            "or": [
                { "name": { "containsIgnoreCase": text } },
                { "key": { "containsIgnoreCase": text } },
                { "description": { "containsIgnoreCase": text } },
            ]
        }),
        Resource::User => serde_json::json!({
            "or": [
                { "name": { "containsIgnoreCase": text } },
                { "email": { "containsIgnoreCase": text } },
                { "displayName": { "containsIgnoreCase": text } },
            ]
        }),
        Resource::WorkflowState => serde_json::json!({
            "or": [
                { "name": { "containsIgnoreCase": text } },
                { "description": { "containsIgnoreCase": text } },
                { "type": { "containsIgnoreCase": text } },
            ]
        }),
        _ => serde_json::json!({ "name": { "containsIgnoreCase": text } }),
    }
}

/// Check if a resource supports text search
#[allow(dead_code)]
pub fn supports_search(resource: Resource) -> bool {
    matches!(resource, Resource::AgentActivity | Resource::Attachment | Resource::Comment | Resource::CustomView | Resource::Customer | Resource::CustomerStatus | Resource::CustomerTier | Resource::Cycle | Resource::Document | Resource::Initiative | Resource::Issue | Resource::IssueLabel | Resource::Notification | Resource::Project | Resource::ProjectLabel | Resource::ProjectMilestone | Resource::ProjectStatus | Resource::ProjectUpdates | Resource::Roadmap | Resource::Team | Resource::User | Resource::WorkflowState)
}

/// Get the searchable fields for a resource (for debugging/documentation)
#[allow(dead_code)]
pub fn get_searchable_fields(resource: Resource) -> &'static [&'static str] {
    match resource {
        Resource::AgentActivity => &["agentSessionId", "type"],
        Resource::Attachment => &["title", "url", "subtitle"],
        Resource::Comment => &["body"],
        Resource::CustomView => &["name", "modelName"],
        Resource::Customer => &["name", "slackChannelId"],
        Resource::CustomerStatus => &["name", "description", "color"],
        Resource::CustomerTier => &["description", "color", "displayName"],
        Resource::Cycle => &["name"],
        Resource::Document => &["title", "slugId"],
        Resource::Initiative => &["name", "activityType", "health"],
        Resource::Issue => &["title", "description"],
        Resource::IssueLabel => &["name"],
        Resource::Notification => &["type"],
        Resource::Project => &["name", "activityType", "health"],
        Resource::ProjectLabel => &["name"],
        Resource::ProjectMilestone => &["name"],
        Resource::ProjectStatus => &["name", "description", "type"],
        Resource::ProjectUpdates => &["health"],
        Resource::Roadmap => &["name", "slugId"],
        Resource::Team => &["name", "key", "description"],
        Resource::User => &["name", "email", "displayName"],
        Resource::WorkflowState => &["name", "description", "type"],
        _ => &[],
    }
}
