//! CLI integration tests for linears
//!
//! These tests use httpmock to simulate the Linear GraphQL API
//! and verify end-to-end CLI behavior.

mod factories;
mod support;

use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

use factories::generated::{issue, team, user};
use support::mock_server::{
    cli_with_mock_server, create_mock_server, mock_auth_error, mock_list_issues, mock_list_teams,
};

/// Test that missing API key results in exit code 2
#[test]
fn test_missing_api_key() {
    let mut cmd = Command::cargo_bin("linears").unwrap();
    cmd.arg("list")
        .arg("issue")
        .env_remove("LINEARS_API_KEY")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains("LINEARS_API_KEY"));
}

/// Test that --help works without API key
#[test]
fn test_help_without_api_key() {
    let mut cmd = Command::cargo_bin("linears").unwrap();
    cmd.arg("--help")
        .env_remove("LINEARS_API_KEY")
        .assert()
        .success()
        .stdout(predicate::str::contains("Linear's GraphQL API"));
}

/// Test that --version shows schema info
#[test]
fn test_version_shows_schema() {
    let mut cmd = Command::cargo_bin("linears").unwrap();
    cmd.arg("--version")
        .env_remove("LINEARS_API_KEY")
        .assert()
        .success()
        .stdout(predicate::str::contains("linears"))
        .stdout(predicate::str::contains("schema:"));
}

/// Test that resources command lists available resources
#[test]
fn test_resources_command() {
    let mut cmd = Command::cargo_bin("linears").unwrap();
    cmd.arg("resources")
        .env_remove("LINEARS_API_KEY")
        .assert()
        .success()
        .stdout(predicate::str::contains("issue"))
        .stdout(predicate::str::contains("team"));
}

/// Test that ops command lists available operations
#[test]
fn test_ops_command() {
    let mut cmd = Command::cargo_bin("linears").unwrap();
    cmd.arg("ops")
        .env_remove("LINEARS_API_KEY")
        .assert()
        .success()
        .stdout(predicate::str::contains("issue"));
}

/// Test list issues with mock server
#[test]
fn test_list_issues_json() {
    let server = create_mock_server();
    let issues = vec![issue(1), issue(2), issue(3)];
    let _mock = mock_list_issues(&server, issues);

    cli_with_mock_server(&server)
        .args(["--out", "json", "list", "issue"])
        .assert()
        .success()
        .stdout(predicate::str::contains("ENG-1"))
        .stdout(predicate::str::contains("ENG-2"))
        .stdout(predicate::str::contains("ENG-3"));
}

/// Test list teams with mock server
#[test]
fn test_list_teams_json() {
    let server = create_mock_server();
    // team(0) = Engineering, team(1) = Design, etc.
    let teams = vec![team(0), team(1)];
    let _mock = mock_list_teams(&server, teams);

    cli_with_mock_server(&server)
        .args(["--out", "json", "list", "team"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Engineering"));
}

/// Test authentication error handling
#[test]
fn test_auth_error_handling() {
    let server = create_mock_server();
    let _mock = mock_auth_error(&server);

    cli_with_mock_server(&server)
        .args(["list", "issue"])
        .assert()
        .failure()
        .code(2);
}

/// Test empty results
#[test]
fn test_empty_results() {
    let server = create_mock_server();
    let _mock = mock_list_issues(&server, vec![]);

    cli_with_mock_server(&server)
        .args(["--out", "json", "list", "issue"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"nodes\":[]").or(predicate::str::contains("\"nodes\": []")));
}

/// Test invalid resource name
#[test]
fn test_invalid_resource() {
    let mut cmd = Command::cargo_bin("linears").unwrap();
    cmd.args(["list", "notaresource"])
        .env("LINEARS_API_KEY", "test")
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid value"));
}

/// Test that factories produce deterministic output
#[test]
fn test_factory_determinism() {
    let issue1 = issue(42);
    let issue2 = issue(42);
    assert_eq!(issue1, issue2);

    let team1 = team(1);
    let team2 = team(1);
    assert_eq!(team1, team2);

    let user1 = user(5);
    let user2 = user(5);
    assert_eq!(user1, user2);
}
