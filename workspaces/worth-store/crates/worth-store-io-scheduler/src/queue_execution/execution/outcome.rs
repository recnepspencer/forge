use super::{QueueBackpressureCause, QueueExecutedPlan, QueueExecutionCounterSnapshot};

#[derive(Debug, Eq, PartialEq)]
pub struct ExecutedQueueEvidence {
    pub(crate) plan: QueueExecutedPlan,
    pub(crate) secondary_plan: Option<QueueExecutedPlan>,
    pub(crate) counters: QueueExecutionCounterSnapshot,
}

#[derive(Debug, Eq, PartialEq)]
pub struct QueueExecutionBackpressured {
    pub(crate) plan: QueueExecutedPlan,
    pub(crate) secondary_plan: Option<QueueExecutedPlan>,
    pub(crate) counters: QueueExecutionCounterSnapshot,
    pub(crate) cause: QueueBackpressureCause,
}

#[derive(Debug, Eq, PartialEq)]
pub struct QueueExecutionDenied {
    pub(crate) plan: QueueExecutedPlan,
    pub(crate) secondary_plan: Option<QueueExecutedPlan>,
    pub(crate) counters: QueueExecutionCounterSnapshot,
    pub(crate) cause: QueueBackpressureCause,
}

#[derive(Debug, Eq, PartialEq)]
pub struct QueueExecutionViolation {
    pub(crate) plan: QueueExecutedPlan,
    pub(crate) secondary_plan: Option<QueueExecutedPlan>,
    pub(crate) counters: QueueExecutionCounterSnapshot,
    pub(crate) cause: QueueExecutionViolationCause,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QueueExecutionViolationCause {
    BackendContradictedWitness,
    ExecutionReclassifiedWork,
}

#[derive(Debug, Eq, PartialEq)]
pub enum QueueExecutionOutcome {
    Executed(ExecutedQueueEvidence),
    Backpressured(QueueExecutionBackpressured),
    Denied(QueueExecutionDenied),
    Violation(QueueExecutionViolation),
}
