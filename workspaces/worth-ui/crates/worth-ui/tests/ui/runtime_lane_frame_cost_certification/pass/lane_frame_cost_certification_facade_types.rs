use worth_ui::facade::runtime::{
    WorthUiBroadScanRegressionDenial, WorthUiFrameCostCertification,
    WorthUiLaneAndFrameCostCertification, WorthUiLaneCertification,
    WorthUiLaneFrameCostCertificationCounters, WorthUiLaneFrameCostCertificationDenial,
    WorthUiLaneFrameCostCertificationDenialReason, WorthUiLaneFrameCostCertificationScenario,
    WorthUiLaneFrameCostFoundationalReadiness, WorthUiLaneScaleVariationProof,
    WorthUiNoSourceFrameProof,
};

fn assert_imports(
    _: Option<WorthUiBroadScanRegressionDenial>,
    _: Option<WorthUiFrameCostCertification>,
    _: Option<WorthUiLaneAndFrameCostCertification>,
    _: Option<WorthUiLaneCertification>,
    _: Option<WorthUiLaneFrameCostCertificationCounters>,
    _: Option<WorthUiLaneFrameCostCertificationDenial>,
    _: Option<WorthUiLaneFrameCostCertificationDenialReason>,
    _: Option<WorthUiLaneFrameCostFoundationalReadiness>,
    _: Option<WorthUiLaneScaleVariationProof>,
    _: Option<WorthUiNoSourceFrameProof>,
) {
}

fn main() {
    let _scenario = WorthUiLaneFrameCostCertificationScenario::named("compile");
    assert_imports(None, None, None, None, None, None, None, None, None, None);
}
