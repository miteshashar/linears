//! Schema diff implementation for comparing local and upstream schemas

use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

use anyhow::{Context, Result};

/// URL to fetch the upstream schema from Linear's SDK
const SCHEMA_URL: &str =
    "https://raw.githubusercontent.com/linear/linear/master/packages/sdk/src/schema.graphql";

/// Difference between two schemas
#[derive(Debug, Default)]
pub struct SchemaDiff {
    /// Types added in upstream (not in local)
    pub types_added: Vec<String>,
    /// Types removed from upstream (in local but not upstream)
    pub types_removed: Vec<String>,
    /// Fields added to existing types
    pub fields_added: Vec<(String, String)>, // (TypeName, FieldName)
    /// Fields removed from existing types
    pub fields_removed: Vec<(String, String)>, // (TypeName, FieldName)
    /// Enum values added
    pub enum_values_added: Vec<(String, String)>, // (EnumName, ValueName)
    /// Enum values removed
    pub enum_values_removed: Vec<(String, String)>, // (EnumName, ValueName)
}

impl SchemaDiff {
    /// Check if there are any differences
    pub fn is_empty(&self) -> bool {
        self.types_added.is_empty()
            && self.types_removed.is_empty()
            && self.fields_added.is_empty()
            && self.fields_removed.is_empty()
            && self.enum_values_added.is_empty()
            && self.enum_values_removed.is_empty()
    }

    /// Format the diff for human-readable output
    pub fn format(&self) -> String {
        if self.is_empty() {
            return "Schema is up to date with upstream.".to_string();
        }

        let mut output = Vec::new();

        // Summary
        let total_changes = self.types_added.len()
            + self.types_removed.len()
            + self.fields_added.len()
            + self.fields_removed.len()
            + self.enum_values_added.len()
            + self.enum_values_removed.len();
        output.push(format!("Found {} differences:\n", total_changes));

        // Types added
        if !self.types_added.is_empty() {
            output.push("Types added in upstream:".to_string());
            for type_name in &self.types_added {
                output.push(format!("  + {}", type_name));
            }
            output.push(String::new());
        }

        // Types removed
        if !self.types_removed.is_empty() {
            output.push("Types removed in upstream:".to_string());
            for type_name in &self.types_removed {
                output.push(format!("  - {}", type_name));
            }
            output.push(String::new());
        }

        // Fields added
        if !self.fields_added.is_empty() {
            output.push("Fields added in upstream:".to_string());
            for (type_name, field_name) in &self.fields_added {
                output.push(format!("  + {}.{}", type_name, field_name));
            }
            output.push(String::new());
        }

        // Fields removed
        if !self.fields_removed.is_empty() {
            output.push("Fields removed in upstream:".to_string());
            for (type_name, field_name) in &self.fields_removed {
                output.push(format!("  - {}.{}", type_name, field_name));
            }
            output.push(String::new());
        }

        // Enum values added
        if !self.enum_values_added.is_empty() {
            output.push("Enum values added in upstream:".to_string());
            for (enum_name, value_name) in &self.enum_values_added {
                output.push(format!("  + {}.{}", enum_name, value_name));
            }
            output.push(String::new());
        }

        // Enum values removed
        if !self.enum_values_removed.is_empty() {
            output.push("Enum values removed in upstream:".to_string());
            for (enum_name, value_name) in &self.enum_values_removed {
                output.push(format!("  - {}.{}", enum_name, value_name));
            }
            output.push(String::new());
        }

        output.join("\n")
    }
}

/// Parsed schema representation for comparison
#[derive(Debug, Default)]
struct ParsedSchema {
    /// Object types with their field names
    objects: HashMap<String, HashSet<String>>,
    /// Input object types with their field names
    input_objects: HashMap<String, HashSet<String>>,
    /// Interface types with their field names
    interfaces: HashMap<String, HashSet<String>>,
    /// Enum types with their values
    enums: HashMap<String, HashSet<String>>,
    /// Scalar types
    scalars: HashSet<String>,
    /// Union types
    unions: HashSet<String>,
}

/// Fetch the upstream schema from Linear's SDK repository
pub async fn fetch_upstream_schema() -> Result<String> {
    let client = reqwest::Client::builder()
        .user_agent("linears-cli/0.1.0")
        .build()
        .context("Failed to create HTTP client")?;

    let response = client
        .get(SCHEMA_URL)
        .send()
        .await
        .context("Failed to fetch upstream schema")?;

    if !response.status().is_success() {
        anyhow::bail!(
            "Failed to fetch upstream schema: HTTP {}",
            response.status()
        );
    }

    response
        .text()
        .await
        .context("Failed to read upstream schema response")
}

/// Read the local schema from disk
pub fn read_local_schema(schema_path: &Path) -> Result<String> {
    fs::read_to_string(schema_path).context("Failed to read local schema file")
}

/// Parse a GraphQL schema into a comparable structure
fn parse_schema(schema_content: &str) -> Result<ParsedSchema> {
    use graphql_parser::parse_schema;
    use graphql_parser::schema::{Definition, TypeDefinition};

    let ast = parse_schema::<String>(schema_content)
        .map_err(|e| anyhow::anyhow!("Failed to parse schema: {}", e))?;

    let mut parsed = ParsedSchema::default();

    for def in ast.definitions {
        match def {
            Definition::TypeDefinition(type_def) => match type_def {
                TypeDefinition::Object(obj) => {
                    let fields: HashSet<String> =
                        obj.fields.iter().map(|f| f.name.clone()).collect();
                    parsed.objects.insert(obj.name, fields);
                }
                TypeDefinition::InputObject(input) => {
                    let fields: HashSet<String> =
                        input.fields.iter().map(|f| f.name.clone()).collect();
                    parsed.input_objects.insert(input.name, fields);
                }
                TypeDefinition::Interface(iface) => {
                    let fields: HashSet<String> =
                        iface.fields.iter().map(|f| f.name.clone()).collect();
                    parsed.interfaces.insert(iface.name, fields);
                }
                TypeDefinition::Enum(enum_def) => {
                    let values: HashSet<String> =
                        enum_def.values.iter().map(|v| v.name.clone()).collect();
                    parsed.enums.insert(enum_def.name, values);
                }
                TypeDefinition::Scalar(scalar) => {
                    parsed.scalars.insert(scalar.name);
                }
                TypeDefinition::Union(union_def) => {
                    parsed.unions.insert(union_def.name);
                }
            },
            Definition::SchemaDefinition(_) | Definition::DirectiveDefinition(_) => {
                // Skip schema and directive definitions
            }
            Definition::TypeExtension(_) => {
                // Skip type extensions for now
            }
        }
    }

    Ok(parsed)
}

/// Compare two parsed schemas and return the differences
pub fn diff_schemas(local_content: &str, upstream_content: &str) -> Result<SchemaDiff> {
    let local = parse_schema(local_content)?;
    let upstream = parse_schema(upstream_content)?;

    let mut diff = SchemaDiff::default();

    // Compare object types
    diff_type_maps(
        &local.objects,
        &upstream.objects,
        &mut diff.types_added,
        &mut diff.types_removed,
        &mut diff.fields_added,
        &mut diff.fields_removed,
    );

    // Compare input object types
    diff_type_maps(
        &local.input_objects,
        &upstream.input_objects,
        &mut diff.types_added,
        &mut diff.types_removed,
        &mut diff.fields_added,
        &mut diff.fields_removed,
    );

    // Compare interface types
    diff_type_maps(
        &local.interfaces,
        &upstream.interfaces,
        &mut diff.types_added,
        &mut diff.types_removed,
        &mut diff.fields_added,
        &mut diff.fields_removed,
    );

    // Compare enum types
    for (enum_name, upstream_values) in &upstream.enums {
        if let Some(local_values) = local.enums.get(enum_name) {
            // Check for added values
            for value in upstream_values.difference(local_values) {
                diff.enum_values_added
                    .push((enum_name.clone(), value.clone()));
            }
            // Check for removed values
            for value in local_values.difference(upstream_values) {
                diff.enum_values_removed
                    .push((enum_name.clone(), value.clone()));
            }
        } else {
            // New enum type
            diff.types_added.push(enum_name.clone());
        }
    }
    for enum_name in local.enums.keys() {
        if !upstream.enums.contains_key(enum_name) {
            diff.types_removed.push(enum_name.clone());
        }
    }

    // Compare scalar types
    for scalar in upstream.scalars.difference(&local.scalars) {
        diff.types_added.push(scalar.clone());
    }
    for scalar in local.scalars.difference(&upstream.scalars) {
        diff.types_removed.push(scalar.clone());
    }

    // Compare union types
    for union_name in upstream.unions.difference(&local.unions) {
        diff.types_added.push(union_name.clone());
    }
    for union_name in local.unions.difference(&upstream.unions) {
        diff.types_removed.push(union_name.clone());
    }

    // Sort all vectors for consistent output
    diff.types_added.sort();
    diff.types_removed.sort();
    diff.fields_added.sort();
    diff.fields_removed.sort();
    diff.enum_values_added.sort();
    diff.enum_values_removed.sort();

    Ok(diff)
}

/// Helper to diff type maps (objects, input objects, interfaces)
fn diff_type_maps(
    local: &HashMap<String, HashSet<String>>,
    upstream: &HashMap<String, HashSet<String>>,
    types_added: &mut Vec<String>,
    types_removed: &mut Vec<String>,
    fields_added: &mut Vec<(String, String)>,
    fields_removed: &mut Vec<(String, String)>,
) {
    // Find types added in upstream
    for (type_name, upstream_fields) in upstream {
        if let Some(local_fields) = local.get(type_name) {
            // Type exists in both - check for field differences
            for field in upstream_fields.difference(local_fields) {
                fields_added.push((type_name.clone(), field.clone()));
            }
            for field in local_fields.difference(upstream_fields) {
                fields_removed.push((type_name.clone(), field.clone()));
            }
        } else {
            // Type only in upstream
            types_added.push(type_name.clone());
        }
    }

    // Find types removed from upstream
    for type_name in local.keys() {
        if !upstream.contains_key(type_name) {
            types_removed.push(type_name.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_schema() {
        let schema = r#"
            type Query {
                user: User
            }

            type User {
                id: ID!
                name: String!
            }

            enum Status {
                ACTIVE
                INACTIVE
            }
        "#;

        let parsed = parse_schema(schema).unwrap();
        assert!(parsed.objects.contains_key("Query"));
        assert!(parsed.objects.contains_key("User"));
        assert!(parsed.enums.contains_key("Status"));
        assert!(parsed.enums.get("Status").unwrap().contains("ACTIVE"));
    }

    #[test]
    fn test_diff_no_changes() {
        let schema = "type Query { user: User }";
        let diff = diff_schemas(schema, schema).unwrap();
        assert!(diff.is_empty());
    }

    #[test]
    fn test_diff_type_added() {
        let local = "type Query { user: User }";
        let upstream = r#"
            type Query { user: User }
            type NewType { id: ID! }
        "#;
        let diff = diff_schemas(local, upstream).unwrap();
        assert!(diff.types_added.contains(&"NewType".to_string()));
    }

    #[test]
    fn test_diff_field_added() {
        let local = "type User { id: ID! }";
        let upstream = "type User { id: ID! name: String }";
        let diff = diff_schemas(local, upstream).unwrap();
        assert!(diff
            .fields_added
            .contains(&("User".to_string(), "name".to_string())));
    }

    #[test]
    fn test_format_empty_diff() {
        let diff = SchemaDiff::default();
        assert_eq!(diff.format(), "Schema is up to date with upstream.");
    }
}
