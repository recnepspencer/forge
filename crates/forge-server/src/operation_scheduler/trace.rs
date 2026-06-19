use super::{
    ForgeServerScheduledOperationOutcome, ForgeServerSchedulerCancellationPosture,
    ForgeServerSchedulerFailurePosture,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerScheduledOperationTraceEntry {
    slot_ordinal: usize,
    scheduler_lane: String,
    execution_digest: Option<String>,
    failure_posture: Option<ForgeServerSchedulerFailurePosture>,
    cancellation_posture: Option<ForgeServerSchedulerCancellationPosture>,
}

impl ForgeServerScheduledOperationTraceEntry {
    pub(crate) fn from_outcome(outcome: &ForgeServerScheduledOperationOutcome) -> Self {
        Self {
            slot_ordinal: outcome.slot().ordinal(),
            scheduler_lane: outcome.slot().scheduler_lane(),
            execution_digest: outcome.execution_digest().map(str::to_string),
            failure_posture: outcome.failure_posture().cloned(),
            cancellation_posture: outcome.cancellation_posture(),
        }
    }

    pub fn slot_ordinal(&self) -> usize {
        self.slot_ordinal
    }

    pub fn scheduler_lane(&self) -> &str {
        &self.scheduler_lane
    }

    pub fn execution_digest(&self) -> Option<&str> {
        self.execution_digest.as_deref()
    }

    pub fn failure_posture(&self) -> Option<&ForgeServerSchedulerFailurePosture> {
        self.failure_posture.as_ref()
    }

    pub fn cancellation_posture(&self) -> Option<ForgeServerSchedulerCancellationPosture> {
        self.cancellation_posture
    }
}
