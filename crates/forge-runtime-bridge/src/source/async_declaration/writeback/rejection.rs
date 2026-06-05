use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::BridgeAsyncWritebackCounters;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeAsyncWritebackRejectionKind {
    CompletionMustBeAuthoritative,
    PreviewCompletionForbidden,
    CurrentAuthorityDrifted,
    MapperEffectClassUnsupported,
    MapperFailed,
    PolicyAdmissionRejected,
    WritebackContractRejected,
    CandidateValidationRejected,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeAsyncWritebackRejection {
    kind: BridgeAsyncWritebackRejectionKind,
    detail: Arc<str>,
    counters: BridgeAsyncWritebackCounters,
    digest: Arc<str>,
}

impl BridgeAsyncWritebackRejection {
    pub(crate) fn new(
        kind: BridgeAsyncWritebackRejectionKind,
        detail: impl Into<Arc<str>>,
    ) -> Self {
        let detail = detail.into();
        let canonical_basis = format!(
            "bridge-async-writeback-rejection|kind={kind:?}|detail={}",
            detail.as_ref()
        );
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            kind,
            detail,
            counters: BridgeAsyncWritebackCounters::rejected(),
            digest: Arc::from(format!(
                "bridge-async-writeback-rejection:sha256:{digest:x}"
            )),
        }
    }

    pub fn kind(&self) -> BridgeAsyncWritebackRejectionKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        self.detail.as_ref()
    }

    pub fn counters(&self) -> &BridgeAsyncWritebackCounters {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
