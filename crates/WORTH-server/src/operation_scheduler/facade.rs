use crate::{WorthServerLoweredOperationPlan, WorthServerResponseFacade};

use super::{WorthServerScheduledOperationBatch, WorthServerSchedulerConflictDenial};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthServerSchedulerCancellationDirective {
    BeforeAdmission { slot_ordinal: usize },
    AfterAdmissionBeforeExecution { slot_ordinal: usize },
    DuringExecution { slot_ordinal: usize },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthServerSchedulerCertificationSabotage {
    ForbiddenGlobalLockAfterAdmission { slot_ordinal: usize },
}

#[derive(Clone, Debug)]
pub struct WorthServerOperationScheduler {
    responses: WorthServerResponseFacade,
}

impl WorthServerOperationScheduler {
    pub fn schedule_batch(
        &self,
        plans: impl IntoIterator<Item = WorthServerLoweredOperationPlan>,
    ) -> Result<WorthServerScheduledOperationBatch, WorthServerSchedulerConflictDenial> {
        WorthServerScheduledOperationBatch::new(self.responses.clone(), plans)
    }

    pub(crate) fn new(responses: WorthServerResponseFacade) -> Self {
        Self { responses }
    }

    pub fn schedule_shared_read_batch(
        &self,
        plans: impl IntoIterator<Item = WorthServerLoweredOperationPlan>,
    ) -> Result<WorthServerScheduledOperationBatch, WorthServerSchedulerConflictDenial> {
        self.schedule_batch(plans)
    }
}
