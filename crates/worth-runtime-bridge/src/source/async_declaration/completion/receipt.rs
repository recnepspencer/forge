use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::identity::{
    AsyncCompletionDenialReceiptIdentityTag, AsyncCompletionReceiptIdentityTag, BridgeIdentity,
};

use super::completion::{
    BridgeAsyncCompletionDenialIdentity, BridgeAsyncCompletionIdentity, BridgeAsyncCompletionState,
};

pub type BridgeAsyncCompletionReceiptIdentity = BridgeIdentity<AsyncCompletionReceiptIdentityTag>;
pub type BridgeAsyncDeniedCompletionReceiptIdentity =
    BridgeIdentity<AsyncCompletionDenialReceiptIdentityTag>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeAsyncCompletionReceipt {
    receipt_identity: BridgeAsyncCompletionReceiptIdentity,
    completion_identity: BridgeAsyncCompletionIdentity,
    state: BridgeAsyncCompletionState,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeAsyncCompletionReceipt {
    pub(super) fn admitted(
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
    pub(super) fn denied(
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
