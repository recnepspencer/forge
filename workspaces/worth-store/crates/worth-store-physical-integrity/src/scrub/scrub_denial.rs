use crate::{ScrubCounterSnapshot, ScrubWindowOrdinal};
use worth_store::physical_runtime::LifecycleGeneration;
use worth_store_physical_format::store_namespace::StableStoreIdentity;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrubOverBudgetClass {
    Allocation,
    StreamingWindow,
    ProtectedRead,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrubPlanDenialKind {
    EmptyWindowSet,
    ZeroYieldWindowBudget,
    EmptyWindow {
        ordinal: ScrubWindowOrdinal,
    },
    OnlineWindowStoreMismatch {
        ordinal: ScrubWindowOrdinal,
        expected: StableStoreIdentity,
        actual: StableStoreIdentity,
    },
    OnlineWindowGenerationMismatch {
        ordinal: ScrubWindowOrdinal,
        expected: LifecycleGeneration,
        actual: LifecycleGeneration,
    },
    AllocationLimitExceeded {
        requested: u64,
        limit: u64,
    },
    StreamingWindowLimitExceeded {
        requested: u64,
        limit: u64,
    },
    ProtectedReadLimitExceeded {
        requested: u64,
        limit: u64,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScrubPlanDenial {
    kind: ScrubPlanDenialKind,
    counters: ScrubCounterSnapshot,
}

impl ScrubPlanDenial {
    pub(crate) const fn new(kind: ScrubPlanDenialKind, counters: ScrubCounterSnapshot) -> Self {
        Self { kind, counters }
    }

    pub const fn kind(self) -> ScrubPlanDenialKind {
        self.kind
    }

    pub const fn counters(self) -> ScrubCounterSnapshot {
        self.counters
    }
}
