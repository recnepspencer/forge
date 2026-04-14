use forge_relational::facade::schema::{RelationalSchemaRegistry, SchemaRegistryError};

use crate::data::bootstrap::domain_registry::{
    register_diagnostics_schema, register_geometry_schema, register_lineage_schema,
    register_naming_schema, register_topology_schema,
};

pub fn worth_bootstrap_schema_registry() -> Result<RelationalSchemaRegistry, SchemaRegistryError> {
    let registry = RelationalSchemaRegistry::new();
    let registry = register_topology_schema(registry)?;
    let registry = register_geometry_schema(registry)?;
    let registry = register_lineage_schema(registry)?;
    let registry = register_naming_schema(registry)?;
    register_diagnostics_schema(registry)
}
