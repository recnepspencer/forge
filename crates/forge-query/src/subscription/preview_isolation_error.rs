use super::active_counters::ActiveSubscriptionCounters;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PreviewSubscriptionIsolationDenialKind {
    PreviewAuthoritativeSharingDenied,
    PreviewDiscardResidueDenied,
    PreviewLifecycleStateMismatch,
}

impl PreviewSubscriptionIsolationDenialKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::PreviewAuthoritativeSharingDenied => "preview_authoritative_sharing_denied",
            Self::PreviewDiscardResidueDenied => "preview_discard_residue_denied",
            Self::PreviewLifecycleStateMismatch => "preview_lifecycle_state_mismatch",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreviewSubscriptionIsolationError {
    denial_kind: PreviewSubscriptionIsolationDenialKind,
    message: String,
    source_digest: String,
    counters: ActiveSubscriptionCounters,
}

impl PreviewSubscriptionIsolationError {
    pub(super) fn new(
        denial_kind: PreviewSubscriptionIsolationDenialKind,
        message: impl Into<String>,
        source_digest: impl Into<String>,
        counters: ActiveSubscriptionCounters,
    ) -> Self {
        Self {
            denial_kind,
            message: message.into(),
            source_digest: source_digest.into(),
            counters,
        }
    }

    pub fn denial_kind(&self) -> &PreviewSubscriptionIsolationDenialKind {
        &self.denial_kind
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn source_digest(&self) -> &str {
        &self.source_digest
    }

    pub fn counters(&self) -> &ActiveSubscriptionCounters {
        &self.counters
    }
}
