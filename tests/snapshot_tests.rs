//! Snapshot tests for generated code
//!
//! These tests verify that codegen output is stable and catch unintended changes.
//! Run `cargo insta test` to review snapshots after changes.

use linears::cli::ListOptions;
use linears::common::FieldsetPreset;
use linears::generated::{get_searchable_fields, supports_search, MutationOp, Resource};
use linears::mutation_builder::build_mutation;
use linears::query_builder::{build_get_query, build_list_query_with_filter, build_search_query};

/// Snapshot all Resource enum variants
#[test]
fn registry_snapshot() {
    let resources: Vec<&str> = Resource::all()
        .iter()
        .map(|r| r.field_name())
        .collect();

    insta::assert_yaml_snapshot!("resources", resources);
}

/// Snapshot all MutationOp enum variants
#[test]
fn mutation_ops_snapshot() {
    let ops: Vec<&str> = MutationOp::all()
        .iter()
        .map(|op| op.operation_name())
        .collect();

    insta::assert_yaml_snapshot!("mutation_ops", ops);
}

/// Snapshot resource count for quick drift detection
#[test]
fn resource_count_snapshot() {
    let count = Resource::all().len();
    insta::assert_snapshot!("resource_count", count.to_string());
}

/// Snapshot mutation op count for quick drift detection
#[test]
fn mutation_op_count_snapshot() {
    let count = MutationOp::all().len();
    insta::assert_snapshot!("mutation_op_count", count.to_string());
}

/// Snapshot key resources that are commonly used
#[test]
fn key_resources_snapshot() {
    use Resource::*;
    let key_resources = vec![
        Issue,
        Issues,
        Team,
        Teams,
        User,
        Users,
        Project,
        Projects,
        Cycle,
        Cycles,
        Comment,
        Comments,
        IssueLabel,
        IssueLabels,
        WorkflowState,
        WorkflowStates,
    ];

    let names: Vec<&str> = key_resources
        .iter()
        .map(|r| r.field_name())
        .collect();

    insta::assert_yaml_snapshot!("key_resources", names);
}

/// Snapshot key mutations that are commonly used
#[test]
fn key_mutations_snapshot() {
    use MutationOp::*;
    let key_mutations = vec![
        IssueCreate,
        IssueUpdate,
        IssueDelete,
        IssueArchive,
        CommentCreate,
        CommentUpdate,
        CommentDelete,
    ];

    let names: Vec<&str> = key_mutations
        .iter()
        .map(|op| op.operation_name())
        .collect();

    insta::assert_yaml_snapshot!("key_mutations", names);
}

// ============================================================================
// Query String Snapshots (per PRD §10)
// These catch field selection drift - if codegen changes field selection,
// these snapshots will fail.
// ============================================================================

/// Default list options for snapshot consistency
fn default_list_options() -> ListOptions {
    ListOptions {
        first: Some(20),
        after: None,
        last: None,
        before: None,
        all: false,
        include_archived: false,
        order_by: None,
        filter: None,
        filter_file: None,
        preset: FieldsetPreset::Default,
        select: None,
        expand: None,
    }
}

/// Snapshot list query for key resources
/// Tests the generated GraphQL query text and field selection
#[test]
fn query_snapshot_list_resources() {
    use Resource::*;

    // Key resources to snapshot (most commonly used)
    let resources = vec![Issues, Teams, Users, Projects, Cycles, Comments, IssueLabels];

    let mut queries = Vec::new();
    for resource in resources {
        let (query, _vars) = build_list_query_with_filter(resource, &default_list_options(), None);
        queries.push(format!("=== {} ===\n{}", resource.field_name(), query));
    }

    insta::assert_snapshot!("list_queries", queries.join("\n\n"));
}

/// Snapshot get query for key resources
/// Tests single entity retrieval queries
#[test]
fn query_snapshot_get_resources() {
    use Resource::*;

    // Key singular resources (for get operations)
    let resources = vec![Issue, Team, User, Project, Cycle, Comment, IssueLabel];

    let mut queries = Vec::new();
    for resource in resources {
        let (query, _vars) = build_get_query(resource, "test-id-123");
        queries.push(format!("=== {} ===\n{}", resource.field_name(), query));
    }

    insta::assert_snapshot!("get_queries", queries.join("\n\n"));
}

/// Snapshot search query for resources that support search
/// Tests filter-based search query generation
#[test]
fn query_snapshot_search_resources() {
    use Resource::*;

    // Resources with searchable fields
    let resources = vec![Issues, Teams, Users, Projects, Comments];

    let mut queries = Vec::new();
    for resource in resources {
        let (query, _vars, _strategy) = build_search_query(resource, "test search");
        queries.push(format!("=== {} ===\n{}", resource.field_name(), query));
    }

    insta::assert_snapshot!("search_queries", queries.join("\n\n"));
}

/// Snapshot mutation queries for key operations
/// Tests mutation document structure and entity field selection
#[test]
fn mutation_snapshot_selected_ops() {
    use MutationOp::*;

    // Representative mutations covering different patterns
    let test_cases = vec![
        (IssueCreate, r#"{"input":{"title":"Test","teamId":"..."}}"#),
        (IssueUpdate, r#"{"input":{"title":"Updated"}}"#),
        (IssueDelete, r#"{"id":"test-123"}"#),
        (CommentCreate, r#"{"input":{"body":"Test comment","issueId":"..."}}"#),
        (ProjectCreate, r#"{"input":{"name":"Test Project","teamIds":["..."]}}"#),
    ];

    let mut queries = Vec::new();
    for (op, vars_json) in test_cases {
        let vars: serde_json::Value = serde_json::from_str(vars_json).unwrap();
        let (query, _) = build_mutation(op, vars);
        queries.push(format!("=== {} ===\n{}", op.operation_name(), query));
    }

    insta::assert_snapshot!("mutation_queries", queries.join("\n\n"));
}

// ============================================================================
// Search Plan Snapshot (per PRD §10)
// Documents which resources support search and their searchable fields.
// ============================================================================

/// Snapshot search plan: per-resource search support and searchable fields
/// This fulfills PRD requirement for search_plan_snapshot
#[test]
fn search_plan_snapshot() {
    let mut plan_entries = Vec::new();

    for resource in Resource::all() {
        if supports_search(*resource) {
            let fields = get_searchable_fields(*resource);
            plan_entries.push(format!(
                "{}: [{}]",
                resource.field_name(),
                fields.join(", ")
            ));
        }
    }

    insta::assert_snapshot!("search_plan", plan_entries.join("\n"));
}
