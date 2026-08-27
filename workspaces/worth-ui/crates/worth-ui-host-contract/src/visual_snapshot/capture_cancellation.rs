#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[must_use = "capture cancellation posture must be handled"]
pub enum UiHostCaptureCancellationOutcome {
    CancelledBeforeReadback,
    ReadbackMayHaveBegun,
    CleanupIndeterminate,
}
