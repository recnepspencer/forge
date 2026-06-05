use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::BridgeAsyncRequestIdentityCounters;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeAsyncRequestIdentityRejectionKind {
    FamilyKindMismatch,
    LoweringIdentityMismatch,
    SubscriptionInstanceRequired,
    SubscriptionInstanceUnexpected,
    PreviewBasisSubscriptionInstanceMismatch,
    SignalRuntimeThreadAffinityViolation,
    SignalRequestAdmissionRejected,
    SignalAsyncRequestBlocked,
    InFlightRequestMissing,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeAsyncRequestIdentityRejection {
    kind: BridgeAsyncRequestIdentityRejectionKind,
    detail: Arc<str>,
    counters: BridgeAsyncRequestIdentityCounters,
    digest: Arc<str>,
}

impl BridgeAsyncRequestIdentityRejection {
    pub fn new(kind: BridgeAsyncRequestIdentityRejectionKind, detail: impl Into<Arc<str>>) -> Self {
        let detail = detail.into();
        let canonical_basis = format!(
            "bridge-async-request-identity-rejection|kind={kind:?}|detail={}",
            detail.as_ref()
        );
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            kind,
            detail,
            counters: BridgeAsyncRequestIdentityCounters::rejected(),
            digest: Arc::from(format!(
                "bridge-async-request-identity-rejection:sha256:{digest:x}"
            )),
        }
    }

    pub fn kind(&self) -> BridgeAsyncRequestIdentityRejectionKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        self.detail.as_ref()
    }

    pub fn counters(&self) -> &BridgeAsyncRequestIdentityCounters {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
