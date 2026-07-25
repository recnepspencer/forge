//! Test-only host lane fixtures. Production callers use the runtime collector capability.

use worth_ui_host_contract::{
    UiHostMeasurementObservationValue, UiHostMeasurementRequest, UiMeasurementEvidenceFamily,
    UiMeasurementRequestIdentity, UiScrollContainerViewportRequest, WorthUiHostCapabilityReport,
    WorthUiMeasurementHostAdapter,
};
use worth_ui_inspection::UiEvidenceAuthorityGeneration;

use crate::evidence::{UiMeasurementEvidenceCategory, UiMeasurementResult};
use crate::host::{
    UiHostMeasurementAssumptionProfile, UiHostMeasurementNeed,
    UiHostMeasurementNormalizationContext, WorthUiHostMeasurementCollector,
};

use super::measurement_result_test_support::normalization_context_for;

struct ValueStubAdapter {
    value: UiHostMeasurementObservationValue,
}

impl WorthUiMeasurementHostAdapter for ValueStubAdapter {
    fn observe_measurement(
        &self,
        _request: &UiHostMeasurementRequest,
    ) -> UiHostMeasurementObservationValue {
        self.value.clone()
    }
}

pub(crate) fn collect_measurement_via_host_lane_for_test(
    request: &UiHostMeasurementRequest,
    value: UiHostMeasurementObservationValue,
    need: UiHostMeasurementNeed,
    evidence_family: UiMeasurementEvidenceFamily,
    generation: UiEvidenceAuthorityGeneration,
    report: &WorthUiHostCapabilityReport,
    normalization_context: UiHostMeasurementNormalizationContext,
) -> UiMeasurementResult {
    WorthUiHostMeasurementCollector::for_internal_proof()
        .collect(
            &ValueStubAdapter { value },
            crate::host::UiHostMeasurementCollectionInput {
                identity: request.identity(),
                evidence_family,
                need,
                capability_report: report,
                evidence_generation: generation,
                normalization_context,
            },
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
    WorthUiHostMeasurementCollector::for_internal_proof()
        .collect(
            &CountingAdapter::new(),
            crate::host::UiHostMeasurementCollectionInput {
                identity: UiMeasurementRequestIdentity::new(request_seed),
                evidence_family: UiMeasurementEvidenceFamily::ScrollContainerViewport,
                need: UiHostMeasurementNeed::ScrollContainerViewport(
                    UiScrollContainerViewportRequest::new(55),
                ),
                capability_report: report,
                evidence_generation: generation,
                normalization_context: normalization_context_for(
                    UiMeasurementEvidenceCategory::ScrollContainerViewport,
                    profile,
                ),
            },
        )
        .expect("scroll container host lane collection should admit")
}
