use super::active_counters::ActiveSubscriptionCounters;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SubscriptionContinuationDenialKind {
    UnsupportedContinuationClass,
    ContinuationEvidenceMismatch,
    ContinuationRemapBudgetExceeded,
}

impl SubscriptionContinuationDenialKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::UnsupportedContinuationClass => "unsupported_continuation_class",
            Self::ContinuationEvidenceMismatch => "continuation_evidence_mismatch",
            Self::ContinuationRemapBudgetExceeded => "continuation_remap_budget_exceeded",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SubscriptionContinuationError {
    denial_kind: SubscriptionContinuationDenialKind,
    message: String,
    source_digest: String,
    counters: ActiveSubscriptionCounters,
}

impl SubscriptionContinuationError {
    pub(super) fn new(
        denial_kind: SubscriptionContinuationDenialKind,
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

    pub fn denial_kind(&self) -> &SubscriptionContinuationDenialKind {
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
