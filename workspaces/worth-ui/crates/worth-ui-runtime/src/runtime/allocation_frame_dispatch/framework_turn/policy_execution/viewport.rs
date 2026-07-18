pub(in crate::runtime::allocation_frame_dispatch::framework_turn) fn execute<'runtime>(
    ledger: &'runtime crate::runtime::allocation_receipt::UiAllocationReceiptLedger,
    invalidation_authority: &'runtime std::cell::RefCell<
        crate::runtime::invalidation_narrowing::UiAllocationInvalidationAuthority,
    >,
    execution: super::super::transition_planning::UiViewportResizeExecutionPlan,
    counters: super::super::UiFrameworkTransitionPlanningCounters,
) -> super::super::WorthUiFrameworkTurnCompletion<'runtime> {
    let transaction = super::super::allocation_transaction::publish_pending(
        ledger,
        &mut invalidation_authority.borrow_mut(),
        execution.transaction,
    );
    match crate::runtime::UiViewportResizeOutcome::resolve(transaction) {
        Ok(outcome) => super::super::WorthUiFrameworkTurnCompletion::ViewportResizeResolved {
            outcome,
            planning_counters: counters,
        },
        Err(denial) => super::super::WorthUiFrameworkTurnCompletion::ViewportResizeDenied {
            denial,
            planning_counters: counters,
        },
    }
}

pub(in crate::runtime::allocation_frame_dispatch::framework_turn) fn deny<'runtime>(
    denial: crate::runtime::UiViewportResizeDenial,
    counters: super::super::UiFrameworkTransitionPlanningCounters,
) -> super::super::WorthUiFrameworkTurnCompletion<'runtime> {
    super::super::WorthUiFrameworkTurnCompletion::ViewportResizeDenied {
        denial,
        planning_counters: counters,
    }
}
