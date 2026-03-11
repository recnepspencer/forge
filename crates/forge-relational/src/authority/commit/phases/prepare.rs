use crate::authority::commit::plan_building::bulk_reservations_for_plan;
use crate::authority::commit::touched_scope::touched_partitions_for_plan_set;
use crate::storage::overlay::RelationalDraft;
use crate::transactions::data::{MergedCommitPlan, TransactionCommitError};
use crate::transactions::logic::RelationalTransaction;

pub(crate) struct PreparedDraftScope {
    pub(crate) planning_state: crate::logic::runtime::WorkingState,
    pub(crate) merged_plan: MergedCommitPlan,
    pub(crate) draft: RelationalDraft,
}

pub(crate) fn prepare_draft_scope(
    transaction: &mut RelationalTransaction<'_>,
) -> Result<PreparedDraftScope, TransactionCommitError> {
    let planning_state = crate::logic::runtime::WorkingState::new(
        transaction.runtime.partitions.clone(),
        transaction.runtime.config.adjacency_policy.clone(),
    );
    let merged_plan = transaction
        .build_merged_plan_for_state(&planning_state)
        .map_err(TransactionCommitError::Conflict)?;
    let touched_partitions =
        touched_partitions_for_plan_set(&transaction.runtime.current_state(), &merged_plan);
    let draft = transaction
        .runtime
        .touched_partition_overlay(touched_partitions.iter().copied());

    Ok(PreparedDraftScope {
        planning_state,
        merged_plan,
        draft,
    })
}

pub(crate) fn record_preparation_counters(
    runtime: &mut crate::logic::runtime::RelationalRuntime,
    draft: &RelationalDraft,
    planning_state: &crate::logic::runtime::WorkingState,
    merged_plan: &MergedCommitPlan,
) {
    let mut counters = runtime
        .instrumentation
        .complexity_counters
        .lock()
        .expect("complexity counter lock poisoned");
    counters.partitions_touched_by_commit = draft.touched_partitions().len();
    let (bulk_entity_slots_reserved, bulk_relation_slots_reserved) =
        bulk_reservations_for_plan(planning_state, merged_plan);
    counters.bulk_entity_slots_reserved = bulk_entity_slots_reserved;
    counters.bulk_relation_slots_reserved = bulk_relation_slots_reserved;
}

pub(crate) fn record_mutation_counters(
    runtime: &mut crate::logic::runtime::RelationalRuntime,
    draft: &RelationalDraft,
) {
    let mut counters = runtime
        .instrumentation
        .complexity_counters
        .lock()
        .expect("complexity counter lock poisoned");
    counters.entity_slots_touched_by_commit = draft
        .mutation_journal()
        .values()
        .map(|journal| journal.entity_slots.len())
        .sum();
    counters.relation_slots_touched_by_commit = draft
        .mutation_journal()
        .values()
        .map(|journal| journal.relation_slots.len())
        .sum();
}
