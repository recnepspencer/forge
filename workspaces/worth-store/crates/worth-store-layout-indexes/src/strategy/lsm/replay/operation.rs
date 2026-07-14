use super::super::{
    BaselineLsmReplayAdmission, BaselineLsmReplayExecution, LsmExecutionOperation,
    LsmExecutionOwnerCaseDeclaration, LsmExecutionOwnerCaseId, LsmExecutionOwnerCaseObservation,
};

#[derive(Debug)]
pub struct LsmReplayExecutionOutcome {
    execution: BaselineLsmReplayExecution,
}

#[derive(Debug)]
pub enum LsmReplayExecutionView<'a> {
    Admitted(&'a BaselineLsmReplayExecution),
}

impl LsmReplayExecutionOutcome {
    fn issue(execution: BaselineLsmReplayExecution) -> Self {
        Self { execution }
    }

    pub const fn view(&self) -> LsmReplayExecutionView<'_> {
        LsmReplayExecutionView::Admitted(&self.execution)
    }

    pub fn into_result(
        self,
    ) -> Result<BaselineLsmReplayExecution, super::super::BaselineLsmExecutionAdmissionDenial> {
        Ok(self.execution)
    }

    pub const fn owner_case_observation(&self) -> LsmExecutionOwnerCaseObservation {
        LsmExecutionOwnerCaseObservation::new(LsmExecutionOwnerCaseId::admitted(
            LsmExecutionOperation::ExecuteReplay,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LsmReplayRuntime;

pub const fn lsm_replay_runtime() -> LsmReplayRuntime {
    LsmReplayRuntime
}

impl LsmReplayRuntime {
    pub fn execute(self, admission: BaselineLsmReplayAdmission) -> LsmReplayExecutionOutcome {
        let (source, current_materialization) = admission.into_execution_basis();
        let plan = source.execution_plan();
        LsmReplayExecutionOutcome::issue(BaselineLsmReplayExecution::new(
            plan.replay_tail(),
            plan.replayable_count(),
            plan.stale_run_count(),
            plan.cleanup_batch_count(),
            plan.remaining_run_count(),
            current_materialization,
        ))
    }
}

pub(super) fn owner_cases() -> impl Iterator<Item = LsmExecutionOwnerCaseDeclaration> {
    std::iter::once(LsmExecutionOwnerCaseDeclaration::new(
        LsmExecutionOwnerCaseId::admitted(LsmExecutionOperation::ExecuteReplay),
    ))
}
