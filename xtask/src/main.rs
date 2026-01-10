//! xtask - Build automation tasks for linears
//!
//! Run with: cargo xtask <command>

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::fs;
use std::path::PathBuf;

const SCHEMA_URL: &str =
    "https://raw.githubusercontent.com/linear/linear/master/packages/sdk/src/schema.graphql";

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

    let response = reqwest::blocking::get(SCHEMA_URL)
        .context("Failed to fetch schema")?
        .text()
        .context("Failed to read schema response")?;

    let schema_path = schema_dir().join("schema.graphql");
    fs::create_dir_all(schema_dir())?;
    fs::write(&schema_path, &response).context("Failed to write schema file")?;

    println!("Schema written to: {}", schema_path.display());

    // Update metadata
    let meta = serde_json::json!({
        "source": SCHEMA_URL,
        "syncedAt": chrono::Utc::now().to_rfc3339(),
        "note": "Schema synced from Linear SDK repository"
    });

    let meta_path = schema_dir().join("schema.meta.json");
    fs::write(&meta_path, serde_json::to_string_pretty(&meta)?)
        .context("Failed to write metadata")?;

    println!("Metadata written to: {}", meta_path.display());
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
    println!(
        "  Synced At: {}",
        meta["syncedAt"].as_str().unwrap_or("unknown")
    );

    let schema_path = schema_dir().join("schema.graphql");
    if schema_path.exists() {
        let content = fs::read_to_string(&schema_path)?;
        let lines = content.lines().count();
        println!("  Schema Lines: {}", lines);
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

fn generate_mod_rs(output_dir: &PathBuf) -> Result<()> {
    let code = r#"//! Generated code - DO NOT EDIT
//! Run `cargo xtask codegen` to regenerate

mod mutation_ops;
mod resources;

pub use mutation_ops::MutationOp;
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
