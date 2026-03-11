use crate::diagnostics::data::DiagnosticCode;
use crate::storage::data::RecordLifecycleState;
use crate::storage::logic::state::{EntityRecordKind, RecordId, RecordKind, RelationRecordKind};
use crate::transactions::data::CommitConflict;

use crate::logic::runtime::PartitionAccess;

pub(super) fn ensure_entity_target_is_current(
    staged: &impl PartitionAccess,
    entity_id: crate::identity::data::EntityId,
) -> Result<(), CommitConflict> {
    ensure_target_is_current::<EntityRecordKind>(staged, entity_id, "entity")
}

pub(super) fn ensure_relation_target_is_current(
    staged: &impl PartitionAccess,
    relation_id: crate::identity::data::RelationId,
) -> Result<(), CommitConflict> {
    ensure_target_is_current::<RelationRecordKind>(staged, relation_id, "relation")
}

fn ensure_target_is_current<K: RecordKind>(
    staged: &impl PartitionAccess,
    record_id: K::Id,
    record_kind: &str,
) -> Result<(), CommitConflict> {
    let slot = record_id.local_slot();
    let Some(partition) = staged.get_partition(record_id.partition_id()) else {
        return stale_handle_conflict(
            record_kind,
            record_id.partition_id().0,
            slot as u64,
        );
    };
    let arena = K::arena(partition);
    if arena.generations.get(slot) != Some(&record_id.generation())
        || arena.lifecycle.get(slot) != Some(&RecordLifecycleState::Live)
    {
        return stale_handle_conflict(
            record_kind,
            record_id.partition_id().0,
            slot as u64,
        );
    }
    Ok(())
}

fn stale_handle_conflict(
    record_kind: &str,
    partition_id: u32,
    local_slot: u64,
) -> Result<(), CommitConflict> {
    Err(CommitConflict {
        code: DiagnosticCode::StaleHandle,
        detail: format!(
            "{record_kind} target changed before authoritative apply at {partition_id}:{local_slot}"
        ),
    })
}
