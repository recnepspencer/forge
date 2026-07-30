use crate::{ScrubCounterSnapshot, ScrubLocalitySummary, ScrubMode};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrubIntegrityFinding {
    Intact,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScrubProgressReport {
    counters: ScrubCounterSnapshot,
    locality: Option<ScrubLocalitySummary>,
    interrupted: bool,
}

impl ScrubProgressReport {
    pub(super) const fn new(
        counters: ScrubCounterSnapshot,
        locality: Option<ScrubLocalitySummary>,
        interrupted: bool,
    ) -> Self {
        Self {
            counters,
            locality,
            interrupted,
        }
    }

    pub const fn counters(self) -> ScrubCounterSnapshot {
        self.counters
    }

    pub const fn locality(self) -> Option<ScrubLocalitySummary> {
        self.locality
    }

    pub const fn interrupted(self) -> bool {
        self.interrupted
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScrubExecutionReceipt {
    mode: ScrubMode,
    finding: ScrubIntegrityFinding,
    progress: ScrubProgressReport,
}

impl ScrubExecutionReceipt {
    pub(super) const fn completed(
        mode: ScrubMode,
        counters: ScrubCounterSnapshot,
        locality: Option<ScrubLocalitySummary>,
    ) -> Self {
        Self {
            mode,
            finding: ScrubIntegrityFinding::Intact,
            progress: ScrubProgressReport::new(counters, locality, false),
        }
    }

    pub const fn mode(self) -> ScrubMode {
        self.mode
    }

    pub const fn finding(self) -> ScrubIntegrityFinding {
        self.finding
    }

    pub const fn counters(self) -> ScrubCounterSnapshot {
        self.progress.counters()
    }

    pub const fn progress(self) -> ScrubProgressReport {
        self.progress
    }

    pub const fn locality(self) -> Option<ScrubLocalitySummary> {
        self.progress.locality()
    }

    pub const fn proves_recovery_behavior(self) -> bool {
        false
    }

    pub const fn proves_repair_behavior(self) -> bool {
        false
    }

    pub const fn proves_blob_lifecycle(self) -> bool {
        false
    }
}
