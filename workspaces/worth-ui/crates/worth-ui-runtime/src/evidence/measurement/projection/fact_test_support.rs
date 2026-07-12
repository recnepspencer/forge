use worth_ui_host_contract::{
    UiFontMeasurementKey, UiFontMetricsObservation, UiFontMetricsRequest, UiHostObservationValue,
    UiMeasurementEvidenceFamily, UiMeasurementRequest, UiMeasurementRequestIdentity,
    UiPortalAnchorRectObservation, UiPortalAnchorRectRequest, UiTextIntrinsicSizeObservation,
    UiTextIntrinsicSizeRequest, UiViewportExtentObservation, UiViewportExtentRequest,
    WorthUiHostCapability, WorthUiHostCapabilityObservationGeneration, WorthUiHostCapabilityReport,
};
use worth_ui_inspection::UiEvidenceAuthorityGeneration;

use crate::declaration::{
    UiDeclarationAspectDigest, UiDeclarationFamilyDigest, UiDeclarationIdentity,
    UiDeclarationPostureDigest, UiDeclarationStructuralDigest, UiDeclaredMeasurementBasisSource,
    UiDeclaredMeasurementConstraintModifier, UiDeclaredMeasurementEvidenceRequirement,
    UiDeclaredMeasurementMode, UiDeclaredMeasurementPolicyPosture,
};
use crate::host::tests::measurement_fixture::collect_measurement_via_host_lane_for_test;
use crate::host::tests::measurement_result_test_support::normalization_context_for;
use crate::host::UiHostMeasurementNormalizationContext;
use crate::host::{UiHostMeasurementAssumptionProfile, UiHostMeasurementNeed};

use crate::evidence::measurement::UiMeasurementResult;

pub(crate) use super::query_context_test_support::display_field_projection_authority_outcome;
pub(crate) use super::query_context_test_support::display_field_projection_consumption;
pub(crate) use super::query_context_test_support::{
    display_field_projection_context, entity_identity_projection_context,
};

pub(crate) fn synthetic_declaration_identity(label: &str) -> UiDeclarationIdentity {
    UiDeclarationIdentity::new(
        UiDeclarationFamilyDigest::new(1),
        UiDeclarationAspectDigest::new(2),
        UiDeclarationStructuralDigest::new(3),
        UiDeclarationPostureDigest::new(4),
        label,
    )
}

pub(crate) fn scroll_viewport_policy() -> UiDeclaredMeasurementPolicyPosture {
    UiDeclaredMeasurementPolicyPosture::new(
        Some(UiDeclaredMeasurementMode::HugHeight),
        Some(UiDeclaredMeasurementConstraintModifier::Bounded),
        Some(UiDeclaredMeasurementBasisSource::ScrollViewport),
        None,
        vec![
            UiDeclaredMeasurementEvidenceRequirement::HostFontMetrics,
            UiDeclaredMeasurementEvidenceRequirement::ScrollContentExtent,
        ],
    )
    .expect("measurement policy should admit")
}

pub(crate) fn viewport_extent_policy() -> UiDeclaredMeasurementPolicyPosture {
    UiDeclaredMeasurementPolicyPosture::new(
        Some(UiDeclaredMeasurementMode::HugHeight),
        Some(UiDeclaredMeasurementConstraintModifier::Bounded),
        Some(UiDeclaredMeasurementBasisSource::ViewportExtent),
        None,
        Vec::new(),
    )
    .expect("viewport measurement policy should admit")
}

pub(crate) fn host_font_metrics_policy() -> UiDeclaredMeasurementPolicyPosture {
    UiDeclaredMeasurementPolicyPosture::new(
        Some(UiDeclaredMeasurementMode::HugHeight),
        None,
        None,
        None,
        vec![UiDeclaredMeasurementEvidenceRequirement::HostFontMetrics],
    )
    .expect("host policy should admit")
}

pub(crate) fn capability_report(generation: u64) -> WorthUiHostCapabilityReport {
    capability_report_with_capabilities(
        generation,
        vec![
            WorthUiHostCapability::TextIntrinsicMeasurement,
            WorthUiHostCapability::FontMetrics,
            WorthUiHostCapability::NativeControlIntrinsicMeasurement,
            WorthUiHostCapability::PortalAnchorObservation,
            WorthUiHostCapability::ScrollContainerObservation,
            WorthUiHostCapability::ViewportObservation,
        ],
    )
}

pub(crate) fn capability_report_with_capabilities(
    generation: u64,
    capabilities: Vec<WorthUiHostCapability>,
) -> WorthUiHostCapabilityReport {
    WorthUiHostCapabilityReport::available(capabilities)
        .with_observation_generation(WorthUiHostCapabilityObservationGeneration::new(generation))
}

pub(crate) fn host_result_font_metrics(
    request_seed: u64,
    report: &WorthUiHostCapabilityReport,
    generation: UiEvidenceAuthorityGeneration,
) -> UiMeasurementResult {
    host_result_font_metrics_with_assumption_profile(
        request_seed,
        report,
        generation,
        UiHostMeasurementAssumptionProfile::from_capability_report(report, 11, 22, 33, 44),
    )
}

pub(crate) fn host_result_text_intrinsic_size(
    request_seed: u64,
    report: &WorthUiHostCapabilityReport,
    generation: UiEvidenceAuthorityGeneration,
) -> UiMeasurementResult {
    host_result_text_intrinsic_size_with_value(request_seed, report, generation, 240.0, 48.0)
}

pub(crate) fn host_result_text_intrinsic_size_with_value(
    request_seed: u64,
    report: &WorthUiHostCapabilityReport,
    generation: UiEvidenceAuthorityGeneration,
    width: f32,
    height: f32,
) -> UiMeasurementResult {
    let request = UiMeasurementRequest::text_intrinsic_size(
        UiMeasurementRequestIdentity::new(request_seed),
        UiMeasurementEvidenceFamily::TextIntrinsicSize,
        UiTextIntrinsicSizeRequest::single_line("Inbox", UiFontMeasurementKey::new("body-md")),
        report,
    )
    .expect("text intrinsic request should admit");
    host_result(
        &request,
        UiHostObservationValue::TextIntrinsicSize(UiTextIntrinsicSizeObservation { width, height }),
        report,
        generation,
        UiHostMeasurementAssumptionProfile::from_capability_report(report, 11, 22, 33, 44),
    )
}

pub(crate) fn host_result_font_metrics_with_assumption_profile(
    request_seed: u64,
    report: &WorthUiHostCapabilityReport,
    generation: UiEvidenceAuthorityGeneration,
    assumption_profile: UiHostMeasurementAssumptionProfile,
) -> UiMeasurementResult {
    let request = UiMeasurementRequest::font_metrics(
        UiMeasurementRequestIdentity::new(request_seed),
        UiMeasurementEvidenceFamily::FontMetrics,
        UiFontMetricsRequest::new(UiFontMeasurementKey::new("body-md")),
        report,
    )
    .expect("font metrics request should admit");
    host_result(
        &request,
        UiHostObservationValue::FontMetrics(UiFontMetricsObservation {
            ascent: 10.0,
            descent: 2.0,
            line_gap: 1.0,
        }),
        report,
        generation,
        assumption_profile,
    )
}

pub(crate) fn host_result_viewport_extent(
    request_seed: u64,
    report: &WorthUiHostCapabilityReport,
    generation: UiEvidenceAuthorityGeneration,
) -> UiMeasurementResult {
    host_result_viewport_extent_with_value(request_seed, report, generation, 100.0, 50.0)
}

pub(crate) fn host_result_viewport_extent_with_value(
    request_seed: u64,
    report: &WorthUiHostCapabilityReport,
    generation: UiEvidenceAuthorityGeneration,
    width: f32,
    height: f32,
) -> UiMeasurementResult {
    let request = UiMeasurementRequest::viewport_extent(
        UiMeasurementRequestIdentity::new(request_seed),
        UiMeasurementEvidenceFamily::ViewportExtent,
        UiViewportExtentRequest,
        report,
    )
    .expect("viewport request should admit");
    host_result(
        &request,
        UiHostObservationValue::ViewportExtent(UiViewportExtentObservation { width, height }),
        report,
        generation,
        UiHostMeasurementAssumptionProfile::from_capability_report(report, 11, 22, 33, 44),
    )
}

pub(crate) fn host_result_portal_anchor(
    request_seed: u64,
    report: &WorthUiHostCapabilityReport,
    generation: UiEvidenceAuthorityGeneration,
) -> UiMeasurementResult {
    host_result_portal_anchor_at(request_seed, 44, [1.0, 2.0, 3.0, 4.0], report, generation)
}

pub(crate) fn host_result_portal_anchor_at(
    request_seed: u64,
    target_identity: u64,
    rect: [f32; 4],
    report: &WorthUiHostCapabilityReport,
    generation: UiEvidenceAuthorityGeneration,
) -> UiMeasurementResult {
    let request = UiMeasurementRequest::portal_anchor_rect(
        UiMeasurementRequestIdentity::new(request_seed),
        UiMeasurementEvidenceFamily::PortalAnchorRect,
        UiPortalAnchorRectRequest::new(target_identity),
        report,
    )
    .expect("portal request should admit");
    host_result(
        &request,
        UiHostObservationValue::PortalAnchorRect(UiPortalAnchorRectObservation {
            x: rect[0],
            y: rect[1],
            width: rect[2],
            height: rect[3],
        }),
        report,
        generation,
        UiHostMeasurementAssumptionProfile::from_capability_report(report, 11, 22, 33, 44),
    )
}

pub(crate) fn host_result_scroll_container_viewport(
    request_seed: u64,
    report: &WorthUiHostCapabilityReport,
    generation: UiEvidenceAuthorityGeneration,
) -> UiMeasurementResult {
    host_result_scroll_container_viewport_with_value(request_seed, report, generation, 120.0, 60.0)
}

pub(crate) fn host_result_scroll_container_viewport_with_value(
    request_seed: u64,
    report: &WorthUiHostCapabilityReport,
    generation: UiEvidenceAuthorityGeneration,
    width: f32,
    height: f32,
) -> UiMeasurementResult {
    let request = UiMeasurementRequest::scroll_container_viewport(
        UiMeasurementRequestIdentity::new(request_seed),
        UiMeasurementEvidenceFamily::ScrollContainerViewport,
        worth_ui_host_contract::UiScrollContainerViewportRequest::new(55),
        report,
    )
    .expect("scroll container request should admit");
    host_result(
        &request,
        UiHostObservationValue::ScrollContainerViewport(
            worth_ui_host_contract::UiScrollContainerViewportObservation { width, height },
        ),
        report,
        generation,
        UiHostMeasurementAssumptionProfile::from_capability_report(report, 11, 22, 33, 44),
    )
}

fn host_result(
    request: &UiMeasurementRequest,
    value: UiHostObservationValue,
    report: &WorthUiHostCapabilityReport,
    generation: UiEvidenceAuthorityGeneration,
    assumption_profile: UiHostMeasurementAssumptionProfile,
) -> UiMeasurementResult {
    let category =
        crate::evidence::UiMeasurementEvidenceCategory::from_request_family(request.family());
    let normalization_context = match category {
        crate::evidence::UiMeasurementEvidenceCategory::TextIntrinsicSize => {
            UiHostMeasurementNormalizationContext::text_intrinsic_graph_node_local_logical_exact(
                assumption_profile,
            )
        }
        _ => normalization_context_for(category, assumption_profile),
    };
    collect_measurement_via_host_lane_for_test(
        request,
        value,
        projection_host_need_from_request(request),
        request.evidence_family(),
        generation,
        report,
        normalization_context,
    )
}

fn projection_host_need_from_request(request: &UiMeasurementRequest) -> UiHostMeasurementNeed {
    match request.family() {
        worth_ui_host_contract::UiMeasurementRequestFamily::TextIntrinsicSize => {
            UiHostMeasurementNeed::TextIntrinsicSize(
                request
                    .text_intrinsic_size_input()
                    .expect("text request")
                    .clone(),
            )
        }
        worth_ui_host_contract::UiMeasurementRequestFamily::FontMetrics => {
            UiHostMeasurementNeed::FontMetrics(
                request.font_metrics_input().expect("font request").clone(),
            )
        }
        worth_ui_host_contract::UiMeasurementRequestFamily::ViewportExtent => {
            UiHostMeasurementNeed::ViewportExtent(
                request
                    .viewport_extent_input()
                    .expect("viewport request")
                    .clone(),
            )
        }
        worth_ui_host_contract::UiMeasurementRequestFamily::PortalAnchorRect => {
            UiHostMeasurementNeed::PortalAnchorRect(
                request
                    .portal_anchor_rect_input()
                    .expect("portal request")
                    .clone(),
            )
        }
        worth_ui_host_contract::UiMeasurementRequestFamily::NativeControlIntrinsicSize => {
            UiHostMeasurementNeed::NativeControlIntrinsicSize(
                request
                    .native_control_intrinsic_size_input()
                    .expect("native control request")
                    .clone(),
            )
        }
        worth_ui_host_contract::UiMeasurementRequestFamily::ScrollContainerViewport => {
            UiHostMeasurementNeed::ScrollContainerViewport(
                request
                    .scroll_container_viewport_input()
                    .expect("scroll request")
                    .clone(),
            )
        }
        other => panic!("projection test support does not model host need for {other:?}"),
    }
}
