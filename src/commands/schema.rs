//! Schema commands: info, diff

use anyhow::Result;

use crate::cli::{Cli, SchemaAction};
use crate::schema_diff;

/// Handle schema-related commands
pub async fn cmd_schema(_cli: &Cli, action: SchemaAction) -> Result<()> {
    match action {
        SchemaAction::Info => {
            let meta_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("schemas/linear/schema.meta.json");

            if meta_path.exists() {
                let content = std::fs::read_to_string(&meta_path)?;
                let meta: serde_json::Value = serde_json::from_str(&content)?;
                println!("Schema Information:");
                println!(
                    "  Source: {}",
                    meta["source"].as_str().unwrap_or("unknown")
                );
                println!(
                    "  Commit: {}",
                    meta["commit"].as_str().unwrap_or("unknown")
                );
                println!(
                    "  Synced At: {}",
                    meta["syncedAt"].as_str().unwrap_or("unknown")
                );
                // Build GitHub permalink if commit is available
                if let (Some(source), Some(commit)) =
                    (meta["source"].as_str(), meta["commit"].as_str())
                {
                    if source.contains("github.com") {
                        println!("  Permalink: {}/tree/{}", source, commit);
                    }
                }
            } else {
                println!("No schema metadata found.");
                println!("Run 'cargo xtask schema sync' to sync the schema.");
            }
        }
        SchemaAction::Diff => {
            use std::path::PathBuf;

            // Path to local schema
            let schema_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("schemas")
                .join("linear")
                .join("schema.graphql");

            if !schema_path.exists() {
                println!("No local schema found.");
                println!("Run 'cargo xtask schema sync' to sync the schema first.");
                return Ok(());
            }

            println!("Fetching upstream schema from Linear SDK...");

            // Fetch upstream schema
            let upstream_content = schema_diff::fetch_upstream_schema().await?;

            // Read local schema
            let local_content = schema_diff::read_local_schema(&schema_path)?;

            println!("Comparing schemas...\n");

            // Compare schemas
            let diff = schema_diff::diff_schemas(&local_content, &upstream_content)?;

            // Print the diff
            println!("{}", diff.format());

            if !diff.is_empty() {
                println!("Run 'cargo xtask schema sync' to update the local schema.");
            }
        }
    }

    Ok(())
}
