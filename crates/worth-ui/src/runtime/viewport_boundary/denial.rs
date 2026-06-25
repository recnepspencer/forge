#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiViewportBoundaryDenial {
    reason: WorthUiViewportBoundaryDenialReason,
    subject: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiViewportBoundaryDenialReason {
    UnsupportedPolicyIdentity,
    NestedCompositionScrollOwner,
    MissingAllocatedFrame,
    MissingHostViewportObservation,
}

impl WorthUiViewportBoundaryDenial {
    pub(super) fn new(
        reason: WorthUiViewportBoundaryDenialReason,
        subject: impl Into<String>,
    ) -> Self {
        Self {
            reason,
            subject: subject.into(),
        }
    }

    pub fn reason(&self) -> WorthUiViewportBoundaryDenialReason {
        self.reason
    }

    pub fn subject(&self) -> &str {
        &self.subject
    }
}

impl WorthUiViewportBoundaryDenialReason {
    pub const fn token(self) -> &'static str {
        match self {
            Self::UnsupportedPolicyIdentity => "unsupported_policy_identity",
            Self::NestedCompositionScrollOwner => "nested_composition_scroll_owner",
            Self::MissingAllocatedFrame => "missing_allocated_frame",
            Self::MissingHostViewportObservation => "missing_host_viewport_observation",
        }
    }
}
