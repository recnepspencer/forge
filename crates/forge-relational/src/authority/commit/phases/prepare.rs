use crate::authority::commit::structural_summary::CommitStructuralSummary;
use crate::logic::runtime::RelationalRuntime;
use crate::storage::overlay::WorkingState;
use crate::storage::overlay::PartitionAccess;
use crate::storage::overlay::PartitionCloneMode;
use crate::transactions::data::CommitPhaseTiming;
use crate::transactions::data::{CommitTopology, MergedCommitPlan, TransactionCommitError};
use crate::transactions::logic::RelationalTransaction;
use std::time::Instant;

pub(crate) struct PreparedWorkingStateScope {
    pub(crate) merged_plan: MergedCommitPlan,
    pub(crate) structural_summary: CommitStructuralSummary,
    pub(crate) working_state: WorkingState,
    pub(crate) phase_timing: CommitPhaseTiming,
}

pub(crate) fn prepare_working_state_scope(
    transaction: &mut RelationalTransaction<'_>,
) -> Result<PreparedWorkingStateScope, TransactionCommitError> {
    let mut phase_timing = CommitPhaseTiming::default();
    let merge_plan_started = Instant::now();
    let intents = transaction.normalized_intents_for_merge();
    let planning_state = transaction.runtime.storage_access().current_state();
    let merged_plan = transaction
        .build_merged_plan_for_state(&planning_state, intents)
        .map_err(TransactionCommitError::conflict)?;
    phase_timing.draft_merge_plan_micros = merge_plan_started.elapsed().as_micros() as u64;
    let (structural_summary, working_state, prepare_phase_timing) = prepare_authoritative_working_state_scope(
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
    phase_timing.draft_structural_summary_micros =
        summary_started.elapsed().as_micros() as u64;
    let clone_mode = match structural_summary.commit_topology {
        CommitTopology::FlatEntityBatch => PartitionCloneMode::EntityOnly,
        CommitTopology::GraphMutation | CommitTopology::BranchMerge => PartitionCloneMode::Full,
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
    let clone_started = Instant::now();
    let working_state = runtime
        .storage_authority()
        .working_state_for_touched_partitions(
            structural_summary.touched_partitions.iter().copied(),
            clone_mode,
        );
    phase_timing.draft_working_state_clone_micros =
        clone_started.elapsed().as_micros() as u64;
    runtime.performance_access().count_working_state_clone(
        cloned_partition_count,
        cloned_entity_slots,
        cloned_relation_slots,
    );

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
