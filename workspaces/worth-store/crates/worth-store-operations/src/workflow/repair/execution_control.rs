use crate::OwnerPlanNodeIdentity;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepairExecutionBoundaryMoment {
    BeforeOwnerEffect,
    AfterOwnerEffectBeforeReceipt,
    AfterReceiptPersistence,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RepairExecutionBoundary {
    node: OwnerPlanNodeIdentity,
    moment: RepairExecutionBoundaryMoment,
}

impl RepairExecutionBoundary {
    pub(crate) const fn new(
        node: OwnerPlanNodeIdentity,
        moment: RepairExecutionBoundaryMoment,
    ) -> Self {
        Self { node, moment }
    }
    pub const fn node(self) -> OwnerPlanNodeIdentity {
        self.node
    }
    pub const fn moment(self) -> RepairExecutionBoundaryMoment {
        self.moment
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RepairExecutionInterrupted {
    boundary: RepairExecutionBoundary,
    cause: RepairExecutionInterruptionCause,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepairExecutionInterruptionCause {
    ProcessLoss,
    SchedulerDenied,
    BackendIndeterminate,
}

impl RepairExecutionInterrupted {
    pub const fn at(boundary: RepairExecutionBoundary) -> Self {
        Self {
            boundary,
            cause: RepairExecutionInterruptionCause::ProcessLoss,
        }
    }
    pub const fn scheduler_denied(boundary: RepairExecutionBoundary) -> Self {
        Self {
            boundary,
            cause: RepairExecutionInterruptionCause::SchedulerDenied,
        }
    }
    pub const fn backend_indeterminate(boundary: RepairExecutionBoundary) -> Self {
        Self {
            boundary,
            cause: RepairExecutionInterruptionCause::BackendIndeterminate,
        }
    }
    pub const fn boundary(self) -> RepairExecutionBoundary {
        self.boundary
    }
    pub const fn cause(self) -> RepairExecutionInterruptionCause {
        self.cause
    }
}

pub trait RepairExecutionControlPort {
    fn observe(&self, boundary: RepairExecutionBoundary) -> Result<(), RepairExecutionInterrupted>;
}

pub struct UninterruptedRepairExecution;

impl RepairExecutionControlPort for UninterruptedRepairExecution {
    fn observe(
        &self,
        _boundary: RepairExecutionBoundary,
    ) -> Result<(), RepairExecutionInterrupted> {
        Ok(())
    }
}
