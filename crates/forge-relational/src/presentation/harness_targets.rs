use crate::config::data::{CascadeDeletePolicy, CrossContextPolicy};
use crate::facade::{
    EntityId, EntityKindRegistration, KindId, PartitionId, RelationId, RelationKindRegistration,
    RelationalSchemaRegistry, SchemaId, SchemaVersionId,
};
use crate::query::data::ReadTarget;
use crate::schema::data::RelationPayloadClass;

use super::harness_data::RelationalHarnessError;

pub(super) fn resolve_targets(
    request: &forge_harness::facade::ExecutionRequest<String>,
) -> Vec<String> {
    if request.targets.is_empty() {
        Vec::new()
    } else {
        request.targets.clone()
    }
}

pub(super) fn parse_target(target: &str) -> Result<ReadTarget, RelationalHarnessError> {
    let mut parts = target.split(':');
    let kind = parts
        .next()
        .ok_or_else(|| RelationalHarnessError("missing target kind".to_string()))?;
    let remainder = parts.collect::<Vec<_>>();
    let (partition_id, slot, generation) = match remainder.as_slice() {
        [slot, generation] => (
            PartitionId::main(),
            slot.parse::<u64>()
                .map_err(|_| RelationalHarnessError("invalid target slot".to_string()))?,
            generation
                .parse::<u32>()
                .map_err(|_| RelationalHarnessError("invalid target generation".to_string()))?,
        ),
        [partition, slot, generation] => (
            PartitionId(
                partition
                    .parse::<u32>()
                    .map_err(|_| RelationalHarnessError("invalid target partition".to_string()))?,
            ),
            slot.parse::<u64>()
                .map_err(|_| RelationalHarnessError("invalid target slot".to_string()))?,
            generation
                .parse::<u32>()
                .map_err(|_| RelationalHarnessError("invalid target generation".to_string()))?,
        ),
        _ => {
            return Err(RelationalHarnessError(
                "target must be kind:slot:generation or kind:partition:slot:generation".to_string(),
            ))
        }
    };
    match kind {
        "entity" => Ok(ReadTarget::Entity(EntityId::new(
            partition_id,
            slot,
            generation,
        ))),
        "relation" => Ok(ReadTarget::Relation(RelationId::new(
            partition_id,
            slot,
            generation,
        ))),
        _ => Err(RelationalHarnessError("unknown target kind".to_string())),
    }
}

pub(super) fn commit_error_to_harness_error(
    error: crate::transactions::data::TransactionCommitError,
) -> RelationalHarnessError {
    match error {
        crate::transactions::data::TransactionCommitError::Conflict(conflict) => {
            RelationalHarnessError(conflict.detail)
        }
        crate::transactions::data::TransactionCommitError::Publication(publication) => {
            RelationalHarnessError(publication.detail)
        }
    }
}

pub(super) fn default_harness_schema_registry() -> RelationalSchemaRegistry {
    RelationalSchemaRegistry::new()
        .register_entity_kind(EntityKindRegistration {
            kind_id: KindId(1),
            kind_name: "fixture.entity".to_string(),
            schema_id: SchemaId("fixture".to_string()),
            schema_version_id: SchemaVersionId(1),
        })
        .and_then(|registry| {
            registry.register_relation_kind(RelationKindRegistration {
                kind_id: KindId(2),
                kind_name: "fixture.relation".to_string(),
                schema_id: SchemaId("fixture".to_string()),
                schema_version_id: SchemaVersionId(1),
                payload_class: RelationPayloadClass::PayloadBearingRelation,
                cross_context_policy: CrossContextPolicy::AllowExplicit,
                cascade_delete_policy: CascadeDeletePolicy::CascadeDeleteRelations,
            })
        })
        .expect("valid default harness schema registry")
}
