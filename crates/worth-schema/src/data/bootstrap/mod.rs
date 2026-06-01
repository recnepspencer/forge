mod builder;
mod domain_registry;
mod entity_aspects;
mod registry;
mod relation_aspects;
mod relation_integrity;
mod schema_identity;

pub use builder::{SchemaBuildError, SchemaBuilder};
pub use registry::bootstrap_schema_registry;
pub use schema_identity::{SCHEMA_ID, SCHEMA_VERSION_ID};

#[cfg(test)]
mod tests;
