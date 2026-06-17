use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::identity::{
    AsyncCompletionDenialIdentityTag, AsyncCompletionDenialReceiptIdentityTag,
    AsyncCompletionIdentityTag, AsyncCompletionReceiptIdentityTag, BridgeIdentity,
};
use forge_signal::facade::{
    AdmittedResourceCompletion, CompletionDenialClass, DeniedResourceCompletion,
};

use super::super::request_identity::AdmittedBridgeAsyncRequestIdentity;
use super::counters::BridgeAsyncCompletionCounters;
use super::envelope::ValidatedBridgeAsyncCompletionEnvelope;

type BridgeAsyncCompletionIdentity = BridgeIdentity<AsyncCompletionIdentityTag>;
type BridgeAsyncCompletionDenialIdentity = BridgeIdentity<AsyncCompletionDenialIdentityTag>;
pub type BridgeAsyncCompletionReceiptIdentity = BridgeIdentity<AsyncCompletionReceiptIdentityTag>;
pub type BridgeAsyncDeniedCompletionReceiptIdentity =
    BridgeIdentity<AsyncCompletionDenialReceiptIdentityTag>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeAsyncCompletionClass {
    Fulfilled,
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
pub struct BridgeAsyncCompletionReceipt {
    receipt_identity: BridgeAsyncCompletionReceiptIdentity,
    completion_identity: BridgeAsyncCompletionIdentity,
    state: BridgeAsyncCompletionState,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeAsyncCompletionReceipt {
    fn admitted(
        completion_identity: &BridgeAsyncCompletionIdentity,
        state: BridgeAsyncCompletionState,
        canonical_basis: Arc<str>,
    ) -> Self {
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            receipt_identity: BridgeAsyncCompletionReceiptIdentity::admit_bridge_owned(format!(
                "bridge-async-completion-receipt-id:sha256:{digest:x}"
            )),
            completion_identity: completion_identity.clone(),
            state,
            canonical_basis,
            digest: Arc::from(format!("bridge-async-completion-receipt:sha256:{digest:x}")),
        }
    }

    pub fn receipt_identity(&self) -> &BridgeAsyncCompletionReceiptIdentity {
        &self.receipt_identity
    }

    pub fn completion_identity(&self) -> &str {
        self.completion_identity.as_str()
    }

    pub fn state(&self) -> BridgeAsyncCompletionState {
        self.state
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeAsyncDeniedCompletionReceipt {
    receipt_identity: BridgeAsyncDeniedCompletionReceiptIdentity,
    denial_identity: BridgeAsyncCompletionDenialIdentity,
    state: BridgeAsyncCompletionState,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeAsyncDeniedCompletionReceipt {
    fn denied(
        denial_identity: &BridgeAsyncCompletionDenialIdentity,
        state: BridgeAsyncCompletionState,
        canonical_basis: Arc<str>,
    ) -> Self {
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            receipt_identity: BridgeAsyncDeniedCompletionReceiptIdentity::admit_bridge_owned(
                format!("bridge-async-denied-completion-receipt-id:sha256:{digest:x}"),
            ),
            denial_identity: denial_identity.clone(),
            state,
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-async-denied-completion-receipt:sha256:{digest:x}"
            )),
        }
    }

    pub fn receipt_identity(&self) -> &BridgeAsyncDeniedCompletionReceiptIdentity {
        &self.receipt_identity
    }

    pub fn denial_identity(&self) -> &str {
        self.denial_identity.as_str()
    }

    pub fn state(&self) -> BridgeAsyncCompletionState {
        self.state
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdmittedBridgeAsyncCompletion {
    completion_identity: BridgeAsyncCompletionIdentity,
    request_identity: AdmittedBridgeAsyncRequestIdentity,
    validated_envelope: ValidatedBridgeAsyncCompletionEnvelope,
    admitted_completion: AdmittedResourceCompletion,
    completion_class: BridgeAsyncCompletionClass,
    counters: BridgeAsyncCompletionCounters,
    receipt: BridgeAsyncCompletionReceipt,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl AdmittedBridgeAsyncCompletion {
    pub(crate) fn new(
        request_identity: AdmittedBridgeAsyncRequestIdentity,
        validated_envelope: ValidatedBridgeAsyncCompletionEnvelope,
        admitted_completion: AdmittedResourceCompletion,
        counters: BridgeAsyncCompletionCounters,
    ) -> Self {
        let completion_class = BridgeAsyncCompletionClass::Fulfilled;
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-async-completion|request={}|envelope={}|admitted-handle={}#{}|attempt={}|node={}|descriptor={}|ordinal={}|transition={:?}->{:?}|payload-bytes={}|class={:?}",
            request_identity.request_identity().as_str(),
            validated_envelope.envelope().digest(),
            admitted_completion.handle().request_id().get(),
            admitted_completion.handle().generation().get(),
            validated_envelope.raw().attempt().get(),
            admitted_completion.node().node(),
            admitted_completion.descriptor_id().get(),
            admitted_completion.completion_ordinal().get(),
            admitted_completion.lifecycle_transition().from(),
            admitted_completion.lifecycle_transition().to(),
            admitted_completion.payload_byte_len(),
            completion_class,
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        let completion_identity = BridgeAsyncCompletionIdentity::admit_bridge_owned(format!(
            "bridge-async-completion-id:sha256:{digest:x}"
        ));
        let receipt = BridgeAsyncCompletionReceipt::admitted(
            &completion_identity,
            BridgeAsyncCompletionState::Admitted(completion_class),
            Arc::from(format!(
                "bridge-async-completion-receipt|completion={}|request={}|transition={:?}->{:?}|ordinal={}",
                completion_identity.as_str(),
                request_identity.request_identity().as_str(),
                admitted_completion.lifecycle_transition().from(),
                admitted_completion.lifecycle_transition().to(),
                admitted_completion.completion_ordinal().get(),
            )),
        );

        Self {
            completion_identity,
            request_identity,
            validated_envelope,
            admitted_completion,
            completion_class,
            counters,
            receipt,
            canonical_basis,
            digest: Arc::from(format!("bridge-async-completion:sha256:{digest:x}")),
        }
    }

    pub fn completion_identity(&self) -> &str {
        self.completion_identity.as_str()
    }

    pub fn request_identity(&self) -> &AdmittedBridgeAsyncRequestIdentity {
        &self.request_identity
    }

    pub fn validated_envelope(&self) -> &ValidatedBridgeAsyncCompletionEnvelope {
        &self.validated_envelope
    }

    pub fn admitted_completion(&self) -> AdmittedResourceCompletion {
        self.admitted_completion
    }

    pub fn completion_class(&self) -> BridgeAsyncCompletionClass {
        self.completion_class
    }

    pub fn state(&self) -> BridgeAsyncCompletionState {
        BridgeAsyncCompletionState::Admitted(self.completion_class)
    }

    pub fn counters(&self) -> &BridgeAsyncCompletionCounters {
        &self.counters
    }

    pub fn receipt(&self) -> &BridgeAsyncCompletionReceipt {
        &self.receipt
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeAsyncDeniedCompletion {
    denial_identity: BridgeAsyncCompletionDenialIdentity,
    request_identity: AdmittedBridgeAsyncRequestIdentity,
    validated_envelope: ValidatedBridgeAsyncCompletionEnvelope,
    denied_completion: DeniedResourceCompletion,
    denial_class: BridgeAsyncCompletionDenialClass,
    signal_denial_class: CompletionDenialClass,
    counters: BridgeAsyncCompletionCounters,
    receipt: BridgeAsyncDeniedCompletionReceipt,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeAsyncDeniedCompletion {
    pub(crate) fn new(
        request_identity: AdmittedBridgeAsyncRequestIdentity,
        validated_envelope: ValidatedBridgeAsyncCompletionEnvelope,
        denied_completion: DeniedResourceCompletion,
        counters: BridgeAsyncCompletionCounters,
    ) -> Self {
        let denial_class = map_denial_class(denied_completion.class());
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-async-denied-completion|request={}|envelope={}|denial-id={}|signal-class={:?}|bridge-class={:?}|request={}#{}|branch={}#{}|attempt={}|node={}|payload-bytes={}",
            request_identity.request_identity().as_str(),
            validated_envelope.envelope().digest(),
            denied_completion.denial_id().get(),
            denied_completion.class(),
            denial_class,
            denied_completion.request_id().get(),
            denied_completion.generation().get(),
            denied_completion.branch_epoch().branch_id().0,
            denied_completion.branch_epoch().restore_epoch(),
            denied_completion.attempt().get(),
            denied_completion
                .node()
                .map(|node| node.node().index().to_string())
                .unwrap_or_else(|| "-".to_owned()),
            denied_completion.payload_byte_len(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        let denial_identity = BridgeAsyncCompletionDenialIdentity::admit_bridge_owned(format!(
            "bridge-async-denied-completion-id:sha256:{digest:x}"
        ));
        let receipt = BridgeAsyncDeniedCompletionReceipt::denied(
            &denial_identity,
            BridgeAsyncCompletionState::Denied(denial_class),
            Arc::from(format!(
                "bridge-async-denied-completion-receipt|denial={}|request={}|signal-class={:?}|bridge-class={:?}",
                denial_identity.as_str(),
                request_identity.request_identity().as_str(),
                denied_completion.class(),
                denial_class,
            )),
        );

        Self {
            denial_identity,
            request_identity,
            validated_envelope,
            denied_completion,
            denial_class,
            signal_denial_class: denied_completion.class(),
            counters,
            receipt,
            canonical_basis,
            digest: Arc::from(format!("bridge-async-denied-completion:sha256:{digest:x}")),
        }
    }

    pub fn denial_identity(&self) -> &str {
        self.denial_identity.as_str()
    }

    pub fn request_identity(&self) -> &AdmittedBridgeAsyncRequestIdentity {
        &self.request_identity
    }

    pub fn validated_envelope(&self) -> &ValidatedBridgeAsyncCompletionEnvelope {
        &self.validated_envelope
    }

    pub fn denied_completion(&self) -> DeniedResourceCompletion {
        self.denied_completion
    }

    pub fn denial_class(&self) -> BridgeAsyncCompletionDenialClass {
        self.denial_class
    }

    pub fn signal_denial_class(&self) -> CompletionDenialClass {
        self.signal_denial_class
    }

    pub fn state(&self) -> BridgeAsyncCompletionState {
        BridgeAsyncCompletionState::Denied(self.denial_class)
    }

    pub fn counters(&self) -> &BridgeAsyncCompletionCounters {
        &self.counters
    }

    pub fn receipt(&self) -> &BridgeAsyncDeniedCompletionReceipt {
        &self.receipt
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
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
}

fn map_denial_class(class: CompletionDenialClass) -> BridgeAsyncCompletionDenialClass {
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
