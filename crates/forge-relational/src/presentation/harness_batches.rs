use crate::facade::harness::{FixtureEntity, FixtureRelation};
use crate::facade::identity::PartitionId;
use crate::facade::payloads::RecordPayload;
use crate::facade::transactions::{CreateIntent, MutationIntent, WorkerIntentBatch};
use crate::symbols::data::InternedString;
use crate::transactions::data::{EntityReference, EntitySpec, RelationSpec};

use super::harness_data::RelationalHarnessError;

pub(super) fn entity_fixture_batch(entities: &[FixtureEntity]) -> WorkerIntentBatch {
    let mut batch = WorkerIntentBatch::new("fixture");
    for entity in entities {
        batch
            .intents
            .push(MutationIntent::Create(CreateIntent::Entity(EntitySpec {
                partition_id: PartitionId::main(),
                kind_id: entity.kind_id,
                client_key: InternedString::Raw(entity.client_key.clone()),
                payload: RecordPayload::StructuredJson(entity.payload.clone()),
            })));
    }
    batch
}

pub(super) fn relation_fixture_batch(
    relations: &[FixtureRelation],
    entity_ids: &[crate::identity::data::EntityId],
) -> Result<WorkerIntentBatch, RelationalHarnessError> {
    let mut batch = WorkerIntentBatch::new("fixture-relations");
    for relation in relations {
        let source = entity_ids
            .get(relation.source_slot as usize)
            .copied()
            .ok_or_else(|| {
                RelationalHarnessError("fixture relation source is missing".to_string())
            })?;
        let target = entity_ids
            .get(relation.target_slot as usize)
            .copied()
            .ok_or_else(|| {
                RelationalHarnessError("fixture relation target is missing".to_string())
            })?;
        batch
            .intents
            .push(MutationIntent::Create(CreateIntent::Relation(
                RelationSpec {
                    partition_id: PartitionId::main(),
                    kind_id: relation.kind_id,
                    client_key: InternedString::Raw(relation.client_key.clone()),
                    source: EntityReference::Existing(source),
                    target: EntityReference::Existing(target),
                    payload: Some(RecordPayload::StructuredJson(relation.payload.clone())),
                },
            )));
    }
    Ok(batch)
}
