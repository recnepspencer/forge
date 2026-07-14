use crate::ScrubCounterSnapshot;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScrubResumeToken {
    plan_identity: u64,
    next_window_index: usize,
    counters: ScrubCounterSnapshot,
}

impl ScrubResumeToken {
    pub(crate) const fn new(
        plan_identity: u64,
        next_window_index: usize,
        counters: ScrubCounterSnapshot,
    ) -> Self {
        Self {
            plan_identity,
            next_window_index,
            counters,
        }
    }

    pub const fn plan_identity(self) -> u64 {
        self.plan_identity
    }

    pub const fn next_window_index(self) -> usize {
        self.next_window_index
    }

    pub const fn counters(self) -> ScrubCounterSnapshot {
        self.counters
    }

    pub const fn proves_unchanged_unvisited_bytes(self) -> bool {
        false
    }
}
