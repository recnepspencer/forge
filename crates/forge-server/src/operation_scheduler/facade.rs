use crate::{ForgeServerLoweredOperationPlan, ForgeServerResponseFacade};

use super::{ForgeServerScheduledOperationBatch, ForgeServerSchedulerConflictDenial};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeServerSchedulerCancellationDirective {
    BeforeAdmission { slot_ordinal: usize },
    AfterAdmissionBeforeExecution { slot_ordinal: usize },
    DuringExecution { slot_ordinal: usize },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeServerSchedulerCertificationSabotage {
    ForbiddenGlobalLockAfterAdmission { slot_ordinal: usize },
}

#[derive(Clone, Debug)]
pub struct ForgeServerOperationScheduler {
    responses: ForgeServerResponseFacade,
}

impl ForgeServerOperationScheduler {
    pub fn schedule_batch(
        &self,
        plans: impl IntoIterator<Item = ForgeServerLoweredOperationPlan>,
    ) -> Result<ForgeServerScheduledOperationBatch, ForgeServerSchedulerConflictDenial> {
        ForgeServerScheduledOperationBatch::new(self.responses.clone(), plans)
    }

    pub(crate) fn new(responses: ForgeServerResponseFacade) -> Self {
        Self { responses }
    }

    pub fn schedule_shared_read_batch(
        &self,
        plans: impl IntoIterator<Item = ForgeServerLoweredOperationPlan>,
    ) -> Result<ForgeServerScheduledOperationBatch, ForgeServerSchedulerConflictDenial> {
        self.schedule_batch(plans)
    }
}
