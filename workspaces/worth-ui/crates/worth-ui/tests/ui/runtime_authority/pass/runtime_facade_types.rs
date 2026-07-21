use worth_ui::facade::{
    app::{
        WorthUi, WorthUiActiveApplicationSession, WorthUiApplicationReplacementOutcome,
        WorthUiApplicationSemanticNoOpReceipt, WorthUiLoweredApplicationReplacement,
        WorthUiPendingApplicationCutover, WorthUiPreparedApplicationReplacement,
        WorthUiReloadLoweringCounterReceipt, WorthUiReplacementCandidateSummary,
        WorthUiReplacementPlannedCostEnvelope, WorthUiActiveFrameworkTurnCompletion,
        WorthUiActiveFrameworkTurnExecution, WorthUiActiveOrdinaryFrameCompletion,
        WorthUiOrdinaryFrameTarget,
    },
    runtime::{
        UiAllocationFrameDispatcherState, WorthUiFrameBoundary, WorthUiFrameExecutionReceipt,
        WorthUiFrameWorkScope, WorthUiLaneParityReport, WorthUiSteadyFrameCounterDenial,
    },
    source::WorthUiWatchedCandidateSubmission,
};

fn accepts_canonical_lifecycle<'session, 'execution>(
    _app_entry: fn() -> worth_ui::facade::app::WorthUiBuilder,
    _session: Option<WorthUiActiveApplicationSession>,
    _submission: Option<WorthUiWatchedCandidateSubmission>,
    _prepared: Option<Box<WorthUiPreparedApplicationReplacement>>,
    _lowered: Option<WorthUiLoweredApplicationReplacement>,
    _summary: Option<WorthUiReplacementCandidateSummary>,
    _cost: Option<WorthUiReplacementPlannedCostEnvelope>,
    _pending: Option<WorthUiPendingApplicationCutover>,
    _outcome: Option<WorthUiApplicationReplacementOutcome>,
    _no_op: Option<WorthUiApplicationSemanticNoOpReceipt>,
    _reload: Option<WorthUiReloadLoweringCounterReceipt>,
    _turn: Option<WorthUiActiveFrameworkTurnCompletion<'session>>,
    _execution: Option<WorthUiActiveFrameworkTurnExecution<'execution>>,
    _target: Option<WorthUiOrdinaryFrameTarget>,
    _completion: Option<WorthUiActiveOrdinaryFrameCompletion<'execution>>,
    _frame_cost: Option<WorthUiFrameExecutionReceipt>,
    _work_scope: Option<WorthUiFrameWorkScope>,
    _counter_denial: Option<WorthUiSteadyFrameCounterDenial>,
    _boundary: Option<WorthUiFrameBoundary>,
    _parity: Option<WorthUiLaneParityReport>,
    _scheduler: Option<UiAllocationFrameDispatcherState>,
) {
    let _ = WorthUi::app;
}

fn main() {
    let _ = accepts_canonical_lifecycle;
}
