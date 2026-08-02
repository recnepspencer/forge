#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalMutationObservation {
    counters: [u64; 11],
}

impl PhysicalMutationObservation {
    pub(in crate::physical_runtime) const fn new(counters: [u64; 11]) -> Self {
        Self { counters }
    }

    pub const fn started(self) -> u64 {
        self.counters[0]
    }
    pub const fn completed(self) -> u64 {
        self.counters[1]
    }
    pub const fn proven_no_effect(self) -> u64 {
        self.counters[2]
    }
    pub const fn indeterminate(self) -> u64 {
        self.counters[3]
    }
    pub const fn completed_unobserved(self) -> u64 {
        self.counters[4]
    }
    pub const fn worker_panics(self) -> u64 {
        self.counters[5]
    }
    pub const fn cancellation_accepted(self) -> u64 {
        self.counters[6]
    }
    pub const fn cancellation_effectful(self) -> u64 {
        self.counters[7]
    }
    pub const fn cancellation_terminal(self) -> u64 {
        self.counters[8]
    }
    pub const fn cancellation_stale(self) -> u64 {
        self.counters[9]
    }
    pub const fn cancellation_runtime_closing(self) -> u64 {
        self.counters[10]
    }

    pub const fn requires_inspection(self) -> bool {
        self.indeterminate() != 0 || self.worker_panics() != 0
    }
}
