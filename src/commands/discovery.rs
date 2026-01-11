//! Discovery commands: resources, ops

use anyhow::Result;

use crate::cli::Cli;
use crate::generated::{MutationOp, Resource};
use crate::render;

/// List all available query resources
pub fn cmd_resources(cli: &Cli) -> Result<()> {
    let resources = Resource::all();
    let resource_names: Vec<&str> = resources.iter().map(|r| r.field_name()).collect();

    println!(
        "{}",
        render::render_resources(cli.global.output, &resource_names, cli.global.pretty)
    );

    Ok(())
}

/// List all available mutation operations
pub fn cmd_ops(cli: &Cli) -> Result<()> {
    let ops = MutationOp::all();
    let op_names: Vec<&str> = ops.iter().map(|o| o.operation_name()).collect();

    println!(
        "{}",
        render::render_ops(cli.global.output, &op_names, cli.global.pretty)
    );

    Ok(())
}
