use worth_ui_host_contract::{
    UiHostObservationValue, UiMeasurementEvidenceFamily, UiMeasurementRequest,
    UiMeasurementRequestIdentity, UiPortalAnchorRectObservation, UiPortalAnchorRectRequest,
    UiScrollContainerViewportObservation, UiScrollContainerViewportRequest,
    UiTextIntrinsicSizeObservation, UiTextIntrinsicSizeRequest, WorthUiHostCapability,
    WorthUiHostCapabilityObservationGeneration, WorthUiHostCapabilityReport,
};
use worth_ui_inspection::UiEvidenceAuthorityGeneration;

use crate::evidence::UiMeasurementResult;
use crate::host::{
    UiHostMeasurementAssumptionProfile, UiHostMeasurementNeed,
    UiHostMeasurementNormalizationContext, WorthUiHostMeasurementCollector,
};

pub(super) fn capability_report(generation: u64) -> WorthUiHostCapabilityReport {
    WorthUiHostCapabilityReport::available(vec![
        WorthUiHostCapability::TextIntrinsicMeasurement,
        WorthUiHostCapability::ScrollContainerObservation,
        WorthUiHostCapability::PortalAnchorObservation,
    ])
    .with_observation_generation(WorthUiHostCapabilityObservationGeneration::new(generation))
}

pub(super) fn host_text_intrinsic_result(
    request_seed: u64,
    report: &WorthUiHostCapabilityReport,
    generation: UiEvidenceAuthorityGeneration,
) -> UiMeasurementResult {
    let request = UiMeasurementRequest::text_intrinsic_size(
        UiMeasurementRequestIdentity::new(request_seed),
        UiMeasurementEvidenceFamily::TextIntrinsicSize,
        UiTextIntrinsicSizeRequest::single_line(
            "Inbox",
            worth_ui_host_contract::UiFontMeasurementKey::new("body-md"),
        ),
        report,
    )
    .expect("suite text request should admit");
    measurement_result_from_request(
        &request,
        UiHostObservationValue::TextIntrinsicSize(UiTextIntrinsicSizeObservation {
            width: 240.0,
            height: 48.0,
        }),
        generation,
        report,
    )
}

pub(super) fn host_scroll_viewport_result(
    request_seed: u64,
    report: &WorthUiHostCapabilityReport,
    generation: UiEvidenceAuthorityGeneration,
) -> UiMeasurementResult {
    let request = UiMeasurementRequest::scroll_container_viewport(
        UiMeasurementRequestIdentity::new(request_seed),
        UiMeasurementEvidenceFamily::ScrollContainerViewport,
        UiScrollContainerViewportRequest::new(55),
        report,
    )
    .expect("suite scroll request should admit");
    measurement_result_from_request(
        &request,
        UiHostObservationValue::ScrollContainerViewport(UiScrollContainerViewportObservation {
            width: 120.0,
            height: 60.0,
        }),
        generation,
        report,
    )
}

pub(super) fn host_portal_anchor_result(
    request_seed: u64,
    report: &WorthUiHostCapabilityReport,
    generation: UiEvidenceAuthorityGeneration,
) -> UiMeasurementResult {
    let request = UiMeasurementRequest::portal_anchor_rect(
        UiMeasurementRequestIdentity::new(request_seed),
        UiMeasurementEvidenceFamily::PortalAnchorRect,
        UiPortalAnchorRectRequest::new(66),
        report,
    )
    .expect("suite portal request should admit");
    measurement_result_from_request(
        &request,
        UiHostObservationValue::PortalAnchorRect(UiPortalAnchorRectObservation {
            x: 12.0,
            y: 24.0,
            width: 120.0,
            height: 32.0,
        }),
        generation,
        report,
    )
}

fn measurement_result_from_request(
    request: &UiMeasurementRequest,
    value: UiHostObservationValue,
    generation: UiEvidenceAuthorityGeneration,
    report: &WorthUiHostCapabilityReport,
) -> UiMeasurementResult {
    let assumption_profile =
        UiHostMeasurementAssumptionProfile::from_capability_report(report, 11, 22, 33, 44);
    let normalization_context =
        suite_normalization_context_for_request(request, assumption_profile);
    WorthUiHostMeasurementCollector::for_internal_proof()
        .collect(
            &SuiteValueStubAdapter { value },
            request.identity(),
            request.evidence_family(),
            suite_host_need_from_request(request),
            report,
            generation,
            normalization_context,
        )
        .expect("suite host lane collection should admit")
}

struct SuiteValueStubAdapter {
    value: UiHostObservationValue,
}

impl worth_ui_host_contract::WorthUiMeasurementHostAdapter for SuiteValueStubAdapter {
    fn observe_measurement(&self, _request: &UiMeasurementRequest) -> UiHostObservationValue {
        self.value.clone()
    }
}

fn suite_host_need_from_request(request: &UiMeasurementRequest) -> UiHostMeasurementNeed {
    match request.family() {
        worth_ui_host_contract::UiMeasurementRequestFamily::TextIntrinsicSize => {
            UiHostMeasurementNeed::TextIntrinsicSize(
                request
                    .text_intrinsic_size_input()
                    .expect("suite text request")
                    .clone(),
            )
        }
        worth_ui_host_contract::UiMeasurementRequestFamily::ScrollContainerViewport => {
            UiHostMeasurementNeed::ScrollContainerViewport(
                request
                    .scroll_container_viewport_input()
                    .expect("suite scroll request")
                    .clone(),
            )
        }
        worth_ui_host_contract::UiMeasurementRequestFamily::PortalAnchorRect => {
            UiHostMeasurementNeed::PortalAnchorRect(
                request
                    .portal_anchor_rect_input()
                    .expect("suite portal request")
                    .clone(),
            )
        }
        other => panic!("suite fixture does not model host need for {other:?}"),
    }
}

fn suite_normalization_context_for_request(
    request: &UiMeasurementRequest,
    assumption_profile: UiHostMeasurementAssumptionProfile,
) -> UiHostMeasurementNormalizationContext {
    match request.family() {
        worth_ui_host_contract::UiMeasurementRequestFamily::TextIntrinsicSize => {
            UiHostMeasurementNormalizationContext::text_intrinsic_surface_logical_exact(
                assumption_profile,
            )
        }
        worth_ui_host_contract::UiMeasurementRequestFamily::ScrollContainerViewport => {
            UiHostMeasurementNormalizationContext::scroll_container_logical_exact(
                assumption_profile,
            )
        }
        worth_ui_host_contract::UiMeasurementRequestFamily::PortalAnchorRect => {
            UiHostMeasurementNormalizationContext::portal_anchor_logical_exact_in(
                crate::host::UiPortalAnchorCoordinateSpacePosture::PortalLayer,
                assumption_profile,
            )
        }
        other => panic!("suite fixture does not model normalization for {other:?}"),
    }
}
