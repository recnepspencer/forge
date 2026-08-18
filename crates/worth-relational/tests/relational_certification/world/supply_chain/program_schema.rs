use super::schema_vocabulary::{
    endpoint_integrity, entity_aspects, entity_kind_id, relation_aspects, relation_kind_id,
};
use super::semantic_key::{EntityKind, RelationKind};
use worth_relational::facade::config::{CascadeDeletePolicy, CrossContextPolicy};
use worth_relational::facade::schema::{
    EntityKindRegistration, KindAspectContractDeclarations, RelationKindRegistration,
    RelationalSchemaRegistry, SchemaId, SchemaRegistryError, SchemaVersionId,
};

pub(crate) fn schema_registry() -> Result<RelationalSchemaRegistry, SchemaRegistryError> {
    let mut registry = RelationalSchemaRegistry::new();
    for kind in [
        EntityKind::Port,
        EntityKind::Terminal,
        EntityKind::Berth,
        EntityKind::Vessel,
        EntityKind::Voyage,
        EntityKind::PortCall,
        EntityKind::CargoLot,
        EntityKind::Inspection,
    ] {
        registry = registry.register_entity_kind(EntityKindRegistration {
            kind_id: entity_kind_id(kind),
            kind_name: format!("supply_chain.entity.{kind:?}"),
            schema_id: SchemaId("supply_chain".to_owned()),
            schema_version_id: SchemaVersionId(1),
            aspect_contract_declarations: KindAspectContractDeclarations::new(entity_aspects(kind)),
        })?;
    }
    for kind in [
        RelationKind::TerminalAtPort,
        RelationKind::BerthAtTerminal,
        RelationKind::VesselAssignedToBerth,
        RelationKind::VoyageUsesVessel,
        RelationKind::VoyageHasCall,
        RelationKind::CallAtPort,
        RelationKind::CallPrecedes,
        RelationKind::CargoBookedOnVoyage,
        RelationKind::InspectionCoversVessel,
        RelationKind::SharesPilotageZone,
    ] {
        registry = registry.register_relation_kind(RelationKindRegistration {
            kind_id: relation_kind_id(kind),
            kind_name: format!("supply_chain.relation.{kind:?}"),
            schema_id: SchemaId("supply_chain".to_owned()),
            schema_version_id: SchemaVersionId(1),
            cross_context_policy: CrossContextPolicy::AllowExplicit,
            cascade_delete_policy: CascadeDeletePolicy::CascadeDeleteRelations,
            aspect_contract_declarations: KindAspectContractDeclarations::new(relation_aspects()),
            relation_integrity: endpoint_integrity(kind),
        })?;
    }
    Ok(registry)
}
