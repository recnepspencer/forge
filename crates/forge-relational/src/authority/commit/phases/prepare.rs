use crate::authority::commit::structural_summary::CommitStructuralSummary;
use crate::logic::runtime::RelationalRuntime;
use crate::storage::overlay::summarize_entity_chunk_plan;
use crate::storage::overlay::EntityWorkingSetLayout;
use crate::storage::overlay::PartitionAccess;
use crate::storage::overlay::PartitionCloneMode;
use crate::storage::overlay::WorkingState;
use crate::transactions::data::CommitPhaseTiming;
use crate::transactions::data::{
    CommitTopology, CreateIntent, EntityMutationIntent, MergedCommitPlan, MutationIntent,
    TransactionCommitError,
};
use crate::transactions::logic::RelationalTransaction;
use std::collections::{BTreeMap, BTreeSet};
use std::time::Instant;

pub(crate) struct PreparedWorkingStateScope {
    pub(crate) merged_plan: MergedCommitPlan,
    pub(crate) structural_summary: CommitStructuralSummary,
    pub(crate) working_state: WorkingState,
    pub(crate) phase_timing: CommitPhaseTiming,
}

const AOSOA_ENTITY_CHUNK_WIDTH_SMALL: usize = 128;
const AOSOA_ENTITY_CHUNK_WIDTH_MEDIUM: usize = 256;
const AOSOA_ENTITY_CHUNK_WIDTH_LARGE: usize = 512;
const AOSOA_SPARSE_ENTITY_SLOT_LIMIT: usize = 1024;
const AOSOA_SPARSE_PARTITION_LIMIT: usize = 8;

fn select_entity_working_set_layout(
    clone_mode: PartitionCloneMode,
    cloned_entity_slots: usize,
) -> EntityWorkingSetLayout {
    if !matches!(
        clone_mode,
        PartitionCloneMode::EntityOnly | PartitionCloneMode::GraphSparseEntities
    ) {
        return EntityWorkingSetLayout::CanonicalSoA;
    }

    let chunk_width = if cloned_entity_slots <= 4_096 {
        AOSOA_ENTITY_CHUNK_WIDTH_SMALL
    } else if cloned_entity_slots <= 16_384 {
        AOSOA_ENTITY_CHUNK_WIDTH_MEDIUM
    } else {
        AOSOA_ENTITY_CHUNK_WIDTH_LARGE
    };

    EntityWorkingSetLayout::AoSoACandidate { chunk_width }
}

fn sparse_entity_slots_for_plan(
    clone_mode: PartitionCloneMode,
    merged_plan: &MergedCommitPlan,
) -> Option<BTreeMap<crate::identity::data::PartitionId, BTreeSet<usize>>> {
    if !matches!(
        clone_mode,
        PartitionCloneMode::EntityOnly | PartitionCloneMode::GraphSparseEntities
    ) {
        return None;
    }

    let mut slots_by_partition = BTreeMap::new();
    for intent in &merged_plan.merged_intents {
        match intent {
            MutationIntent::Entity(EntityMutationIntent::UpdateFields(spec)) => {
                slots_by_partition
                    .entry(spec.entity_id.partition_id)
                    .or_insert_with(BTreeSet::new)
                    .insert(spec.entity_id.slot_index());
            }
            MutationIntent::Create(_)
                if matches!(clone_mode, PartitionCloneMode::GraphSparseEntities) => {}
            _ => return None,
        }
    }

    let total_slots: usize = slots_by_partition.values().map(BTreeSet::len).sum();
    (total_slots > 0
        && total_slots <= AOSOA_SPARSE_ENTITY_SLOT_LIMIT
        && slots_by_partition.len() <= AOSOA_SPARSE_PARTITION_LIMIT)
        .then_some(slots_by_partition)
}

fn sparse_relation_overlay_partitions_for_plan(
    clone_mode: PartitionCloneMode,
    merged_plan: &MergedCommitPlan,
) -> Option<BTreeSet<crate::identity::data::PartitionId>> {
    if !matches!(clone_mode, PartitionCloneMode::GraphSparseEntities) {
        return None;
    }

    let mut partitions = BTreeSet::new();
    let mut saw_relation_create = false;
    for intent in &merged_plan.merged_intents {
        match intent {
            MutationIntent::Create(CreateIntent::Relation(spec)) => {
                saw_relation_create = true;
                partitions.insert(spec.partition_id);
            }
            MutationIntent::Create(CreateIntent::BulkRelations(spec)) => {
                saw_relation_create = true;
                partitions.insert(spec.partition_id);
            }
            MutationIntent::Create(CreateIntent::Entity(_))
            | MutationIntent::Create(CreateIntent::BulkEntities(_))
            | MutationIntent::Entity(EntityMutationIntent::UpdateFields(_)) => {}
            _ => return None,
        }
    }

    saw_relation_create.then_some(partitions)
}

pub(crate) fn prepare_working_state_scope(
    transaction: &mut RelationalTransaction<'_>,
) -> Result<PreparedWorkingStateScope, TransactionCommitError> {
    let mut phase_timing = CommitPhaseTiming::default();
    let normalization_started = Instant::now();
    let intents = transaction.normalized_intents_for_merge();
    phase_timing.draft_intent_normalization_micros =
        normalization_started.elapsed().as_micros() as u64;
    let planning_state = transaction.runtime.storage_access().current_state();
    let merge_plan_started = Instant::now();
    let (merged_plan, merged_plan_timing) = transaction
        .build_merged_plan_for_state_with_timing(&planning_state, intents)
        .map_err(TransactionCommitError::conflict)?;
    phase_timing.draft_merge_plan_micros = merge_plan_started.elapsed().as_micros() as u64;
    phase_timing.draft_intent_validation_micros = merged_plan_timing.validation_micros;
    phase_timing.draft_intent_sort_micros = merged_plan_timing.sort_micros;
    phase_timing.draft_conflict_detection_micros = merged_plan_timing.conflict_detection_micros;
    let (structural_summary, working_state, prepare_phase_timing) =
        prepare_authoritative_working_state_scope(
            transaction.runtime,
            &merged_plan,
            transaction.options.merge_parent_branches.len(),
        );
    phase_timing.draft_structural_summary_micros =
        prepare_phase_timing.draft_structural_summary_micros;
    phase_timing.draft_working_state_clone_micros =
        prepare_phase_timing.draft_working_state_clone_micros;

    Ok(PreparedWorkingStateScope {
        merged_plan,
        structural_summary,
        working_state,
        phase_timing,
    })
}

pub(crate) fn prepare_authoritative_working_state_scope(
    runtime: &mut RelationalRuntime,
    merged_plan: &MergedCommitPlan,
    merge_parent_count: usize,
) -> (CommitStructuralSummary, WorkingState, CommitPhaseTiming) {
    let mut phase_timing = CommitPhaseTiming::default();
    let current_state = runtime.storage_access().current_state();
    let summary_started = Instant::now();
    let structural_summary = CommitStructuralSummary::derive(
        &current_state,
        &current_state,
        &merged_plan,
        merge_parent_count,
    );
    phase_timing.draft_structural_summary_micros = summary_started.elapsed().as_micros() as u64;
    let clone_mode = match structural_summary.commit_topology {
        CommitTopology::FlatEntityBatch => PartitionCloneMode::EntityOnly,
        CommitTopology::GraphMutation => PartitionCloneMode::GraphSparseEntities,
        CommitTopology::BranchMerge => PartitionCloneMode::Full,
    };
    let cloned_partition_count = structural_summary.touched_partitions.len();
    let cloned_entity_slots = structural_summary
        .touched_partitions
        .iter()
        .map(|partition_id| {
            current_state
                .get_partition(*partition_id)
                .map(|partition| partition.entity_arena.slot_count())
                .unwrap_or(0)
        })
        .sum();
    let cloned_relation_slots = if matches!(clone_mode, PartitionCloneMode::Full) {
        structural_summary
            .touched_partitions
            .iter()
            .map(|partition_id| {
                current_state
                    .get_partition(*partition_id)
                    .map(|partition| partition.relation_arena.slot_count())
                    .unwrap_or(0)
            })
            .sum()
    } else {
        0
    };
    let sparse_entity_slots = sparse_entity_slots_for_plan(clone_mode, merged_plan);
    let sparse_relation_overlay_partitions =
        sparse_relation_overlay_partitions_for_plan(clone_mode, merged_plan);
    let clone_mode = if matches!(clone_mode, PartitionCloneMode::GraphSparseEntities)
        && sparse_entity_slots.is_none()
        && sparse_relation_overlay_partitions.is_none()
    {
        PartitionCloneMode::Full
    } else {
        clone_mode
    };
    let sparse_entity_slot_count = sparse_entity_slots
        .as_ref()
        .map(|slots| slots.values().map(BTreeSet::len).sum::<usize>());
    let entity_working_set_layout = select_entity_working_set_layout(
        clone_mode,
        sparse_entity_slot_count.unwrap_or(cloned_entity_slots),
    );
    let candidate_chunk_plan = summarize_entity_chunk_plan(
        sparse_entity_slot_count.unwrap_or(cloned_entity_slots),
        entity_working_set_layout,
    );
    let clone_started = Instant::now();
    let working_state = runtime
        .storage_authority()
        .working_state_for_touched_partitions(
            structural_summary.touched_partitions.iter().copied(),
            clone_mode,
            entity_working_set_layout,
            sparse_entity_slots.as_ref(),
            sparse_relation_overlay_partitions.as_ref(),
        );
    phase_timing.draft_working_state_clone_micros = clone_started.elapsed().as_micros() as u64;
    if matches!(clone_mode, PartitionCloneMode::Full) {
        runtime.performance_access().count_working_state_clone(
            cloned_partition_count,
            cloned_entity_slots,
            cloned_relation_slots,
        );
    }
    match working_state.entity_working_set_layout() {
        EntityWorkingSetLayout::CanonicalSoA => {}
        EntityWorkingSetLayout::AoSoACandidate { .. } => {
            runtime.performance_access().count_aosoa_prepare_chunks(
                candidate_chunk_plan.chunk_count,
                candidate_chunk_plan.slot_count,
            )
        }
    }

    (structural_summary, working_state, phase_timing)
}

pub(crate) fn record_preparation_counters(
    runtime: &mut crate::logic::runtime::RelationalRuntime,
    working_state: &WorkingState,
    structural_summary: &CommitStructuralSummary,
) {
    let mut counters = runtime
        .services
        .instrumentation
        .complexity_counters
        .lock()
        .expect("complexity counter lock poisoned");
    counters.commit_topology_flags = structural_summary.commit_topology.mask();
    counters.partitions_touched_by_commit = working_state.touched_partition_count();
    counters.bulk_entity_slots_reserved = structural_summary.bulk_entity_slots_reserved;
    counters.bulk_relation_slots_reserved = structural_summary.bulk_relation_slots_reserved;
}

pub(crate) fn record_mutation_counters(
    runtime: &mut crate::logic::runtime::RelationalRuntime,
    working_state: &WorkingState,
) {
    let mut counters = runtime
        .services
        .instrumentation
        .complexity_counters
        .lock()
        .expect("complexity counter lock poisoned");
    counters.entity_slots_touched_by_commit = working_state
        .mutation_journal()
        .values()
        .map(|journal| journal.entity_slots.len())
        .sum();
    counters.relation_slots_touched_by_commit = working_state
        .mutation_journal()
        .values()
        .map(|journal| journal.relation_slots.len())
        .sum();
}

#[cfg(test)]
mod tests {
    use crate::identity::data::{EntityId, PartitionId};
    use crate::transactions::data::{
        AspectFieldPatch, EntityMutationIntent, MergedCommitPlan, MutationIntent, TransactionId,
        UpdateEntityFieldsIntent,
    };

    use super::{
        select_entity_working_set_layout, sparse_entity_slots_for_plan, EntityWorkingSetLayout,
        PartitionCloneMode, AOSOA_ENTITY_CHUNK_WIDTH_SMALL,
    };

    fn update_intent(partition: u32, slot: u64) -> MutationIntent {
        MutationIntent::Entity(EntityMutationIntent::UpdateFields(
            UpdateEntityFieldsIntent {
                entity_id: EntityId::new(PartitionId(partition), slot, 1),
                fields: AspectFieldPatch::default(),
            },
        ))
    }

    fn merged_plan(intents: Vec<MutationIntent>) -> MergedCommitPlan {
        MergedCommitPlan {
            transaction_id: TransactionId(1),
            merged_intents: intents,
        }
    }

    #[test]
    fn sparse_entity_slots_accepts_multi_partition_update_batches() {
        let plan = merged_plan(vec![
            update_intent(1, 0),
            update_intent(1, 1),
            update_intent(2, 7),
            update_intent(3, 9),
        ]);

        let sparse = sparse_entity_slots_for_plan(PartitionCloneMode::EntityOnly, &plan)
            .expect("multi-partition entity batch should stay on sparse path");

        assert_eq!(sparse.len(), 3);
        assert_eq!(
            sparse.get(&PartitionId(1)).map(|slots| slots.len()),
            Some(2)
        );
        assert_eq!(
            sparse.get(&PartitionId(2)).map(|slots| slots.len()),
            Some(1)
        );
        assert_eq!(
            sparse.get(&PartitionId(3)).map(|slots| slots.len()),
            Some(1)
        );
    }

    #[test]
    fn sparse_layout_selection_uses_sparse_slot_count_not_full_partition_clone_width() {
        let layout = select_entity_working_set_layout(PartitionCloneMode::EntityOnly, 96);
        assert_eq!(
            layout,
            EntityWorkingSetLayout::AoSoACandidate {
                chunk_width: AOSOA_ENTITY_CHUNK_WIDTH_SMALL,
            }
        );
    }
}
