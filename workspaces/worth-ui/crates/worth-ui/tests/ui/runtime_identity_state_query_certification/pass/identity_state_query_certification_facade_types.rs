use worth_ui::facade::{
    WorthUiIdentityStateCertification, WorthUiIdentityStateQueryCertificationCounters,
    WorthUiIdentityStateQueryCertificationDenial,
    WorthUiIdentityStateQueryCertificationDenialReason,
    WorthUiIdentityStateQueryCertificationScenario, WorthUiQueryDriftCertification,
    WorthUiQueryDriftCertificationScenarioStep, WorthUiStateCarryForwardReceipt,
    WorthUiStateCertificationScenarioStep, WorthUiStateLifecycleReceipt,
    WorthUiStateQueryResidueScan,
};

fn assert_imports(
    _: Option<WorthUiIdentityStateCertification>,
    _: Option<WorthUiIdentityStateQueryCertificationCounters>,
    _: Option<WorthUiIdentityStateQueryCertificationDenial>,
    _: Option<WorthUiIdentityStateQueryCertificationDenialReason>,
    _: Option<WorthUiQueryDriftCertification>,
    _: Option<WorthUiQueryDriftCertificationScenarioStep>,
    _: Option<WorthUiStateCarryForwardReceipt>,
    _: Option<WorthUiStateCertificationScenarioStep>,
    _: Option<WorthUiStateLifecycleReceipt>,
    _: Option<WorthUiStateQueryResidueScan>,
) {
}

fn main() {
    let _scenario = WorthUiIdentityStateQueryCertificationScenario::named("compile");
    assert_imports(None, None, None, None, None, None, None, None, None, None);
}
