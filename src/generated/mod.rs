//! Generated code - DO NOT EDIT
//! Run `cargo xtask codegen` to regenerate

mod mutation_ops;
mod mutation_registry;
mod registry;
mod resources;
mod search_plan;

pub use mutation_ops::MutationOp;
pub use mutation_registry::{
    get_mutation_entity_field, get_mutation_result_fields, mutation_returns_entity,
};
pub use registry::{
    get_default_fields, get_entity_fields, get_minimal_fields, get_preset_fields,
    get_relation_fields, get_wide_fields, FieldPreset,
};
pub use resources::Resource;
pub use search_plan::{get_search_filter, get_searchable_fields, supports_search};
