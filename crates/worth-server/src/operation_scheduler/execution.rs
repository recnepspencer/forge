use super::{
    WorthServerOperationSchedulerCounters, WorthServerScheduledOperationOutcome,
    WorthServerScheduledOperationTraceEntry,
};

#[derive(Debug)]
pub struct WorthServerExecutedOperationBatch {
    outcomes: Vec<WorthServerScheduledOperationOutcome>,
    counters: WorthServerOperationSchedulerCounters,
}

impl WorthServerExecutedOperationBatch {
    pub(crate) fn new(
        outcomes: Vec<WorthServerScheduledOperationOutcome>,
        counters: WorthServerOperationSchedulerCounters,
    ) -> Self {
        Self { outcomes, counters }
    }

    pub fn outcomes(&self) -> &[WorthServerScheduledOperationOutcome] {
        &self.outcomes
    }

    pub fn counters(&self) -> &WorthServerOperationSchedulerCounters {
        &self.counters
    }

    pub fn execution_trace(&self) -> Vec<WorthServerScheduledOperationTraceEntry> {
        self.outcomes
            .iter()
            .map(WorthServerScheduledOperationTraceEntry::from_outcome)
            .collect()
    }
}
