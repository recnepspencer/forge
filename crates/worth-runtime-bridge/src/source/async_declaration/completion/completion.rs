use crate::identity::{
    AsyncCompletionDenialIdentityTag, AsyncCompletionIdentityTag, BridgeIdentity,
};
use worth_signal::facade::CompletionDenialClass;

use super::admitted::AdmittedBridgeAsyncCompletion;
use super::denied::BridgeAsyncDeniedCompletion;

pub(super) type BridgeAsyncCompletionIdentity = BridgeIdentity<AsyncCompletionIdentityTag>;
pub(super) type BridgeAsyncCompletionDenialIdentity =
    BridgeIdentity<AsyncCompletionDenialIdentityTag>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeAsyncCompletionClass {
    Fulfilled,
    EffectsIndeterminate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeAsyncCompletionDenialClass {
    Rejected,
    Cancelled,
    TimedOut,
    Superseded,
    StaleDenied,
    SignalLifecycleDenied,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeAsyncCompletionState {
    Admitted(BridgeAsyncCompletionClass),
    Denied(BridgeAsyncCompletionDenialClass),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum BridgeAsyncCompletionAdmissionOutcome {
    Admitted(AdmittedBridgeAsyncCompletion),
    Denied(BridgeAsyncDeniedCompletion),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeAsyncCompletionAdmissionReport {
    outcome: BridgeAsyncCompletionAdmissionOutcome,
}

impl BridgeAsyncCompletionAdmissionReport {
    pub(crate) fn admitted(admitted_completion: AdmittedBridgeAsyncCompletion) -> Self {
        Self {
            outcome: BridgeAsyncCompletionAdmissionOutcome::Admitted(admitted_completion),
        }
    }

    pub(crate) fn denied(denied_completion: BridgeAsyncDeniedCompletion) -> Self {
        Self {
            outcome: BridgeAsyncCompletionAdmissionOutcome::Denied(denied_completion),
        }
    }

    pub fn admitted_completion(&self) -> Option<&AdmittedBridgeAsyncCompletion> {
        match &self.outcome {
            BridgeAsyncCompletionAdmissionOutcome::Admitted(admitted) => Some(admitted),
            BridgeAsyncCompletionAdmissionOutcome::Denied(_) => None,
        }
    }

    pub fn denied_completion(&self) -> Option<&BridgeAsyncDeniedCompletion> {
        match &self.outcome {
            BridgeAsyncCompletionAdmissionOutcome::Admitted(_) => None,
            BridgeAsyncCompletionAdmissionOutcome::Denied(denied) => Some(denied),
        }
    }

    pub(crate) fn from_owner_effects_indeterminate(
        self,
        observation: super::BridgeAsyncEffectsIndeterminateCompletion,
    ) -> Self {
        match self.outcome {
            BridgeAsyncCompletionAdmissionOutcome::Admitted(completion) => {
                Self::admitted(completion.from_owner_effects_indeterminate(observation))
            }
            BridgeAsyncCompletionAdmissionOutcome::Denied(completion) => Self::denied(completion),
        }
    }
}

pub(super) fn map_denial_class(class: CompletionDenialClass) -> BridgeAsyncCompletionDenialClass {
    match class {
        CompletionDenialClass::Rejected => BridgeAsyncCompletionDenialClass::Rejected,
        CompletionDenialClass::Cancelled => BridgeAsyncCompletionDenialClass::Cancelled,
        CompletionDenialClass::TimedOut => BridgeAsyncCompletionDenialClass::TimedOut,
        CompletionDenialClass::Superseded => BridgeAsyncCompletionDenialClass::Superseded,
        CompletionDenialClass::Stale
        | CompletionDenialClass::Retired
        | CompletionDenialClass::RetainedHistoryUnavailable => {
            BridgeAsyncCompletionDenialClass::StaleDenied
        }
        CompletionDenialClass::Malformed
        | CompletionDenialClass::Partial
        | CompletionDenialClass::Contradictory
        | CompletionDenialClass::Duplicate
        | CompletionDenialClass::UnknownRequest
        | CompletionDenialClass::Impossible => {
            BridgeAsyncCompletionDenialClass::SignalLifecycleDenied
        }
    }
}
