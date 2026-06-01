use forge_relational::facade::config::{CascadeDeletePolicy, CrossContextPolicy};
use forge_relational::facade::schema::{
    EntityKindRegistration, RelationKindRegistration, RelationalSchemaRegistry, SchemaRegistryError,
};

use crate::data::bootstrap::entity_aspects::entity_aspects;
use crate::data::bootstrap::relation_aspects::relation_aspects;
use crate::data::bootstrap::relation_integrity::relation_integrity;
use crate::data::bootstrap::schema_identity::{schema_id, schema_version_id};
use crate::data::entities::{
    DiagnosticsEntityKind, GeometryEntityKind, NamingEntityKind, TopologyEntityKind,
};
use crate::data::relations::{
    DiagnosticsRelationKind, GeometryRelationKind, NamingRelationKind, TopologyRelationKind,
};

pub fn register_topology_schema(
    mut registry: RelationalSchemaRegistry,
) -> Result<RelationalSchemaRegistry, SchemaRegistryError> {
    for kind in TopologyEntityKind::WRAPPED_ALL {
        registry = registry.register_entity_kind(EntityKindRegistration {
            kind_id: kind.kind_id(),
            kind_name: kind.kind_name().to_string(),
            schema_id: schema_id(),
            schema_version_id: schema_version_id(),
            aspect_contract_declarations: entity_aspects(kind),
        })?;
    }

    for kind in TopologyRelationKind::WRAPPED_ALL {
        registry = registry.register_relation_kind(RelationKindRegistration {
            kind_id: kind.kind_id(),
            kind_name: kind.kind_name().to_string(),
            schema_id: schema_id(),
            schema_version_id: schema_version_id(),
            cross_context_policy: CrossContextPolicy::Forbid,
            cascade_delete_policy: CascadeDeletePolicy::CascadeDeleteRelations,
            aspect_contract_declarations: relation_aspects(kind),
            relation_integrity: relation_integrity(kind),
        })?;
    }

    Ok(registry)
}

pub fn register_geometry_schema(
    mut registry: RelationalSchemaRegistry,
) -> Result<RelationalSchemaRegistry, SchemaRegistryError> {
    for kind in GeometryEntityKind::WRAPPED_ALL {
        registry = registry.register_entity_kind(EntityKindRegistration {
            kind_id: kind.kind_id(),
            kind_name: kind.kind_name().to_string(),
            schema_id: schema_id(),
            schema_version_id: schema_version_id(),
            aspect_contract_declarations: entity_aspects(kind),
        })?;
    }

    for kind in GeometryRelationKind::WRAPPED_ALL {
        registry = registry.register_relation_kind(RelationKindRegistration {
            kind_id: kind.kind_id(),
            kind_name: kind.kind_name().to_string(),
            schema_id: schema_id(),
            schema_version_id: schema_version_id(),
            cross_context_policy: CrossContextPolicy::Forbid,
            cascade_delete_policy: CascadeDeletePolicy::CascadeDeleteRelations,
            aspect_contract_declarations: relation_aspects(kind),
            relation_integrity: relation_integrity(kind),
        })?;
    }

    Ok(registry)
}

pub fn register_lineage_schema(
    registry: RelationalSchemaRegistry,
) -> Result<RelationalSchemaRegistry, SchemaRegistryError> {
    Ok(registry)
}

pub fn register_naming_schema(
    mut registry: RelationalSchemaRegistry,
) -> Result<RelationalSchemaRegistry, SchemaRegistryError> {
    for kind in NamingEntityKind::WRAPPED_ALL {
        registry = registry.register_entity_kind(EntityKindRegistration {
            kind_id: kind.kind_id(),
            kind_name: kind.kind_name().to_string(),
            schema_id: schema_id(),
            schema_version_id: schema_version_id(),
            aspect_contract_declarations: entity_aspects(kind),
        })?;
    }

    for kind in NamingRelationKind::WRAPPED_ALL {
        registry = registry.register_relation_kind(RelationKindRegistration {
            kind_id: kind.kind_id(),
            kind_name: kind.kind_name().to_string(),
            schema_id: schema_id(),
            schema_version_id: schema_version_id(),
            cross_context_policy: CrossContextPolicy::Forbid,
            cascade_delete_policy: CascadeDeletePolicy::CascadeDeleteRelations,
            aspect_contract_declarations: relation_aspects(kind),
            relation_integrity: relation_integrity(kind),
        })?;
    }

    Ok(registry)
}

pub fn register_diagnostics_schema(
    mut registry: RelationalSchemaRegistry,
) -> Result<RelationalSchemaRegistry, SchemaRegistryError> {
    for kind in DiagnosticsEntityKind::WRAPPED_ALL {
        registry = registry.register_entity_kind(EntityKindRegistration {
            kind_id: kind.kind_id(),
            kind_name: kind.kind_name().to_string(),
            schema_id: schema_id(),
            schema_version_id: schema_version_id(),
            aspect_contract_declarations: entity_aspects(kind),
        })?;
    }

    for kind in DiagnosticsRelationKind::WRAPPED_ALL {
        registry = registry.register_relation_kind(RelationKindRegistration {
            kind_id: kind.kind_id(),
            kind_name: kind.kind_name().to_string(),
            schema_id: schema_id(),
            schema_version_id: schema_version_id(),
            cross_context_policy: CrossContextPolicy::Forbid,
            cascade_delete_policy: CascadeDeletePolicy::CascadeDeleteRelations,
            aspect_contract_declarations: relation_aspects(kind),
            relation_integrity: relation_integrity(kind),
        })?;
    }

    Ok(registry)
}
