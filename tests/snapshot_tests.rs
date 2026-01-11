//! Snapshot tests for generated code
//!
//! These tests verify that codegen output is stable and catch unintended changes.
//! Run `cargo insta test` to review snapshots after changes.

use linears::generated::{MutationOp, Resource};

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
