use std::collections::BTreeSet;

use crate::config::data::{CascadeDeletePolicy, PatchSurfacePolicy};
use crate::identity::data::{EntityId, KindId, PartitionId, RelationId};
use crate::payloads::data::RecordPayload;
use crate::publication::data::diff::{PatchRecord, PatchRecordKind};
use crate::schema::data::RelationalSchemaRegistry;
use crate::storage::data::RecordLifecycleState;
use crate::storage::logic::state::{
    AdjacencySet, PartitionAccess, PartitionState, RelationEndpoints, WorkingState,
};
use crate::transactions::data::{RecordRef, RelationSpec};

use super::apply_patching::{patch_detail_for_entity, patch_detail_for_relation};

pub(super) fn allocate_entity(
    staged: &mut WorkingState,
    version_id: crate::identity::data::VersionId,
    partition_id: PartitionId,
    kind_id: KindId,
    payload: RecordPayload,
) -> EntityId {
    let partition = ensure_partition_state(staged, partition_id);
    let (slot, generation, reused) =
        partition
            .entity_arena
            .allocate(partition_id, kind_id, payload, version_id);
    if reused {
        let idx = slot;
        partition.adjacency[idx].clear();
        partition.reverse_adjacency[idx].clear();
    } else {
        partition
            .adjacency
            .push(AdjacencySet::new(&partition.adjacency_policy));
        partition
            .reverse_adjacency
            .push(AdjacencySet::new(&partition.adjacency_policy));
    }
    EntityId::new(partition_id, slot as u64, generation)
}

pub(super) fn allocate_relation(
    staged: &mut WorkingState,
    version_id: crate::identity::data::VersionId,
    spec: &RelationSpec,
) -> RelationId {
    let relation_id = {
        let partition = ensure_partition_state(staged, spec.partition_id);
        let (slot, generation) = partition.relation_arena.allocate(
            spec.partition_id,
            spec.kind_id,
            spec.payload.clone(),
            version_id,
            RelationEndpoints {
                source: spec.source,
                target: spec.target,
            },
        );
        RelationId::new(spec.partition_id, slot as u64, generation)
    };

    let source_partition = ensure_partition_state(staged, spec.source.partition_id);
    ensure_entity_adjacency_capacity(source_partition, spec.source.local_slot.0 as usize);
    source_partition.adjacency[spec.source.local_slot.0 as usize].insert(relation_id);

    let target_partition = ensure_partition_state(staged, spec.target.partition_id);
    ensure_entity_adjacency_capacity(target_partition, spec.target.local_slot.0 as usize);
    target_partition.reverse_adjacency[spec.target.local_slot.0 as usize].insert(relation_id);

    relation_id
}

pub(super) fn delete_entity_with_cascade(
    staged: &mut WorkingState,
    version_id: crate::identity::data::VersionId,
    entity_id: EntityId,
    patch_surface_policy: PatchSurfacePolicy,
    schema_registry: &RelationalSchemaRegistry,
    default_cascade_delete_policy: CascadeDeletePolicy,
    changed_records: &mut Vec<RecordRef>,
    patch_records: &mut Vec<PatchRecord>,
) {
    let slot = entity_id.local_slot.0 as usize;
    staged.mark_entity_slot_touched(entity_id.partition_id, slot);
    let partition = staged.get_partition_mut(entity_id.partition_id);
    partition.entity_arena.retire(slot, version_id);
    changed_records.push(RecordRef::Entity(entity_id));
    patch_records.push(PatchRecord {
        kind: PatchRecordKind::EntityDeleted,
        entity_id: Some(entity_id),
        relation_id: None,
        aspects: Vec::new(),
        detail: patch_detail_for_entity(
            patch_surface_policy,
            PatchRecordKind::EntityDeleted,
            entity_id,
            None,
        ),
    });

    let mut attached = BTreeSet::new();
    partition.adjacency[slot].extend_into(&mut attached);
    partition.reverse_adjacency[slot].extend_into(&mut attached);
    for relation_id in attached {
        let cascade_policy = staged
            .get_partition(relation_id.partition_id)
            .and_then(|partition| {
                partition
                    .relation_arena
                    .kind_ids
                    .get(relation_id.local_slot.0 as usize)
            })
            .and_then(|kind_id| kind_id.as_ref().copied())
            .and_then(|kind_id| {
                schema_registry
                    .relation_registration(kind_id)
                    .ok()
                    .map(|registration| registration.cascade_delete_policy)
            })
            .unwrap_or(default_cascade_delete_policy);
        match cascade_policy {
            CascadeDeletePolicy::CascadeDeleteRelations
            | CascadeDeletePolicy::RetainDanglingForAudit => {
                delete_relation(
                    staged,
                    version_id,
                    relation_id,
                    patch_surface_policy,
                    changed_records,
                    patch_records,
                );
            }
        }
    }
}

pub(super) fn delete_relation(
    staged: &mut WorkingState,
    version_id: crate::identity::data::VersionId,
    relation_id: RelationId,
    patch_surface_policy: PatchSurfacePolicy,
    changed_records: &mut Vec<RecordRef>,
    patch_records: &mut Vec<PatchRecord>,
) {
    let slot = relation_id.local_slot.0 as usize;
    let relation_is_live =
        staged
            .get_partition(relation_id.partition_id)
            .is_some_and(|partition| {
                partition.relation_arena.lifecycle[slot] == RecordLifecycleState::Live
            });
    if !relation_is_live {
        return;
    }
    staged.mark_relation_slot_touched(relation_id.partition_id, slot);
    let partition = staged.get_partition_mut(relation_id.partition_id);
    let endpoints = partition.relation_arena.endpoints[slot].clone();
    partition.relation_arena.retire(slot, version_id);
    let fallback_source = endpoints
        .as_ref()
        .map(|value| value.source)
        .unwrap_or(EntityId::new(relation_id.partition_id, 0, 0));
    let fallback_target = endpoints
        .as_ref()
        .map(|value| value.target)
        .unwrap_or(EntityId::new(relation_id.partition_id, 0, 0));
    if let Some(endpoints) = endpoints {
        staged.mark_adjacency_slot_touched(
            endpoints.source.partition_id,
            endpoints.source.local_slot.0 as usize,
        );
        staged.mark_reverse_adjacency_slot_touched(
            endpoints.target.partition_id,
            endpoints.target.local_slot.0 as usize,
        );
        let source_partition = staged.get_partition_mut(endpoints.source.partition_id);
        if let Some(relations) = source_partition
            .adjacency
            .get_mut(endpoints.source.local_slot.0 as usize)
        {
            relations.remove(&relation_id);
        }
        let target_partition = staged.get_partition_mut(endpoints.target.partition_id);
        if let Some(relations) = target_partition
            .reverse_adjacency
            .get_mut(endpoints.target.local_slot.0 as usize)
        {
            relations.remove(&relation_id);
        }
    }
    changed_records.push(RecordRef::Relation(relation_id));
    patch_records.push(PatchRecord {
        kind: PatchRecordKind::RelationDeleted,
        entity_id: None,
        relation_id: Some(relation_id),
        aspects: Vec::new(),
        detail: patch_detail_for_relation(
            patch_surface_policy,
            PatchRecordKind::RelationDeleted,
            relation_id,
            fallback_source,
            fallback_target,
            None,
        ),
    });
}

pub(super) fn reserve_bulk_entity_capacity(
    staged: &mut WorkingState,
    partition_id: PartitionId,
    requested_slots: usize,
) {
    let reusable_slots = staged
        .get_partition(partition_id)
        .map(|partition| partition.entity_arena.free_list.len())
        .unwrap_or(0);
    let additional = requested_slots.saturating_sub(reusable_slots);
    staged.reserve_entity_slots(partition_id, additional);
}

pub(super) fn reserve_bulk_relation_capacity(
    staged: &mut WorkingState,
    partition_id: PartitionId,
    requested_slots: usize,
) {
    let reusable_slots = staged
        .get_partition(partition_id)
        .map(|partition| partition.relation_arena.free_list.len())
        .unwrap_or(0);
    let additional = requested_slots.saturating_sub(reusable_slots);
    staged.reserve_relation_slots(partition_id, additional);
}

fn ensure_partition_state(
    staged: &mut WorkingState,
    partition_id: PartitionId,
) -> &mut PartitionState {
    staged.get_partition_mut(partition_id)
}

fn ensure_entity_adjacency_capacity(partition: &mut PartitionState, slot: usize) {
    while partition.adjacency.len() <= slot {
        partition
            .adjacency
            .push(AdjacencySet::new(&partition.adjacency_policy));
    }
    while partition.reverse_adjacency.len() <= slot {
        partition
            .reverse_adjacency
            .push(AdjacencySet::new(&partition.adjacency_policy));
    }
}
