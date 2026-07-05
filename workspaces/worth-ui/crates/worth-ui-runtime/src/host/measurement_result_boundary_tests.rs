use worth_ui_host_contract::{
    UiFontMeasurementKey, UiFontMetricsRequest, UiMeasurementRequestIdentity,
    UiNativeControlIntrinsicSizeRequest, UiNativeControlKind, UiPortalAnchorRectRequest,
    UiScrollContainerViewportRequest, UiTextBaselineMetricsRequest, UiTextIntrinsicSizeRequest,
    UiViewportExtentRequest, WorthUiHostCapability, WorthUiHostCapabilityObservationGeneration,
    WorthUiHostCapabilityReport,
};
use worth_ui_inspection::UiEvidenceAuthorityGeneration;

use super::{
    collect_host_measurement_evidence, normalize_host_measurement_evidence,
    request_host_measurement, UiHostMeasurementAssumptionProfile, UiHostMeasurementEvidenceDenial,
    UiHostMeasurementFreshnessWitness, UiHostMeasurementNeed,
    UiHostMeasurementNormalizationContext, UiHostMeasurementNormalizationDenial,
};
use crate::evidence::{
    measurement_result_identity_digest, UiMeasurementCoordinateSpace,
    UiMeasurementEvidenceCategory, UiMeasurementRoundingPosture, UiMeasurementUnitPosture,
};

use super::measurement_result_test_support::{
    collected_text_result_for_request, measurement_evidence_family_for, normalization_context_for,
    CountingAdapter,
};

#[test]
fn normalized_host_measurement_results_preserve_identity_generation_and_posture() {
    let cases = vec![
        (
            UiHostMeasurementNeed::TextIntrinsicSize(UiTextIntrinsicSizeRequest::single_line(
                "Inbox",
                UiFontMeasurementKey::new("body-md"),
            )),
            WorthUiHostCapability::TextIntrinsicMeasurement,
            UiMeasurementEvidenceCategory::TextIntrinsicSize,
            UiMeasurementUnitPosture::LogicalPx,
            UiMeasurementCoordinateSpace::HostSurface,
            UiMeasurementRoundingPosture::ExactFloat,
        ),
        (
            UiHostMeasurementNeed::TextBaselineMetrics(UiTextBaselineMetricsRequest::single_line(
                "Inbox",
                UiFontMeasurementKey::new("body-md"),
            )),
            WorthUiHostCapability::TextBaselineMeasurement,
            UiMeasurementEvidenceCategory::TextBaselineMetrics,
            UiMeasurementUnitPosture::LogicalPx,
            UiMeasurementCoordinateSpace::HostSurface,
            UiMeasurementRoundingPosture::ExactFloat,
        ),
        (
            UiHostMeasurementNeed::FontMetrics(UiFontMetricsRequest::new(
                UiFontMeasurementKey::new("body-md"),
            )),
            WorthUiHostCapability::FontMetrics,
            UiMeasurementEvidenceCategory::FontMetrics,
            UiMeasurementUnitPosture::LogicalPx,
            UiMeasurementCoordinateSpace::HostSurface,
            UiMeasurementRoundingPosture::ExactFloat,
        ),
        (
            UiHostMeasurementNeed::NativeControlIntrinsicSize(
                UiNativeControlIntrinsicSizeRequest::new(UiNativeControlKind::Button, Some("Save")),
            ),
            WorthUiHostCapability::NativeControlIntrinsicMeasurement,
            UiMeasurementEvidenceCategory::NativeControlIntrinsicSize,
            UiMeasurementUnitPosture::LogicalPx,
            UiMeasurementCoordinateSpace::HostSurface,
            UiMeasurementRoundingPosture::HostRounded,
        ),
        (
            UiHostMeasurementNeed::ViewportExtent(UiViewportExtentRequest),
            WorthUiHostCapability::ViewportObservation,
            UiMeasurementEvidenceCategory::ViewportExtent,
            UiMeasurementUnitPosture::LogicalPx,
            UiMeasurementCoordinateSpace::Viewport,
            UiMeasurementRoundingPosture::ExactFloat,
        ),
        (
            UiHostMeasurementNeed::DpiScaleFactor(worth_ui_host_contract::UiDpiScaleFactorRequest),
            WorthUiHostCapability::DpiObservation,
            UiMeasurementEvidenceCategory::DpiScaleFactor,
            UiMeasurementUnitPosture::UnitlessScale,
            UiMeasurementCoordinateSpace::Window,
            UiMeasurementRoundingPosture::ExactFloat,
        ),
        (
            UiHostMeasurementNeed::ScrollContainerViewport(UiScrollContainerViewportRequest::new(
                91,
            )),
            WorthUiHostCapability::ScrollContainerObservation,
            UiMeasurementEvidenceCategory::ScrollContainerViewport,
            UiMeasurementUnitPosture::LogicalPx,
            UiMeasurementCoordinateSpace::Viewport,
            UiMeasurementRoundingPosture::ExactFloat,
        ),
        (
            UiHostMeasurementNeed::PortalAnchorRect(UiPortalAnchorRectRequest::new(77)),
            WorthUiHostCapability::PortalAnchorObservation,
            UiMeasurementEvidenceCategory::PortalAnchorRect,
            UiMeasurementUnitPosture::LogicalPx,
            UiMeasurementCoordinateSpace::PortalLayer,
            UiMeasurementRoundingPosture::ExactFloat,
        ),
    ];

    for (index, (need, capability, category, unit, space, rounding)) in
        cases.into_iter().enumerate()
    {
        let request_identity = UiMeasurementRequestIdentity::new(index as u64 + 1);
        let report = WorthUiHostCapabilityReport::available(vec![capability])
            .with_observation_generation(WorthUiHostCapabilityObservationGeneration::new(9));
        let profile =
            UiHostMeasurementAssumptionProfile::from_capability_report(&report, 100, 200, 300, 400);
        let context = normalization_context_for(category, profile);
        let result = collect_host_measurement_evidence(
            &CountingAdapter::new(),
            request_identity,
            measurement_evidence_family_for(category),
            need,
            &report,
            UiEvidenceAuthorityGeneration::new(77),
            context,
        )
        .unwrap();

        assert_eq!(result.request_identity(), request_identity);
        assert_eq!(result.evidence_category(), category);
        assert_eq!(
            result.evidence_generation(),
            UiEvidenceAuthorityGeneration::new(77)
        );
        assert_eq!(result.unit_posture(), unit);
        assert_eq!(result.coordinate_space(), space);
        assert_eq!(result.rounding_posture(), rounding);
        assert_eq!(result.assumption_profile(), profile);
    }
}

#[test]
fn equivalent_host_observations_converge_on_the_same_normalized_evidence_artifact() {
    let report = WorthUiHostCapabilityReport::available(vec![
        WorthUiHostCapability::TextIntrinsicMeasurement,
    ])
    .with_observation_generation(WorthUiHostCapabilityObservationGeneration::new(1));
    let profile =
        UiHostMeasurementAssumptionProfile::from_capability_report(&report, 11, 22, 33, 44);
    let context =
        UiHostMeasurementNormalizationContext::text_intrinsic_surface_logical_exact(profile);

    let first = collected_text_result_for_request(
        UiMeasurementRequestIdentity::new(1),
        &report,
        UiEvidenceAuthorityGeneration::new(8),
        context,
    );
    let second = collected_text_result_for_request(
        UiMeasurementRequestIdentity::new(1),
        &report,
        UiEvidenceAuthorityGeneration::new(8),
        context,
    );

    assert_eq!(
        measurement_result_identity_digest(&first),
        measurement_result_identity_digest(&second)
    );
    assert_eq!(first, second);
}

#[test]
fn normalization_context_cannot_reclassify_observed_measurement_evidence() {
    let report =
        WorthUiHostCapabilityReport::available(vec![WorthUiHostCapability::ViewportObservation]);
    let observation = request_host_measurement(
        &CountingAdapter::new(),
        UiMeasurementRequestIdentity::new(9),
        worth_ui_host_contract::UiMeasurementEvidenceFamily::ViewportExtent,
        UiHostMeasurementNeed::ViewportExtent(UiViewportExtentRequest),
        &report,
    )
    .unwrap();
    let hostile_profile =
        UiHostMeasurementAssumptionProfile::from_capability_report(&report, 1, 2, 3, 4);
    let denial = normalize_host_measurement_evidence(
        observation,
        UiEvidenceAuthorityGeneration::new(1),
        UiHostMeasurementNormalizationContext::portal_anchor_logical_exact(hostile_profile),
    )
    .unwrap_err();

    assert_eq!(
        denial,
        UiHostMeasurementNormalizationDenial::CategoryMismatch {
            observed: UiMeasurementEvidenceCategory::ViewportExtent,
            normalized: UiMeasurementEvidenceCategory::PortalAnchorRect,
        }
    );
}

#[test]
fn stale_measurement_results_must_be_readmitted_before_reuse() {
    let report =
        WorthUiHostCapabilityReport::available(vec![WorthUiHostCapability::ViewportObservation])
            .with_observation_generation(WorthUiHostCapabilityObservationGeneration::new(1));
    let profile = UiHostMeasurementAssumptionProfile::from_capability_report(&report, 1, 2, 3, 4);
    let result = collect_host_measurement_evidence(
        &CountingAdapter::new(),
        UiMeasurementRequestIdentity::new(77),
        worth_ui_host_contract::UiMeasurementEvidenceFamily::ViewportExtent,
        UiHostMeasurementNeed::ViewportExtent(UiViewportExtentRequest),
        &report,
        UiEvidenceAuthorityGeneration::new(9),
        UiHostMeasurementNormalizationContext::viewport_logical_exact(profile),
    )
    .unwrap();

    let denial = super::admit_current_host_measurement_evidence(
        &result,
        UiHostMeasurementFreshnessWitness::new(UiEvidenceAuthorityGeneration::new(10), profile),
    )
    .unwrap_err();

    assert_eq!(
        denial,
        UiHostMeasurementEvidenceDenial::Stale(
            super::UiHostMeasurementInvalidationReason::EvidenceGenerationDrift {
                recorded: UiEvidenceAuthorityGeneration::new(9),
                current: UiEvidenceAuthorityGeneration::new(10),
            }
        )
    );
}
