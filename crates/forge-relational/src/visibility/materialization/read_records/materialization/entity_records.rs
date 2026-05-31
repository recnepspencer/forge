use crate::logic::runtime::RelationalRuntime;
use crate::storage::data::{EntityReadRecord, RecordLifecycleState};
use crate::storage::logic::state::{HistoricalMetadata, PartitionState};

use super::super::visibility::{
    entity_visible_in_partition_at_version, historical_lifecycle, visible_metadata,
};
pub(in super::super) fn materialize_current_unmasked_entity_record(
    runtime: &RelationalRuntime,
    partition: &PartitionState,
    partition_id: crate::identity::data::PartitionId,
    slot: usize,
) -> Option<EntityReadRecord> {
    let entity_slot = partition.entity_arena.get_slot(slot)?;
    if entity_slot.lifecycle() != RecordLifecycleState::Live {
        return None;
    }
    let kind_id = entity_slot.kind_id()?;
    let kind = runtime
        .config
        .schema
        .registry
        .resolve_entity(kind_id)
        .ok()?;
    Some(EntityReadRecord {
        entity_id: crate::identity::data::EntityId::new(
            partition_id,
            slot as u64,
            entity_slot.generation(),
        ),
        lineage_id: entity_slot.extra().lineage_id,
        kind,
        lifecycle: entity_slot.lifecycle(),
        created_at_version: partition.entity_arena.created_at[slot],
        retired_at_version: entity_slot.retired_at(),
        authoritative_aspect_state: entity_slot.extra().authoritative_aspect_state.clone(),
    })
}

pub(in super::super) fn materialize_unmasked_entity_record_at_version(
    runtime: &RelationalRuntime,
    partition: &PartitionState,
    partition_id: crate::identity::data::PartitionId,
    slot: usize,
    version_id: crate::identity::data::VersionId,
) -> Option<EntityReadRecord> {
    if !entity_visible_in_partition_at_version(partition, slot, version_id) {
        return None;
    }
    let metadata = visible_metadata(
        partition.entity_arena.metadata_history_at(slot)?,
        version_id,
    )?;
    let kind = runtime
        .config
        .schema
        .registry
        .resolve_entity(metadata.kind_id)
        .ok()?;
    Some(EntityReadRecord {
        entity_id: crate::identity::data::EntityId::new(
            partition_id,
            slot as u64,
            metadata.generation,
        ),
        lineage_id: metadata.lineage_id,
        kind,
        lifecycle: historical_lifecycle(metadata.retired_at(), version_id),
        created_at_version: metadata.effective_at(),
        retired_at_version: metadata.retired_at(),
        authoritative_aspect_state: metadata.authoritative_aspect_state.clone(),
    })
}
