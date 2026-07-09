//! Test-only host lane fixtures. Production callers must use `collect_host_measurement_evidence`.

use worth_ui_host_contract::{
    UiHostObservationValue, UiMeasurementEvidenceFamily, UiMeasurementRequest,
    UiMeasurementRequestIdentity, UiScrollContainerViewportRequest, WorthUiHostCapabilityReport,
    WorthUiMeasurementHostAdapter,
};
use worth_ui_inspection::UiEvidenceAuthorityGeneration;

use crate::evidence::{UiMeasurementEvidenceCategory, UiMeasurementResult};
use crate::host::{
    collect_host_measurement_evidence, UiHostMeasurementAssumptionProfile, UiHostMeasurementNeed,
    UiHostMeasurementNormalizationContext,
};

use super::measurement_result_test_support::normalization_context_for;

struct ValueStubAdapter {
    value: UiHostObservationValue,
}

impl WorthUiMeasurementHostAdapter for ValueStubAdapter {
    fn observe_measurement(&self, _request: &UiMeasurementRequest) -> UiHostObservationValue {
        self.value.clone()
    }
}

pub(crate) fn collect_measurement_via_host_lane_for_test(
    request: &UiMeasurementRequest,
    value: UiHostObservationValue,
    need: UiHostMeasurementNeed,
    evidence_family: UiMeasurementEvidenceFamily,
    generation: UiEvidenceAuthorityGeneration,
    report: &WorthUiHostCapabilityReport,
    normalization_context: UiHostMeasurementNormalizationContext,
) -> UiMeasurementResult {
    collect_host_measurement_evidence(
        &ValueStubAdapter { value },
        request.identity(),
        evidence_family,
        need,
        report,
        generation,
        normalization_context,
    )
    .expect("test host lane collection should admit")
}

pub(crate) fn collected_scroll_container_viewport_for_test(
    request_seed: u64,
    report: &WorthUiHostCapabilityReport,
    generation: UiEvidenceAuthorityGeneration,
) -> UiMeasurementResult {
    use super::measurement_result_test_support::CountingAdapter;

    let profile =
        UiHostMeasurementAssumptionProfile::from_capability_report(report, 11, 22, 33, 44);
    collect_host_measurement_evidence(
        &CountingAdapter::new(),
        UiMeasurementRequestIdentity::new(request_seed),
        UiMeasurementEvidenceFamily::ScrollContainerViewport,
        UiHostMeasurementNeed::ScrollContainerViewport(UiScrollContainerViewportRequest::new(55)),
        report,
        generation,
        normalization_context_for(
            UiMeasurementEvidenceCategory::ScrollContainerViewport,
            profile,
        ),
    )
    .expect("scroll container host lane collection should admit")
}
