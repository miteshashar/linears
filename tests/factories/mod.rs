//! Test factories for generating deterministic test data
//!
//! All factories produce predictable, reproducible output based on input parameters.
//! This enables snapshot testing and reliable test assertions.

pub mod generated;

pub use generated::*;
