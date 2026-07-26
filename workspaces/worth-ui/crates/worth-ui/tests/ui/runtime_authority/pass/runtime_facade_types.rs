use worth_ui::facade::app::{
    WorthUi, WorthUiActiveApplicationSession, WorthUiApplicationReplacementOutcome,
    WorthUiApplicationSemanticNoOpReceipt, WorthUiLoweredApplicationReplacement,
    WorthUiPendingApplicationCutover, WorthUiPreparedApplicationReplacement,
    WorthUiReplacementCandidateSummary, WorthUiReplacementPlannedCostEnvelope,
};
use worth_ui::facade::source::WorthUiWatchedCandidateSubmission;
use worth_ui_runtime::facade::application::{
    WorthUiOrdinaryFrameTarget, WorthUiReloadLoweringCounterReceipt,
};
use worth_ui_runtime::facade::execution::{
    UiAllocationFrameDispatcherState, WorthUiFrameBoundary, WorthUiFrameExecutionReceipt,
    WorthUiFrameWorkScope, WorthUiLaneParityReport, WorthUiSteadyFrameCounterDenial,
};
use worth_ui_runtime::facade::{
    WorthUiActiveFrameworkTurnCompletion, WorthUiActiveFrameworkTurnExecution,
    WorthUiActiveOrdinaryFrameCompletion,
};

fn accepts_app_entry(
    _app_entry: fn() -> worth_ui::facade::app::WorthUiApplicationBuilder,
    _session: Option<WorthUiActiveApplicationSession>,
    _submission: Option<WorthUiWatchedCandidateSubmission>,
    _prepared: Option<Box<WorthUiPreparedApplicationReplacement>>,
) {
    let _ = WorthUi::app;
}

fn accepts_replacement_preparation(
    _lowered: Option<WorthUiLoweredApplicationReplacement>,
    _summary: Option<WorthUiReplacementCandidateSummary>,
    _cost: Option<WorthUiReplacementPlannedCostEnvelope>,
    _pending: Option<WorthUiPendingApplicationCutover>,
) {
}

fn accepts_replacement_outcome(
    _outcome: Option<WorthUiApplicationReplacementOutcome>,
    _no_op: Option<WorthUiApplicationSemanticNoOpReceipt>,
    _reload: Option<WorthUiReloadLoweringCounterReceipt>,
) {
}

fn accepts_framework_execution<'session, 'execution>(
    _turn: Option<WorthUiActiveFrameworkTurnCompletion<'session>>,
    _execution: Option<WorthUiActiveFrameworkTurnExecution<'execution>>,
    _target: Option<WorthUiOrdinaryFrameTarget>,
    _completion: Option<WorthUiActiveOrdinaryFrameCompletion<'execution>>,
) {
}

fn accepts_frame_evidence(
    _frame_cost: Option<WorthUiFrameExecutionReceipt>,
    _work_scope: Option<WorthUiFrameWorkScope>,
    _counter_denial: Option<WorthUiSteadyFrameCounterDenial>,
) {
}

fn accepts_frame_scheduling(
    _boundary: Option<WorthUiFrameBoundary>,
    _parity: Option<WorthUiLaneParityReport>,
    _scheduler: Option<UiAllocationFrameDispatcherState>,
) {
}

fn main() {
    let _ = (
        accepts_app_entry,
        accepts_replacement_preparation,
        accepts_replacement_outcome,
        accepts_framework_execution,
        accepts_frame_evidence,
        accepts_frame_scheduling,
    );
}
