use std::collections::BTreeMap;

use forge_runtime_bridge::facade::{
    BridgeAsyncCompletionClass, BridgeAsyncCompletionDenialClass, BridgeAsyncCompletionReceipt,
    BridgeAsyncCompletionState, BridgeAsyncCompletionSupersessionReceipt,
    BridgeAsyncDeniedCompletionReceipt, BridgeAsyncForwardCausalityClass,
    BridgeAsyncForwardCausalityReceipt,
};

use crate::identity::hash_parts;

use super::{
    ForgeQueryRuntimeError, ForgeQueryRuntimeLiveSubscriptionState, ForgeQueryRuntimeStateKind,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryRuntimeAsyncResultStateKind {
    Pending,
    Current,
    Failed,
    Stale,
    Cancelled,
    Retried,
    Revalidating,
    Superseded,
    Denied,
}

impl ForgeQueryRuntimeAsyncResultStateKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Current => "current",
            Self::Failed => "failed",
            Self::Stale => "stale",
            Self::Cancelled => "cancelled",
            Self::Retried => "retried",
            Self::Revalidating => "revalidating",
            Self::Superseded => "superseded",
            Self::Denied => "denied",
        }
    }

    pub(crate) fn permits_basis_or_generation_drift(self) -> bool {
        matches!(self, Self::Stale | Self::Superseded | Self::Denied)
    }

    pub(crate) fn state_kind(self) -> ForgeQueryRuntimeStateKind {
        match self {
            Self::Pending => ForgeQueryRuntimeStateKind::Pending,
            Self::Current => ForgeQueryRuntimeStateKind::Ready,
            Self::Failed => ForgeQueryRuntimeStateKind::Failed,
            Self::Stale => ForgeQueryRuntimeStateKind::Stale,
            Self::Cancelled => ForgeQueryRuntimeStateKind::Cancelled,
            Self::Retried => ForgeQueryRuntimeStateKind::Retried,
            Self::Revalidating => ForgeQueryRuntimeStateKind::Revalidating,
            Self::Superseded => ForgeQueryRuntimeStateKind::Superseded,
            Self::Denied => ForgeQueryRuntimeStateKind::Denied,
        }
    }
}

impl std::fmt::Display for ForgeQueryRuntimeAsyncResultStateKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum ForgeQueryRuntimeAsyncResultProjection {
    Pending {
        causality_digest: String,
    },
    CompletionState {
        state: BridgeAsyncCompletionState,
        causality_digest: String,
    },
    ForwardCausality {
        class: BridgeAsyncForwardCausalityClass,
        causality_digest: String,
    },
    Supersession {
        causality_digest: String,
    },
}

impl ForgeQueryRuntimeAsyncResultProjection {
    fn kind(&self) -> ForgeQueryRuntimeAsyncResultStateKind {
        match self {
            Self::Pending { .. } => ForgeQueryRuntimeAsyncResultStateKind::Pending,
            Self::CompletionState { state, .. } => match state {
                BridgeAsyncCompletionState::Admitted(BridgeAsyncCompletionClass::Fulfilled) => {
                    ForgeQueryRuntimeAsyncResultStateKind::Current
                }
                BridgeAsyncCompletionState::Denied(BridgeAsyncCompletionDenialClass::Rejected)
                | BridgeAsyncCompletionState::Denied(BridgeAsyncCompletionDenialClass::TimedOut) => {
                    ForgeQueryRuntimeAsyncResultStateKind::Failed
                }
                BridgeAsyncCompletionState::Denied(BridgeAsyncCompletionDenialClass::Cancelled) => {
                    ForgeQueryRuntimeAsyncResultStateKind::Cancelled
                }
                BridgeAsyncCompletionState::Denied(
                    BridgeAsyncCompletionDenialClass::StaleDenied,
                ) => ForgeQueryRuntimeAsyncResultStateKind::Stale,
                BridgeAsyncCompletionState::Denied(
                    BridgeAsyncCompletionDenialClass::Superseded,
                ) => ForgeQueryRuntimeAsyncResultStateKind::Superseded,
                BridgeAsyncCompletionState::Denied(
                    BridgeAsyncCompletionDenialClass::SignalLifecycleDenied,
                ) => ForgeQueryRuntimeAsyncResultStateKind::Denied,
            },
            Self::ForwardCausality { class, .. } => match class {
                BridgeAsyncForwardCausalityClass::RetryAfterTimeout
                | BridgeAsyncForwardCausalityClass::RetryAfterCancellation => {
                    ForgeQueryRuntimeAsyncResultStateKind::Retried
                }
                BridgeAsyncForwardCausalityClass::RevalidationAfterTruthBasisDrift
                | BridgeAsyncForwardCausalityClass::RevalidationAfterPreviewBasisDrift
                | BridgeAsyncForwardCausalityClass::RevalidationAfterSubscriptionInstanceDrift => {
                    ForgeQueryRuntimeAsyncResultStateKind::Revalidating
                }
            },
            Self::Supersession { .. } => ForgeQueryRuntimeAsyncResultStateKind::Superseded,
        }
    }

    fn causality_digest(&self) -> &str {
        match self {
            Self::Pending { causality_digest }
            | Self::CompletionState {
                causality_digest, ..
            }
            | Self::ForwardCausality {
                causality_digest, ..
            }
            | Self::Supersession { causality_digest } => causality_digest,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn from_completion_receipt(receipt: &BridgeAsyncCompletionReceipt) -> Self {
        Self::CompletionState {
            state: receipt.state(),
            causality_digest: receipt.digest().to_string(),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn from_denied_completion_receipt(
        receipt: &BridgeAsyncDeniedCompletionReceipt,
    ) -> Self {
        Self::CompletionState {
            state: receipt.state(),
            causality_digest: receipt.digest().to_string(),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn from_forward_causality_receipt(
        receipt: &BridgeAsyncForwardCausalityReceipt,
    ) -> Self {
        Self::ForwardCausality {
            class: receipt.class(),
            causality_digest: receipt.digest().to_string(),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn from_supersession_receipt(
        receipt: &BridgeAsyncCompletionSupersessionReceipt,
    ) -> Self {
        Self::Supersession {
            causality_digest: receipt.digest().to_string(),
        }
    }

    pub(crate) fn pending(causality_digest: &str) -> Self {
        Self::Pending {
            causality_digest: causality_digest.to_string(),
        }
    }

    #[cfg(test)]
    pub(crate) fn completion_state(
        state: BridgeAsyncCompletionState,
        causality_digest: &str,
    ) -> Self {
        Self::CompletionState {
            state,
            causality_digest: causality_digest.to_string(),
        }
    }

    #[cfg(test)]
    pub(crate) fn forward_causality(
        class: BridgeAsyncForwardCausalityClass,
        causality_digest: &str,
    ) -> Self {
        Self::ForwardCausality {
            class,
            causality_digest: causality_digest.to_string(),
        }
    }

    #[cfg(test)]
    pub(crate) fn supersession(causality_digest: &str) -> Self {
        Self::Supersession {
            causality_digest: causality_digest.to_string(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryRuntimeAsyncResultState {
    kind: ForgeQueryRuntimeAsyncResultStateKind,
    causality_digest: String,
    basis_digest: String,
    generation_digest: String,
    result_state_digest: String,
}

impl ForgeQueryRuntimeAsyncResultState {
    pub fn new(
        kind: ForgeQueryRuntimeAsyncResultStateKind,
        causality_digest: impl Into<String>,
        basis_digest: impl Into<String>,
        generation_digest: impl Into<String>,
    ) -> Self {
        let causality_digest = causality_digest.into();
        let basis_digest = basis_digest.into();
        let generation_digest = generation_digest.into();
        let result_state_digest = hash_parts(&[
            "forge_query_runtime_async_result_state_v1".to_string(),
            format!("kind:{}", kind.as_str()),
            format!("causality:{causality_digest}"),
            format!("basis:{basis_digest}"),
            format!("generation:{generation_digest}"),
        ]);
        Self {
            kind,
            causality_digest,
            basis_digest,
            generation_digest,
            result_state_digest,
        }
    }

    pub fn kind(&self) -> ForgeQueryRuntimeAsyncResultStateKind {
        self.kind
    }

    pub fn causality_digest(&self) -> &str {
        &self.causality_digest
    }

    pub fn basis_digest(&self) -> &str {
        &self.basis_digest
    }

    pub fn generation_digest(&self) -> &str {
        &self.generation_digest
    }

    pub fn result_state_digest(&self) -> &str {
        &self.result_state_digest
    }
}

pub(crate) fn project_live_async_result_state(
    live_subscriptions: &mut BTreeMap<String, ForgeQueryRuntimeLiveSubscriptionState>,
    view_name: &str,
    projection: &ForgeQueryRuntimeAsyncResultProjection,
    basis_digest: &str,
    generation_digest: &str,
) -> Result<ForgeQueryRuntimeAsyncResultState, ForgeQueryRuntimeError> {
    let state = live_subscriptions
        .get_mut(view_name)
        .ok_or_else(|| ForgeQueryRuntimeError::MissingLiveSubscription(view_name.to_string()))?;
    let expected_basis = state.installation.basis_binding_for_reporting();
    let expected_generation = state.active_lane_handle.checkpoint_identity_digest();
    let kind = projection.kind();
    if basis_digest != expected_basis && !kind.permits_basis_or_generation_drift() {
        return Err(async_result_state_error(
            view_name,
            "PreviewBasisMismatchRequiresTypedState",
        ));
    }
    if generation_digest != expected_generation && !kind.permits_basis_or_generation_drift() {
        return Err(async_result_state_error(
            view_name,
            "GenerationDriftRequiresTypedState",
        ));
    }

    let async_result_state = ForgeQueryRuntimeAsyncResultState::new(
        kind,
        projection.causality_digest(),
        basis_digest,
        generation_digest,
    );
    state.async_result_state = Some(async_result_state.clone());
    Ok(async_result_state)
}

fn async_result_state_error(view_name: &str, message: &str) -> ForgeQueryRuntimeError {
    ForgeQueryRuntimeError::LiveSubscriptionInstallation {
        view_name: view_name.to_string(),
        stage: "async-result-state",
        message: message.to_string(),
    }
}

impl super::ForgeQueryRuntime {
    pub(crate) fn project_async_result_state(
        &mut self,
        view_name: &str,
        projection: &ForgeQueryRuntimeAsyncResultProjection,
        basis_digest: &str,
        generation_digest: &str,
    ) -> Result<ForgeQueryRuntimeAsyncResultState, ForgeQueryRuntimeError> {
        project_live_async_result_state(
            &mut self.live_subscriptions,
            view_name,
            projection,
            basis_digest,
            generation_digest,
        )
    }
}
