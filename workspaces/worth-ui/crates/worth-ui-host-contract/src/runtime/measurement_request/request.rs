use super::{
    request_payload::UiMeasurementRequestPayload, UiDpiScaleFactorRequest, UiFontMetricsRequest,
    UiMeasurementCapabilityGrant, UiMeasurementCapabilityPosture, UiMeasurementEvidenceFamily,
    UiMeasurementRequestDenial, UiMeasurementRequestFamily, UiMeasurementRequestIdentity,
    UiNativeControlIntrinsicSizeRequest, UiPortalAnchorRectRequest,
    UiScrollContainerViewportRequest, UiTextBaselineMetricsRequest, UiTextIntrinsicSizeRequest,
    UiViewportExtentRequest,
};
use crate::runtime::{WorthUiHostCapability, WorthUiHostCapabilityReport};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiHostMeasurementRequest {
    identity: UiMeasurementRequestIdentity,
    family: UiMeasurementRequestFamily,
    evidence_family: UiMeasurementEvidenceFamily,
    capability_grant: UiMeasurementCapabilityGrant,
    payload: UiMeasurementRequestPayload,
}

impl UiHostMeasurementRequest {
    pub fn text_intrinsic_size(
        identity: UiMeasurementRequestIdentity,
        evidence_family: UiMeasurementEvidenceFamily,
        input: UiTextIntrinsicSizeRequest,
        capability_report: &WorthUiHostCapabilityReport,
    ) -> Result<Self, UiMeasurementRequestDenial> {
        Self::new(
            identity,
            UiMeasurementRequestFamily::TextIntrinsicSize,
            UiMeasurementEvidenceFamily::TextIntrinsicSize,
            evidence_family,
            vec![WorthUiHostCapability::TextIntrinsicMeasurement],
            capability_report,
            UiMeasurementRequestPayload::TextIntrinsicSize(input),
        )
    }

    pub fn text_baseline_metrics(
        identity: UiMeasurementRequestIdentity,
        evidence_family: UiMeasurementEvidenceFamily,
        input: UiTextBaselineMetricsRequest,
        capability_report: &WorthUiHostCapabilityReport,
    ) -> Result<Self, UiMeasurementRequestDenial> {
        Self::new(
            identity,
            UiMeasurementRequestFamily::TextBaselineMetrics,
            UiMeasurementEvidenceFamily::TextBaselineMetrics,
            evidence_family,
            vec![WorthUiHostCapability::TextBaselineMeasurement],
            capability_report,
            UiMeasurementRequestPayload::TextBaselineMetrics(input),
        )
    }

    pub fn font_metrics(
        identity: UiMeasurementRequestIdentity,
        evidence_family: UiMeasurementEvidenceFamily,
        input: UiFontMetricsRequest,
        capability_report: &WorthUiHostCapabilityReport,
    ) -> Result<Self, UiMeasurementRequestDenial> {
        Self::new(
            identity,
            UiMeasurementRequestFamily::FontMetrics,
            UiMeasurementEvidenceFamily::FontMetrics,
            evidence_family,
            vec![WorthUiHostCapability::FontMetrics],
            capability_report,
            UiMeasurementRequestPayload::FontMetrics(input),
        )
    }

    pub fn native_control_intrinsic_size(
        identity: UiMeasurementRequestIdentity,
        evidence_family: UiMeasurementEvidenceFamily,
        input: UiNativeControlIntrinsicSizeRequest,
        capability_report: &WorthUiHostCapabilityReport,
    ) -> Result<Self, UiMeasurementRequestDenial> {
        Self::new(
            identity,
            UiMeasurementRequestFamily::NativeControlIntrinsicSize,
            UiMeasurementEvidenceFamily::NativeControlIntrinsicSize,
            evidence_family,
            vec![WorthUiHostCapability::NativeControlIntrinsicMeasurement],
            capability_report,
            UiMeasurementRequestPayload::NativeControlIntrinsicSize(input),
        )
    }

    pub fn viewport_extent(
        identity: UiMeasurementRequestIdentity,
        evidence_family: UiMeasurementEvidenceFamily,
        input: UiViewportExtentRequest,
        capability_report: &WorthUiHostCapabilityReport,
    ) -> Result<Self, UiMeasurementRequestDenial> {
        Self::new(
            identity,
            UiMeasurementRequestFamily::ViewportExtent,
            UiMeasurementEvidenceFamily::ViewportExtent,
            evidence_family,
            vec![WorthUiHostCapability::ViewportObservation],
            capability_report,
            UiMeasurementRequestPayload::ViewportExtent(input),
        )
    }

    pub fn dpi_scale_factor(
        identity: UiMeasurementRequestIdentity,
        evidence_family: UiMeasurementEvidenceFamily,
        input: UiDpiScaleFactorRequest,
        capability_report: &WorthUiHostCapabilityReport,
    ) -> Result<Self, UiMeasurementRequestDenial> {
        Self::new(
            identity,
            UiMeasurementRequestFamily::DpiScaleFactor,
            UiMeasurementEvidenceFamily::DpiScaleFactor,
            evidence_family,
            vec![WorthUiHostCapability::DpiObservation],
            capability_report,
            UiMeasurementRequestPayload::DpiScaleFactor(input),
        )
    }

    pub fn portal_anchor_rect(
        identity: UiMeasurementRequestIdentity,
        evidence_family: UiMeasurementEvidenceFamily,
        input: UiPortalAnchorRectRequest,
        capability_report: &WorthUiHostCapabilityReport,
    ) -> Result<Self, UiMeasurementRequestDenial> {
        Self::new(
            identity,
            UiMeasurementRequestFamily::PortalAnchorRect,
            UiMeasurementEvidenceFamily::PortalAnchorRect,
            evidence_family,
            vec![WorthUiHostCapability::PortalAnchorObservation],
            capability_report,
            UiMeasurementRequestPayload::PortalAnchorRect(input),
        )
    }

    pub fn scroll_container_viewport(
        identity: UiMeasurementRequestIdentity,
        evidence_family: UiMeasurementEvidenceFamily,
        input: UiScrollContainerViewportRequest,
        capability_report: &WorthUiHostCapabilityReport,
    ) -> Result<Self, UiMeasurementRequestDenial> {
        Self::new(
            identity,
            UiMeasurementRequestFamily::ScrollContainerViewport,
            UiMeasurementEvidenceFamily::ScrollContainerViewport,
            evidence_family,
            vec![WorthUiHostCapability::ScrollContainerObservation],
            capability_report,
            UiMeasurementRequestPayload::ScrollContainerViewport(input),
        )
    }

    pub(super) fn new(
        identity: UiMeasurementRequestIdentity,
        family: UiMeasurementRequestFamily,
        expected_evidence_family: UiMeasurementEvidenceFamily,
        evidence_family: UiMeasurementEvidenceFamily,
        required_capabilities: Vec<WorthUiHostCapability>,
        capability_report: &WorthUiHostCapabilityReport,
        payload: UiMeasurementRequestPayload,
    ) -> Result<Self, UiMeasurementRequestDenial> {
        if evidence_family != expected_evidence_family {
            return Err(UiMeasurementRequestDenial::IncompatibleEvidenceFamily {
                family,
                evidence_family,
            });
        }

        let capability_grant =
            UiMeasurementCapabilityGrant::new(capability_report, required_capabilities)?;
        Ok(Self {
            identity,
            family,
            evidence_family,
            capability_grant,
            payload,
        })
    }

    pub fn identity(&self) -> UiMeasurementRequestIdentity {
        self.identity
    }

    pub fn family(&self) -> UiMeasurementRequestFamily {
        self.family
    }

    pub fn evidence_family(&self) -> UiMeasurementEvidenceFamily {
        self.evidence_family
    }

    pub fn capability_posture(&self) -> UiMeasurementCapabilityPosture {
        self.capability_grant.posture()
    }

    pub fn required_capabilities(&self) -> &[WorthUiHostCapability] {
        self.capability_grant.required_capabilities()
    }

    pub fn text_intrinsic_size_input(&self) -> Option<&UiTextIntrinsicSizeRequest> {
        match &self.payload {
            UiMeasurementRequestPayload::TextIntrinsicSize(value) => Some(value),
            _ => None,
        }
    }

    pub fn text_baseline_metrics_input(&self) -> Option<&UiTextBaselineMetricsRequest> {
        match &self.payload {
            UiMeasurementRequestPayload::TextBaselineMetrics(value) => Some(value),
            _ => None,
        }
    }

    pub fn font_metrics_input(&self) -> Option<&UiFontMetricsRequest> {
        match &self.payload {
            UiMeasurementRequestPayload::FontMetrics(value) => Some(value),
            _ => None,
        }
    }

    pub fn native_control_intrinsic_size_input(
        &self,
    ) -> Option<&UiNativeControlIntrinsicSizeRequest> {
        match &self.payload {
            UiMeasurementRequestPayload::NativeControlIntrinsicSize(value) => Some(value),
            _ => None,
        }
    }

    pub fn viewport_extent_input(&self) -> Option<&UiViewportExtentRequest> {
        match &self.payload {
            UiMeasurementRequestPayload::ViewportExtent(value) => Some(value),
            _ => None,
        }
    }

    pub fn dpi_scale_factor_input(&self) -> Option<&UiDpiScaleFactorRequest> {
        match &self.payload {
            UiMeasurementRequestPayload::DpiScaleFactor(value) => Some(value),
            _ => None,
        }
    }

    pub fn portal_anchor_rect_input(&self) -> Option<&UiPortalAnchorRectRequest> {
        match &self.payload {
            UiMeasurementRequestPayload::PortalAnchorRect(value) => Some(value),
            _ => None,
        }
    }

    pub fn scroll_container_viewport_input(&self) -> Option<&UiScrollContainerViewportRequest> {
        match &self.payload {
            UiMeasurementRequestPayload::ScrollContainerViewport(value) => Some(value),
            _ => None,
        }
    }

    pub fn encoded_len(&self) -> usize {
        let payload = match &self.payload {
            UiMeasurementRequestPayload::TextIntrinsicSize(value) => {
                value.text().len() + value.font().token().len()
            }
            UiMeasurementRequestPayload::TextBaselineMetrics(value) => {
                value.text().len() + value.font().token().len()
            }
            UiMeasurementRequestPayload::FontMetrics(value) => value.font().token().len(),
            UiMeasurementRequestPayload::NativeControlIntrinsicSize(value) => {
                1 + value.label().map_or(0, str::len)
            }
            UiMeasurementRequestPayload::ViewportExtent(_)
            | UiMeasurementRequestPayload::DpiScaleFactor(_) => 0,
            UiMeasurementRequestPayload::PortalAnchorRect(_)
            | UiMeasurementRequestPayload::ScrollContainerViewport(_) => 8,
        };
        24 + payload + self.required_capabilities().len() * 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::{UiFontMeasurementKey, WorthUiHostCapabilityReport};

    #[test]
    fn incompatible_evidence_family_denies_before_request_construction() {
        let capability_report = WorthUiHostCapabilityReport::available(vec![
            WorthUiHostCapability::TextIntrinsicMeasurement,
        ]);
        let denial = UiHostMeasurementRequest::text_intrinsic_size(
            UiMeasurementRequestIdentity::new(7),
            UiMeasurementEvidenceFamily::FontMetrics,
            UiTextIntrinsicSizeRequest::single_line("Inbox", UiFontMeasurementKey::new("body")),
            &capability_report,
        )
        .unwrap_err();

        assert_eq!(
            denial,
            UiMeasurementRequestDenial::IncompatibleEvidenceFamily {
                family: UiMeasurementRequestFamily::TextIntrinsicSize,
                evidence_family: UiMeasurementEvidenceFamily::FontMetrics,
            }
        );
    }
}
