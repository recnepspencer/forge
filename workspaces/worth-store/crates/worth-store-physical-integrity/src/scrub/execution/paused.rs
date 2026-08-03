use crate::{ScrubCounterSnapshot, ScrubLocalitySummary, ScrubPlan};

use super::{run::resume_execution, ScrubExecutionOutcome, ScrubProgressReport};

#[derive(Debug)]
pub struct PausedScrubExecution<'runtime, 'lease> {
    pub(super) plan: ScrubPlan<'runtime, 'lease>,
    pub(super) next_window_index: usize,
    pub(super) counters: ScrubCounterSnapshot,
    pub(super) locality: Option<ScrubLocalitySummary>,
}

impl<'runtime, 'lease> PausedScrubExecution<'runtime, 'lease> {
    pub const fn plan_identity(&self) -> u64 {
        self.plan.plan_identity()
    }

    pub const fn progress(&self) -> ScrubProgressReport {
        ScrubProgressReport::new(self.counters, self.locality, true)
    }

    pub fn resume(self) -> ScrubExecutionOutcome<'runtime, 'lease> {
        resume_execution(self)
    }
}
