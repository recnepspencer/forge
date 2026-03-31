use crate::authority::commit::structural_summary::CommitStructuralSummary;
use crate::logic::runtime::RelationalRuntime;
use crate::storage::overlay::WorkingState;
use crate::transactions::data::{MergedCommitPlan, TransactionCommitError};
use crate::transactions::logic::RelationalTransaction;

pub(crate) struct PreparedWorkingStateScope {
    pub(crate) merged_plan: MergedCommitPlan,
    pub(crate) structural_summary: CommitStructuralSummary,
    pub(crate) working_state: WorkingState,
}

pub(crate) fn prepare_working_state_scope(
    transaction: &mut RelationalTransaction<'_>,
) -> Result<PreparedWorkingStateScope, TransactionCommitError> {
    let intents = transaction.normalized_intents_for_merge();
    let planning_state = transaction.runtime.storage_access().current_state();
    let merged_plan = transaction
        .build_merged_plan_for_state(&planning_state, intents)
        .map_err(TransactionCommitError::conflict)?;
    let (structural_summary, working_state) = prepare_authoritative_working_state_scope(
        transaction.runtime,
        &merged_plan,
        transaction.options.merge_parent_branches.len(),
    );

    Ok(PreparedWorkingStateScope {
        merged_plan,
        structural_summary,
        working_state,
    })
}

pub(crate) fn prepare_authoritative_working_state_scope(
    runtime: &mut RelationalRuntime,
    merged_plan: &MergedCommitPlan,
    merge_parent_count: usize,
) -> (CommitStructuralSummary, WorkingState) {
    let current_state = runtime.storage_access().current_state();
    let structural_summary = CommitStructuralSummary::derive(
        &current_state,
        &current_state,
        &merged_plan,
        merge_parent_count,
    );
    let working_state = runtime
        .storage_authority()
        .working_state_for_touched_partitions(
            structural_summary.touched_partitions.iter().copied(),
        );

    (structural_summary, working_state)
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
