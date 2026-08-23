use super::PresentationAdmissionKey;

pub const WORTH_UI_PRESENTATION_TRANSITION_CAPACITY: usize = 64;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiPresentationTransitionKind {
    Pending,
    Superseded,
    StaleCompletionRejected,
    Completed,
    DuplicateCompletionRejected,
    Cancelled,
    Unresolved,
    RecoveryRequired,
    ReconstructionCurrent,
    TerminalClosed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiPresentationTransitionObservation {
    kind: WorthUiPresentationTransitionKind,
    attempt: worth_ui_host_contract::UiMountedPresentationAttemptIdentity,
    binding: worth_ui_host_contract::UiSurfaceBindingGeneration,
}

impl WorthUiPresentationTransitionObservation {
    pub(super) const fn new(
        kind: WorthUiPresentationTransitionKind,
        key: PresentationAdmissionKey,
    ) -> Self {
        Self {
            kind,
            attempt: key.attempt,
            binding: key.binding,
        }
    }

    pub const fn kind(self) -> WorthUiPresentationTransitionKind {
        self.kind
    }

    pub const fn attempt(self) -> worth_ui_host_contract::UiMountedPresentationAttemptIdentity {
        self.attempt
    }

    pub const fn binding(self) -> worth_ui_host_contract::UiSurfaceBindingGeneration {
        self.binding
    }
}
