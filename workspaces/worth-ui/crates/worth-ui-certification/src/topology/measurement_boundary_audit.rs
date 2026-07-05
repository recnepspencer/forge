use std::path::Path;

use worth_ui_host_contract::{
    UiDpiScaleFactorRequest, UiFontMeasurementKey, UiFontMetricsRequest,
    UiForbiddenHostAuthorityAsk, UiMeasurementEvidenceFamily, UiMeasurementRequest,
    UiMeasurementRequestDenial, UiMeasurementRequestFamily, UiMeasurementRequestIdentity,
    UiNativeControlIntrinsicSizeRequest, UiNativeControlKind, UiPortalAnchorRectRequest,
    UiScrollContainerViewportRequest, UiTextBaselineMetricsRequest, UiTextIntrinsicSizeRequest,
    UiViewportExtentRequest, WorthUiHostCapabilityReport, WorthUiHostContract,
};

pub fn certify_measurement_host_boundary_purity(workspace_root: &Path) -> Result<(), Vec<String>> {
    let mut violations = audit_measurement_host_request_surface(workspace_root);
    violations.extend(audit_measurement_forbidden_host_authority_denial_surface());
    violations.sort();
    violations.dedup();
    if violations.is_empty() {
        Ok(())
    } else {
        Err(violations)
    }
}

pub fn audit_measurement_host_request_surface(_workspace_root: &Path) -> Vec<String> {
    let capability_report = WorthUiHostCapabilityReport::from_contract(WorthUiHostContract::egui());
    let mut violations = Vec::new();

    for (label, family, result) in allowed_measurement_requests(&capability_report) {
        match result {
            Ok(request) if request.family() == family => {}
            Ok(request) => violations.push(format!(
                "public measurement request constructor `{label}` resolved to {:?} instead of {:?}",
                request.family(),
                family
            )),
            Err(denial) => violations.push(format!(
                "public measurement request constructor `{label}` denied ordinary admitted construction with {:?}",
                denial
            )),
        }
    }

    violations.sort();
    violations.dedup();
    violations
}

pub fn audit_measurement_forbidden_host_authority_denial_surface() -> Vec<String> {
    let mut violations = Vec::new();

    for ask in forbidden_host_authority_asks() {
        let denial = UiMeasurementRequestDenial::ForbiddenAuthorityAsk { ask };
        match denial {
            UiMeasurementRequestDenial::ForbiddenAuthorityAsk { ask: actual } if actual == ask => {}
            UiMeasurementRequestDenial::ForbiddenAuthorityAsk { ask: actual } => violations.push(
                format!(
                    "forbidden host authority ask {:?} widened to {:?} instead of round-tripping explicitly",
                    ask, actual
                ),
            ),
            other => violations.push(format!(
                "forbidden host authority ask {:?} lowered to {:?} instead of staying typed",
                ask, other
            )),
        }
    }

    violations.sort();
    violations.dedup();
    violations
}

fn allowed_measurement_requests(
    capability_report: &WorthUiHostCapabilityReport,
) -> [(
    &'static str,
    UiMeasurementRequestFamily,
    Result<UiMeasurementRequest, worth_ui_host_contract::UiMeasurementRequestDenial>,
); 8] {
    [
        (
            "text_intrinsic_size",
            UiMeasurementRequestFamily::TextIntrinsicSize,
            UiMeasurementRequest::text_intrinsic_size(
                UiMeasurementRequestIdentity::new(1),
                UiMeasurementEvidenceFamily::TextIntrinsicSize,
                UiTextIntrinsicSizeRequest::single_line("Inbox", UiFontMeasurementKey::new("body")),
                capability_report,
            ),
        ),
        (
            "text_baseline_metrics",
            UiMeasurementRequestFamily::TextBaselineMetrics,
            UiMeasurementRequest::text_baseline_metrics(
                UiMeasurementRequestIdentity::new(2),
                UiMeasurementEvidenceFamily::TextBaselineMetrics,
                UiTextBaselineMetricsRequest::single_line(
                    "Inbox",
                    UiFontMeasurementKey::new("body"),
                ),
                capability_report,
            ),
        ),
        (
            "font_metrics",
            UiMeasurementRequestFamily::FontMetrics,
            UiMeasurementRequest::font_metrics(
                UiMeasurementRequestIdentity::new(3),
                UiMeasurementEvidenceFamily::FontMetrics,
                UiFontMetricsRequest::new(UiFontMeasurementKey::new("body")),
                capability_report,
            ),
        ),
        (
            "native_control_intrinsic_size",
            UiMeasurementRequestFamily::NativeControlIntrinsicSize,
            UiMeasurementRequest::native_control_intrinsic_size(
                UiMeasurementRequestIdentity::new(4),
                UiMeasurementEvidenceFamily::NativeControlIntrinsicSize,
                UiNativeControlIntrinsicSizeRequest::new(UiNativeControlKind::Button, Some("OK")),
                capability_report,
            ),
        ),
        (
            "viewport_extent",
            UiMeasurementRequestFamily::ViewportExtent,
            UiMeasurementRequest::viewport_extent(
                UiMeasurementRequestIdentity::new(5),
                UiMeasurementEvidenceFamily::ViewportExtent,
                UiViewportExtentRequest,
                capability_report,
            ),
        ),
        (
            "dpi_scale_factor",
            UiMeasurementRequestFamily::DpiScaleFactor,
            UiMeasurementRequest::dpi_scale_factor(
                UiMeasurementRequestIdentity::new(6),
                UiMeasurementEvidenceFamily::DpiScaleFactor,
                UiDpiScaleFactorRequest,
                capability_report,
            ),
        ),
        (
            "portal_anchor_rect",
            UiMeasurementRequestFamily::PortalAnchorRect,
            UiMeasurementRequest::portal_anchor_rect(
                UiMeasurementRequestIdentity::new(7),
                UiMeasurementEvidenceFamily::PortalAnchorRect,
                UiPortalAnchorRectRequest::new(77),
                capability_report,
            ),
        ),
        (
            "scroll_container_viewport",
            UiMeasurementRequestFamily::ScrollContainerViewport,
            UiMeasurementRequest::scroll_container_viewport(
                UiMeasurementRequestIdentity::new(8),
                UiMeasurementEvidenceFamily::ScrollContainerViewport,
                UiScrollContainerViewportRequest::new(88),
                capability_report,
            ),
        ),
    ]
}

fn forbidden_host_authority_asks() -> [UiForbiddenHostAuthorityAsk; 5] {
    [
        UiForbiddenHostAuthorityAsk::FinalLayoutSize,
        UiForbiddenHostAuthorityAsk::OverflowDecision,
        UiForbiddenHostAuthorityAsk::ScrollExtentAuthority,
        UiForbiddenHostAuthorityAsk::PortalPositionDecision,
        UiForbiddenHostAuthorityAsk::AllocationBox,
    ]
}
