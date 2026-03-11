use std::collections::BTreeSet;

use crate::diagnostics::data::DiagnosticCode;
use crate::payloads::data::RecordPayload;
use crate::schema::data::SchemaRegistryError;
use crate::storage::logic::state::{EntityArena, PartitionAccess, VersionedValue};
use crate::transactions::data::CommitConflict;
use crate::validation::data::{InvariantCheckResult, InvariantFailureEffect};

pub(crate) fn touched_visible_entity_ids(
    state: &impl PartitionAccess,
    version_id: crate::identity::data::VersionId,
) -> Option<Vec<crate::identity::data::EntityId>> {
    let mut ids = Vec::new();
    let mut saw_any = false;
    for partition_id in state.partition_ids() {
        let partition = state.get_partition(partition_id)?;
        let Some(slots) = state.touched_entity_slots(partition_id) else {
            continue;
        };
        saw_any = true;
        for slot in slots {
            if slot >= partition.entity_arena.generations.len() {
                continue;
            }
            let Some(metadata) = partition
                .entity_arena
                .metadata_history
                .get(slot)
                .and_then(|history| visible_entity_metadata(history, version_id))
            else {
                continue;
            };
            ids.push(crate::identity::data::EntityId::new(
                partition_id,
                slot as u64,
                metadata.generation,
            ));
        }
    }
    if saw_any {
        Some(ids)
    } else {
        None
    }
}

pub(crate) fn entity_payload_for_state(
    state: &impl PartitionAccess,
    entity_id: crate::identity::data::EntityId,
    version_id: crate::identity::data::VersionId,
) -> Option<&RecordPayload> {
    let partition = state.get_partition(entity_id.partition_id)?;
    let slot = entity_id.local_slot.0 as usize;
    if slot >= partition.entity_arena.payload_history.len() {
        return None;
    }
    visible_payload(&partition.entity_arena.payload_history[slot], version_id)
}

pub(crate) fn visible_payload(
    history: &[VersionedValue],
    version_id: crate::identity::data::VersionId,
) -> Option<&RecordPayload> {
    let end = history.partition_point(|entry| entry.effective_at <= version_id);
    history[..end]
        .iter()
        .rev()
        .find(|entry| {
            entry.effective_at <= version_id
                && entry.retired_at.is_none_or(|retired| version_id < retired)
        })
        .map(|entry| &entry.value)
}

pub(crate) fn entity_visible_at_version(
    arena: &EntityArena,
    slot: usize,
    version_id: crate::identity::data::VersionId,
) -> bool {
    arena
        .metadata_history
        .get(slot)
        .and_then(|history| visible_entity_metadata(history, version_id))
        .is_some()
}

pub(crate) fn visible_entity_metadata(
    history: &[crate::storage::logic::state::VersionedEntityMetadata],
    version_id: crate::identity::data::VersionId,
) -> Option<&crate::storage::logic::state::VersionedEntityMetadata> {
    let end = history.partition_point(|entry| entry.effective_at <= version_id);
    history[..end].iter().rev().find(|entry| {
        entry.effective_at <= version_id
            && entry.retired_at.is_none_or(|retired| version_id < retired)
    })
}

pub(crate) fn first_blocking_invariant_error(
    results: &[InvariantCheckResult],
) -> Option<CommitConflict> {
    results
        .iter()
        .find(|result| {
            result.failure_effect == InvariantFailureEffect::BlockCommit
                && !result.violations.is_empty()
        })
        .and_then(|result| result.violations.first())
        .map(|violation| CommitConflict {
            code: violation.code,
            detail: violation.detail.clone(),
        })
}

pub(crate) fn first_publication_invariant_error(
    results: &[InvariantCheckResult],
) -> Option<CommitConflict> {
    results
        .iter()
        .find(|result| {
            result.failure_effect == InvariantFailureEffect::BlockPublication
                && !result.violations.is_empty()
        })
        .and_then(|result| result.violations.first())
        .map(|violation| CommitConflict {
            code: violation.code,
            detail: violation.detail.clone(),
        })
}

pub(crate) fn schema_error_to_commit_conflict(error: SchemaRegistryError) -> CommitConflict {
    CommitConflict {
        code: DiagnosticCode::InvariantViolation,
        detail: format!("{error:?}"),
    }
}

pub(crate) fn touched_entity_set(
    ids: &[crate::identity::data::EntityId],
) -> BTreeSet<crate::identity::data::EntityId> {
    ids.iter().copied().collect()
}
