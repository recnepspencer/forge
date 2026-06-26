use crate::{AllocationDenial, SpeculativeWorkCounterSnapshot};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpeculativePhysicalWorkDenialKind {
    ResidentBudgetWouldBeExceeded {
        requested_frames: u32,
        free_frames: u32,
    },
    ProtectedEvictionPressure {
        requested_frames: u32,
    },
    DirtyBudgetWouldBeExceeded {
        requested_pages: u32,
        dirty_pages_used: u32,
        dirty_page_budget: u32,
    },
    DirtyWorkNotResident {
        requested_pages: u32,
        dirty_pages_used: u32,
    },
    PinBudgetWouldBeExceeded {
        requested_pages: u32,
        pinned_pages_used: u32,
        pinned_page_budget: u32,
    },
    ForegroundAllocationInterference {
        requested_bytes: u64,
    },
    AllocationDenied(AllocationDenial),
    UnsupportedQosClaim,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpeculativePhysicalWorkDenial {
    kind: SpeculativePhysicalWorkDenialKind,
    counters: SpeculativeWorkCounterSnapshot,
}

pub type SpeculativeResidencyDenial = SpeculativePhysicalWorkDenial;

impl SpeculativePhysicalWorkDenial {
    pub(crate) const fn new(
        kind: SpeculativePhysicalWorkDenialKind,
        counters: SpeculativeWorkCounterSnapshot,
    ) -> Self {
        Self { kind, counters }
    }

    pub const fn kind(self) -> SpeculativePhysicalWorkDenialKind {
        self.kind
    }

    pub const fn counters(self) -> SpeculativeWorkCounterSnapshot {
        self.counters
    }
}
