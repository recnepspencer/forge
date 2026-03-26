use std::collections::BTreeSet;

use crate::config::data::CascadeDeletePolicy;
use crate::identity::data::{EntityId, KindId, PartitionId, RelationId};
use crate::payloads::data::RecordPayload;
use crate::schema::data::RelationalSchemaRegistry;
use crate::storage::data::RecordLifecycleState;
use crate::storage::logic::state::{
    AdjacencySet, EntityRecordKind, PartitionAccess, PartitionState, RecordKind, RelationEndpoints,
    RelationRecordKind,
};
use crate::storage::overlay::WorkingState;
use crate::transactions::data::{CommitConflict, ConflictClass, RelationSpec};

use super::outcomes::{MutationOutcome, RecordMutation};
use super::{AdjacencyDelta, AdjacencyDeltaKind};

pub(super) fn allocate_entity(
    state: &mut WorkingState,
    version_id: crate::identity::data::VersionId,
    partition_id: PartitionId,
    kind_id: KindId,
    payload: RecordPayload,
) -> EntityId {
    let (slot, generation, reused) = allocate_record::<EntityRecordKind>(
        state,
        partition_id,
        kind_id,
        Some(payload),
        version_id,
        EntityRecordKind::empty_extra(),
    );
    let partition = ensure_partition_state(state, partition_id);
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
    state: &mut WorkingState,
    version_id: crate::identity::data::VersionId,
    spec: &RelationSpec,
) -> RelationId {
    let (slot, generation, _) = allocate_record::<RelationRecordKind>(
        state,
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
    state: &mut WorkingState,
    version_id: crate::identity::data::VersionId,
    entity_id: EntityId,
    schema_registry: &RelationalSchemaRegistry,
    default_cascade_delete_policy: CascadeDeletePolicy,
    outcome: &mut MutationOutcome,
) -> Result<(), CommitConflict> {
    let slot = entity_id.local_slot.0 as usize;
    state.mark_entity_slot_touched(entity_id.partition_id, slot);
    let partition = state.get_partition_mut(entity_id.partition_id);
    let slot_view = partition.entity_arena.get_slot(slot).ok_or_else(|| {
        mutation_state_inconsistency(
            "entity delete requires an existing slot after stale-target validation",
            serde_json::json!({
                "record_class": "entity",
                "entity_id": entity_id,
                "phase": "delete_with_cascade",
                "missing": "slot",
            }),
        )
    })?;
    let kind_id = slot_view.kind_id().ok_or_else(|| {
        mutation_state_inconsistency(
            "entity delete requires a retained kind id after stale-target validation",
            serde_json::json!({
                "record_class": "entity",
                "entity_id": entity_id,
                "phase": "delete_with_cascade",
                "missing": "kind_id",
            }),
        )
    })?;
    let payload = slot_view.payload().cloned().ok_or_else(|| {
        mutation_state_inconsistency(
            "entity delete requires a retained payload after stale-target validation",
            serde_json::json!({
                "record_class": "entity",
                "entity_id": entity_id,
                "phase": "delete_with_cascade",
                "missing": "payload",
            }),
        )
    })?;
    partition.entity_arena.retire(slot, version_id);
    outcome.record_change(RecordMutation::EntityDeleted {
        entity_id,
        kind_id,
        payload,
    });

    let mut attached = BTreeSet::new();
    partition.adjacency[slot].extend_into(&mut attached);
    partition.reverse_adjacency[slot].extend_into(&mut attached);
    for relation_id in attached {
        let cascade_policy = state
            .get_partition(relation_id.partition_id)
            .and_then(|partition| {
                partition
                    .relation_arena
                    .get(&relation_id)
                    .and_then(|slot| slot.kind_id())
            })
            .and_then(|kind_id| {
                schema_registry
                    .relation_registration(kind_id)
                    .ok()
                    .map(|registration| registration.cascade_delete_policy)
            })
            .unwrap_or(default_cascade_delete_policy);
        match cascade_policy {
            CascadeDeletePolicy::CascadeDeleteRelations => {
                delete_relation(state, version_id, relation_id, outcome);
            }
            CascadeDeletePolicy::RetainDanglingForAudit => {
                retain_relation_dangling_for_audit(state, version_id, relation_id, outcome);
            }
        }
    }
    Ok(())
}

pub(super) fn retain_relation_dangling_for_audit(
    state: &mut WorkingState,
    version_id: crate::identity::data::VersionId,
    relation_id: RelationId,
    outcome: &mut MutationOutcome,
) {
    let slot = relation_id.local_slot.0 as usize;
    let relation_is_visible = state
        .get_partition(relation_id.partition_id)
        .and_then(|partition| partition.relation_arena.get(&relation_id))
        .is_some_and(|relation_slot| {
            matches!(
                relation_slot.lifecycle(),
                RecordLifecycleState::Live | RecordLifecycleState::RetainedDanglingForAudit
            )
        });
    if !relation_is_visible {
        return;
    }
    state.mark_relation_slot_touched(relation_id.partition_id, slot);
    let partition = state.get_partition_mut(relation_id.partition_id);
    partition.relation_arena.lifecycle[slot] = RecordLifecycleState::RetainedDanglingForAudit;
    let _ = version_id;
    partition.relation_arena.live_bitset.set(slot, true);
    partition.relation_arena.reclaimable_bitset.set(slot, false);
    let endpoints = partition
        .relation_arena
        .get_slot(slot)
        .and_then(|relation_slot| relation_slot.extra().clone());
    let payload = partition
        .relation_arena
        .get_slot(slot)
        .and_then(|relation_slot| relation_slot.payload().cloned());
    let kind_id = partition
        .relation_arena
        .get_slot(slot)
        .and_then(|relation_slot| relation_slot.kind_id());
    let Some(endpoints) = endpoints else {
        return;
    };
    let Some(kind_id) = kind_id else {
        return;
    };
    outcome.record_change(RecordMutation::RelationRetainedForAudit {
        relation_id,
        kind_id,
        source: endpoints.source,
        target: endpoints.target,
        payload,
    });
}

pub(super) fn delete_relation(
    state: &mut WorkingState,
    version_id: crate::identity::data::VersionId,
    relation_id: RelationId,
    outcome: &mut MutationOutcome,
) {
    let slot = relation_id.local_slot.0 as usize;
    let relation_is_live = state
        .get_partition(relation_id.partition_id)
        .and_then(|partition| partition.relation_arena.get(&relation_id))
        .is_some_and(|relation_slot| relation_slot.lifecycle() == RecordLifecycleState::Live);
    if !relation_is_live {
        return;
    }
    state.mark_relation_slot_touched(relation_id.partition_id, slot);
    let partition = state.get_partition_mut(relation_id.partition_id);
    let endpoints = partition
        .relation_arena
        .get_slot(slot)
        .and_then(|relation_slot| relation_slot.extra().clone());
    let payload = partition
        .relation_arena
        .get_slot(slot)
        .and_then(|relation_slot| relation_slot.payload().cloned());
    let kind_id = partition
        .relation_arena
        .get_slot(slot)
        .and_then(|relation_slot| relation_slot.kind_id());
    partition.relation_arena.retire(slot, version_id);
    if let (Some(endpoints), Some(kind_id)) = (endpoints, kind_id) {
        outcome.record_change(RecordMutation::RelationDeleted {
            relation_id,
            kind_id,
            source: endpoints.source,
            target: endpoints.target,
            payload,
        });
    }
}

pub(crate) fn apply_adjacency_deltas(state: &mut WorkingState, deltas: &[AdjacencyDelta]) {
    for delta in deltas {
        let (source, target) = match delta.kind {
            AdjacencyDeltaKind::Created { source, target }
            | AdjacencyDeltaKind::Deleted { source, target } => (source, target),
        };
        state.mark_adjacency_slot_touched(source.partition_id, source.local_slot.0 as usize);
        state
            .mark_reverse_adjacency_slot_touched(target.partition_id, target.local_slot.0 as usize);

        let source_partition = ensure_partition_state(state, source.partition_id);
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

        let target_partition = ensure_partition_state(state, target.partition_id);
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
    state: &mut WorkingState,
    partition_id: PartitionId,
    requested_slots: usize,
) {
    let reusable_slots = state
        .get_partition(partition_id)
        .map(|partition| partition.entity_arena.free_list.len())
        .unwrap_or(0);
    let additional = requested_slots.saturating_sub(reusable_slots);
    state.reserve_entity_slots(partition_id, additional);
}

pub(super) fn reserve_bulk_relation_capacity(
    state: &mut WorkingState,
    partition_id: PartitionId,
    requested_slots: usize,
) {
    let reusable_slots = state
        .get_partition(partition_id)
        .map(|partition| partition.relation_arena.free_list.len())
        .unwrap_or(0);
    let additional = requested_slots.saturating_sub(reusable_slots);
    state.reserve_relation_slots(partition_id, additional);
}

fn ensure_partition_state(
    state: &mut WorkingState,
    partition_id: PartitionId,
) -> &mut PartitionState {
    state.get_partition_mut(partition_id)
}

fn allocate_record<K: RecordKind>(
    state: &mut WorkingState,
    partition_id: PartitionId,
    kind_id: KindId,
    payload: Option<RecordPayload>,
    version_id: crate::identity::data::VersionId,
    extra: K::Extra,
) -> (usize, u32, bool) {
    let partition = ensure_partition_state(state, partition_id);
    let arena = K::arena_mut(partition);
    let (slot, generation, reused) = arena.push_slot(crate::storage::logic::state::SlotInit {
        partition_id,
        kind_id,
        payload,
        version_id,
        extra,
    });
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

fn mutation_state_inconsistency(
    detail: impl Into<String>,
    fields: serde_json::Value,
) -> CommitConflict {
    CommitConflict::new(ConflictClass::MutationStateInconsistency {
        detail: detail.into(),
        fields,
    })
}
