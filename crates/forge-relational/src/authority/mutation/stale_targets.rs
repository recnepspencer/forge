use crate::capabilities::StorageRead;
use crate::storage::logic::state::{EntityRecordKind, RecordKind, RelationRecordKind};
use crate::transactions::data::{CommitConflict, ConflictClass, ExistingRecordTarget};

pub(super) fn ensure_entity_target_is_current(
    staged: &impl StorageRead,
    entity_id: crate::identity::data::EntityId,
) -> Result<(), CommitConflict> {
    ensure_target_is_current::<EntityRecordKind>(staged, entity_id, "entity")
}

pub(super) fn ensure_relation_target_is_current(
    staged: &impl StorageRead,
    relation_id: crate::identity::data::RelationId,
) -> Result<(), CommitConflict> {
    ensure_target_is_current::<RelationRecordKind>(staged, relation_id, "relation")
}

fn ensure_target_is_current<K: RecordKind>(
    staged: &impl StorageRead,
    record_id: K::Id,
    record_kind: &str,
) -> Result<(), CommitConflict> {
    let slot = K::slot_of(&record_id);
    let partition_id = K::partition_of(&record_id);
    let generation = K::generation_of(&record_id);
    let Some(partition) = staged.get_partition(partition_id) else {
        return stale_handle_conflict(
            record_kind,
            if record_kind == "entity" {
                ExistingRecordTarget::Entity(crate::identity::data::EntityId::new(
                    partition_id,
                    slot as u64,
                    generation,
                ))
            } else {
                ExistingRecordTarget::Relation(crate::identity::data::RelationId::new(
                    partition_id,
                    slot as u64,
                    generation,
                ))
            },
        );
    };
    let arena = K::arena(partition);
    if !arena.contains_live_id(&record_id) {
        return stale_handle_conflict(
            record_kind,
            if record_kind == "entity" {
                ExistingRecordTarget::Entity(crate::identity::data::EntityId::new(
                    partition_id,
                    slot as u64,
                    generation,
                ))
            } else {
                ExistingRecordTarget::Relation(crate::identity::data::RelationId::new(
                    partition_id,
                    slot as u64,
                    generation,
                ))
            },
        );
    }
    Ok(())
}

fn stale_handle_conflict(
    record_kind: &str,
    target: ExistingRecordTarget,
) -> Result<(), CommitConflict> {
    Err(CommitConflict::new(ConflictClass::StaleTarget {
        target,
        context: format!("{record_kind} authoritative apply"),
    }))
}
