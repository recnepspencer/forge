use worth_store_io_scheduler::execute_ready_queue_plan;
use worth_store_physical_backend::{
    BackendQueueExecutionAdaptation, ScheduledArtifactRangeReadOutcome,
};

use super::PhysicalWorkExecutor;
use crate::physical_runtime::{
    record_serving::residency::artifact_tree::PhysicalRecordArtifactTree,
    PhysicalEffectRecoveryObligation, PhysicalExecutorDispatch, PhysicalExecutorOutcome,
    PhysicalReadExecutorCommand,
};

impl PhysicalWorkExecutor {
    pub(super) fn dispatch_read(
        &self,
        command: PhysicalReadExecutorCommand,
    ) -> Result<PhysicalExecutorDispatch, crate::physical_runtime::PhysicalWorkPreEffectDenial>
    {
        let PhysicalReadExecutorCommand {
            work,
            coordinate,
            mut destination,
        } = command;
        let (dispatched, plan) = work.into_execution_parts(None)?;
        let physical = PhysicalRecordArtifactTree::new(&self.media).read_scheduled_exact_at(
            coordinate,
            &mut destination,
            plan.backend_completion_binding()
                .backend_execution_binding(),
            BackendQueueExecutionAdaptation::None,
        );
        let outcome = match physical {
            ScheduledArtifactRangeReadOutcome::Completed(completed) => {
                #[cfg(feature = "certification-test-authority")]
                self.certification_yieldpoints.pause(
                    super::CertificationPhysicalExecutionCheckpoint::
                        AfterReadBeforeSchedulerSettlement,
                );
                PhysicalExecutorOutcome::ReadCompleted {
                    physical: completed.physical(),
                    bytes: destination,
                    scheduler: execute_ready_queue_plan(plan, completed.queue()),
                }
            }
            ScheduledArtifactRangeReadOutcome::DeniedBeforeEffect(failure) => {
                PhysicalExecutorOutcome::DeniedBeforeEffect {
                    failure,
                    retry: crate::physical_runtime::work::PhysicalRetryPayload::Read,
                }
            }
        };
        Ok(PhysicalExecutorDispatch::new(
            dispatched,
            outcome,
            PhysicalEffectRecoveryObligation::Cleared,
        ))
    }
}
