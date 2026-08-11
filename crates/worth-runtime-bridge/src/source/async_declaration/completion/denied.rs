use std::sync::Arc;

use sha2::{Digest, Sha256};

use worth_signal::facade::{CompletionDenialClass, DeniedResourceCompletion};

use super::super::request_identity::AdmittedBridgeAsyncRequestIdentity;
use super::completion::{
    map_denial_class, BridgeAsyncCompletionDenialClass, BridgeAsyncCompletionDenialIdentity,
    BridgeAsyncCompletionState,
};
use super::counters::BridgeAsyncCompletionCounters;
use super::envelope::ValidatedBridgeAsyncCompletionEnvelope;
use super::receipt::BridgeAsyncDeniedCompletionReceipt;

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
