use crate::logic::runtime::RelationalRuntime;
use crate::storage::data::{EntityReadRecord, RecordLifecycleState, RelationReadRecord};
use crate::storage::logic::state::{HistoricalMetadata, PartitionState};

use super::visibility::{
    entity_visible_in_partition_at_version, historical_lifecycle, lifecycle_storage_visible,
    relation_visible_in_partition_at_version, visible_metadata, visible_payload_for_generation,
};

pub(super) fn materialize_current_entity_record(
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
    let payload = partition
        .entity_arena
        .payload_history_at(slot)?
        .last()?
        .value
        .clone();
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
        payload,
    })
}

pub(super) fn materialize_entity_record_at_version(
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
    let payload = visible_payload_for_generation(
        partition.entity_arena.payload_history_at(slot)?,
        version_id,
        metadata.generation,
    )?
    .clone();
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
        payload,
    })
}

pub(super) fn materialize_current_relation_record(
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
    let endpoints = relation_slot.extra().as_ref()?;
    let payload = partition
        .relation_arena
        .payload_history_at(slot)
        .and_then(|history| history.last())
        .map(|entry| entry.value.clone());
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
        payload,
    })
}

pub(super) fn materialize_relation_record_at_version(
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
    let payload = visible_payload_for_generation(
        partition.relation_arena.payload_history_at(slot)?,
        version_id,
        metadata.generation,
    )
    .cloned();
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
        payload,
    })
}
