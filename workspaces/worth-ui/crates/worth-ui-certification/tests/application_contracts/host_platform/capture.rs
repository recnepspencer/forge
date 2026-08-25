use worth_ui_host_contract::{UiHostCaptureCancellationOutcome, UiHostCaptureObservationOutcome};
use worth_ui_host_native::{
    UiNativeCaptureExternalObservation as External, UiNativeCaptureProtocolWorld,
};

#[test]
fn native_capture_protocol_keeps_readback_external_observations_distinct() {
    let mut world = UiNativeCaptureProtocolWorld::new([
        External::Pending,
        External::CapturedRgba8([47, 129, 247, 255]),
    ]);
    assert!(matches!(
        world.observe(),
        UiHostCaptureObservationOutcome::Pending
    ));
    assert!(world.current_census().is_zero());
    assert!(matches!(
        world.observe(),
        UiHostCaptureObservationOutcome::Pending
    ));
    assert_eq!(world.current_census().readback_buffers, 1);
    assert_eq!(world.current_census().pending_submissions, 1);
    assert!(matches!(
        world.observe(),
        UiHostCaptureObservationOutcome::Pending
    ));
    let UiHostCaptureObservationOutcome::Captured(observation) = world.observe() else {
        panic!("the external completion must settle through the production capture state")
    };
    let pixels = observation.pixels().expect("required canonical pixels");
    assert_eq!(pixels.dimensions(), [2, 1]);
    assert_eq!(pixels.stride(), 8);
    assert_eq!(pixels.bytes(), [47, 129, 247, 255, 47, 129, 247, 255]);
    assert!(world.current_census().is_zero());
    assert!(world.close().is_zero());

    let mut artifact = UiNativeCaptureProtocolWorld::new([External::ArtifactIndeterminate]);
    assert!(matches!(
        artifact.observe(),
        UiHostCaptureObservationOutcome::Pending
    ));
    assert!(matches!(
        artifact.observe(),
        UiHostCaptureObservationOutcome::Pending
    ));
    assert!(matches!(
        artifact.observe(),
        UiHostCaptureObservationOutcome::ReadbackCompletionIndeterminate
    ));
    assert!(artifact.current_census().is_zero());
}

#[test]
fn physically_indeterminate_completion_remains_charged_until_recovery() {
    let mut world = UiNativeCaptureProtocolWorld::new([
        External::PhysicalCompletionIndeterminate,
        External::CapturedRgba8([47, 129, 247, 255]),
    ]);
    assert!(matches!(
        world.observe(),
        UiHostCaptureObservationOutcome::Pending
    ));
    assert!(matches!(
        world.observe(),
        UiHostCaptureObservationOutcome::Pending
    ));
    assert!(matches!(
        world.observe(),
        UiHostCaptureObservationOutcome::ReadbackCompletionIndeterminate
    ));
    assert_eq!(world.current_census().readback_buffers, 1);
    assert_eq!(world.current_census().pending_submissions, 1);
    assert!(world.close().is_zero());
}

#[test]
fn native_capture_protocol_preserves_cancellation_effect_posture() {
    let mut before_effects = UiNativeCaptureProtocolWorld::new([External::Pending]);
    assert!(matches!(
        before_effects.observe(),
        UiHostCaptureObservationOutcome::Pending
    ));
    assert_eq!(
        before_effects.cancel(),
        UiHostCaptureCancellationOutcome::CancelledBeforeReadback
    );
    assert!(before_effects.current_census().is_zero());

    let mut after_submission = UiNativeCaptureProtocolWorld::new([
        External::Pending,
        External::CapturedRgba8([47, 129, 247, 255]),
    ]);
    assert!(matches!(
        after_submission.observe(),
        UiHostCaptureObservationOutcome::Pending
    ));
    assert!(matches!(
        after_submission.observe(),
        UiHostCaptureObservationOutcome::Pending
    ));
    assert_eq!(
        after_submission.cancel(),
        UiHostCaptureCancellationOutcome::ReadbackMayHaveBegun
    );
    assert_eq!(after_submission.current_census().readback_buffers, 1);
    assert_eq!(after_submission.current_census().pending_submissions, 1);
    assert!(after_submission.close().is_zero());
}
