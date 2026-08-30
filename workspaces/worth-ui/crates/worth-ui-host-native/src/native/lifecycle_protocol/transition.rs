use worth_ui_host_contract::UiHostPresentationEpoch;

use crate::UiNativeInputObservationStop;

use super::UiNativeLifecyclePhase;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiNativeLifecycleRequiredAction {
    CompletePresentation,
    EmitProfileEvidence,
    DrainRetained,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiNativeLifecycleEffect {
    Retained,
    Ignored,
    Denied(UiNativeInputObservationStop),
    PresentationCompleted,
    CloseDeferred,
    Closed,
    NoOp,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiNativeLifecycleTransition {
    phase: UiNativeLifecyclePhase,
    effect: UiNativeLifecycleEffect,
    retained_delta: u64,
    predecessor: Option<UiHostPresentationEpoch>,
    required_action: Option<UiNativeLifecycleRequiredAction>,
}

impl UiNativeLifecycleTransition {
    pub(super) const fn new(
        phase: UiNativeLifecyclePhase,
        effect: UiNativeLifecycleEffect,
        retained_delta: u64,
        predecessor: Option<UiHostPresentationEpoch>,
        required_action: Option<UiNativeLifecycleRequiredAction>,
    ) -> Self {
        Self {
            phase,
            effect,
            retained_delta,
            predecessor,
            required_action,
        }
    }

    #[cfg(feature = "certification-support")]
    pub const fn phase(self) -> UiNativeLifecyclePhase {
        self.phase
    }

    pub const fn effect(self) -> UiNativeLifecycleEffect {
        self.effect
    }

    #[cfg(feature = "certification-support")]
    pub const fn retained_delta(self) -> u64 {
        self.retained_delta
    }

    #[cfg(feature = "certification-support")]
    pub const fn predecessor(self) -> Option<UiHostPresentationEpoch> {
        self.predecessor
    }

    pub const fn required_action(self) -> Option<UiNativeLifecycleRequiredAction> {
        self.required_action
    }
}
