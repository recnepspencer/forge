use std::time::{Duration, Instant};

use worth_ui::facade::inspection::{
    UiPixelsRequired, UiVisualSnapshotOutcome, UiVisualSnapshotSuperseded,
};

use super::{capture_wall_deadline, resolve_capture, PlatformPulseVisualCaptureResolution};

#[test]
fn superseded_capture_retries_without_renewing_its_wall_deadline() {
    let deadline = Instant::now() + Duration::from_secs(5);
    let outcome = UiVisualSnapshotOutcome::<UiPixelsRequired>::Superseded(
        UiVisualSnapshotSuperseded::from_runtime_projection(false),
    );

    let resolution = resolve_capture(outcome, deadline).expect("supersession is retryable");
    let PlatformPulseVisualCaptureResolution::RetryBefore { deadline: observed } = resolution
    else {
        panic!("supersession must not masquerade as a captured artifact")
    };
    assert_eq!(observed, deadline);
}

#[test]
fn stale_completed_frame_starts_successor_budget_at_capture_admission() {
    let replacement_observed = Instant::now();
    let successor_admitted = replacement_observed + Duration::from_millis(250);
    let readiness_deadline = super::super::replacement_frame_deadline(replacement_observed)
        .expect("replacement readiness deadline");
    let capture_deadline =
        capture_wall_deadline(successor_admitted).expect("successor capture deadline");

    assert_eq!(
        capture_deadline.duration_since(readiness_deadline),
        Duration::from_millis(250)
    );
}
