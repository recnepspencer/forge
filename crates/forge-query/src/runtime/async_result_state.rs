#![allow(dead_code)]

use std::collections::BTreeMap;

use forge_runtime_bridge::facade::{
    BridgeAsyncCompletionClass, BridgeAsyncCompletionDenialClass, BridgeAsyncCompletionReceipt,
    BridgeAsyncCompletionState, BridgeAsyncCompletionSupersessionReceipt,
    BridgeAsyncDeniedCompletionReceipt, BridgeAsyncForwardCausalityClass,
    BridgeAsyncForwardCausalityReceipt,
};

use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};

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
        causality_identity: ForgeQueryEvidenceIdentity,
    },
    CompletionState {
        state: BridgeAsyncCompletionState,
        causality_identity: ForgeQueryEvidenceIdentity,
    },
    ForwardCausality {
        class: BridgeAsyncForwardCausalityClass,
        causality_identity: ForgeQueryEvidenceIdentity,
    },
    Supersession {
        causality_identity: ForgeQueryEvidenceIdentity,
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

    fn causality_identity(&self) -> &ForgeQueryEvidenceIdentity {
        match self {
            Self::Pending {
                causality_identity, ..
            }
            | Self::CompletionState {
                causality_identity, ..
            }
            | Self::ForwardCausality {
                causality_identity, ..
            }
            | Self::Supersession {
                causality_identity, ..
            } => causality_identity,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn from_completion_receipt(receipt: &BridgeAsyncCompletionReceipt) -> Self {
        Self::CompletionState {
            state: receipt.state(),
            causality_identity: runtime_async_causality_identity(
                &bridge_async_causality_source_identity(receipt.completion_identity()),
            ),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn from_denied_completion_receipt(
        receipt: &BridgeAsyncDeniedCompletionReceipt,
    ) -> Self {
        Self::CompletionState {
            state: receipt.state(),
            causality_identity: runtime_async_causality_identity(
                &bridge_async_causality_source_identity(receipt.denial_identity()),
            ),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn from_forward_causality_receipt(
        receipt: &BridgeAsyncForwardCausalityReceipt,
    ) -> Self {
        Self::ForwardCausality {
            class: receipt.class(),
            causality_identity: runtime_async_causality_identity(
                &bridge_async_causality_source_identity(receipt.causality_identity()),
            ),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn from_supersession_receipt(
        receipt: &BridgeAsyncCompletionSupersessionReceipt,
    ) -> Self {
        Self::Supersession {
            causality_identity: runtime_async_causality_identity(
                &bridge_async_causality_source_identity(receipt.supersession_identity()),
            ),
        }
    }

    pub(crate) fn pending(causality_label: &str) -> Self {
        Self::Pending {
            causality_identity: runtime_async_causality_identity(
                &runtime_async_causality_label_identity(causality_label),
            ),
        }
    }

    #[cfg(test)]
    pub(crate) fn completion_state(
        state: BridgeAsyncCompletionState,
        causality_label: &str,
    ) -> Self {
        Self::CompletionState {
            state,
            causality_identity: runtime_async_causality_identity(
                &runtime_async_causality_label_identity(causality_label),
            ),
        }
    }

    #[cfg(test)]
    pub(crate) fn forward_causality(
        class: BridgeAsyncForwardCausalityClass,
        causality_label: &str,
    ) -> Self {
        Self::ForwardCausality {
            class,
            causality_identity: runtime_async_causality_identity(
                &runtime_async_causality_label_identity(causality_label),
            ),
        }
    }

    #[cfg(test)]
    pub(crate) fn supersession(causality_label: &str) -> Self {
        Self::Supersession {
            causality_identity: runtime_async_causality_identity(
                &runtime_async_causality_label_identity(causality_label),
            ),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryRuntimeAsyncResultState {
    kind: ForgeQueryRuntimeAsyncResultStateKind,
    causality_identity: ForgeQueryEvidenceIdentity,
    basis_identity: ForgeQueryEvidenceIdentity,
    checkpoint_identity: ForgeQueryEvidenceIdentity,
    result_state_identity: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryRuntimeAsyncResultState {
    pub fn new(
        kind: ForgeQueryRuntimeAsyncResultStateKind,
        causality_identity: &ForgeQueryEvidenceIdentity,
        basis_identity: &ForgeQueryEvidenceIdentity,
        checkpoint_identity: &ForgeQueryEvidenceIdentity,
    ) -> Self {
        let causality_identity = causality_identity.clone();
        let basis_identity = basis_identity.clone();
        let checkpoint_identity = checkpoint_identity.clone();
        let result_state_identity = runtime_async_result_state_identity(
            kind,
            &causality_identity,
            &basis_identity,
            &checkpoint_identity,
        );
        Self {
            kind,
            causality_identity,
            basis_identity,
            checkpoint_identity,
            result_state_identity,
        }
    }

    pub fn kind(&self) -> ForgeQueryRuntimeAsyncResultStateKind {
        self.kind
    }

    pub fn causality_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.causality_identity
    }

    pub fn causality_for_reporting(&self) -> &str {
        self.causality_identity.as_str()
    }

    pub fn basis_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.basis_identity
    }

    pub fn basis_for_reporting(&self) -> &str {
        self.basis_identity.as_str()
    }

    pub fn checkpoint_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.checkpoint_identity
    }

    pub fn checkpoint_for_reporting(&self) -> &str {
        self.checkpoint_identity.as_str()
    }

    pub fn result_state_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.result_state_identity
    }

    pub fn result_state_for_reporting(&self) -> &str {
        self.result_state_identity.as_str()
    }
}

fn bridge_async_causality_source_identity(bridge_identity: &str) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "forge_query_bridge_async_causality_source_v1",
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("bridge_identity"),
            bridge_identity,
        )
        .seal()
}

fn runtime_async_causality_label_identity(label: &str) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "forge_query_runtime_async_causality_label_v1",
        )
        .field_shape(ForgeQueryEvidenceTag::new("label"), label)
        .seal()
}

pub(crate) fn runtime_async_causality_identity(
    causality_identity: &ForgeQueryEvidenceIdentity,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "forge_query_runtime_async_causality_v1",
        )
        .field_evidence_identity(ForgeQueryEvidenceTag::new("causality"), causality_identity)
        .seal()
}

fn runtime_async_result_state_identity(
    kind: ForgeQueryRuntimeAsyncResultStateKind,
    causality_identity: &ForgeQueryEvidenceIdentity,
    basis_identity: &ForgeQueryEvidenceIdentity,
    checkpoint_identity: &ForgeQueryEvidenceIdentity,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::RuntimeStateSnapshot)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "forge_query_runtime_async_result_state_v1",
        )
        .field_shape(ForgeQueryEvidenceTag::new("kind"), kind.as_str())
        .field_evidence_identity(ForgeQueryEvidenceTag::new("causality"), causality_identity)
        .field_evidence_identity(ForgeQueryEvidenceTag::new("basis"), basis_identity)
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("checkpoint"),
            checkpoint_identity,
        )
        .seal()
}

pub(crate) fn runtime_async_causality_from_label(label: &str) -> ForgeQueryEvidenceIdentity {
    runtime_async_causality_identity(&runtime_async_causality_label_identity(label))
}

pub(crate) fn runtime_async_checkpoint_label_identity(label: &str) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "forge_query_runtime_async_checkpoint_label_v1",
        )
        .field_shape(ForgeQueryEvidenceTag::new("label"), label)
        .seal()
}

pub(crate) fn project_live_async_result_state(
    live_subscriptions: &mut BTreeMap<String, ForgeQueryRuntimeLiveSubscriptionState>,
    view_name: &str,
    projection: &ForgeQueryRuntimeAsyncResultProjection,
    basis_identity: &ForgeQueryEvidenceIdentity,
    checkpoint_identity: &ForgeQueryEvidenceIdentity,
) -> Result<ForgeQueryRuntimeAsyncResultState, ForgeQueryRuntimeError> {
    let state = live_subscriptions
        .get_mut(view_name)
        .ok_or_else(|| ForgeQueryRuntimeError::MissingLiveSubscription(view_name.to_string()))?;
    let expected_basis = state.installation.basis_binding_identity();
    let expected_checkpoint = state.active_lane_handle.checkpoint_identity();
    let kind = projection.kind();
    if basis_identity != expected_basis && !kind.permits_basis_or_generation_drift() {
        return Err(async_result_state_error(
            view_name,
            "PreviewBasisMismatchRequiresTypedState",
        ));
    }
    if checkpoint_identity != expected_checkpoint && !kind.permits_basis_or_generation_drift() {
        return Err(async_result_state_error(
            view_name,
            "GenerationDriftRequiresTypedState",
        ));
    }

    let async_result_state = ForgeQueryRuntimeAsyncResultState::new(
        kind,
        projection.causality_identity(),
        basis_identity,
        checkpoint_identity,
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
        basis_identity: &ForgeQueryEvidenceIdentity,
        checkpoint_identity: &ForgeQueryEvidenceIdentity,
    ) -> Result<ForgeQueryRuntimeAsyncResultState, ForgeQueryRuntimeError> {
        project_live_async_result_state(
            &mut self.live_subscriptions,
            view_name,
            projection,
            basis_identity,
            checkpoint_identity,
        )
    }
}
