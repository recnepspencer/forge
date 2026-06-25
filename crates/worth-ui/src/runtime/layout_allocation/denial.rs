#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiLayoutAllocationDenial {
    reason: WorthUiLayoutAllocationDenialReason,
    subject: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiLayoutAllocationDenialReason {
    MissingAvailableBounds,
    UnknownAllocationRoot,
    UnsupportedFlowKind,
    NonContainerAllocationRoot,
    MissingMountedChild,
}

impl WorthUiLayoutAllocationDenial {
    pub(super) fn new(
        reason: WorthUiLayoutAllocationDenialReason,
        subject: impl Into<String>,
    ) -> Self {
        Self {
            reason,
            subject: subject.into(),
        }
    }

    pub fn reason(&self) -> WorthUiLayoutAllocationDenialReason {
        self.reason
    }

    pub fn subject(&self) -> &str {
        &self.subject
    }
}

impl WorthUiLayoutAllocationDenialReason {
    pub fn token(self) -> &'static str {
        match self {
            Self::MissingAvailableBounds => "missing_available_bounds",
            Self::UnknownAllocationRoot => "unknown_allocation_root",
            Self::UnsupportedFlowKind => "unsupported_flow_kind",
            Self::NonContainerAllocationRoot => "non_container_allocation_root",
            Self::MissingMountedChild => "missing_mounted_child",
        }
    }
}
