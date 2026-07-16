use std::collections::BTreeSet;

use crate::config::data::CascadeDeletePolicy;
use crate::identity::data::{EntityId, KindId, PartitionId, RelationId};
use crate::schema::data::RelationalSchemaRegistry;
use crate::storage::logic::state::{
    AdjacencySet, EntityExtra, EntityRecordKind, PartitionAccess, PartitionState, RecordKind,
    RelationEndpoints, RelationExtra, RelationRecordKind,
};
use crate::storage::overlay::WorkingState;
use crate::transactions::data::{
    CommitConflict, ConflictClass, EntityCascadeDeleteMissingState,
    MutationStateInconsistencyEvidence,
};

use super::intents::entity_authoritative_deletion_patch::plan_entity_authoritative_deletion_patch;
use super::outcomes::{MutationOutcome, RecordMutation};
use super::{AdjacencyDelta, AdjacencyDeltaKind};

mod relation_lifecycle;

pub(super) use relation_lifecycle::{delete_relation, retain_relation_dangling_for_audit};

pub(super) fn allocate_entity_with_extra(
    state: &mut WorkingState,
    version_id: crate::identity::data::VersionId,
    partition_id: PartitionId,
    kind_id: KindId,
    extra: EntityExtra,
) -> EntityId {
    let (slot, generation, reused) =
        allocate_record::<EntityRecordKind>(state, partition_id, kind_id, version_id, extra);
    if reused {
        state.mark_entity_free_list_changed(partition_id);
    }
    let partition = ensure_partition_state(state, partition_id);
    if reused {
        let idx = slot;
        while partition.adjacency.len() <= idx {
            partition
                .adjacency
                .push(AdjacencySet::new(&partition.adjacency_policy));
        }
        while partition.reverse_adjacency.len() <= idx {
            partition
                .reverse_adjacency
                .push(AdjacencySet::new(&partition.adjacency_policy));
        }
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
    partition_id: PartitionId,
    kind_id: KindId,
    source: EntityId,
    target: EntityId,
    authoritative_aspect_state: Option<worth_foundational::facade::AuthoritativeRecordAspectState>,
) -> RelationId {
    let extra = RelationExtra {
        endpoints: Some(RelationEndpoints { source, target }),
        authoritative_aspect_state,
    };
    let (slot, generation, reused) =
        allocate_record::<RelationRecordKind>(state, partition_id, kind_id, version_id, extra);
    if reused {
        state.mark_relation_free_list_changed(partition_id);
    }
    RelationId::new(partition_id, slot as u64, generation)
}

pub(super) fn delete_entity_with_cascade(
    state: &mut WorkingState,
    version_id: crate::identity::data::VersionId,
    entity_id: EntityId,
    schema_registry: &RelationalSchemaRegistry,
    default_cascade_delete_policy: CascadeDeletePolicy,
    outcome: &mut MutationOutcome,
) -> Result<(), CommitConflict> {
    let slot = entity_id.slot_index();
    state.mark_entity_slot_touched(entity_id.partition_id, slot);
    let partition = state.get_partition_mut(entity_id.partition_id);
    let slot_view = partition.entity_arena.get_slot(slot).ok_or_else(|| {
        mutation_state_inconsistency(
            "entity delete requires an existing slot after stale-target validation",
            entity_cascade_delete_missing(entity_id, EntityCascadeDeleteMissingState::Slot),
        )
    })?;
    let kind_id = slot_view.kind_id().ok_or_else(|| {
        mutation_state_inconsistency(
            "entity delete requires a retained kind id after stale-target validation",
            entity_cascade_delete_missing(entity_id, EntityCascadeDeleteMissingState::KindId),
        )
    })?;
    let authoritative_patch = plan_entity_authoritative_deletion_patch(
        slot_view.extra().authoritative_aspect_state.as_ref(),
        &schema_registry
            .entity_registration(kind_id)
            .map_err(|error| {
                CommitConflict::new(ConflictClass::KindSchemaMismatch {
                    detail: format!(
                        "entity delete requires registered aspect contracts: {error:?}"
                    ),
                })
            })?
            .aspect_contract_declarations,
    )
    .map_err(|denial| {
        CommitConflict::new(ConflictClass::EntityAuthoritativeAspectStateDenied { kind_id, denial })
    })?;
    partition.entity_arena.retire(slot, version_id);
    outcome.record_change(RecordMutation::EntityDeleted {
        entity_id,
        kind_id,
        authoritative_patch,
    });

    let mut attached = BTreeSet::new();
    if let Some(adjacency) = partition.adjacency.get(slot) {
        adjacency.extend_into(&mut attached);
    }
    if let Some(reverse_adjacency) = partition.reverse_adjacency.get(slot) {
        reverse_adjacency.extend_into(&mut attached);
    }
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

pub(crate) fn apply_adjacency_deltas(state: &mut WorkingState, deltas: &[AdjacencyDelta]) {
    for delta in deltas {
        let (source, target) = match delta.kind {
            AdjacencyDeltaKind::Created { source, target }
            | AdjacencyDeltaKind::Deleted { source, target } => (source, target),
        };
        state.mark_adjacency_slot_touched(source.partition_id, source.slot_index());
        state.mark_reverse_adjacency_slot_touched(target.partition_id, target.slot_index());

        let source_partition = ensure_partition_state(state, source.partition_id);
        ensure_entity_adjacency_capacity(source_partition, source.slot_index());
        match delta.kind {
            AdjacencyDeltaKind::Created { .. } => {
                source_partition.adjacency[source.slot_index()].insert(delta.relation_id);
            }
            AdjacencyDeltaKind::Deleted { .. } => {
                if let Some(relations) = source_partition.adjacency.get_mut(source.slot_index()) {
                    relations.remove(&delta.relation_id);
                }
            }
        }

        let target_partition = ensure_partition_state(state, target.partition_id);
        ensure_entity_adjacency_capacity(target_partition, target.slot_index());
        match delta.kind {
            AdjacencyDeltaKind::Created { .. } => {
                target_partition.reverse_adjacency[target.slot_index()].insert(delta.relation_id);
            }
            AdjacencyDeltaKind::Deleted { .. } => {
                if let Some(relations) = target_partition
                    .reverse_adjacency
                    .get_mut(target.slot_index())
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
    version_id: crate::identity::data::VersionId,
    extra: K::Extra,
) -> (usize, u32, bool) {
    let partition = ensure_partition_state(state, partition_id);
    let arena = K::arena_mut(partition);
    let (slot, generation, reused) = arena.push_slot(crate::storage::logic::state::SlotInit {
        partition_id,
        kind_id,
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
    evidence: MutationStateInconsistencyEvidence,
) -> CommitConflict {
    CommitConflict::new(ConflictClass::MutationStateInconsistency {
        detail: detail.into(),
        evidence,
    })
}

fn entity_cascade_delete_missing(
    entity_id: EntityId,
    missing: EntityCascadeDeleteMissingState,
) -> MutationStateInconsistencyEvidence {
    MutationStateInconsistencyEvidence::EntityCascadeDelete { entity_id, missing }
}
