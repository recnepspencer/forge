use std::cell::Cell;

use worth_ui_host_contract::{
    UiDpiScaleFactorObservation, UiFontMeasurementKey, UiFontMetricsObservation,
    UiHostMeasurementObservationValue, UiHostMeasurementRequest, UiMeasurementRequestFamily,
    UiMeasurementRequestIdentity, UiPortalAnchorRectObservation,
    UiScrollContainerViewportObservation, UiTextBaselineMetricsObservation,
    UiTextIntrinsicSizeObservation, UiTextIntrinsicSizeRequest, UiViewportExtentObservation,
    UiViewportExtentRequest, WorthUiHostCapabilityReport, WorthUiMeasurementHostAdapter,
};
use worth_ui_inspection::UiEvidenceAuthorityGeneration;

use crate::evidence::{UiMeasurementEvidenceCategory, UiMeasurementResult};
use crate::host::{
    UiHostMeasurementAssumptionProfile, UiHostMeasurementNeed,
    UiHostMeasurementNormalizationContext, WorthUiHostMeasurementCollector,
};

pub(crate) struct CountingAdapter {
    call_count: Cell<u32>,
}

impl CountingAdapter {
    pub(crate) fn new() -> Self {
        Self {
            call_count: Cell::new(0),
        }
    }
}

impl WorthUiMeasurementHostAdapter for CountingAdapter {
    fn observe_measurement(
        &self,
        request: &UiHostMeasurementRequest,
    ) -> UiHostMeasurementObservationValue {
        self.call_count.set(self.call_count.get() + 1);
        matching_observation_for(request.family())
    }
}

pub(crate) fn matching_observation_for(
    family: UiMeasurementRequestFamily,
) -> UiHostMeasurementObservationValue {
    match family {
        UiMeasurementRequestFamily::TextIntrinsicSize => {
            UiHostMeasurementObservationValue::TextIntrinsicSize(UiTextIntrinsicSizeObservation {
                width: 40.0,
                height: 12.0,
            })
        }
        UiMeasurementRequestFamily::TextBaselineMetrics => {
            UiHostMeasurementObservationValue::TextBaselineMetrics(
                UiTextBaselineMetricsObservation {
                    ascent: 10.0,
                    descent: 2.0,
                    baseline: 9.0,
                },
            )
        }
        UiMeasurementRequestFamily::FontMetrics => {
            UiHostMeasurementObservationValue::FontMetrics(UiFontMetricsObservation {
                ascent: 10.0,
                descent: 2.0,
                line_gap: 1.0,
            })
        }
        UiMeasurementRequestFamily::NativeControlIntrinsicSize => {
            UiHostMeasurementObservationValue::NativeControlIntrinsicSize(
                worth_ui_host_contract::UiNativeControlIntrinsicSizeObservation {
                    width: 80.0,
                    height: 24.0,
                },
            )
        }
        UiMeasurementRequestFamily::ViewportExtent => {
            UiHostMeasurementObservationValue::ViewportExtent(UiViewportExtentObservation {
                width: 100.0,
                height: 50.0,
            })
        }
        UiMeasurementRequestFamily::DpiScaleFactor => {
            UiHostMeasurementObservationValue::DpiScaleFactor(UiDpiScaleFactorObservation {
                scale_factor: 2.0,
            })
        }
        UiMeasurementRequestFamily::PortalAnchorRect => {
            UiHostMeasurementObservationValue::PortalAnchorRect(UiPortalAnchorRectObservation {
                x: 1.0,
                y: 2.0,
                width: 3.0,
                height: 4.0,
            })
        }
        UiMeasurementRequestFamily::ScrollContainerViewport => {
            UiHostMeasurementObservationValue::ScrollContainerViewport(
                UiScrollContainerViewportObservation {
                    width: 120.0,
                    height: 60.0,
                },
            )
        }
    }
}

pub(crate) fn collected_text_result_for_request(
    request_identity: UiMeasurementRequestIdentity,
    report: &WorthUiHostCapabilityReport,
    generation: UiEvidenceAuthorityGeneration,
    context: UiHostMeasurementNormalizationContext,
) -> UiMeasurementResult {
    WorthUiHostMeasurementCollector::for_internal_proof()
        .collect(
            &CountingAdapter::new(),
            crate::host::UiHostMeasurementCollectionInput {
                identity: request_identity,
                evidence_family:
                    worth_ui_host_contract::UiMeasurementEvidenceFamily::TextIntrinsicSize,
                need: UiHostMeasurementNeed::TextIntrinsicSize(
                    UiTextIntrinsicSizeRequest::single_line(
                        "Inbox",
                        UiFontMeasurementKey::new("body-md"),
                    ),
                ),
                capability_report: report,
                evidence_generation: generation,
                normalization_context: context,
            },
        )
        .unwrap()
}

pub(crate) fn normalized_viewport_result(
    request_identity: UiMeasurementRequestIdentity,
    report: &WorthUiHostCapabilityReport,
    generation: UiEvidenceAuthorityGeneration,
    profile: UiHostMeasurementAssumptionProfile,
) -> UiMeasurementResult {
    WorthUiHostMeasurementCollector::for_internal_proof()
        .collect(
            &CountingAdapter::new(),
            crate::host::UiHostMeasurementCollectionInput {
                identity: request_identity,
                evidence_family:
                    worth_ui_host_contract::UiMeasurementEvidenceFamily::ViewportExtent,
                need: UiHostMeasurementNeed::ViewportExtent(UiViewportExtentRequest),
                capability_report: report,
                evidence_generation: generation,
                normalization_context:
                    UiHostMeasurementNormalizationContext::viewport_logical_exact(profile),
            },
        )
        .unwrap()
}

pub(crate) fn measurement_evidence_family_for(
    category: UiMeasurementEvidenceCategory,
) -> worth_ui_host_contract::UiMeasurementEvidenceFamily {
    match category {
        UiMeasurementEvidenceCategory::TextIntrinsicSize => {
            worth_ui_host_contract::UiMeasurementEvidenceFamily::TextIntrinsicSize
        }
        UiMeasurementEvidenceCategory::TextBaselineMetrics => {
            worth_ui_host_contract::UiMeasurementEvidenceFamily::TextBaselineMetrics
        }
        UiMeasurementEvidenceCategory::FontMetrics => {
            worth_ui_host_contract::UiMeasurementEvidenceFamily::FontMetrics
        }
        UiMeasurementEvidenceCategory::NativeControlIntrinsicSize => {
            worth_ui_host_contract::UiMeasurementEvidenceFamily::NativeControlIntrinsicSize
        }
        UiMeasurementEvidenceCategory::ViewportExtent => {
            worth_ui_host_contract::UiMeasurementEvidenceFamily::ViewportExtent
        }
        UiMeasurementEvidenceCategory::DpiScaleFactor => {
            worth_ui_host_contract::UiMeasurementEvidenceFamily::DpiScaleFactor
        }
        UiMeasurementEvidenceCategory::PortalAnchorRect => {
            worth_ui_host_contract::UiMeasurementEvidenceFamily::PortalAnchorRect
        }
        UiMeasurementEvidenceCategory::ScrollContainerViewport => {
            worth_ui_host_contract::UiMeasurementEvidenceFamily::ScrollContainerViewport
        }
    }
}

pub(crate) fn normalization_context_for(
    category: UiMeasurementEvidenceCategory,
    profile: UiHostMeasurementAssumptionProfile,
) -> UiHostMeasurementNormalizationContext {
    match category {
        UiMeasurementEvidenceCategory::TextIntrinsicSize => {
            UiHostMeasurementNormalizationContext::text_intrinsic_surface_logical_exact(profile)
        }
        UiMeasurementEvidenceCategory::TextBaselineMetrics => {
            UiHostMeasurementNormalizationContext::text_baseline_surface_logical_exact(profile)
        }
        UiMeasurementEvidenceCategory::FontMetrics => {
            UiHostMeasurementNormalizationContext::font_metrics_surface_logical_exact(profile)
        }
        UiMeasurementEvidenceCategory::NativeControlIntrinsicSize => {
            UiHostMeasurementNormalizationContext::native_control_surface_logical_host_rounded(
                profile,
            )
        }
        UiMeasurementEvidenceCategory::ViewportExtent => {
            UiHostMeasurementNormalizationContext::viewport_logical_exact(profile)
        }
        UiMeasurementEvidenceCategory::DpiScaleFactor => {
            UiHostMeasurementNormalizationContext::dpi_scale_window_exact(profile)
        }
        UiMeasurementEvidenceCategory::PortalAnchorRect => {
            UiHostMeasurementNormalizationContext::portal_anchor_logical_exact_in(
                crate::host::UiPortalAnchorCoordinateSpacePosture::PortalLayer,
                profile,
            )
        }
        UiMeasurementEvidenceCategory::ScrollContainerViewport => {
            UiHostMeasurementNormalizationContext::scroll_container_logical_exact(profile)
        }
    }
}
