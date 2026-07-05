use std::cell::Cell;

use worth_ui_host_contract::{
    UiDpiScaleFactorObservation, UiFontMeasurementKey, UiFontMetricsObservation,
    UiFontMetricsRequest, UiForbiddenHostAuthorityAsk, UiHostObservationContractDenial,
    UiHostObservationValue, UiMeasurementCapabilityPosture, UiMeasurementEvidenceFamily,
    UiMeasurementRequest, UiMeasurementRequestDenial, UiMeasurementRequestFamily,
    UiMeasurementRequestIdentity, UiNativeControlIntrinsicSizeRequest, UiNativeControlKind,
    UiPortalAnchorRectObservation, UiPortalAnchorRectRequest, UiScrollContainerViewportObservation,
    UiScrollContainerViewportRequest, UiTextBaselineMetricsObservation,
    UiTextBaselineMetricsRequest, UiTextIntrinsicSizeObservation, UiTextIntrinsicSizeRequest,
    UiViewportExtentObservation, UiViewportExtentRequest, WorthUiHostCapability,
    WorthUiHostCapabilityReport, WorthUiMeasurementHostAdapter,
};

use super::{
    freeze_measurement_request, request_host_measurement, UiHostMeasurementExecutionDenial,
    UiHostMeasurementNeed,
};
use crate::evidence::host_measurement_request_shape_digest;

struct CountingAdapter {
    call_count: Cell<u32>,
}

impl CountingAdapter {
    fn new() -> Self {
        Self {
            call_count: Cell::new(0),
        }
    }

    fn call_count(&self) -> u32 {
        self.call_count.get()
    }
}

fn matching_observation_for(request: &UiMeasurementRequest) -> UiHostObservationValue {
    match request.family() {
        UiMeasurementRequestFamily::TextIntrinsicSize => {
            UiHostObservationValue::TextIntrinsicSize(UiTextIntrinsicSizeObservation {
                width: 40.0,
                height: 12.0,
            })
        }
        UiMeasurementRequestFamily::TextBaselineMetrics => {
            UiHostObservationValue::TextBaselineMetrics(UiTextBaselineMetricsObservation {
                ascent: 10.0,
                descent: 2.0,
                baseline: 9.0,
            })
        }
        UiMeasurementRequestFamily::FontMetrics => {
            UiHostObservationValue::FontMetrics(UiFontMetricsObservation {
                ascent: 10.0,
                descent: 2.0,
                line_gap: 1.0,
            })
        }
        UiMeasurementRequestFamily::NativeControlIntrinsicSize => {
            UiHostObservationValue::NativeControlIntrinsicSize(
                worth_ui_host_contract::UiNativeControlIntrinsicSizeObservation {
                    width: 80.0,
                    height: 24.0,
                },
            )
        }
        UiMeasurementRequestFamily::ViewportExtent => {
            UiHostObservationValue::ViewportExtent(UiViewportExtentObservation {
                width: 100.0,
                height: 50.0,
            })
        }
        UiMeasurementRequestFamily::DpiScaleFactor => {
            UiHostObservationValue::DpiScaleFactor(UiDpiScaleFactorObservation {
                scale_factor: 2.0,
            })
        }
        UiMeasurementRequestFamily::PortalAnchorRect => {
            UiHostObservationValue::PortalAnchorRect(UiPortalAnchorRectObservation {
                x: 1.0,
                y: 2.0,
                width: 3.0,
                height: 4.0,
            })
        }
        UiMeasurementRequestFamily::ScrollContainerViewport => {
            UiHostObservationValue::ScrollContainerViewport(UiScrollContainerViewportObservation {
                width: 120.0,
                height: 60.0,
            })
        }
    }
}

impl WorthUiMeasurementHostAdapter for CountingAdapter {
    fn observe_measurement(&self, request: &UiMeasurementRequest) -> UiHostObservationValue {
        self.call_count.set(self.call_count.get() + 1);
        matching_observation_for(request)
    }
}

#[test]
fn all_admitted_request_families_hold_their_declared_contracts_on_the_ordinary_lane() {
    let cases = vec![
        (
            UiHostMeasurementNeed::TextIntrinsicSize(UiTextIntrinsicSizeRequest::single_line(
                "Inbox",
                UiFontMeasurementKey::new("body-md"),
            )),
            UiMeasurementRequestFamily::TextIntrinsicSize,
            UiMeasurementEvidenceFamily::TextIntrinsicSize,
            WorthUiHostCapability::TextIntrinsicMeasurement,
        ),
        (
            UiHostMeasurementNeed::TextBaselineMetrics(UiTextBaselineMetricsRequest::single_line(
                "Inbox",
                UiFontMeasurementKey::new("body-md"),
            )),
            UiMeasurementRequestFamily::TextBaselineMetrics,
            UiMeasurementEvidenceFamily::TextBaselineMetrics,
            WorthUiHostCapability::TextBaselineMeasurement,
        ),
        (
            UiHostMeasurementNeed::FontMetrics(UiFontMetricsRequest::new(
                UiFontMeasurementKey::new("body-md"),
            )),
            UiMeasurementRequestFamily::FontMetrics,
            UiMeasurementEvidenceFamily::FontMetrics,
            WorthUiHostCapability::FontMetrics,
        ),
        (
            UiHostMeasurementNeed::NativeControlIntrinsicSize(
                UiNativeControlIntrinsicSizeRequest::new(UiNativeControlKind::Button, Some("Save")),
            ),
            UiMeasurementRequestFamily::NativeControlIntrinsicSize,
            UiMeasurementEvidenceFamily::NativeControlIntrinsicSize,
            WorthUiHostCapability::NativeControlIntrinsicMeasurement,
        ),
        (
            UiHostMeasurementNeed::ViewportExtent(UiViewportExtentRequest),
            UiMeasurementRequestFamily::ViewportExtent,
            UiMeasurementEvidenceFamily::ViewportExtent,
            WorthUiHostCapability::ViewportObservation,
        ),
        (
            UiHostMeasurementNeed::DpiScaleFactor(worth_ui_host_contract::UiDpiScaleFactorRequest),
            UiMeasurementRequestFamily::DpiScaleFactor,
            UiMeasurementEvidenceFamily::DpiScaleFactor,
            WorthUiHostCapability::DpiObservation,
        ),
        (
            UiHostMeasurementNeed::PortalAnchorRect(UiPortalAnchorRectRequest::new(77)),
            UiMeasurementRequestFamily::PortalAnchorRect,
            UiMeasurementEvidenceFamily::PortalAnchorRect,
            WorthUiHostCapability::PortalAnchorObservation,
        ),
        (
            UiHostMeasurementNeed::ScrollContainerViewport(UiScrollContainerViewportRequest::new(
                91,
            )),
            UiMeasurementRequestFamily::ScrollContainerViewport,
            UiMeasurementEvidenceFamily::ScrollContainerViewport,
            WorthUiHostCapability::ScrollContainerObservation,
        ),
    ];

    for (index, (need, family, evidence_family, capability)) in cases.into_iter().enumerate() {
        let identity = UiMeasurementRequestIdentity::new(index as u64 + 1);
        let capability_report = WorthUiHostCapabilityReport::available(vec![capability]);
        let request =
            freeze_measurement_request(identity, evidence_family, need.clone(), &capability_report)
                .unwrap();
        assert_eq!(request.family(), family);
        assert_eq!(request.evidence_family(), evidence_family);
        assert_eq!(
            request.capability_posture(),
            UiMeasurementCapabilityPosture::Available {
                required_capabilities: Box::new([capability]),
            }
        );

        let observation = request_host_measurement(
            &CountingAdapter::new(),
            identity,
            evidence_family,
            need,
            &capability_report,
        )
        .unwrap();
        assert_eq!(observation.request_identity(), identity);
        assert_eq!(observation.family(), family);
        assert_eq!(observation.evidence_family(), evidence_family);
    }
}

#[test]
fn request_shape_digest_distinguishes_distinct_families() {
    let identity = UiMeasurementRequestIdentity::new(0x11);
    let text = freeze_measurement_request(
        identity,
        UiMeasurementEvidenceFamily::TextIntrinsicSize,
        UiHostMeasurementNeed::TextIntrinsicSize(UiTextIntrinsicSizeRequest::single_line(
            "Inbox",
            UiFontMeasurementKey::new("body-md"),
        )),
        &WorthUiHostCapabilityReport::available(vec![
            WorthUiHostCapability::TextIntrinsicMeasurement,
        ]),
    )
    .unwrap();
    let text_again = freeze_measurement_request(
        identity,
        UiMeasurementEvidenceFamily::TextIntrinsicSize,
        UiHostMeasurementNeed::TextIntrinsicSize(UiTextIntrinsicSizeRequest::single_line(
            "Inbox",
            UiFontMeasurementKey::new("body-md"),
        )),
        &WorthUiHostCapabilityReport::available(vec![
            WorthUiHostCapability::TextIntrinsicMeasurement,
        ]),
    )
    .unwrap();
    let font = freeze_measurement_request(
        identity,
        UiMeasurementEvidenceFamily::FontMetrics,
        UiHostMeasurementNeed::FontMetrics(UiFontMetricsRequest::new(UiFontMeasurementKey::new(
            "body-md",
        ))),
        &WorthUiHostCapabilityReport::available(vec![WorthUiHostCapability::FontMetrics]),
    )
    .unwrap();

    assert_eq!(
        host_measurement_request_shape_digest(&text),
        host_measurement_request_shape_digest(&text_again)
    );
    assert_ne!(
        host_measurement_request_shape_digest(&text),
        host_measurement_request_shape_digest(&font)
    );
}

#[test]
fn all_forbidden_authority_asks_deny_before_adapter_execution() {
    let adapter = CountingAdapter::new();
    let capability_report =
        WorthUiHostCapabilityReport::available(vec![WorthUiHostCapability::ViewportObservation]);
    let asks = [
        UiForbiddenHostAuthorityAsk::FinalLayoutSize,
        UiForbiddenHostAuthorityAsk::OverflowDecision,
        UiForbiddenHostAuthorityAsk::ScrollExtentAuthority,
        UiForbiddenHostAuthorityAsk::PortalPositionDecision,
        UiForbiddenHostAuthorityAsk::AllocationBox,
    ];

    for (index, ask) in asks.into_iter().enumerate() {
        let denial = request_host_measurement(
            &adapter,
            UiMeasurementRequestIdentity::new(index as u64 + 200),
            UiMeasurementEvidenceFamily::ViewportExtent,
            UiHostMeasurementNeed::ForbiddenAuthorityAsk(ask),
            &capability_report,
        )
        .unwrap_err();

        assert_eq!(
            denial,
            UiHostMeasurementExecutionDenial::Request(
                UiMeasurementRequestDenial::ForbiddenAuthorityAsk { ask }
            )
        );
    }
    assert_eq!(adapter.call_count(), 0);
}

#[test]
fn capability_posture_denials_never_reach_the_adapter() {
    let adapter = CountingAdapter::new();
    let need = UiHostMeasurementNeed::NativeControlIntrinsicSize(
        UiNativeControlIntrinsicSizeRequest::new(UiNativeControlKind::Button, Some("Save")),
    );
    let cases = [
        (
            WorthUiHostCapabilityReport::available(vec![WorthUiHostCapability::TextInput]),
            UiHostMeasurementExecutionDenial::Request(
                UiMeasurementRequestDenial::MissingCapability {
                    required_capabilities: Box::new([
                        WorthUiHostCapability::NativeControlIntrinsicMeasurement,
                    ]),
                },
            ),
        ),
        (
            WorthUiHostCapabilityReport::ambiguous(vec![
                WorthUiHostCapability::NativeControlIntrinsicMeasurement,
            ]),
            UiHostMeasurementExecutionDenial::Request(
                UiMeasurementRequestDenial::AmbiguousCapability {
                    required_capabilities: Box::new([
                        WorthUiHostCapability::NativeControlIntrinsicMeasurement,
                    ]),
                },
            ),
        ),
        (
            WorthUiHostCapabilityReport::diagnostic_only(vec![
                WorthUiHostCapability::NativeControlIntrinsicMeasurement,
            ]),
            UiHostMeasurementExecutionDenial::Request(
                UiMeasurementRequestDenial::DiagnosticOnlyCapability {
                    required_capabilities: Box::new([
                        WorthUiHostCapability::NativeControlIntrinsicMeasurement,
                    ]),
                },
            ),
        ),
    ];

    for (index, (report, expected)) in cases.into_iter().enumerate() {
        let denial = request_host_measurement(
            &adapter,
            UiMeasurementRequestIdentity::new(index as u64 + 300),
            UiMeasurementEvidenceFamily::NativeControlIntrinsicSize,
            need.clone(),
            &report,
        )
        .unwrap_err();

        assert_eq!(denial, expected);
    }
    assert_eq!(adapter.call_count(), 0);
}

#[test]
fn mismatched_observation_families_deny_before_reentering_runtime() {
    struct MismatchedAdapter {
        value: UiHostObservationValue,
    }

    impl WorthUiMeasurementHostAdapter for MismatchedAdapter {
        fn observe_measurement(&self, _request: &UiMeasurementRequest) -> UiHostObservationValue {
            self.value.clone()
        }
    }

    let cases = [
        (
            UiHostMeasurementNeed::ViewportExtent(UiViewportExtentRequest),
            UiMeasurementEvidenceFamily::ViewportExtent,
            WorthUiHostCapability::ViewportObservation,
            UiHostObservationValue::PortalAnchorRect(UiPortalAnchorRectObservation {
                x: 1.0,
                y: 2.0,
                width: 3.0,
                height: 4.0,
            }),
            UiMeasurementRequestFamily::ViewportExtent,
            UiMeasurementRequestFamily::PortalAnchorRect,
        ),
        (
            UiHostMeasurementNeed::TextIntrinsicSize(UiTextIntrinsicSizeRequest::single_line(
                "Inbox",
                UiFontMeasurementKey::new("body-md"),
            )),
            UiMeasurementEvidenceFamily::TextIntrinsicSize,
            WorthUiHostCapability::TextIntrinsicMeasurement,
            UiHostObservationValue::FontMetrics(UiFontMetricsObservation {
                ascent: 10.0,
                descent: 2.0,
                line_gap: 1.0,
            }),
            UiMeasurementRequestFamily::TextIntrinsicSize,
            UiMeasurementRequestFamily::FontMetrics,
        ),
    ];

    for (index, (need, evidence_family, capability, value, requested, observed)) in
        cases.into_iter().enumerate()
    {
        let denial = request_host_measurement(
            &MismatchedAdapter { value },
            UiMeasurementRequestIdentity::new(index as u64 + 400),
            evidence_family,
            need,
            &WorthUiHostCapabilityReport::available(vec![capability]),
        )
        .unwrap_err();

        assert_eq!(
            denial,
            UiHostMeasurementExecutionDenial::Observation(
                UiHostObservationContractDenial::FamilyMismatch {
                    requested,
                    observed,
                }
            )
        );
    }
}
