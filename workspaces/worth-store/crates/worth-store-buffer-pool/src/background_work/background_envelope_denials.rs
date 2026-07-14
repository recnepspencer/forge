use crate::{
    AllocationDenial, AllocationScope, BackgroundEnvelopeCounterSnapshot, BackgroundWorkClass,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackgroundEnvelopeDenialKind {
    ForegroundResidencyInterference {
        requested_frames: u32,
        background_available_frames: u32,
        foreground_reserved_frames: u32,
    },
    ForegroundAllocationInterference {
        requested_scope: AllocationScope,
    },
    PinBudgetWouldBeExceeded {
        requested_pages: u32,
        pinned_pages_used: u32,
        pinned_page_budget: u32,
    },
    IndefinitePinRequested {
        requested_pages: u32,
    },
    WholeObjectMemoryRequired {
        object_bytes: u64,
        envelope_bytes: u64,
    },
    StreamingWindowExceedsEnvelope {
        window_bytes: u64,
        envelope_bytes: u64,
    },
    StreamingEnvelopeExceedsWindow {
        envelope_bytes: u64,
        window_bytes: u64,
    },
    AllocationDenied(AllocationDenial),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BackgroundMemoryInterferenceReport {
    work_class: BackgroundWorkClass,
    kind: BackgroundEnvelopeDenialKind,
    counters: BackgroundEnvelopeCounterSnapshot,
}

impl BackgroundMemoryInterferenceReport {
    pub(crate) const fn new(
        work_class: BackgroundWorkClass,
        kind: BackgroundEnvelopeDenialKind,
        counters: BackgroundEnvelopeCounterSnapshot,
    ) -> Self {
        Self {
            work_class,
            kind,
            counters,
        }
    }

    pub const fn work_class(self) -> BackgroundWorkClass {
        self.work_class
    }

    pub const fn kind(self) -> BackgroundEnvelopeDenialKind {
        self.kind
    }

    pub const fn counters(self) -> BackgroundEnvelopeCounterSnapshot {
        self.counters
    }
}
