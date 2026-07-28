#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiHostCaptureCancellationOutcome {
    CancelledBeforeReadback,
    ReadbackMayHaveBegun,
    CleanupIndeterminate,
}
