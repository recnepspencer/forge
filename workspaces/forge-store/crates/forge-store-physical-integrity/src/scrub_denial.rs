use crate::{ScrubCounterSnapshot, ScrubWindowOrdinal};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrubOverBudgetClass {
    ResidentMemory,
    PinPage,
    Allocation,
    StreamingWindow,
    ProtectedRead,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrubPlanDenialKind {
    EmptyWindowSet,
    ZeroYieldWindowBudget,
    EmptyWindow { ordinal: ScrubWindowOrdinal },
    ResidentMemoryLimitExceeded { requested: u64, limit: u64 },
    PinPageLimitExceeded { requested: u32, limit: u32 },
    AllocationLimitExceeded { requested: u64, limit: u64 },
    StreamingWindowLimitExceeded { requested: u64, limit: u64 },
    ProtectedReadLimitExceeded { requested: u64, limit: u64 },
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrubExecutionDenialKind {
    ResumeTokenForDifferentPlan,
    ResumeTokenPastEnd,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScrubExecutionDenial {
    kind: ScrubExecutionDenialKind,
}

impl ScrubExecutionDenial {
    pub(crate) const fn new(kind: ScrubExecutionDenialKind) -> Self {
        Self { kind }
    }

    pub const fn kind(self) -> ScrubExecutionDenialKind {
        self.kind
    }
}
