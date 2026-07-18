pub(in crate::runtime::allocation_frame_dispatch::framework_turn) fn execute<'runtime>(
    ledger: &'runtime crate::runtime::allocation_receipt::UiAllocationReceiptLedger,
    invalidation_authority: &'runtime std::cell::RefCell<
        crate::runtime::invalidation_narrowing::UiAllocationInvalidationAuthority,
    >,
    execution: super::super::transition_planning::UiDragResizeExecutionPlan,
    counters: super::super::UiFrameworkTransitionPlanningCounters,
) -> super::super::WorthUiFrameworkTurnCompletion<'runtime> {
    super::super::WorthUiFrameworkTurnCompletion::DragResizePreviewPending {
        preview: super::super::WorthUiPendingPreviewPaint::new(
            crate::host::seal_preview_paint_input(execution.preview),
            crate::runtime::allocation_receipt::UiPreviewPaintIsolationPort::new(ledger),
        ),
        durable: super::super::WorthUiPendingDurableResize::new(
            super::super::UiPendingDurableResizeCommitPort::from_planned_transaction(
                ledger,
                invalidation_authority,
                execution.transaction,
            ),
            execution.selection,
            execution.identity_digest,
            execution.extent,
        ),
        planning_counters: counters,
    }
}
