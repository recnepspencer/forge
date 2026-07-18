pub(in crate::runtime::allocation_frame_dispatch::framework_turn) fn execute<'runtime>(
    ledger: &'runtime crate::runtime::allocation_receipt::UiAllocationReceiptLedger,
    invalidation_authority: &'runtime std::cell::RefCell<
        crate::runtime::invalidation_narrowing::UiAllocationInvalidationAuthority,
    >,
    execution: super::super::transition_planning::UiOrdinaryAllocationExecutionPlan,
    counters: super::super::UiFrameworkTransitionPlanningCounters,
) -> super::super::WorthUiFrameworkTurnCompletion<'runtime> {
    let transaction = super::super::allocation_transaction::publish_pending(
        ledger,
        &mut invalidation_authority.borrow_mut(),
        execution.transaction,
    );
    super::super::WorthUiFrameworkTurnCompletion::AllocationInvalidationsNarrowed {
        plan: execution.plan,
        selection: execution.selection,
        transaction,
        planning_counters: counters,
    }
}

pub(in crate::runtime::allocation_frame_dispatch::framework_turn) fn deny<'runtime>(
    execution: super::super::transition_planning::UiDeniedAllocationExecutionPlan,
    counters: super::super::UiFrameworkTransitionPlanningCounters,
) -> super::super::WorthUiFrameworkTurnCompletion<'runtime> {
    super::super::WorthUiFrameworkTurnCompletion::AllocationInvalidationsNarrowed {
        plan: execution.plan,
        selection: execution.selection,
        transaction: crate::runtime::UiAllocationReplanTransactionOutcome::Denied(execution.denial),
        planning_counters: counters,
    }
}
