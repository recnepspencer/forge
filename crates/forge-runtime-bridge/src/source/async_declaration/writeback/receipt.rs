use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::identity::{
    AsyncWritebackCausalityTransferReceiptIdentityTag, AsyncWritebackCommittedIdentityTag,
    AsyncWritebackNoopIdentityTag, AsyncWritebackRejectedIdentityTag, BridgeIdentity,
};

use super::{
    BridgeAsyncWritebackFamily, BridgeAsyncWritebackNoopClass, BridgeAsyncWritebackRejectedClass,
};

pub type BridgeAsyncWritebackReceiptIdentity = BridgeIdentity<AsyncWritebackCommittedIdentityTag>;
pub type BridgeAsyncWritebackRejectedReceipt = BridgeIdentity<AsyncWritebackRejectedIdentityTag>;
pub type BridgeAsyncWritebackNoopReceipt = BridgeIdentity<AsyncWritebackNoopIdentityTag>;
pub type BridgeAsyncWritebackCausalityTransferReceiptIdentity =
    BridgeIdentity<AsyncWritebackCausalityTransferReceiptIdentityTag>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeAsyncWritebackCausalityTransferReceipt {
    receipt_identity: BridgeAsyncWritebackCausalityTransferReceiptIdentity,
    writeback_family: BridgeAsyncWritebackFamily,
    completion_identity: Arc<str>,
    request_identity: Arc<str>,
    authoritative_artifact_digest: Arc<str>,
    writeback_request_digest: Arc<str>,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeAsyncWritebackCausalityTransferReceipt {
    pub(crate) fn committed(
        family: BridgeAsyncWritebackFamily,
        completion_identity: &str,
        request_identity: &str,
        authoritative_artifact_digest: &str,
        writeback_request_digest: &str,
    ) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-async-writeback-causality-transfer|family={family:?}|completion={completion_identity}|request={request_identity}|authoritative={authoritative_artifact_digest}|writeback-request={writeback_request_digest}",
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            receipt_identity: BridgeAsyncWritebackCausalityTransferReceiptIdentity::new(format!(
                "bridge-async-writeback-causality-transfer-id:sha256:{digest:x}"
            )),
            writeback_family: family,
            completion_identity: Arc::from(completion_identity.to_owned()),
            request_identity: Arc::from(request_identity.to_owned()),
            authoritative_artifact_digest: Arc::from(authoritative_artifact_digest.to_owned()),
            writeback_request_digest: Arc::from(writeback_request_digest.to_owned()),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-async-writeback-causality-transfer:sha256:{digest:x}"
            )),
        }
    }

    pub fn receipt_identity(&self) -> &BridgeAsyncWritebackCausalityTransferReceiptIdentity {
        &self.receipt_identity
    }

    pub fn writeback_family(&self) -> BridgeAsyncWritebackFamily {
        self.writeback_family
    }

    pub fn completion_identity(&self) -> &str {
        self.completion_identity.as_ref()
    }

    pub fn request_identity(&self) -> &str {
        self.request_identity.as_ref()
    }

    pub fn authoritative_artifact_digest(&self) -> &str {
        self.authoritative_artifact_digest.as_ref()
    }

    pub fn writeback_request_digest(&self) -> &str {
        self.writeback_request_digest.as_ref()
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

pub(crate) fn committed_receipt_identity(
    completion_identity: &str,
    authoritative_artifact_digest: &str,
) -> BridgeAsyncWritebackReceiptIdentity {
    let digest = Sha256::digest(
        format!(
            "bridge-async-committed-writeback|completion={completion_identity}|authoritative={authoritative_artifact_digest}"
        )
        .as_bytes(),
    );
    BridgeAsyncWritebackReceiptIdentity::new(format!(
        "bridge-async-committed-writeback-id:sha256:{digest:x}"
    ))
}

pub(crate) fn noop_receipt_identity(
    completion_identity: &str,
    class: BridgeAsyncWritebackNoopClass,
) -> BridgeAsyncWritebackNoopReceipt {
    let digest = Sha256::digest(
        format!("bridge-async-noop-writeback|completion={completion_identity}|class={class:?}")
            .as_bytes(),
    );
    BridgeAsyncWritebackNoopReceipt::new(format!(
        "bridge-async-noop-writeback-id:sha256:{digest:x}"
    ))
}

pub(crate) fn rejected_receipt_identity(
    completion_identity: &str,
    class: BridgeAsyncWritebackRejectedClass,
) -> BridgeAsyncWritebackRejectedReceipt {
    let digest = Sha256::digest(
        format!("bridge-async-rejected-writeback|completion={completion_identity}|class={class:?}")
            .as_bytes(),
    );
    BridgeAsyncWritebackRejectedReceipt::new(format!(
        "bridge-async-rejected-writeback-id:sha256:{digest:x}"
    ))
}
