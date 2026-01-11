//! Core commonal types, utilities, and constants
//!
//! This module provides app-level abstractions that are imported by other modules.
//! Dependency direction: core → generated → cli/commands

pub mod constants;
mod strings;
mod types;

pub use strings::*;
pub use types::*;
