use std::cell::Cell;

use worth_ui_host_contract::{
    UiHostCaptureArtifactBudget, UiHostCaptureFrameAffinity, UiHostCaptureObservationOutcome,
    UiHostCaptureRequestIdentity, UiHostCaptureSurfaceAffinity, UiHostMeasurementObservationValue,
    UiHostMeasurementRequest, UiHostPresentationEpoch, UiHostSessionReleaseOutcome,
    UiHostSessionReleaseReceipt, UiHostSurfaceIdentity, UiHostVisualCaptureRequest,
    UiMountedFrameIdentity, UiMountedPresentationAttemptIdentity, UiSurfaceBindingGeneration,
    WorthUiHostCapabilityReport, WorthUiHostContract, WorthUiHostMechanicsAdapter,
    WorthUiMeasurementHostAdapter,
};

use super::{UiHostAdapterSessionAuthority, WorthUiOperationalHostAdapter};

#[derive(Default)]
struct PendingCaptureMechanics {
    capture_calls: Cell<usize>,
}

impl WorthUiMeasurementHostAdapter for PendingCaptureMechanics {
    fn observe_measurement(
        &self,
        _request: &UiHostMeasurementRequest,
    ) -> UiHostMeasurementObservationValue {
        unreachable!("the focused capture mechanics declare no measurement capability")
    }
}

impl WorthUiHostMechanicsAdapter for PendingCaptureMechanics {
    fn mechanical_host_contract(&self) -> WorthUiHostContract {
        WorthUiHostContract::headless()
    }

    fn mechanical_capability_report(&self) -> WorthUiHostCapabilityReport {
        WorthUiHostCapabilityReport::available(Vec::new())
    }

    fn perform_visual_capture(
        &self,
        _request: UiHostVisualCaptureRequest,
    ) -> UiHostCaptureObservationOutcome {
        self.capture_calls.set(self.capture_calls.get() + 1);
        UiHostCaptureObservationOutcome::Pending
    }

    fn release_mechanical_host_session(
        &self,
        host_session_identity: u64,
    ) -> UiHostSessionReleaseOutcome {
        UiHostSessionReleaseOutcome::Released(UiHostSessionReleaseReceipt::released(
            host_session_identity,
            0,
        ))
    }
}

#[test]
fn mechanics_capture_port_rejects_foreign_session_before_adapter_effects() {
    let mechanics = PendingCaptureMechanics::default();
    let authority = UiHostAdapterSessionAuthority::activate(7);
    assert!(matches!(
        mechanics.capture_visual_presentation(&authority, capture_request(7)),
        UiHostCaptureObservationOutcome::Pending
    ));
    assert!(matches!(
        mechanics.capture_visual_presentation(&authority, capture_request(8)),
        UiHostCaptureObservationOutcome::Unsupported
    ));
    assert_eq!(mechanics.capture_calls.get(), 1);
}

fn capture_request(host_session_identity: u64) -> UiHostVisualCaptureRequest {
    UiHostVisualCaptureRequest::admitted_by_runtime(
        UiHostCaptureRequestIdentity::issued_by_runtime(1),
        UiHostCaptureFrameAffinity::observed_by_runtime(
            UiMountedFrameIdentity::mint_unbound().unwrap(),
            UiMountedPresentationAttemptIdentity::mint_unbound().unwrap(),
        ),
        UiHostCaptureSurfaceAffinity::observed_by_runtime(
            host_session_identity,
            UiHostSurfaceIdentity::mint_unbound().unwrap(),
            UiSurfaceBindingGeneration::mint_unbound().unwrap(),
            UiHostPresentationEpoch::issued_by_host(1),
        ),
        UiHostCaptureArtifactBudget::admitted_by_runtime(false, 0),
    )
}
