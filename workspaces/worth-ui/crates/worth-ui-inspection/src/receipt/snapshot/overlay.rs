#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum UiVisualOverlayDenial {
    ForeignSession,
    ForeignSnapshotTarget,
    TargetNotRetained,
    CapacityExceeded,
    Superseded,
    Expired,
    Presentation,
}
