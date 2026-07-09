use std::sync::Arc;

use super::BridgeAsyncSourceDeclarationCounters;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeAsyncSourceDeclarationRejectionKind {
    RequestResponseObservationPolicyMismatch,
    SubscriptionBackedObservationPolicyMismatch,
    SignalDeclarationRejected,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeAsyncSourceDeclarationRejection {
    kind: BridgeAsyncSourceDeclarationRejectionKind,
    detail: Arc<str>,
    counters: BridgeAsyncSourceDeclarationCounters,
}

impl BridgeAsyncSourceDeclarationRejection {
    pub fn new(
        kind: BridgeAsyncSourceDeclarationRejectionKind,
        detail: impl Into<Arc<str>>,
    ) -> Self {
        Self {
            kind,
            detail: detail.into(),
            counters: BridgeAsyncSourceDeclarationCounters::rejected(),
        }
    }

    pub fn kind(&self) -> BridgeAsyncSourceDeclarationRejectionKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        self.detail.as_ref()
    }

    pub fn counters(&self) -> &BridgeAsyncSourceDeclarationCounters {
        &self.counters
    }
}
