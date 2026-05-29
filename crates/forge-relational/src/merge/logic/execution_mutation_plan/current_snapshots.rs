use crate::storage::data::{EntityReadRecord, RelationReadRecord};
use crate::storage::overlay::PartitionAccess;

pub(super) fn current_entity_snapshot(
    runtime: &crate::logic::runtime::RelationalRuntime,
    entity_id: crate::identity::data::EntityId,
) -> Option<EntityReadRecord> {
    let current_state = runtime.storage_access().current_state();
    let partition = current_state.get_partition(entity_id.partition_id)?;
    let slot = partition.entity_arena.get(&entity_id)?;
    let kind_id = slot.kind_id()?;
    let kind = runtime
        .config
        .schema
        .registry
        .resolve_entity(kind_id)
        .ok()?;
    Some(EntityReadRecord {
        entity_id,
        lineage_id: None,
        kind,
        lifecycle: slot.lifecycle(),
        created_at_version: partition.entity_arena.created_at[entity_id.local_slot.0 as usize],
        retired_at_version: slot.retired_at(),
        authoritative_aspect_state: slot.extra().authoritative_aspect_state.clone(),
    })
}

#[allow(dead_code)]
pub(super) fn current_relation_snapshot(
    runtime: &crate::logic::runtime::RelationalRuntime,
    relation_id: crate::identity::data::RelationId,
) -> Option<RelationReadRecord> {
    let current_state = runtime.storage_access().current_state();
    let partition = current_state.get_partition(relation_id.partition_id)?;
    let slot = partition.relation_arena.get(&relation_id)?;
    let kind_id = slot.kind_id()?;
    let endpoints = slot.extra().endpoints.as_ref()?;
    let kind = runtime
        .config
        .schema
        .registry
        .resolve_relation(kind_id)
        .ok()?;
    Some(RelationReadRecord {
        relation_id,
        kind,
        lifecycle: slot.lifecycle(),
        created_at_version: partition.relation_arena.created_at[relation_id.local_slot.0 as usize],
        retired_at_version: slot.retired_at(),
        source: endpoints.source,
        target: endpoints.target,
        authoritative_aspect_state: slot.extra().authoritative_aspect_state.clone(),
    })
}
