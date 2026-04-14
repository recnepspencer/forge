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

pub use builder::{WorthSchemaBuildError, WorthSchemaBuilder};
pub use invariant_plan::{worth_bootstrap_invariant_plan, WorthBootstrapInvariantPlan};
pub use registry::worth_bootstrap_schema_registry;
pub use runtime_invariants::{
    worth_bootstrap_runtime_invariant_plan, WorthBootstrapRuntimeInvariant,
    WorthBootstrapRuntimeInvariantPlan,
};
pub use schema_identity::{WORTH_SCHEMA_ID, WORTH_SCHEMA_VERSION_ID};
pub use tracing_plan::{worth_bootstrap_tracing_plan, WorthBootstrapTracingPlan};

#[cfg(test)]
mod tests;
