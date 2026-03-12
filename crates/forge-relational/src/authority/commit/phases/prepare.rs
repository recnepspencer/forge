use crate::authority::commit::plan_building::bulk_reservations_for_plan;
use crate::authority::commit::touched_scope::touched_partitions_for_plan_set;
use crate::storage::overlay::WorkingState;
use crate::transactions::data::{MergedCommitPlan, TransactionCommitError};
use crate::transactions::logic::RelationalTransaction;

pub(crate) struct PreparedWorkingStateScope {
    pub(crate) planning_state: crate::logic::runtime::WorkingState,
    pub(crate) merged_plan: MergedCommitPlan,
    pub(crate) working_state: WorkingState,
}

pub(crate) fn prepare_working_state_scope(
    transaction: &mut RelationalTransaction<'_>,
) -> Result<PreparedWorkingStateScope, TransactionCommitError> {
    let planning_state = crate::logic::runtime::WorkingState::new(
        transaction.runtime.partitions.clone(),
        transaction.runtime.config.storage.adjacency_policy.clone(),
    );
    let merged_plan = transaction
        .build_merged_plan_for_state(&planning_state)
        .map_err(TransactionCommitError::Conflict)?;
    let touched_partitions =
        touched_partitions_for_plan_set(&transaction.runtime.current_state(), &merged_plan);
    let working_state = transaction
        .runtime
        .working_state_for_touched_partitions(touched_partitions.iter().copied());

    Ok(PreparedWorkingStateScope {
        planning_state,
        merged_plan,
        working_state,
    })
}

pub(crate) fn record_preparation_counters(
    runtime: &mut crate::logic::runtime::RelationalRuntime,
    working_state: &WorkingState,
    planning_state: &crate::logic::runtime::WorkingState,
    merged_plan: &MergedCommitPlan,
) {
    let mut counters = runtime
        .services
        .instrumentation
        .complexity_counters
        .lock()
        .expect("complexity counter lock poisoned");
    counters.partitions_touched_by_commit = working_state.touched_partitions().len();
    let (bulk_entity_slots_reserved, bulk_relation_slots_reserved) =
        bulk_reservations_for_plan(planning_state, merged_plan);
    counters.bulk_entity_slots_reserved = bulk_entity_slots_reserved;
    counters.bulk_relation_slots_reserved = bulk_relation_slots_reserved;
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
