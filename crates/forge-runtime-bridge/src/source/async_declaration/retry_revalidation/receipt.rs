use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::identity::{
    AsyncForwardCausalityIdentityTag, AsyncForwardCausalityReceiptIdentityTag, BridgeIdentity,
};

use super::class::BridgeAsyncForwardCausalityClass;

pub type BridgeAsyncForwardCausalityIdentity = BridgeIdentity<AsyncForwardCausalityIdentityTag>;
pub type BridgeAsyncForwardCausalityReceiptIdentity =
    BridgeIdentity<AsyncForwardCausalityReceiptIdentityTag>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeAsyncForwardCausalityReceipt {
    receipt_identity: BridgeAsyncForwardCausalityReceiptIdentity,
    causality_identity: BridgeAsyncForwardCausalityIdentity,
    class: BridgeAsyncForwardCausalityClass,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeAsyncForwardCausalityReceipt {
    pub(crate) fn new(
        causality_identity: &BridgeAsyncForwardCausalityIdentity,
        class: BridgeAsyncForwardCausalityClass,
        canonical_basis: Arc<str>,
    ) -> Self {
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            receipt_identity: BridgeAsyncForwardCausalityReceiptIdentity::new(format!(
                "bridge-async-forward-causality-receipt-id:sha256:{digest:x}"
            )),
            causality_identity: causality_identity.clone(),
            class,
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-async-forward-causality-receipt:sha256:{digest:x}"
            )),
        }
    }

    pub fn receipt_identity(&self) -> &BridgeAsyncForwardCausalityReceiptIdentity {
        &self.receipt_identity
    }

    pub fn causality_identity(&self) -> &str {
        self.causality_identity.as_str()
    }

    pub fn class(&self) -> BridgeAsyncForwardCausalityClass {
        self.class
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
