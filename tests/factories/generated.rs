//! Auto-generated deterministic test factories
//!
//! These factories produce predictable JSON objects for testing.
//! All data is deterministic based on input parameters.

use serde_json::{json, Value};

/// Fixed timestamp for all factory-generated data
pub const FIXED_TIMESTAMP: &str = "2025-01-10T12:00:00.000Z";
pub const FIXED_UPDATED_AT: &str = "2025-01-10T14:30:00.000Z";

/// Generate a deterministic UUID based on a type prefix and number
///
/// Format: 550e8400-e29b-41d4-a716-{type_prefix}{num:09}
pub fn deterministic_uuid(type_prefix: &str, num: u32) -> String {
    format!(
        "550e8400-e29b-41d4-a716-{}{}",
        type_prefix,
        format!("{:09}", num)
    )
}

/// Generate a deterministic issue
pub fn issue(num: u32) -> Value {
    issue_with_id(&deterministic_uuid("iss", num), num)
}

/// Generate an issue with a specific ID
pub fn issue_with_id(id: &str, num: u32) -> Value {
    let assignee: Option<Value> = if num % 3 == 0 {
        None
    } else {
        Some(json!({
            "id": deterministic_uuid("usr", num % 10),
            "name": format!("User {}", num % 10),
            "email": format!("user{}@example.com", num % 10)
        }))
    };

    let state_name = match num % 5 {
        0 => "Backlog",
        1 => "Todo",
        2 => "In Progress",
        3 => "In Review",
        _ => "Done",
    };

    let state_type = match num % 5 {
        0 => "backlog",
        1 => "unstarted",
        2 => "started",
        3 => "started",
        _ => "completed",
    };

    let priority_label = match num % 5 {
        0 => "No priority",
        1 => "Urgent",
        2 => "High",
        3 => "Medium",
        _ => "Low",
    };

    json!({
        "id": id,
        "identifier": format!("ENG-{}", num),
        "title": format!("Test Issue {}", num),
        "description": format!("Description for test issue {}", num),
        "priority": (num % 5) as i32,
        "priorityLabel": priority_label,
        "createdAt": FIXED_TIMESTAMP,
        "updatedAt": FIXED_UPDATED_AT,
        "archivedAt": null,
        "state": {
            "id": deterministic_uuid("sta", num % 5),
            "name": state_name,
            "type": state_type
        },
        "team": {
            "id": deterministic_uuid("tea", 1),
            "key": "ENG",
            "name": "Engineering"
        },
        "assignee": assignee
    })
}

/// Generate a deterministic team
pub fn team(num: u32) -> Value {
    team_with_id(&deterministic_uuid("tea", num), num)
}

/// Generate a team with a specific ID
pub fn team_with_id(id: &str, num: u32) -> Value {
    let keys = ["ENG", "DES", "MKT", "OPS", "SUP"];
    let names = ["Engineering", "Design", "Marketing", "Operations", "Support"];

    json!({
        "id": id,
        "key": keys[(num as usize) % keys.len()],
        "name": names[(num as usize) % names.len()],
        "description": format!("Team {} description", num),
        "createdAt": FIXED_TIMESTAMP,
        "updatedAt": FIXED_UPDATED_AT,
        "archivedAt": null
    })
}

/// Generate a deterministic user
pub fn user(num: u32) -> Value {
    user_with_id(&deterministic_uuid("usr", num), num)
}

/// Generate a user with a specific ID
pub fn user_with_id(id: &str, num: u32) -> Value {
    json!({
        "id": id,
        "name": format!("Test User {}", num),
        "displayName": format!("User {}", num),
        "email": format!("user{}@example.com", num),
        "active": true,
        "admin": num == 1,
        "guest": false,
        "createdAt": FIXED_TIMESTAMP,
        "updatedAt": FIXED_UPDATED_AT,
        "archivedAt": null
    })
}

/// Generate a deterministic project
pub fn project(num: u32) -> Value {
    project_with_id(&deterministic_uuid("prj", num), num)
}

/// Generate a project with a specific ID
pub fn project_with_id(id: &str, num: u32) -> Value {
    let state = match num % 4 {
        0 => "planned",
        1 => "started",
        2 => "paused",
        _ => "completed",
    };

    json!({
        "id": id,
        "name": format!("Project {}", num),
        "description": format!("Description for project {}", num),
        "slugId": format!("project-{}", num),
        "state": state,
        "progress": ((num * 25) % 101) as f64 / 100.0,
        "createdAt": FIXED_TIMESTAMP,
        "updatedAt": FIXED_UPDATED_AT,
        "archivedAt": null,
        "teams": {
            "nodes": [team(1)]
        }
    })
}

/// Generate a deterministic label
pub fn label(num: u32) -> Value {
    label_with_id(&deterministic_uuid("lbl", num), num)
}

/// Generate a label with a specific ID
pub fn label_with_id(id: &str, num: u32) -> Value {
    let colors = ["#FF5630", "#36B37E", "#0065FF", "#6554C0", "#FFAB00"];
    let names = ["Bug", "Feature", "Documentation", "Enhancement", "Question"];

    json!({
        "id": id,
        "name": names[(num as usize) % names.len()],
        "color": colors[(num as usize) % colors.len()],
        "description": format!("Label {} description", num),
        "createdAt": FIXED_TIMESTAMP,
        "updatedAt": FIXED_UPDATED_AT,
        "archivedAt": null
    })
}

/// Generate a deterministic cycle (sprint)
pub fn cycle(num: u32) -> Value {
    cycle_with_id(&deterministic_uuid("cyc", num), num)
}

/// Generate a cycle with a specific ID
pub fn cycle_with_id(id: &str, num: u32) -> Value {
    json!({
        "id": id,
        "number": num,
        "name": format!("Sprint {}", num),
        "description": format!("Cycle {} description", num),
        "startsAt": "2025-01-06T00:00:00.000Z",
        "endsAt": "2025-01-20T00:00:00.000Z",
        "progress": ((num * 20) % 101) as f64 / 100.0,
        "createdAt": FIXED_TIMESTAMP,
        "updatedAt": FIXED_UPDATED_AT,
        "archivedAt": null,
        "team": {
            "id": deterministic_uuid("tea", 1),
            "key": "ENG",
            "name": "Engineering"
        }
    })
}

/// Generate a deterministic comment
pub fn comment(num: u32) -> Value {
    comment_with_id(&deterministic_uuid("cmt", num), num)
}

/// Generate a comment with a specific ID
pub fn comment_with_id(id: &str, num: u32) -> Value {
    json!({
        "id": id,
        "body": format!("This is test comment {}", num),
        "createdAt": FIXED_TIMESTAMP,
        "updatedAt": FIXED_UPDATED_AT,
        "archivedAt": null,
        "user": {
            "id": deterministic_uuid("usr", num % 10),
            "name": format!("User {}", num % 10),
            "email": format!("user{}@example.com", num % 10)
        },
        "issue": {
            "id": deterministic_uuid("iss", num),
            "identifier": format!("ENG-{}", num),
            "title": format!("Test Issue {}", num)
        }
    })
}

/// Generate a deterministic workflow state
pub fn workflow_state(num: u32) -> Value {
    workflow_state_with_id(&deterministic_uuid("wfs", num), num)
}

/// Generate a workflow state with a specific ID
pub fn workflow_state_with_id(id: &str, num: u32) -> Value {
    let names = ["Backlog", "Todo", "In Progress", "In Review", "Done", "Canceled"];
    let types = ["backlog", "unstarted", "started", "started", "completed", "canceled"];
    let colors = ["#bec2c8", "#e2e2e2", "#f2c94c", "#6fcf97", "#27ae60", "#95a2b3"];

    json!({
        "id": id,
        "name": names[(num as usize) % names.len()],
        "type": types[(num as usize) % types.len()],
        "color": colors[(num as usize) % colors.len()],
        "position": num as f64,
        "createdAt": FIXED_TIMESTAMP,
        "updatedAt": FIXED_UPDATED_AT,
        "archivedAt": null,
        "team": {
            "id": deterministic_uuid("tea", 1),
            "key": "ENG",
            "name": "Engineering"
        }
    })
}

/// Generate a paginated connection response
pub fn connection<F>(factory: F, count: u32, has_next: bool, has_prev: bool) -> Value
where
    F: Fn(u32) -> Value,
{
    let nodes: Vec<Value> = (1..=count).map(|i| factory(i)).collect();

    json!({
        "nodes": nodes,
        "pageInfo": {
            "hasNextPage": has_next,
            "hasPreviousPage": has_prev,
            "startCursor": if count > 0 { Some(format!("cursor-start-{}", 1)) } else { None },
            "endCursor": if count > 0 { Some(format!("cursor-end-{}", count)) } else { None }
        }
    })
}

/// Generate an empty connection response
pub fn empty_connection() -> Value {
    json!({
        "nodes": [],
        "pageInfo": {
            "hasNextPage": false,
            "hasPreviousPage": false,
            "startCursor": null,
            "endCursor": null
        }
    })
}

/// Wrap data in a GraphQL response envelope
pub fn graphql_response(data: Value) -> Value {
    json!({
        "data": data
    })
}

/// Create a GraphQL error response
pub fn graphql_error(message: &str) -> Value {
    json!({
        "data": null,
        "errors": [{
            "message": message,
            "locations": [{"line": 1, "column": 1}],
            "path": ["query"]
        }]
    })
}

/// Create an authentication error response
pub fn auth_error() -> Value {
    graphql_error("Authentication required. Please provide a valid API key.")
}

/// Create a not found error response
pub fn not_found_error(resource: &str, id: &str) -> Value {
    graphql_error(&format!("{} with id {} not found", resource, id))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deterministic_uuid() {
        assert_eq!(
            deterministic_uuid("iss", 1),
            "550e8400-e29b-41d4-a716-iss000000001"
        );
        assert_eq!(
            deterministic_uuid("tea", 123),
            "550e8400-e29b-41d4-a716-tea000000123"
        );
    }

    #[test]
    fn test_issue_is_deterministic() {
        let issue1 = issue(1);
        let issue2 = issue(1);
        assert_eq!(issue1, issue2);
    }

    #[test]
    fn test_connection_pagination() {
        let conn = connection(issue, 5, true, false);
        assert_eq!(conn["nodes"].as_array().unwrap().len(), 5);
        assert!(conn["pageInfo"]["hasNextPage"].as_bool().unwrap());
        assert!(!conn["pageInfo"]["hasPreviousPage"].as_bool().unwrap());
    }

    #[test]
    fn test_empty_connection() {
        let conn = empty_connection();
        assert!(conn["nodes"].as_array().unwrap().is_empty());
        assert!(!conn["pageInfo"]["hasNextPage"].as_bool().unwrap());
    }
}
