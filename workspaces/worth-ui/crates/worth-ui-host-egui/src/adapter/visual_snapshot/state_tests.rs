use super::state::{UiEguiPendingAdmission, UiEguiPresentationAffinity, UiEguiVisualCaptureState};
use worth_ui_host_contract::{
    UiHostCaptureArtifactBudget, UiHostCaptureFrameAffinity, UiHostCaptureRequestIdentity,
    UiHostCaptureSurfaceAffinity, UiHostPresentationEpoch, UiHostSurfaceIdentity,
    UiHostVisualCaptureRequest, UiMountedFrameIdentity, UiMountedPresentationAttemptIdentity,
    UiSurfaceBindingGeneration,
};

#[test]
fn superseded_capture_releases_binding_capacity_for_immediate_readmission() {
    let mut state = UiEguiVisualCaptureState::default();
    let binding = UiSurfaceBindingGeneration::mint_unbound().unwrap();
    let first = request(1, binding);
    assert!(matches!(
        state.admit_pending(first),
        UiEguiPendingAdmission::Admitted(_)
    ));
    assert!(matches!(
        state.presentation_affinity(first),
        UiEguiPresentationAffinity::Superseded
    ));

    assert!(matches!(
        state.admit_pending(request(2, binding)),
        UiEguiPendingAdmission::Admitted(_)
    ));
}

fn request(identity: u64, binding: UiSurfaceBindingGeneration) -> UiHostVisualCaptureRequest {
    UiHostVisualCaptureRequest::admitted_by_runtime(
        UiHostCaptureRequestIdentity::issued_by_runtime(identity),
        UiHostCaptureFrameAffinity::observed_by_runtime(
            UiMountedFrameIdentity::mint_unbound().unwrap(),
            UiMountedPresentationAttemptIdentity::mint_unbound().unwrap(),
        ),
        UiHostCaptureSurfaceAffinity::observed_by_runtime(
            1,
            UiHostSurfaceIdentity::mint_unbound().unwrap(),
            binding,
            UiHostPresentationEpoch::issued_by_host(1),
        ),
        UiHostCaptureArtifactBudget::admitted_by_runtime(true, 1024),
    )
}
