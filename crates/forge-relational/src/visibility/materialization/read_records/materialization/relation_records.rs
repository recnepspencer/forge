use crate::logic::runtime::RelationalRuntime;
use crate::storage::data::{RecordLifecycleState, RelationReadRecord};
use crate::storage::logic::state::{HistoricalMetadata, PartitionState};

use super::super::visibility::{
    historical_lifecycle, lifecycle_storage_visible, relation_visible_in_partition_at_version,
    visible_metadata,
};
pub(in super::super) fn materialize_current_unmasked_relation_record(
    runtime: &RelationalRuntime,
    partition: &PartitionState,
    partition_id: crate::identity::data::PartitionId,
    slot: usize,
) -> Option<RelationReadRecord> {
    let relation_slot = partition.relation_arena.get_slot(slot)?;
    let lifecycle = relation_slot.lifecycle();
    if !lifecycle_storage_visible(lifecycle) {
        return None;
    }
    let kind_id = relation_slot.kind_id()?;
    let kind = runtime
        .config
        .schema
        .registry
        .resolve_relation(kind_id)
        .ok()?;
    let endpoints = relation_slot.extra().endpoints.as_ref()?;
    Some(RelationReadRecord {
        relation_id: crate::identity::data::RelationId::new(
            partition_id,
            slot as u64,
            relation_slot.generation(),
        ),
        kind,
        lifecycle,
        created_at_version: partition.relation_arena.created_at[slot],
        retired_at_version: relation_slot.retired_at(),
        source: endpoints.source,
        target: endpoints.target,
        authoritative_aspect_state: relation_slot.extra().authoritative_aspect_state.clone(),
    })
}

pub(in super::super) fn materialize_unmasked_relation_record_at_version(
    runtime: &RelationalRuntime,
    partition: &PartitionState,
    partition_id: crate::identity::data::PartitionId,
    slot: usize,
    version_id: crate::identity::data::VersionId,
) -> Option<RelationReadRecord> {
    if !relation_visible_in_partition_at_version(partition, slot, version_id) {
        return None;
    }
    let metadata = visible_metadata(
        partition.relation_arena.metadata_history_at(slot)?,
        version_id,
    )?;
    let kind = runtime
        .config
        .schema
        .registry
        .resolve_relation(metadata.kind_id)
        .ok()?;
    Some(RelationReadRecord {
        relation_id: crate::identity::data::RelationId::new(
            partition_id,
            slot as u64,
            metadata.generation,
        ),
        kind,
        lifecycle: if partition
            .relation_arena
            .get_slot(slot)
            .is_some_and(|slot_view| {
                slot_view.generation() == metadata.generation
                    && slot_view.lifecycle() == RecordLifecycleState::RetainedDanglingForAudit
            }) {
            RecordLifecycleState::RetainedDanglingForAudit
        } else {
            historical_lifecycle(metadata.retired_at(), version_id)
        },
        created_at_version: metadata.effective_at(),
        retired_at_version: metadata.retired_at(),
        source: metadata.endpoints.source,
        target: metadata.endpoints.target,
        authoritative_aspect_state: metadata.authoritative_aspect_state.clone(),
    })
}
