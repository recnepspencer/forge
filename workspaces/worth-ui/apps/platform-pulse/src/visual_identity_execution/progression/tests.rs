use std::time::{Duration, Instant};

use worth_ui::facade::inspection::{
    UiPixelsRequired, UiVisualSnapshotOutcome, UiVisualSnapshotSuperseded,
};

use super::{resolve_capture, PlatformPulseVisualCaptureResolution};

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
