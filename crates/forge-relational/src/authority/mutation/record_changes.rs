use std::collections::BTreeSet;

use crate::config::data::{CascadeDeletePolicy, PatchSurfacePolicy};
use crate::identity::data::{EntityId, KindId, PartitionId, RelationId};
use crate::payloads::data::RecordPayload;
use crate::publication::data::diff::{PatchRecord, PatchRecordKind};
use crate::schema::data::RelationalSchemaRegistry;
use crate::storage::data::RecordLifecycleState;
use crate::storage::logic::state::{
    AdjacencySet, EntityRecordKind, PartitionAccess, PartitionState, RecordKind,
    RelationEndpoints, RelationRecordKind,
};
use crate::storage::overlay::RelationalDraft;
use crate::transactions::data::{RecordRef, RelationSpec};

use super::patch_details::{patch_detail_for_entity, patch_detail_for_relation};
use super::{AdjacencyDelta, AdjacencyDeltaKind, MutationEffect};

pub(super) fn allocate_entity(
    draft: &mut RelationalDraft,
    version_id: crate::identity::data::VersionId,
    partition_id: PartitionId,
    kind_id: KindId,
    payload: RecordPayload,
) -> EntityId {
    let (slot, generation, reused) = allocate_record::<EntityRecordKind>(
        draft,
        partition_id,
        kind_id,
        Some(payload),
        version_id,
        EntityRecordKind::empty_extra(),
    );
    let partition = ensure_partition_state(draft, partition_id);
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
    draft: &mut RelationalDraft,
    version_id: crate::identity::data::VersionId,
    spec: &RelationSpec,
) -> RelationId {
    let (slot, generation, _) = allocate_record::<RelationRecordKind>(
        draft,
        spec.partition_id,
        spec.kind_id,
        spec.payload.clone(),
        version_id,
        Some(RelationEndpoints {
            source: spec.source,
            target: spec.target,
        }),
    );
    RelationId::new(spec.partition_id, slot as u64, generation)
}

pub(super) fn delete_entity_with_cascade(
    draft: &mut RelationalDraft,
    version_id: crate::identity::data::VersionId,
    entity_id: EntityId,
    patch_surface_policy: PatchSurfacePolicy,
    schema_registry: &RelationalSchemaRegistry,
    default_cascade_delete_policy: CascadeDeletePolicy,
    effect: &mut MutationEffect,
) {
    let slot = entity_id.local_slot.0 as usize;
    draft.mark_entity_slot_touched(entity_id.partition_id, slot);
    let partition = draft.get_partition_mut(entity_id.partition_id);
    partition.entity_arena.retire(slot, version_id);
    effect.record_change(RecordRef::Entity(entity_id));
    effect.record_patch(PatchRecord {
        kind: PatchRecordKind::Deleted,
        target: RecordRef::Entity(entity_id),
        aspects: Vec::new(),
        detail: patch_detail_for_entity(
            patch_surface_policy,
            PatchRecordKind::Deleted,
            entity_id,
            None,
        ),
    });

    let mut attached = BTreeSet::new();
    partition.adjacency[slot].extend_into(&mut attached);
    partition.reverse_adjacency[slot].extend_into(&mut attached);
    for relation_id in attached {
        let cascade_policy = draft
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
            CascadeDeletePolicy::CascadeDeleteRelations => {
                delete_relation(
                    draft,
                    version_id,
                    relation_id,
                    patch_surface_policy,
                    effect,
                );
            }
            CascadeDeletePolicy::RetainDanglingForAudit => {
                retain_relation_dangling_for_audit(
                    draft,
                    version_id,
                    relation_id,
                    patch_surface_policy,
                    effect,
                );
            }
        }
    }
}

pub(super) fn retain_relation_dangling_for_audit(
    draft: &mut RelationalDraft,
    version_id: crate::identity::data::VersionId,
    relation_id: RelationId,
    patch_surface_policy: PatchSurfacePolicy,
    effect: &mut MutationEffect,
) {
    let slot = relation_id.local_slot.0 as usize;
    let relation_is_visible =
        draft
            .get_partition(relation_id.partition_id)
            .is_some_and(|partition| {
                matches!(
                    partition.relation_arena.lifecycle[slot],
                    RecordLifecycleState::Live | RecordLifecycleState::RetainedDanglingForAudit
                )
            });
    if !relation_is_visible {
        return;
    }
    draft.mark_relation_slot_touched(relation_id.partition_id, slot);
    let partition = draft.get_partition_mut(relation_id.partition_id);
    partition.relation_arena.lifecycle[slot] = RecordLifecycleState::RetainedDanglingForAudit;
    let _ = version_id;
    partition.relation_arena.live_bitset.set(slot, true);
    partition.relation_arena.reclaimable_bitset.set(slot, false);
    let endpoints = partition.relation_arena.extra[slot].clone();
    let payload = partition.relation_arena.payloads[slot].clone();
    let Some(endpoints) = endpoints else {
        return;
    };
    effect.record_change(RecordRef::Relation(relation_id));
    effect.record_patch(PatchRecord {
        kind: PatchRecordKind::RetainedForAudit,
        target: RecordRef::Relation(relation_id),
        aspects: Vec::new(),
        detail: patch_detail_for_relation(
            patch_surface_policy,
            PatchRecordKind::RetainedForAudit,
            relation_id,
            endpoints.source,
            endpoints.target,
            payload.as_ref(),
        ),
    });
}

pub(super) fn delete_relation(
    draft: &mut RelationalDraft,
    version_id: crate::identity::data::VersionId,
    relation_id: RelationId,
    patch_surface_policy: PatchSurfacePolicy,
    effect: &mut MutationEffect,
) {
    let slot = relation_id.local_slot.0 as usize;
    let relation_is_live =
        draft
            .get_partition(relation_id.partition_id)
            .is_some_and(|partition| {
                partition.relation_arena.lifecycle[slot] == RecordLifecycleState::Live
            });
    if !relation_is_live {
        return;
    }
    draft.mark_relation_slot_touched(relation_id.partition_id, slot);
    let partition = draft.get_partition_mut(relation_id.partition_id);
    let endpoints = partition.relation_arena.extra[slot].clone();
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
        effect.record_adjacency_delta(AdjacencyDelta {
            relation_id,
            kind: AdjacencyDeltaKind::Deleted {
                source: endpoints.source,
                target: endpoints.target,
            },
        });
    }
    effect.record_change(RecordRef::Relation(relation_id));
    effect.record_patch(PatchRecord {
        kind: PatchRecordKind::Deleted,
        target: RecordRef::Relation(relation_id),
        aspects: Vec::new(),
        detail: patch_detail_for_relation(
            patch_surface_policy,
            PatchRecordKind::Deleted,
            relation_id,
            fallback_source,
            fallback_target,
            None,
        ),
    });
}

pub(super) fn apply_adjacency_deltas(
    draft: &mut RelationalDraft,
    deltas: &[AdjacencyDelta],
) {
    for delta in deltas {
        let (source, target) = match delta.kind {
            AdjacencyDeltaKind::Created { source, target }
            | AdjacencyDeltaKind::Deleted { source, target } => (source, target),
        };
        draft.mark_adjacency_slot_touched(source.partition_id, source.local_slot.0 as usize);
        draft.mark_reverse_adjacency_slot_touched(
            target.partition_id,
            target.local_slot.0 as usize,
        );

        let source_partition = ensure_partition_state(draft, source.partition_id);
        ensure_entity_adjacency_capacity(source_partition, source.local_slot.0 as usize);
        match delta.kind {
            AdjacencyDeltaKind::Created { .. } => {
                source_partition.adjacency[source.local_slot.0 as usize].insert(delta.relation_id);
            }
            AdjacencyDeltaKind::Deleted { .. } => {
                if let Some(relations) = source_partition
                    .adjacency
                    .get_mut(source.local_slot.0 as usize)
                {
                    relations.remove(&delta.relation_id);
                }
            }
        }

        let target_partition = ensure_partition_state(draft, target.partition_id);
        ensure_entity_adjacency_capacity(target_partition, target.local_slot.0 as usize);
        match delta.kind {
            AdjacencyDeltaKind::Created { .. } => {
                target_partition.reverse_adjacency[target.local_slot.0 as usize]
                    .insert(delta.relation_id);
            }
            AdjacencyDeltaKind::Deleted { .. } => {
                if let Some(relations) = target_partition
                    .reverse_adjacency
                    .get_mut(target.local_slot.0 as usize)
                {
                    relations.remove(&delta.relation_id);
                }
            }
        }
    }
}

pub(super) fn reserve_bulk_entity_capacity(
    draft: &mut RelationalDraft,
    partition_id: PartitionId,
    requested_slots: usize,
) {
    let reusable_slots = draft
        .get_partition(partition_id)
        .map(|partition| partition.entity_arena.free_list.len())
        .unwrap_or(0);
    let additional = requested_slots.saturating_sub(reusable_slots);
    draft.reserve_entity_slots(partition_id, additional);
}

pub(super) fn reserve_bulk_relation_capacity(
    draft: &mut RelationalDraft,
    partition_id: PartitionId,
    requested_slots: usize,
) {
    let reusable_slots = draft
        .get_partition(partition_id)
        .map(|partition| partition.relation_arena.free_list.len())
        .unwrap_or(0);
    let additional = requested_slots.saturating_sub(reusable_slots);
    draft.reserve_relation_slots(partition_id, additional);
}

fn ensure_partition_state(
    draft: &mut RelationalDraft,
    partition_id: PartitionId,
) -> &mut PartitionState {
    draft.get_partition_mut(partition_id)
}

fn allocate_record<K: RecordKind>(
    draft: &mut RelationalDraft,
    partition_id: PartitionId,
    kind_id: KindId,
    payload: Option<RecordPayload>,
    version_id: crate::identity::data::VersionId,
    extra: K::Extra,
) -> (usize, u32, bool) {
    let partition = ensure_partition_state(draft, partition_id);
    let arena = K::arena_mut(partition);
    let (slot, generation, reused) =
        arena.allocate_common(partition_id, kind_id, payload, version_id, extra);
    (slot, generation, reused)
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
