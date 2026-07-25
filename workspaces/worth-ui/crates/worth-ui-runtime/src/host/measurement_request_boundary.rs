use worth_ui_host_contract::{
    UiDpiScaleFactorRequest, UiFontMetricsRequest, UiForbiddenHostAuthorityAsk,
    UiHostMeasurementRequest, UiMeasurementEvidenceFamily, UiMeasurementRequestDenial,
    UiMeasurementRequestIdentity, UiNativeControlIntrinsicSizeRequest, UiPortalAnchorRectRequest,
    UiScrollContainerViewportRequest, UiTextBaselineMetricsRequest, UiTextIntrinsicSizeRequest,
    UiViewportExtentRequest, WorthUiHostCapabilityReport,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiHostMeasurementNeed {
    TextIntrinsicSize(UiTextIntrinsicSizeRequest),
    TextBaselineMetrics(UiTextBaselineMetricsRequest),
    FontMetrics(UiFontMetricsRequest),
    NativeControlIntrinsicSize(UiNativeControlIntrinsicSizeRequest),
    ViewportExtent(UiViewportExtentRequest),
    DpiScaleFactor(UiDpiScaleFactorRequest),
    PortalAnchorRect(UiPortalAnchorRectRequest),
    ScrollContainerViewport(UiScrollContainerViewportRequest),
    ForbiddenAuthorityAsk(UiForbiddenHostAuthorityAsk),
}

pub fn freeze_measurement_request(
    identity: UiMeasurementRequestIdentity,
    evidence_family: UiMeasurementEvidenceFamily,
    need: UiHostMeasurementNeed,
    capability_report: &WorthUiHostCapabilityReport,
) -> Result<UiHostMeasurementRequest, UiMeasurementRequestDenial> {
    match need {
        UiHostMeasurementNeed::TextIntrinsicSize(input) => {
            UiHostMeasurementRequest::text_intrinsic_size(
                identity,
                evidence_family,
                input,
                capability_report,
            )
        }
        UiHostMeasurementNeed::TextBaselineMetrics(input) => {
            UiHostMeasurementRequest::text_baseline_metrics(
                identity,
                evidence_family,
                input,
                capability_report,
            )
        }
        UiHostMeasurementNeed::FontMetrics(input) => UiHostMeasurementRequest::font_metrics(
            identity,
            evidence_family,
            input,
            capability_report,
        ),
        UiHostMeasurementNeed::NativeControlIntrinsicSize(input) => {
            UiHostMeasurementRequest::native_control_intrinsic_size(
                identity,
                evidence_family,
                input,
                capability_report,
            )
        }
        UiHostMeasurementNeed::ViewportExtent(input) => UiHostMeasurementRequest::viewport_extent(
            identity,
            evidence_family,
            input,
            capability_report,
        ),
        UiHostMeasurementNeed::DpiScaleFactor(input) => UiHostMeasurementRequest::dpi_scale_factor(
            identity,
            evidence_family,
            input,
            capability_report,
        ),
        UiHostMeasurementNeed::PortalAnchorRect(input) => {
            UiHostMeasurementRequest::portal_anchor_rect(
                identity,
                evidence_family,
                input,
                capability_report,
            )
        }
        UiHostMeasurementNeed::ScrollContainerViewport(input) => {
            UiHostMeasurementRequest::scroll_container_viewport(
                identity,
                evidence_family,
                input,
                capability_report,
            )
        }
        UiHostMeasurementNeed::ForbiddenAuthorityAsk(ask) => {
            Err(UiMeasurementRequestDenial::ForbiddenAuthorityAsk { ask })
        }
    }
}
