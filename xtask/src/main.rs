//! xtask - Build automation tasks for linears
//!
//! Run with: cargo xtask <command>

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use clap::{Parser, Subcommand};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

const SCHEMA_URL: &str =
    "https://raw.githubusercontent.com/linear/linear/master/packages/sdk/src/schema.graphql";

/// GitHub API URL to get commit info for the schema file
const GITHUB_COMMITS_API: &str =
    "https://api.github.com/repos/linear/linear/commits?path=packages/sdk/src/schema.graphql&per_page=1";

/// GitHub commit response (simplified)
#[derive(Deserialize)]
struct GitHubCommit {
    sha: String,
    commit: CommitInfo,
    html_url: String,
}

#[derive(Deserialize)]
struct CommitInfo {
    committer: CommitterInfo,
}

#[derive(Deserialize)]
struct CommitterInfo {
    date: String,
}

#[derive(Parser)]
#[command(name = "xtask")]
#[command(about = "Build automation tasks for linears")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Schema management commands
    Schema {
        #[command(subcommand)]
        action: SchemaAction,
    },
    /// Run code generation
    Codegen,
}

#[derive(Subcommand)]
enum SchemaAction {
    /// Sync schema from Linear's SDK repository
    Sync,
    /// Show schema info
    Info,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Schema { action } => match action {
            SchemaAction::Sync => sync_schema(),
            SchemaAction::Info => schema_info(),
        },
        Commands::Codegen => run_codegen(),
    }
}

fn project_root() -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    PathBuf::from(manifest_dir).parent().unwrap().to_path_buf()
}

fn schema_dir() -> PathBuf {
    project_root().join("schemas").join("linear")
}

fn sync_schema() -> Result<()> {
    println!("Fetching schema from Linear SDK...");
    println!("URL: {}", SCHEMA_URL);

    // Create HTTP client with user agent (required by GitHub API)
    let client = reqwest::blocking::Client::builder()
        .user_agent("linears-xtask/0.1.0")
        .build()
        .context("Failed to create HTTP client")?;

    // Fetch the schema content
    let response = client
        .get(SCHEMA_URL)
        .send()
        .context("Failed to fetch schema")?
        .text()
        .context("Failed to read schema response")?;

    let schema_path = schema_dir().join("schema.graphql");
    fs::create_dir_all(schema_dir())?;
    fs::write(&schema_path, &response).context("Failed to write schema file")?;

    println!("Schema written to: {}", schema_path.display());
    println!("Schema size: {} bytes", response.len());

    // Fetch commit metadata from GitHub API
    println!("\nFetching commit metadata from GitHub...");
    let commits: Vec<GitHubCommit> = client
        .get(GITHUB_COMMITS_API)
        .send()
        .context("Failed to fetch commit info from GitHub")?
        .json()
        .context("Failed to parse GitHub commit response")?;

    let (commit_sha, commit_date, permalink) = if let Some(commit) = commits.first() {
        let sha = commit.sha.clone();
        let date = commit.commit.committer.date.clone();
        // Create a proper permalink to the exact file version
        let permalink = format!(
            "https://github.com/linear/linear/blob/{}/packages/sdk/src/schema.graphql",
            &sha[..7]
        );
        println!("  Commit: {}", &sha[..7]);
        println!("  Date: {}", date);
        (sha, date, permalink)
    } else {
        eprintln!("Warning: Could not fetch commit info, using fallback values");
        let now = Utc::now().to_rfc3339();
        (
            "unknown".to_string(),
            now.clone(),
            SCHEMA_URL.to_string(),
        )
    };

    // Update metadata with real commit info
    let synced_at: DateTime<Utc> = Utc::now();
    let meta = serde_json::json!({
        "source": SCHEMA_URL,
        "commit": commit_sha,
        "commitDate": commit_date,
        "syncedAt": synced_at.to_rfc3339(),
        "permalink": permalink
    });

    let meta_path = schema_dir().join("schema.meta.json");
    fs::write(&meta_path, serde_json::to_string_pretty(&meta)?)
        .context("Failed to write metadata")?;

    println!("\nMetadata written to: {}", meta_path.display());
    println!("Schema sync complete!");

    Ok(())
}

fn schema_info() -> Result<()> {
    let meta_path = schema_dir().join("schema.meta.json");

    if !meta_path.exists() {
        println!("No schema metadata found. Run 'cargo xtask schema sync' first.");
        return Ok(());
    }

    let meta: serde_json::Value = serde_json::from_str(&fs::read_to_string(&meta_path)?)?;

    println!("Schema Information:");
    println!("  Source: {}", meta["source"].as_str().unwrap_or("unknown"));

    if let Some(commit) = meta["commit"].as_str() {
        let short_sha = if commit.len() >= 7 { &commit[..7] } else { commit };
        println!("  Commit: {}", short_sha);
    }

    if let Some(commit_date) = meta["commitDate"].as_str() {
        println!("  Commit Date: {}", commit_date);
    }

    println!(
        "  Synced At: {}",
        meta["syncedAt"].as_str().unwrap_or("unknown")
    );

    if let Some(permalink) = meta["permalink"].as_str() {
        println!("  Permalink: {}", permalink);
    }

    let schema_path = schema_dir().join("schema.graphql");
    if schema_path.exists() {
        let content = fs::read_to_string(&schema_path)?;
        let lines = content.lines().count();
        println!("  Schema Lines: {}", lines);
    } else {
        println!("  Warning: schema.graphql not found!");
    }

    Ok(())
}

fn run_codegen() -> Result<()> {
    let schema_path = schema_dir().join("schema.graphql");

    if !schema_path.exists() {
        anyhow::bail!(
            "Schema not found. Run 'cargo xtask schema sync' first.\nExpected: {}",
            schema_path.display()
        );
    }

    println!("Running code generation...");
    println!("Schema: {}", schema_path.display());

    let schema_content =
        fs::read_to_string(&schema_path).context("Failed to read schema file")?;

    // Parse the GraphQL schema
    let ast = graphql_parser::parse_schema::<String>(&schema_content)
        .map_err(|e| anyhow::anyhow!("Failed to parse schema: {}", e))?;

    let generated_dir = project_root().join("src").join("generated");
    fs::create_dir_all(&generated_dir)?;

    // Generate resources enum
    generate_resources(&ast, &generated_dir)?;

    // Generate mutation ops enum
    generate_mutation_ops(&ast, &generated_dir)?;

    // Generate registry with field presets
    generate_registry(&ast, &generated_dir)?;

    // Generate mod.rs
    generate_mod_rs(&generated_dir)?;

    println!("Code generation complete!");
    println!("Generated files in: {}", generated_dir.display());

    Ok(())
}

fn generate_resources(
    ast: &graphql_parser::schema::Document<String>,
    output_dir: &PathBuf,
) -> Result<()> {
    use graphql_parser::schema::Definition;
    use graphql_parser::schema::TypeDefinition;

    let mut resources = Vec::new();

    for def in &ast.definitions {
        if let Definition::TypeDefinition(TypeDefinition::Object(obj)) = def {
            if obj.name == "Query" {
                for field in &obj.fields {
                    // Skip internal/connection fields
                    if !field.name.ends_with("Connection")
                        && !field.name.starts_with("_")
                        && !field.name.starts_with("__")
                    {
                        resources.push(field.name.clone());
                    }
                }
            }
        }
    }

    resources.sort();

    let mut code = String::from(
        r#"//! Generated resource types - DO NOT EDIT
//! Run `cargo xtask codegen` to regenerate

use clap::ValueEnum;

/// Available query resources derived from Linear's GraphQL schema
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
#[value(rename_all = "camelCase")]
pub enum Resource {
"#,
    );

    for resource in &resources {
        let variant = to_pascal_case(resource);
        code.push_str(&format!("    /// Query {}\n", resource));
        code.push_str(&format!(
            "    #[value(name = \"{}\")]\n    {},\n",
            resource, variant
        ));
    }

    code.push_str(
        r#"}

impl Resource {
    /// Get all available resources
    pub fn all() -> &'static [Resource] {
        use Resource::*;
        &[
"#,
    );

    for resource in &resources {
        let variant = to_pascal_case(resource);
        code.push_str(&format!("            {},\n", variant));
    }

    code.push_str(
        r#"        ]
    }

    /// Get the GraphQL field name for this resource
    pub fn field_name(&self) -> &'static str {
        match self {
"#,
    );

    for resource in &resources {
        let variant = to_pascal_case(resource);
        code.push_str(&format!(
            "            Resource::{} => \"{}\",\n",
            variant, resource
        ));
    }

    code.push_str(
        r#"        }
    }
}
"#,
    );

    fs::write(output_dir.join("resources.rs"), code)?;
    println!("  Generated resources.rs ({} resources)", resources.len());

    Ok(())
}

fn generate_mutation_ops(
    ast: &graphql_parser::schema::Document<String>,
    output_dir: &PathBuf,
) -> Result<()> {
    use graphql_parser::schema::Definition;
    use graphql_parser::schema::TypeDefinition;

    let mut ops = Vec::new();

    for def in &ast.definitions {
        if let Definition::TypeDefinition(TypeDefinition::Object(obj)) = def {
            if obj.name == "Mutation" {
                for field in &obj.fields {
                    if !field.name.starts_with("_") {
                        ops.push(field.name.clone());
                    }
                }
            }
        }
    }

    ops.sort();

    let mut code = String::from(
        r#"//! Generated mutation operations - DO NOT EDIT
//! Run `cargo xtask codegen` to regenerate

use clap::ValueEnum;

/// Available mutation operations derived from Linear's GraphQL schema
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
#[value(rename_all = "camelCase")]
pub enum MutationOp {
"#,
    );

    for op in &ops {
        let variant = to_pascal_case(op);
        code.push_str(&format!("    /// Execute {} mutation\n", op));
        code.push_str(&format!(
            "    #[value(name = \"{}\")]\n    {},\n",
            op, variant
        ));
    }

    code.push_str(
        r#"}

impl MutationOp {
    /// Get all available mutation operations
    pub fn all() -> &'static [MutationOp] {
        use MutationOp::*;
        &[
"#,
    );

    for op in &ops {
        let variant = to_pascal_case(op);
        code.push_str(&format!("            {},\n", variant));
    }

    code.push_str(
        r#"        ]
    }

    /// Get the GraphQL operation name
    pub fn operation_name(&self) -> &'static str {
        match self {
"#,
    );

    for op in &ops {
        let variant = to_pascal_case(op);
        code.push_str(&format!(
            "            MutationOp::{} => \"{}\",\n",
            variant, op
        ));
    }

    code.push_str(
        r#"        }
    }
}
"#,
    );

    fs::write(output_dir.join("mutation_ops.rs"), code)?;
    println!("  Generated mutation_ops.rs ({} operations)", ops.len());

    Ok(())
}

/// Field information extracted from schema
#[derive(Debug, Clone)]
struct FieldInfo {
    name: String,
    is_scalar: bool,
    type_name: String,
    has_arguments: bool,
}

/// Resource information with its fields
#[derive(Debug)]
struct ResourceInfo {
    /// GraphQL query field name (e.g., "issue")
    field_name: String,
    /// Return type name (e.g., "Issue")
    type_name: String,
    /// All fields on this type
    fields: Vec<FieldInfo>,
}

fn generate_registry(
    ast: &graphql_parser::schema::Document<String>,
    output_dir: &PathBuf,
) -> Result<()> {
    use graphql_parser::schema::{Definition, Type, TypeDefinition};

    // Step 1: Build a map of type name -> fields
    let mut type_fields: HashMap<String, Vec<FieldInfo>> = HashMap::new();

    for def in &ast.definitions {
        if let Definition::TypeDefinition(TypeDefinition::Object(obj)) = def {
            let fields: Vec<FieldInfo> = obj
                .fields
                .iter()
                .map(|f| {
                    let (type_name, is_scalar) = extract_type_info(&f.field_type);
                    FieldInfo {
                        name: f.name.clone(),
                        is_scalar,
                        type_name,
                        has_arguments: !f.arguments.is_empty(),
                    }
                })
                .collect();
            type_fields.insert(obj.name.clone(), fields);
        }
    }

    // Step 2: Get Query fields and their return types
    let mut resources: Vec<ResourceInfo> = Vec::new();

    for def in &ast.definitions {
        if let Definition::TypeDefinition(TypeDefinition::Object(obj)) = def {
            if obj.name == "Query" {
                for field in &obj.fields {
                    // Skip internal/connection fields
                    if field.name.ends_with("Connection")
                        || field.name.starts_with("_")
                        || field.name.starts_with("__")
                    {
                        continue;
                    }

                    let (return_type, _) = extract_type_info(&field.field_type);

                    // Get the fields for this return type
                    if let Some(fields) = type_fields.get(&return_type) {
                        resources.push(ResourceInfo {
                            field_name: field.name.clone(),
                            type_name: return_type,
                            fields: fields.clone(),
                        });
                    }
                }
            }
        }
    }

    resources.sort_by(|a, b| a.field_name.cmp(&b.field_name));

    // Step 3: Generate registry.rs
    let mut code = String::from(
        r#"//! Generated field registry - DO NOT EDIT
//! Run `cargo xtask codegen` to regenerate

use super::Resource;

/// Field selection preset
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldPreset {
    /// Minimal fields (id + name-like field)
    Minimal,
    /// Default fields
    Default,
    /// Wide field selection
    Wide,
}

/// Get the fields to select for a resource type with preset
pub fn get_preset_fields(resource: Resource, preset: FieldPreset) -> &'static str {
    match preset {
        FieldPreset::Minimal => get_minimal_fields(resource),
        FieldPreset::Default => get_default_fields(resource),
        FieldPreset::Wide => get_wide_fields(resource),
    }
}

/// Get minimal fields (id + name-like field)
pub fn get_minimal_fields(resource: Resource) -> &'static str {
    match resource {
"#,
    );

    // Generate minimal presets
    for res in &resources {
        let variant = to_pascal_case(&res.field_name);
        let fields = generate_minimal_fields(&res.fields);
        code.push_str(&format!(
            "        Resource::{} => \"{}\",\n",
            variant, fields
        ));
    }

    // Add catch-all pattern
    code.push_str("        _ => \"id\",\n");

    code.push_str(
        r#"    }
}

/// Get default fields (minimal + createdAt + key relations)
pub fn get_default_fields(resource: Resource) -> &'static str {
    match resource {
"#,
    );

    // Generate default presets
    for res in &resources {
        let variant = to_pascal_case(&res.field_name);
        let fields = generate_default_fields(&res.fields);
        code.push_str(&format!(
            "        Resource::{} => \"{}\",\n",
            variant, fields
        ));
    }

    // Add catch-all pattern
    code.push_str("        _ => \"id\",\n");

    code.push_str(
        r#"    }
}

/// Get wide fields (all scalar fields + relations with name)
pub fn get_wide_fields(resource: Resource) -> &'static str {
    match resource {
"#,
    );

    // Generate wide presets
    for res in &resources {
        let variant = to_pascal_case(&res.field_name);
        let fields = generate_wide_fields(&res.fields);
        code.push_str(&format!(
            "        Resource::{} => \"{}\",\n",
            variant, fields
        ));
    }

    // Add catch-all pattern
    code.push_str("        _ => \"id\",\n");

    code.push_str(
        r#"    }
}

/// Get default fields for single entity queries (more detailed)
pub fn get_entity_fields(resource: Resource) -> &'static str {
    // For single entity queries, use wide preset
    get_wide_fields(resource)
}

/// Get default fields for relation expansion
pub fn get_relation_fields(relation: &str) -> &'static str {
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
"#,
    );

    fs::write(output_dir.join("registry.rs"), code)?;
    println!("  Generated registry.rs ({} resources)", resources.len());

    Ok(())
}

/// Extract type name and whether it's a scalar from a GraphQL type
fn extract_type_info(ty: &graphql_parser::schema::Type<String>) -> (String, bool) {
    use graphql_parser::schema::Type;

    match ty {
        Type::NamedType(name) => {
            let is_scalar = is_scalar_type(name);
            (name.clone(), is_scalar)
        }
        Type::NonNullType(inner) => extract_type_info(inner),
        Type::ListType(inner) => extract_type_info(inner),
    }
}

/// Check if a type name is a scalar type
fn is_scalar_type(name: &str) -> bool {
    matches!(
        name,
        "ID" | "String"
            | "Int"
            | "Float"
            | "Boolean"
            | "DateTime"
            | "JSON"
            | "JSONObject"
            | "TimelessDate"
            | "UUID"
    )
}

/// Name-like fields in order of preference
const NAME_FIELDS: &[&str] = &[
    "identifier",
    "title",
    "name",
    "key",
    "number",
    "url",
    "email",
    "body",
    "service",
    "type",
];

/// Generate minimal fields (id + first name-like field found)
fn generate_minimal_fields(fields: &[FieldInfo]) -> String {
    let mut result = vec!["id".to_string()];

    // Find first name-like field
    for name_field in NAME_FIELDS {
        if fields
            .iter()
            .any(|f| f.name == *name_field && f.is_scalar && !f.has_arguments)
        {
            result.push(name_field.to_string());
            break;
        }
    }

    result.join(" ")
}

/// Generate default fields (minimal + createdAt + state if exists)
fn generate_default_fields(fields: &[FieldInfo]) -> String {
    let mut result = vec!["id".to_string()];

    // Add name-like fields (can have multiple for default)
    for name_field in NAME_FIELDS.iter().take(3) {
        if fields
            .iter()
            .any(|f| f.name == *name_field && f.is_scalar && !f.has_arguments)
        {
            result.push(name_field.to_string());
        }
    }

    // Add common timestamp fields
    for field in ["createdAt", "updatedAt"] {
        if fields
            .iter()
            .any(|f| f.name == field && f.is_scalar && !f.has_arguments)
        {
            result.push(field.to_string());
            break; // Only add one timestamp field for default
        }
    }

    // Add state relation if exists (common pattern)
    if fields.iter().any(|f| f.name == "state" && !f.is_scalar) {
        result.push("state { name }".to_string());
    }

    result.join(" ")
}

/// Generate wide fields (all useful scalar fields + key relations)
fn generate_wide_fields(fields: &[FieldInfo]) -> String {
    let mut result = vec!["id".to_string()];

    // Skip internal/less useful fields
    let skip_fields = [
        "activitySummary",
        "descriptionState",
        "reactionData",
        "sortOrder",
        "boardOrder",
        "subIssueSortOrder",
        "contextualMetadata",
        "signalMetadata",
        "sourceMetadata",
    ];

    // Add all simple scalar fields (no arguments)
    for field in fields {
        if field.is_scalar
            && !field.has_arguments
            && field.name != "id"
            && !skip_fields.contains(&field.name.as_str())
        {
            result.push(field.name.clone());
        }
    }

    // Add key relations with their name field
    let key_relations = [
        ("state", "name color type"),
        ("assignee", "name email"),
        ("creator", "name"),
        ("team", "name key"),
        ("user", "name"),
        ("project", "name"),
        ("cycle", "name number"),
        ("organization", "name"),
    ];

    for (rel_name, rel_fields) in key_relations {
        if fields.iter().any(|f| f.name == rel_name && !f.is_scalar) {
            result.push(format!("{} {{ {} }}", rel_name, rel_fields));
        }
    }

    result.join(" ")
}

fn generate_mod_rs(output_dir: &PathBuf) -> Result<()> {
    let code = r#"//! Generated code - DO NOT EDIT
//! Run `cargo xtask codegen` to regenerate

mod mutation_ops;
mod registry;
mod resources;

pub use mutation_ops::MutationOp;
pub use registry::{
    get_default_fields, get_entity_fields, get_minimal_fields, get_preset_fields,
    get_relation_fields, get_wide_fields, FieldPreset,
};
pub use resources::Resource;
"#;

    fs::write(output_dir.join("mod.rs"), code)?;
    println!("  Generated mod.rs");

    Ok(())
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
