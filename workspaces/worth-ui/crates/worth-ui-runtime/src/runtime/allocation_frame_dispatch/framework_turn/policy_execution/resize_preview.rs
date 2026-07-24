pub(in crate::runtime::allocation_frame_dispatch::framework_turn) fn execute(
    ledger: &crate::runtime::allocation_receipt::UiAllocationReceiptLedger,
    outcome: crate::runtime::UiResizePreviewOutcome,
    counters: super::super::UiFrameworkTransitionPlanningCounters,
) -> super::super::WorthUiFrameworkTurnCompletion<'_> {
    super::super::WorthUiFrameworkTurnCompletion::ResizePreviewPublished {
        pending: super::super::WorthUiPendingMountedPreviewProjection::new(
            outcome,
            crate::runtime::allocation_receipt::UiPreviewPaintIsolationPort::new(ledger),
        ),
        planning_counters: counters,
    }
}
