mod builder;
mod domain_registry;
mod entity_aspects;
mod invariant_plan;
mod registry;
mod relation_aspects;
mod relation_integrity;
mod runtime_invariants;
mod schema_identity;
mod tracing_plan;

pub use builder::{SchemaBuildError, SchemaBuilder};
pub use invariant_plan::{bootstrap_invariant_plan, BootstrapInvariantPlan};
pub use registry::bootstrap_schema_registry;
pub use runtime_invariants::{
    bootstrap_runtime_invariant_plan, BootstrapRuntimeInvariant, BootstrapRuntimeInvariantPlan,
};
pub use schema_identity::{SCHEMA_ID, SCHEMA_VERSION_ID};
pub use tracing_plan::{bootstrap_tracing_plan, BootstrapTracingPlan};

#[cfg(test)]
mod tests;
