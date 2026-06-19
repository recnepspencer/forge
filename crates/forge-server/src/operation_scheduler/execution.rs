use super::{
    ForgeServerOperationSchedulerCounters, ForgeServerScheduledOperationOutcome,
    ForgeServerScheduledOperationTraceEntry,
};

#[derive(Debug)]
pub struct ForgeServerExecutedOperationBatch {
    outcomes: Vec<ForgeServerScheduledOperationOutcome>,
    counters: ForgeServerOperationSchedulerCounters,
}

impl ForgeServerExecutedOperationBatch {
    pub(crate) fn new(
        outcomes: Vec<ForgeServerScheduledOperationOutcome>,
        counters: ForgeServerOperationSchedulerCounters,
    ) -> Self {
        Self { outcomes, counters }
    }

    pub fn outcomes(&self) -> &[ForgeServerScheduledOperationOutcome] {
        &self.outcomes
    }

    pub fn counters(&self) -> &ForgeServerOperationSchedulerCounters {
        &self.counters
    }

    pub fn execution_trace(&self) -> Vec<ForgeServerScheduledOperationTraceEntry> {
        self.outcomes
            .iter()
            .map(ForgeServerScheduledOperationTraceEntry::from_outcome)
            .collect()
    }
}
