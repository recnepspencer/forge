use worth_ui::facade::{
    WorthUiReloadCertificationBundle, WorthUiReloadLatencyCounters,
    WorthUiReloadStormCandidateDenialReason, WorthUiReloadStormCandidateStep,
    WorthUiReloadStormCandidateStepKind, WorthUiReloadStormCertification,
    WorthUiReloadStormCertificationDenial, WorthUiReloadStormCertificationDenialReason,
    WorthUiReloadStormDeniedIteration, WorthUiReloadStormIterationOutcome,
    WorthUiReloadStormNoOpIteration, WorthUiReloadStormOrderedTruth,
    WorthUiReloadStormReceiptBinding, WorthUiReloadStormScenario,
    WorthUiReloadStormSuccessfulIteration, WorthUiSourceProvider, WorthUiWatcherEvent,
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
    _: Option<WorthUiReloadStormSuccessfulIteration>,
) {
}

fn main() {
    let provider = WorthUiSourceProvider::filesystem_root("compile");
    let _scenario = WorthUiReloadStormScenario::named("compile").with_file_candidate_events(
        "event-bearing candidate",
        provider,
        [WorthUiWatcherEvent::write_started("app/main.wui.tmp")],
    );
    assert_imports(
        None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    );
}
