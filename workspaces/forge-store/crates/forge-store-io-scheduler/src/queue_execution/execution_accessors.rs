use super::{
    ExecutedQueueEvidence, QueueBackpressureCause, QueueExecutedPlan, QueueExecutionBackpressured,
    QueueExecutionCounterSnapshot, QueueExecutionDenied, QueueExecutionViolation,
    QueueExecutionViolationCause,
};

impl ExecutedQueueEvidence {
    pub const fn plan(&self) -> &QueueExecutedPlan {
        &self.plan
    }

    pub const fn secondary_plan(&self) -> Option<&QueueExecutedPlan> {
        self.secondary_plan.as_ref()
    }

    pub const fn counters(&self) -> QueueExecutionCounterSnapshot {
        self.counters
    }
}

impl QueueExecutionBackpressured {
    pub const fn plan(&self) -> &QueueExecutedPlan {
        &self.plan
    }

    pub const fn secondary_plan(&self) -> Option<&QueueExecutedPlan> {
        self.secondary_plan.as_ref()
    }

    pub const fn counters(&self) -> QueueExecutionCounterSnapshot {
        self.counters
    }

    pub const fn cause(&self) -> QueueBackpressureCause {
        self.cause
    }
}

impl QueueExecutionDenied {
    pub const fn plan(&self) -> &QueueExecutedPlan {
        &self.plan
    }

    pub const fn secondary_plan(&self) -> Option<&QueueExecutedPlan> {
        self.secondary_plan.as_ref()
    }

    pub const fn counters(&self) -> QueueExecutionCounterSnapshot {
        self.counters
    }

    pub const fn cause(&self) -> QueueBackpressureCause {
        self.cause
    }
}

impl QueueExecutionViolation {
    pub const fn plan(&self) -> &QueueExecutedPlan {
        &self.plan
    }

    pub const fn secondary_plan(&self) -> Option<&QueueExecutedPlan> {
        self.secondary_plan.as_ref()
    }

    pub const fn counters(&self) -> QueueExecutionCounterSnapshot {
        self.counters
    }

    pub const fn cause(&self) -> QueueExecutionViolationCause {
        self.cause
    }
}
