use serde::Deserialize;

use crate::identity::data::KindId;
use crate::schema::data::{
    AspectContractPlanRevision, RelationIntegrityPlanRevision, RelationalSchemaRegistry,
    SchemaAuthorityKindSnapshot, SchemaId, SchemaVersionId,
};

use super::relation_integrity_revision::derive_legacy_relation_integrity_plan_revision;

/// Exact schema-authority bytes written by WORTH Query 9.16.1.1. This type is
/// migration vocabulary and can never act as current schema authority.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub(super) struct LegacySchemaAuthoritySnapshot {
    primary_schema_id: Option<SchemaId>,
    primary_schema_version_id: Option<SchemaVersionId>,
    entity_kinds: Vec<SchemaAuthorityKindSnapshot>,
    relation_kinds: Vec<LegacySchemaAuthorityRelationSnapshot>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct LegacySchemaAuthorityRelationSnapshot {
    kind_id: KindId,
    kind_name: String,
    schema_id: SchemaId,
    schema_version_id: SchemaVersionId,
    aspect_plan_revision: AspectContractPlanRevision,
    relation_integrity_plan_revision: RelationIntegrityPlanRevision,
}

impl LegacySchemaAuthoritySnapshot {
    pub(super) fn readmit(
        &self,
        registry: &RelationalSchemaRegistry,
    ) -> Result<crate::schema::data::SchemaAuthoritySnapshot, String> {
        if self != &expected_legacy_authority(registry) {
            return Err(
                "9.16.1.1 schema authority does not match the configured runtime registry"
                    .to_owned(),
            );
        }
        Ok(registry.authority_snapshot())
    }
}

fn expected_legacy_authority(registry: &RelationalSchemaRegistry) -> LegacySchemaAuthoritySnapshot {
    let current = registry.authority_snapshot();
    let relation_kinds = registry
        .relation_kinds
        .iter()
        .map(
            |(kind_id, registration)| LegacySchemaAuthorityRelationSnapshot {
                kind_id: *kind_id,
                kind_name: registration.kind_name.clone(),
                schema_id: registration.schema_id.clone(),
                schema_version_id: registration.schema_version_id,
                aspect_plan_revision: registration.aspect_contract_declarations.plan_revision,
                relation_integrity_plan_revision: derive_legacy_relation_integrity_plan_revision(
                    &registration.relation_integrity.endpoint_kind_contracts,
                    &registration.relation_integrity.cardinality_contracts,
                    &registration.relation_integrity.uniqueness_contracts,
                    &registration.relation_integrity.symmetry_contracts,
                    &registration
                        .relation_integrity
                        .endpoint_deletion_integrity_contracts,
                    &registration.relation_integrity.acyclicity_contracts,
                    &registration
                        .relation_integrity
                        .partition_isolation_contracts,
                    &registration
                        .relation_integrity
                        .connectivity_minimum_contracts,
                ),
            },
        )
        .collect();
    LegacySchemaAuthoritySnapshot {
        primary_schema_id: current.primary_schema_id,
        primary_schema_version_id: current.primary_schema_version_id,
        entity_kinds: current.entity_kinds,
        relation_kinds,
    }
}
