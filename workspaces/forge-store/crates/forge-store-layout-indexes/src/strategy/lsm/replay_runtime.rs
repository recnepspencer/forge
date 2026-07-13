use super::{
    BaselineLsmExecutionAdmissionDenial, BaselineLsmReplayAdmission, BaselineLsmReplayExecution,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LsmReplayRuntime;

pub const fn lsm_replay_runtime() -> LsmReplayRuntime {
    LsmReplayRuntime
}

impl LsmReplayRuntime {
    pub fn execute(
        self,
        admission: BaselineLsmReplayAdmission,
    ) -> Result<BaselineLsmReplayExecution, BaselineLsmExecutionAdmissionDenial> {
        let (source, current_materialization) = admission.into_execution_basis();
        let plan = source.execution_plan();
        let replayable = plan.replayable_count();
        Ok(BaselineLsmReplayExecution::new(
            plan.replay_tail(),
            replayable,
            plan.stale_run_count(),
            plan.cleanup_batch_count(),
            plan.remaining_run_count(),
            current_materialization,
        ))
    }
}
