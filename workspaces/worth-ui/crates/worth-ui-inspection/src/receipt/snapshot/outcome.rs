#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum UiVisualSnapshotOmission {
    NoCurrentFrame,
    TransitionInFlight,
    UnknownFrame,
    ExpiredFrame,
    NodeNotPresented,
    NodeNotVisible,
    HistoricalPixelsUnavailable,
    HostCapabilityUnsupported,
    PixelsOmittedByPolicy,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum UiVisualSnapshotDenial {
    ForeignSession,
    ForeignSurface,
    ForeignBinding,
    ForeignNode,
    SurfaceSelectionRequired,
    OutsideCapturedPixelExtent,
    InvalidGeometry,
    InvalidCoordinateTransform,
    Disclosure,
    DeadlineAlreadyElapsed,
    ProtocolIncompatible,
    VisibleRegionCapacityExceeded,
    HitTestRegionCapacityExceeded,
    RetainedStructurePerReceiptCapacityExceeded,
    RetainedStructurePerSessionCapacityExceeded,
    RetainedPixelCapacityExceeded,
    SnapshotCapacityExceeded,
    CapacityExceeded,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiVisualSnapshotSuperseded {
    predecessor_artifact_copied: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum UiVisualSnapshotIndeterminate {
    TimeoutAfterHostRequest,
    CaptureAffinity,
    NativePresentation,
    HostCompletion,
    Cleanup,
}

impl UiVisualSnapshotSuperseded {
    #[doc(hidden)]
    pub const fn from_runtime_projection(predecessor_artifact_copied: bool) -> Self {
        Self {
            predecessor_artifact_copied,
        }
    }

    pub const fn predecessor_artifact_copied(self) -> bool {
        self.predecessor_artifact_copied
    }
}
