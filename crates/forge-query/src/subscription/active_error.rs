use super::active_counters::ActiveSubscriptionCounters;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ActiveSubscriptionLifecycleDenialKind {
    WorkBudgetExceeded,
    HeapAllocationForbidden,
    DurableCheckpointOverclaim,
    StoreBackedRestartOverclaim,
    RegistryEquivalenceMismatch,
    LinearScanLookupForbidden,
}

impl ActiveSubscriptionLifecycleDenialKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::WorkBudgetExceeded => "work_budget_exceeded",
            Self::HeapAllocationForbidden => "heap_allocation_forbidden",
            Self::DurableCheckpointOverclaim => "durable_checkpoint_overclaim",
            Self::StoreBackedRestartOverclaim => "store_backed_restart_overclaim",
            Self::RegistryEquivalenceMismatch => "registry_equivalence_mismatch",
            Self::LinearScanLookupForbidden => "linear_scan_lookup_forbidden",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActiveSubscriptionLifecycleError {
    denial_kind: ActiveSubscriptionLifecycleDenialKind,
    message: String,
    source_digest: String,
    counters: ActiveSubscriptionCounters,
}

impl ActiveSubscriptionLifecycleError {
    pub(super) fn new(
        denial_kind: ActiveSubscriptionLifecycleDenialKind,
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

    pub fn denial_kind(&self) -> &ActiveSubscriptionLifecycleDenialKind {
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
