use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::identity::{AsyncCompletionSupersessionReceiptIdentityTag, BridgeIdentity};

use super::BridgeAsyncCompletionCounters;

pub type BridgeAsyncCompletionSupersessionReceiptIdentity =
    BridgeIdentity<AsyncCompletionSupersessionReceiptIdentityTag>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeAsyncCompletionSupersessionReceipt {
    receipt_identity: BridgeAsyncCompletionSupersessionReceiptIdentity,
    supersession_identity: Arc<str>,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeAsyncCompletionSupersessionReceipt {
    pub(crate) fn new(
        supersession_identity: &str,
        supersession_class: super::BridgeAsyncCompletionSupersessionClass,
        evidence_digest: &str,
    ) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-async-completion-supersession-receipt|supersession={supersession_identity}|class={supersession_class:?}|evidence={evidence_digest}"
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            receipt_identity: BridgeAsyncCompletionSupersessionReceiptIdentity::admit_bridge_owned(
                format!("bridge-async-completion-supersession-receipt-id:sha256:{digest:x}"),
            ),
            supersession_identity: Arc::from(supersession_identity.to_owned()),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-async-completion-supersession-receipt:sha256:{digest:x}"
            )),
        }
    }

    pub fn receipt_identity(&self) -> &BridgeAsyncCompletionSupersessionReceiptIdentity {
        &self.receipt_identity
    }

    pub fn supersession_identity(&self) -> &str {
        self.supersession_identity.as_ref()
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeAsyncClassifiedDeniedCompletion {
    denied_completion: super::BridgeAsyncDeniedCompletion,
    supersession_class: super::BridgeAsyncCompletionSupersessionClass,
    evidence: super::BridgeAsyncCompletionSupersessionEvidence,
    receipt: BridgeAsyncCompletionSupersessionReceipt,
    counters: BridgeAsyncCompletionCounters,
}

impl BridgeAsyncClassifiedDeniedCompletion {
    pub(crate) fn new(
        denied_completion: super::BridgeAsyncDeniedCompletion,
        supersession_class: super::BridgeAsyncCompletionSupersessionClass,
        evidence: super::BridgeAsyncCompletionSupersessionEvidence,
        counters: BridgeAsyncCompletionCounters,
    ) -> Self {
        let receipt = BridgeAsyncCompletionSupersessionReceipt::new(
            evidence.supersession_identity().as_str(),
            supersession_class,
            evidence.digest(),
        );
        Self {
            denied_completion,
            supersession_class,
            evidence,
            receipt,
            counters,
        }
    }

    pub fn denied_completion(&self) -> &super::BridgeAsyncDeniedCompletion {
        &self.denied_completion
    }

    pub fn supersession_class(&self) -> super::BridgeAsyncCompletionSupersessionClass {
        self.supersession_class
    }

    pub fn evidence(&self) -> &super::BridgeAsyncCompletionSupersessionEvidence {
        &self.evidence
    }

    pub fn receipt(&self) -> &BridgeAsyncCompletionSupersessionReceipt {
        &self.receipt
    }

    pub fn counters(&self) -> &BridgeAsyncCompletionCounters {
        &self.counters
    }
}
