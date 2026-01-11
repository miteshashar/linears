//! Core type definitions

use clap::ValueEnum;

/// Field selection presets for queries
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum FieldsetPreset {
    /// Minimal fields (id, name/title)
    Minimal,
    /// Default fields
    Default,
    /// Wide field selection
    Wide,
}

/// Exit codes for the CLI (per PRD §9)
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExitCode {
    Success = 0,
    GeneralError = 1,
    AuthError = 2,
    NetworkError = 3,
    GraphQLError = 4,
}

impl From<ExitCode> for std::process::ExitCode {
    fn from(code: ExitCode) -> Self {
        std::process::ExitCode::from(code as u8)
    }
}

/// ID type detection result
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IdType {
    Uuid,
    Identifier,
    Unknown,
}
