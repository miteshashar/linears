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

    // Generate search plans
    generate_search_plans(&ast, &generated_dir)?;

    // Generate mutation registry
    generate_mutation_registry(&ast, &generated_dir)?;

    // Generate order_by enum
    generate_order_by(&ast, &generated_dir)?;

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
    // Map of field_name -> return_type_name for building plural mappings
    let mut field_to_type: HashMap<String, String> = HashMap::new();

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
                        // Extract return type
                        let (type_name, _) = extract_type_info(&field.field_type);
                        field_to_type.insert(field.name.clone(), type_name);
                    }
                }
            }
        }
    }

    resources.sort();

    // Build singular -> plural mapping
    // A field is plural of another if:
    // 1. Plural field returns TypeConnection
    // 2. Singular field returns Type (same base name)
    // 3. The plural field name is a natural plural of the singular (singular + "s" or similar)
    // Example: issue -> Issue, issues -> IssueConnection
    let mut singular_to_plural: HashMap<String, String> = HashMap::new();

    for (field_name, return_type) in &field_to_type {
        // Check if this is a Connection type (plural)
        if return_type.ends_with("Connection") {
            // Extract base type: IssueConnection -> Issue
            let base_type = return_type.strip_suffix("Connection").unwrap_or(return_type);

            // Find singular field that returns this base type AND has matching name pattern
            for (other_field, other_type) in &field_to_type {
                if other_type == base_type && other_field != field_name {
                    // Verify this is a natural plural: plural should be singular+"s" or singular+"es" or "y"->"ies"
                    let is_natural_plural = field_name == &format!("{}s", other_field)
                        || field_name == &format!("{}es", other_field)
                        || (other_field.ends_with('y')
                            && field_name == &format!("{}ies", &other_field[..other_field.len() - 1]));

                    if is_natural_plural {
                        singular_to_plural.insert(other_field.clone(), field_name.clone());
                    }
                }
            }
        }
    }

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

    /// Get the plural GraphQL field name for this resource.
    /// Used for list queries. Returns the schema-defined plural or the field name itself
    /// if this resource already represents a collection.
    pub fn plural_name(&self) -> &'static str {
        match self {
"#,
    );

    // Generate plural_name match arms
    for resource in &resources {
        let variant = to_pascal_case(resource);
        // Use the schema-derived plural if available, otherwise use field name itself
        let plural = singular_to_plural.get(resource).unwrap_or(resource);
        code.push_str(&format!(
            "            Resource::{} => \"{}\",\n",
            variant, plural
        ));
    }

    code.push_str(
        r#"        }
    }
}
"#,
    );

    fs::write(output_dir.join("resources.rs"), code)?;
    println!(
        "  Generated resources.rs ({} resources, {} with plural mappings)",
        resources.len(),
        singular_to_plural.len()
    );

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

/// Search plan information for a resource
#[derive(Debug)]
struct SearchPlan {
    /// GraphQL query field name (e.g., "issue")
    field_name: String,
    /// Text-searchable fields in priority order
    searchable_fields: Vec<String>,
}

fn generate_search_plans(
    ast: &graphql_parser::schema::Document<String>,
    output_dir: &PathBuf,
) -> Result<()> {
    use graphql_parser::schema::{Definition, InputValue, Type, TypeDefinition};

    // Build a map of input type name -> fields with their types
    let mut input_type_fields: HashMap<String, Vec<(String, String)>> = HashMap::new();

    for def in &ast.definitions {
        if let Definition::TypeDefinition(TypeDefinition::InputObject(input)) = def {
            let fields: Vec<(String, String)> = input
                .fields
                .iter()
                .map(|f| {
                    let type_name = extract_input_type_name(&f.value_type);
                    (f.name.clone(), type_name)
                })
                .collect();
            input_type_fields.insert(input.name.clone(), fields);
        }
    }

    // Get Query fields to find resources
    let mut resources: Vec<String> = Vec::new();

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

    // Build search plans by finding Filter input types
    let mut search_plans: Vec<SearchPlan> = Vec::new();

    for resource in &resources {
        // Try to find corresponding filter type
        // Convention: Resource -> ResourceFilter (e.g., issue -> IssueFilter)
        let pascal_name = to_pascal_case(resource);
        let filter_name = format!("{}Filter", pascal_name);

        if let Some(fields) = input_type_fields.get(&filter_name) {
            // Find text-searchable fields (StringComparator or NullableStringComparator)
            let mut searchable: Vec<String> = fields
                .iter()
                .filter(|(_, type_name)| {
                    type_name == "StringComparator" || type_name == "NullableStringComparator"
                })
                .map(|(name, _)| name.clone())
                .collect();

            // Sort by priority: title, name, identifier, description, body, content, then others
            let priority_order = [
                "title",
                "name",
                "identifier",
                "key",
                "description",
                "body",
                "content",
                "email",
                "url",
            ];

            searchable.sort_by(|a, b| {
                let a_priority = priority_order.iter().position(|&x| x == a).unwrap_or(999);
                let b_priority = priority_order.iter().position(|&x| x == b).unwrap_or(999);
                a_priority.cmp(&b_priority)
            });

            // Only keep top 3 searchable fields to avoid overly complex queries
            searchable.truncate(3);

            if !searchable.is_empty() {
                search_plans.push(SearchPlan {
                    field_name: resource.clone(),
                    searchable_fields: searchable,
                });
            }
        }
    }

    // Generate search_plan.rs
    let mut code = String::from(
        r#"//! Generated search plans - DO NOT EDIT
//! Run `cargo xtask codegen` to regenerate

use super::Resource;

/// Get the search filter for a resource with the given search text.
/// Returns a JSON value suitable for use as a filter variable in GraphQL queries.
/// The filter uses OR logic across text-searchable fields.
pub fn get_search_filter(resource: Resource, text: &str) -> serde_json::Value {
    match resource {
"#,
    );

    // Generate match arms for resources with search plans
    for plan in &search_plans {
        let variant = to_pascal_case(&plan.field_name);

        if plan.searchable_fields.len() == 1 {
            // Single field - no OR needed
            let field = &plan.searchable_fields[0];
            code.push_str(&format!(
                "        Resource::{} => serde_json::json!({{ \"{}\": {{ \"containsIgnoreCase\": text }} }}),\n",
                variant, field
            ));
        } else {
            // Multiple fields - use OR
            code.push_str(&format!("        Resource::{} => serde_json::json!({{\n", variant));
            code.push_str("            \"or\": [\n");
            for field in &plan.searchable_fields {
                code.push_str(&format!(
                    "                {{ \"{}\": {{ \"containsIgnoreCase\": text }} }},\n",
                    field
                ));
            }
            code.push_str("            ]\n");
            code.push_str("        }),\n");
        }
    }

    // Default fallback - try "name" field
    code.push_str(
        r#"        _ => serde_json::json!({ "name": { "containsIgnoreCase": text } }),
    }
}

/// Check if a resource supports text search
pub fn supports_search(resource: Resource) -> bool {
    match resource {
"#,
    );

    // Generate supports_search match arms
    for plan in &search_plans {
        let variant = to_pascal_case(&plan.field_name);
        code.push_str(&format!("        Resource::{} => true,\n", variant));
    }

    code.push_str(
        r#"        _ => false,
    }
}

/// Get the searchable fields for a resource (for debugging/documentation)
pub fn get_searchable_fields(resource: Resource) -> &'static [&'static str] {
    match resource {
"#,
    );

    // Generate get_searchable_fields match arms
    for plan in &search_plans {
        let variant = to_pascal_case(&plan.field_name);
        let fields_str = plan
            .searchable_fields
            .iter()
            .map(|f| format!("\"{}\"", f))
            .collect::<Vec<_>>()
            .join(", ");
        code.push_str(&format!(
            "        Resource::{} => &[{}],\n",
            variant, fields_str
        ));
    }

    code.push_str(
        r#"        _ => &[],
    }
}
"#,
    );

    fs::write(output_dir.join("search_plan.rs"), code)?;
    println!(
        "  Generated search_plan.rs ({} resources with search support)",
        search_plans.len()
    );

    Ok(())
}

/// Extract the inner type name from an input type
fn extract_input_type_name(ty: &graphql_parser::schema::Type<String>) -> String {
    use graphql_parser::schema::Type;

    match ty {
        Type::NamedType(name) => name.clone(),
        Type::NonNullType(inner) => extract_input_type_name(inner),
        Type::ListType(inner) => extract_input_type_name(inner),
    }
}

/// Mutation info with its return entity type
#[derive(Debug)]
struct MutationInfo {
    /// Operation name (e.g., "issueCreate")
    op_name: String,
    /// Entity field name in payload (e.g., "issue")
    entity_field: Option<String>,
    /// Entity type name (e.g., "Issue")
    entity_type: Option<String>,
}

fn generate_mutation_registry(
    ast: &graphql_parser::schema::Document<String>,
    output_dir: &PathBuf,
) -> Result<()> {
    use graphql_parser::schema::{Definition, TypeDefinition};

    // Build map of type name -> fields (for payload types)
    let mut type_fields: HashMap<String, Vec<(String, String)>> = HashMap::new();

    for def in &ast.definitions {
        if let Definition::TypeDefinition(TypeDefinition::Object(obj)) = def {
            let fields: Vec<(String, String)> = obj
                .fields
                .iter()
                .map(|f| {
                    let (type_name, _) = extract_type_info(&f.field_type);
                    (f.name.clone(), type_name)
                })
                .collect();
            type_fields.insert(obj.name.clone(), fields);
        }
    }

    // Get all mutations and their return types
    let mut mutations: Vec<MutationInfo> = Vec::new();

    for def in &ast.definitions {
        if let Definition::TypeDefinition(TypeDefinition::Object(obj)) = def {
            if obj.name == "Mutation" {
                for field in &obj.fields {
                    if field.name.starts_with("_") {
                        continue;
                    }

                    let (return_type, _) = extract_type_info(&field.field_type);

                    // Try to find entity field in payload type
                    let mut entity_field = None;
                    let mut entity_type = None;

                    if let Some(payload_fields) = type_fields.get(&return_type) {
                        // Look for entity field (not success, lastSyncId, or archived)
                        for (field_name, field_type) in payload_fields {
                            if field_name != "success"
                                && field_name != "lastSyncId"
                                && field_name != "entity"
                                && !field_name.ends_with("Payload")
                            {
                                // Check if this is a real entity type (has 'id' field)
                                if let Some(type_fields) = type_fields.get(field_type) {
                                    if type_fields.iter().any(|(name, _)| name == "id") {
                                        entity_field = Some(field_name.clone());
                                        entity_type = Some(field_type.clone());
                                        break;
                                    }
                                }
                            }
                        }
                    }

                    mutations.push(MutationInfo {
                        op_name: field.name.clone(),
                        entity_field,
                        entity_type,
                    });
                }
            }
        }
    }

    mutations.sort_by(|a, b| a.op_name.cmp(&b.op_name));

    // Generate mutation_registry.rs
    let mut code = String::from(
        r#"//! Generated mutation registry - DO NOT EDIT
//! Run `cargo xtask codegen` to regenerate

use super::MutationOp;

/// Get the entity field name for a mutation's payload (e.g., "issue" for IssuePayload)
pub fn get_mutation_entity_field(op: MutationOp) -> Option<&'static str> {
    match op {
"#,
    );

    // Generate entity field match arms
    for mutation in &mutations {
        let variant = to_pascal_case(&mutation.op_name);
        if let Some(ref field) = mutation.entity_field {
            code.push_str(&format!(
                "        MutationOp::{} => Some(\"{}\"),\n",
                variant, field
            ));
        }
    }

    code.push_str(
        r#"        _ => None,
    }
}

/// Get the result fields to select for a mutation based on the entity type.
/// Returns appropriate fields for the entity returned by the mutation.
pub fn get_mutation_result_fields(resource_name: &str) -> &'static str {
    match resource_name {
"#,
    );

    // Build a set of unique entity types
    let mut entity_types: Vec<(String, String)> = Vec::new();
    for mutation in &mutations {
        if let (Some(field), Some(type_name)) = (&mutation.entity_field, &mutation.entity_type) {
            // Check if we already have this field -> type mapping
            if !entity_types.iter().any(|(f, _)| f == field) {
                entity_types.push((field.clone(), type_name.clone()));
            }
        }
    }

    // Generate result fields for each entity type
    for (field_name, type_name) in &entity_types {
        // Use minimal field set for mutations - just key identifying fields
        let fields = get_minimal_entity_fields(type_name, &type_fields);
        code.push_str(&format!(
            "        \"{}\" => \"{}\",\n",
            field_name, fields
        ));
    }

    code.push_str(
        r#"        _ => "id",
    }
}

/// Check if a mutation returns an entity (vs just success status)
pub fn mutation_returns_entity(op: MutationOp) -> bool {
    get_mutation_entity_field(op).is_some()
}
"#,
    );

    fs::write(output_dir.join("mutation_registry.rs"), code)?;
    println!(
        "  Generated mutation_registry.rs ({} mutations)",
        mutations.len()
    );

    Ok(())
}

/// Get minimal entity fields for mutation results
fn get_minimal_entity_fields(type_name: &str, type_fields: &HashMap<String, Vec<(String, String)>>) -> String {
    let mut result = vec!["id".to_string()];

    // Name-like fields in order of preference
    let name_fields = ["identifier", "title", "name", "key", "number", "url", "body", "email"];

    if let Some(fields) = type_fields.get(type_name) {
        // Find first name-like field that exists and is scalar
        for name_field in name_fields {
            if fields.iter().any(|(name, type_name)| {
                name == name_field && is_scalar_type(type_name)
            }) {
                result.push(name_field.to_string());
                break;
            }
        }

        // Also add a second identifier if present (like title for issues)
        if result.len() == 2 && result[1] == "identifier" {
            if fields.iter().any(|(name, type_name)| name == "title" && is_scalar_type(type_name)) {
                result.push("title".to_string());
            }
        }
    }

    result.join(" ")
}

fn generate_order_by(
    ast: &graphql_parser::schema::Document<String>,
    output_dir: &PathBuf,
) -> Result<()> {
    use graphql_parser::schema::{Definition, TypeDefinition};

    // Find PaginationOrderBy enum
    let mut order_by_values: Vec<String> = Vec::new();

    for def in &ast.definitions {
        if let Definition::TypeDefinition(TypeDefinition::Enum(enum_def)) = def {
            if enum_def.name == "PaginationOrderBy" {
                for value in &enum_def.values {
                    order_by_values.push(value.name.clone());
                }
            }
        }
    }

    // Generate order_by.rs
    let mut code = String::from(
        r#"//! Generated order_by enum - DO NOT EDIT
//! Run `cargo xtask codegen` to regenerate

use clap::ValueEnum;
use std::fmt;

/// Pagination order by field
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum OrderBy {
"#,
    );

    // Generate enum variants
    for value in &order_by_values {
        let variant = to_pascal_case(value);
        code.push_str(&format!("    /// Order by {}\n", value));
        code.push_str(&format!("    {},\n", variant));
    }

    code.push_str(
        r#"}

impl OrderBy {
    /// Get the GraphQL value for this order by field
    pub fn as_graphql_value(&self) -> &'static str {
        match self {
"#,
    );

    for value in &order_by_values {
        let variant = to_pascal_case(value);
        code.push_str(&format!(
            "            OrderBy::{} => \"{}\",\n",
            variant, value
        ));
    }

    code.push_str(
        r#"        }
    }
}

impl fmt::Display for OrderBy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_graphql_value())
    }
}
"#,
    );

    fs::write(output_dir.join("order_by.rs"), code)?;
    println!(
        "  Generated order_by.rs ({} order by values)",
        order_by_values.len()
    );

    Ok(())
}

fn generate_mod_rs(output_dir: &PathBuf) -> Result<()> {
    let code = r#"//! Generated code - DO NOT EDIT
//! Run `cargo xtask codegen` to regenerate

mod mutation_ops;
mod mutation_registry;
mod order_by;
mod registry;
mod resources;
mod search_plan;

pub use mutation_ops::MutationOp;
pub use mutation_registry::{
    get_mutation_entity_field, get_mutation_result_fields, mutation_returns_entity,
};
pub use order_by::OrderBy;
pub use registry::{
    get_default_fields, get_entity_fields, get_minimal_fields, get_preset_fields,
    get_relation_fields, get_wide_fields, FieldPreset,
};
pub use resources::Resource;
pub use search_plan::{get_search_filter, get_searchable_fields, supports_search};
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
