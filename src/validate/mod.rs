//! Input validation utilities

use anyhow::{Context, Result};
use std::io::{self, Read};

/// Parse input from JSON or YAML string
/// Returns an error if the input is not a valid JSON/YAML object
pub fn parse_input(input: &str) -> Result<serde_json::Value> {
    // Try JSON first
    if let Ok(value) = serde_json::from_str::<serde_json::Value>(input) {
        if value.is_object() {
            return Ok(value);
        }
        anyhow::bail!("Input must be a JSON/YAML object, got: {}", value_type_name(&value));
    }

    // Try YAML
    let value: serde_json::Value = serde_yaml::from_str(input)
        .context("Failed to parse input as JSON or YAML")?;

    if value.is_object() {
        Ok(value)
    } else {
        anyhow::bail!("Input must be a JSON/YAML object, got: {}", value_type_name(&value))
    }
}

/// Get a human-readable name for a JSON value type
fn value_type_name(value: &serde_json::Value) -> &'static str {
    match value {
        serde_json::Value::Null => "null",
        serde_json::Value::Bool(_) => "boolean",
        serde_json::Value::Number(_) => "number",
        serde_json::Value::String(_) => "string",
        serde_json::Value::Array(_) => "array",
        serde_json::Value::Object(_) => "object",
    }
}

/// Read input from stdin
pub fn read_stdin() -> Result<String> {
    let mut buffer = String::new();
    io::stdin()
        .read_to_string(&mut buffer)
        .context("Failed to read from stdin")?;
    Ok(buffer)
}

/// Read input from file
pub fn read_file(path: &str) -> Result<String> {
    std::fs::read_to_string(path).context(format!("Failed to read file: {}", path))
}

/// Resolve input from various sources
pub fn resolve_input(inline: Option<&str>, file: Option<&str>) -> Result<serde_json::Value> {
    if let Some(input) = inline {
        if input == "-" {
            let stdin = read_stdin()?;
            parse_input(&stdin)
        } else {
            parse_input(input)
        }
    } else if let Some(path) = file {
        let content = read_file(path)?;
        parse_input(&content)
    } else {
        anyhow::bail!("No input provided. Use --input or --input-file")
    }
}

/// Check if a string looks like a UUID
#[allow(dead_code)]
pub fn is_uuid(s: &str) -> bool {
    // UUID format: 8-4-4-4-12 hexadecimal characters
    let parts: Vec<&str> = s.split('-').collect();
    if parts.len() != 5 {
        return false;
    }

    let expected_lens = [8, 4, 4, 4, 12];
    parts
        .iter()
        .zip(expected_lens.iter())
        .all(|(part, &len)| part.len() == len && part.chars().all(|c| c.is_ascii_hexdigit()))
}

/// Check if a string looks like an issue identifier (e.g., ENG-123)
#[allow(dead_code)]
pub fn is_identifier(s: &str) -> bool {
    let parts: Vec<&str> = s.split('-').collect();
    if parts.len() != 2 {
        return false;
    }

    let prefix = parts[0];
    let number = parts[1];

    // Prefix should be uppercase letters
    prefix.chars().all(|c| c.is_ascii_uppercase())
        // Number should be digits
        && number.chars().all(|c| c.is_ascii_digit())
        && !number.is_empty()
}

/// Detect if ID is UUID or identifier
#[allow(dead_code)]
pub enum IdType {
    Uuid,
    Identifier,
    Unknown,
}

/// Detect the type of ID
#[allow(dead_code)]
pub fn detect_id_type(id: &str) -> IdType {
    if is_uuid(id) {
        IdType::Uuid
    } else if is_identifier(id) {
        IdType::Identifier
    } else {
        IdType::Unknown
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_uuid() {
        assert!(is_uuid("550e8400-e29b-41d4-a716-446655440000"));
        assert!(!is_uuid("ENG-123"));
        assert!(!is_uuid("not-a-uuid"));
    }

    #[test]
    fn test_is_identifier() {
        assert!(is_identifier("ENG-123"));
        assert!(is_identifier("PROJ-1"));
        assert!(!is_identifier("550e8400-e29b-41d4-a716-446655440000"));
        assert!(!is_identifier("eng-123")); // lowercase
        assert!(!is_identifier("ENG123")); // no dash
    }

    #[test]
    fn test_parse_input_json() {
        let result = parse_input(r#"{"title": "Test"}"#).unwrap();
        assert_eq!(result["title"], "Test");
    }

    #[test]
    fn test_parse_input_yaml() {
        let result = parse_input("title: Test").unwrap();
        assert_eq!(result["title"], "Test");
    }

    #[test]
    fn test_parse_input_invalid() {
        // Test that non-object inputs are rejected
        let result = parse_input("invalid{json");
        assert!(result.is_err(), "Expected error for non-object input");

        // A plain string should also fail
        let result = parse_input("just a string");
        assert!(result.is_err(), "Expected error for plain string");

        // An array should fail
        let result = parse_input("[1, 2, 3]");
        assert!(result.is_err(), "Expected error for array");

        // But an object should succeed
        let result = parse_input(r#"{"key": "value"}"#);
        assert!(result.is_ok(), "Expected success for object");
    }
}
