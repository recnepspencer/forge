use super::*;

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
        value: UiHostMeasurementObservationValue,
    }

    impl WorthUiMeasurementHostAdapter for MismatchedAdapter {
        fn observe_measurement(
            &self,
            _request: &UiHostMeasurementRequest,
        ) -> UiHostMeasurementObservationValue {
            self.value.clone()
        }
    }

    let cases = [
        (
            UiHostMeasurementNeed::ViewportExtent(UiViewportExtentRequest),
            UiMeasurementEvidenceFamily::ViewportExtent,
            WorthUiHostCapability::ViewportObservation,
            UiHostMeasurementObservationValue::PortalAnchorRect(UiPortalAnchorRectObservation {
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
            UiHostMeasurementObservationValue::FontMetrics(UiFontMetricsObservation {
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
                UiHostMeasurementObservationContractDenial::FamilyMismatch {
                    requested,
                    observed,
                }
            )
        );
    }
}
