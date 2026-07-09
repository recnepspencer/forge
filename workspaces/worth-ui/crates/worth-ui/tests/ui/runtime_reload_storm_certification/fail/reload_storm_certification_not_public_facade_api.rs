use worth_ui::facade::{
    WorthUiReloadCertificationBundle, WorthUiReloadLatencyCounters,
    WorthUiReloadStormCandidateDenialReason, WorthUiReloadStormCandidateStep,
    WorthUiReloadStormCandidateStepKind, WorthUiReloadStormCertification,
    WorthUiReloadStormCertificationDenial, WorthUiReloadStormCertificationDenialReason,
    WorthUiReloadStormDeniedIteration, WorthUiReloadStormIterationOutcome,
    WorthUiReloadStormNoOpIteration, WorthUiReloadStormOrderedTruth,
    WorthUiReloadStormReceiptBinding, WorthUiReloadStormScenario,
    WorthUiReloadStormSuccessfulIteration,
};

fn assert_imports(
    _: Option<WorthUiReloadCertificationBundle>,
    _: Option<WorthUiReloadLatencyCounters>,
    _: Option<WorthUiReloadStormCandidateDenialReason>,
    _: Option<WorthUiReloadStormCandidateStep>,
    _: Option<WorthUiReloadStormCandidateStepKind>,
    _: Option<WorthUiReloadStormCertification>,
    _: Option<WorthUiReloadStormCertificationDenial>,
    _: Option<WorthUiReloadStormCertificationDenialReason>,
    _: Option<WorthUiReloadStormDeniedIteration>,
    _: Option<WorthUiReloadStormIterationOutcome>,
    _: Option<WorthUiReloadStormNoOpIteration>,
    _: Option<WorthUiReloadStormOrderedTruth>,
    _: Option<WorthUiReloadStormReceiptBinding>,
    _: Option<WorthUiReloadStormScenario>,
    _: Option<WorthUiReloadStormSuccessfulIteration>,
) {
}

fn main() {
    assert_imports(
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    );
}
