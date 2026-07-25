use super::{
    request_payload::UiMeasurementRequestPayload, UiDpiScaleFactorRequest, UiFontMetricsRequest,
    UiHostMeasurementRequest, UiMeasurementEvidenceFamily, UiMeasurementRequestDenial,
    UiMeasurementRequestFamily, UiMeasurementRequestIdentity, UiNativeControlIntrinsicSizeRequest,
    UiPortalAnchorRectRequest, UiScrollContainerViewportRequest, UiTextBaselineMetricsRequest,
    UiTextIntrinsicSizeRequest, UiViewportExtentRequest,
};
use crate::runtime::{WorthUiHostCapability, WorthUiHostCapabilityReport};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiHostMeasurementRequestIntent {
    family: UiMeasurementRequestFamily,
    evidence_family: UiMeasurementEvidenceFamily,
    required_capabilities: Box<[WorthUiHostCapability]>,
    payload: UiMeasurementRequestPayload,
}

impl UiHostMeasurementRequestIntent {
    pub fn text_intrinsic_size(input: UiTextIntrinsicSizeRequest) -> Self {
        Self::new(
            UiMeasurementRequestFamily::TextIntrinsicSize,
            UiMeasurementEvidenceFamily::TextIntrinsicSize,
            vec![WorthUiHostCapability::TextIntrinsicMeasurement],
            UiMeasurementRequestPayload::TextIntrinsicSize(input),
        )
    }

    pub fn text_baseline_metrics(input: UiTextBaselineMetricsRequest) -> Self {
        Self::new(
            UiMeasurementRequestFamily::TextBaselineMetrics,
            UiMeasurementEvidenceFamily::TextBaselineMetrics,
            vec![WorthUiHostCapability::TextBaselineMeasurement],
            UiMeasurementRequestPayload::TextBaselineMetrics(input),
        )
    }

    pub fn font_metrics(input: UiFontMetricsRequest) -> Self {
        Self::new(
            UiMeasurementRequestFamily::FontMetrics,
            UiMeasurementEvidenceFamily::FontMetrics,
            vec![WorthUiHostCapability::FontMetrics],
            UiMeasurementRequestPayload::FontMetrics(input),
        )
    }

    pub fn native_control_intrinsic_size(input: UiNativeControlIntrinsicSizeRequest) -> Self {
        Self::new(
            UiMeasurementRequestFamily::NativeControlIntrinsicSize,
            UiMeasurementEvidenceFamily::NativeControlIntrinsicSize,
            vec![WorthUiHostCapability::NativeControlIntrinsicMeasurement],
            UiMeasurementRequestPayload::NativeControlIntrinsicSize(input),
        )
    }

    pub fn viewport_extent(input: UiViewportExtentRequest) -> Self {
        Self::new(
            UiMeasurementRequestFamily::ViewportExtent,
            UiMeasurementEvidenceFamily::ViewportExtent,
            vec![WorthUiHostCapability::ViewportObservation],
            UiMeasurementRequestPayload::ViewportExtent(input),
        )
    }

    pub fn dpi_scale_factor(input: UiDpiScaleFactorRequest) -> Self {
        Self::new(
            UiMeasurementRequestFamily::DpiScaleFactor,
            UiMeasurementEvidenceFamily::DpiScaleFactor,
            vec![WorthUiHostCapability::DpiObservation],
            UiMeasurementRequestPayload::DpiScaleFactor(input),
        )
    }

    pub fn portal_anchor_rect(input: UiPortalAnchorRectRequest) -> Self {
        Self::new(
            UiMeasurementRequestFamily::PortalAnchorRect,
            UiMeasurementEvidenceFamily::PortalAnchorRect,
            vec![WorthUiHostCapability::PortalAnchorObservation],
            UiMeasurementRequestPayload::PortalAnchorRect(input),
        )
    }

    pub fn scroll_container_viewport(input: UiScrollContainerViewportRequest) -> Self {
        Self::new(
            UiMeasurementRequestFamily::ScrollContainerViewport,
            UiMeasurementEvidenceFamily::ScrollContainerViewport,
            vec![WorthUiHostCapability::ScrollContainerObservation],
            UiMeasurementRequestPayload::ScrollContainerViewport(input),
        )
    }

    fn new(
        family: UiMeasurementRequestFamily,
        evidence_family: UiMeasurementEvidenceFamily,
        required_capabilities: Vec<WorthUiHostCapability>,
        payload: UiMeasurementRequestPayload,
    ) -> Self {
        Self {
            family,
            evidence_family,
            required_capabilities: required_capabilities.into_boxed_slice(),
            payload,
        }
    }

    pub fn family(&self) -> UiMeasurementRequestFamily {
        self.family
    }

    #[doc(hidden)]
    pub fn issue(
        self,
        identity: UiMeasurementRequestIdentity,
        capability_report: &WorthUiHostCapabilityReport,
    ) -> Result<UiHostMeasurementRequest, UiMeasurementRequestDenial> {
        UiHostMeasurementRequest::new(
            identity,
            self.family,
            self.evidence_family,
            self.evidence_family,
            self.required_capabilities.into_vec(),
            capability_report,
            self.payload,
        )
    }
}
