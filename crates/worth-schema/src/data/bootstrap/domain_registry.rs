use forge_relational::facade::config::{CascadeDeletePolicy, CrossContextPolicy};
use forge_relational::facade::schema::{
    EntityKindRegistration, RelationKindRegistration, RelationPayloadClass,
    RelationalSchemaRegistry, SchemaRegistryError,
};

use crate::data::bootstrap::entity_aspects::entity_aspects;
use crate::data::bootstrap::relation_aspects::relation_aspects;
use crate::data::bootstrap::relation_integrity::relation_integrity;
use crate::data::bootstrap::schema_identity::{schema_id, schema_version_id};
use crate::data::entities::{
    WorthDiagnosticsEntityKind, WorthGeometryEntityKind, WorthNamingEntityKind,
    WorthTopologyEntityKind,
};
use crate::data::relations::{
    WorthDiagnosticsRelationKind, WorthGeometryRelationKind, WorthNamingRelationKind,
    WorthTopologyRelationKind,
};

pub fn register_topology_schema(
    mut registry: RelationalSchemaRegistry,
) -> Result<RelationalSchemaRegistry, SchemaRegistryError> {
    for kind in WorthTopologyEntityKind::WRAPPED_ALL {
        registry = registry.register_entity_kind(EntityKindRegistration {
            kind_id: kind.kind_id(),
            kind_name: kind.kind_name().to_string(),
            schema_id: schema_id(),
            schema_version_id: schema_version_id(),
            aspect_declarations: entity_aspects(kind),
        })?;
    }

    for kind in WorthTopologyRelationKind::WRAPPED_ALL {
        registry = registry.register_relation_kind(RelationKindRegistration {
            kind_id: kind.kind_id(),
            kind_name: kind.kind_name().to_string(),
            schema_id: schema_id(),
            schema_version_id: schema_version_id(),
            payload_class: RelationPayloadClass::TopologyOnlyRelation,
            cross_context_policy: CrossContextPolicy::Forbid,
            cascade_delete_policy: CascadeDeletePolicy::CascadeDeleteRelations,
            aspect_declarations: relation_aspects(kind),
            relation_integrity: relation_integrity(kind),
        })?;
    }

    Ok(registry)
}

pub fn register_geometry_schema(
    mut registry: RelationalSchemaRegistry,
) -> Result<RelationalSchemaRegistry, SchemaRegistryError> {
    for kind in WorthGeometryEntityKind::WRAPPED_ALL {
        registry = registry.register_entity_kind(EntityKindRegistration {
            kind_id: kind.kind_id(),
            kind_name: kind.kind_name().to_string(),
            schema_id: schema_id(),
            schema_version_id: schema_version_id(),
            aspect_declarations: entity_aspects(kind),
        })?;
    }

    for kind in WorthGeometryRelationKind::WRAPPED_ALL {
        registry = registry.register_relation_kind(RelationKindRegistration {
            kind_id: kind.kind_id(),
            kind_name: kind.kind_name().to_string(),
            schema_id: schema_id(),
            schema_version_id: schema_version_id(),
            payload_class: RelationPayloadClass::TopologyOnlyRelation,
            cross_context_policy: CrossContextPolicy::Forbid,
            cascade_delete_policy: CascadeDeletePolicy::CascadeDeleteRelations,
            aspect_declarations: relation_aspects(kind),
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
    for kind in WorthNamingEntityKind::WRAPPED_ALL {
        registry = registry.register_entity_kind(EntityKindRegistration {
            kind_id: kind.kind_id(),
            kind_name: kind.kind_name().to_string(),
            schema_id: schema_id(),
            schema_version_id: schema_version_id(),
            aspect_declarations: entity_aspects(kind),
        })?;
    }

    for kind in WorthNamingRelationKind::WRAPPED_ALL {
        registry = registry.register_relation_kind(RelationKindRegistration {
            kind_id: kind.kind_id(),
            kind_name: kind.kind_name().to_string(),
            schema_id: schema_id(),
            schema_version_id: schema_version_id(),
            payload_class: RelationPayloadClass::TopologyOnlyRelation,
            cross_context_policy: CrossContextPolicy::Forbid,
            cascade_delete_policy: CascadeDeletePolicy::CascadeDeleteRelations,
            aspect_declarations: relation_aspects(kind),
            relation_integrity: relation_integrity(kind),
        })?;
    }

    Ok(registry)
}

pub fn register_diagnostics_schema(
    mut registry: RelationalSchemaRegistry,
) -> Result<RelationalSchemaRegistry, SchemaRegistryError> {
    for kind in WorthDiagnosticsEntityKind::WRAPPED_ALL {
        registry = registry.register_entity_kind(EntityKindRegistration {
            kind_id: kind.kind_id(),
            kind_name: kind.kind_name().to_string(),
            schema_id: schema_id(),
            schema_version_id: schema_version_id(),
            aspect_declarations: entity_aspects(kind),
        })?;
    }

    for kind in WorthDiagnosticsRelationKind::WRAPPED_ALL {
        registry = registry.register_relation_kind(RelationKindRegistration {
            kind_id: kind.kind_id(),
            kind_name: kind.kind_name().to_string(),
            schema_id: schema_id(),
            schema_version_id: schema_version_id(),
            payload_class: RelationPayloadClass::TopologyOnlyRelation,
            cross_context_policy: CrossContextPolicy::Forbid,
            cascade_delete_policy: CascadeDeletePolicy::CascadeDeleteRelations,
            aspect_declarations: relation_aspects(kind),
            relation_integrity: relation_integrity(kind),
        })?;
    }

    Ok(registry)
}
