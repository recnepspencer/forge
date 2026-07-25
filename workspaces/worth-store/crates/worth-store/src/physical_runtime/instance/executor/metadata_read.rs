use worth_store_io_scheduler::execute_ready_queue_plan;
use worth_store_physical_backend::{
    BackendQueueExecutionAdaptation, ScheduledArtifactMetadataReadOutcome,
};

use super::PhysicalWorkExecutor;
use crate::physical_runtime::{
    record_serving::residency::artifact_tree::PhysicalRecordArtifactTree,
    work::PhysicalMetadataExecutorCommand, PhysicalEffectRecoveryObligation,
    PhysicalExecutorDispatch, PhysicalExecutorOutcome,
};

impl PhysicalWorkExecutor {
    pub(super) fn dispatch_metadata(
        &self,
        command: PhysicalMetadataExecutorCommand,
    ) -> Result<PhysicalExecutorDispatch, crate::physical_runtime::PhysicalWorkPreEffectDenial>
    {
        let PhysicalMetadataExecutorCommand { work, artifact } = command;
        let (dispatched, plan) = work.into_execution_parts(None)?;
        let physical = PhysicalRecordArtifactTree::new(&self.media).read_scheduled_file_length(
            artifact,
            plan.backend_completion_binding()
                .backend_execution_binding(),
            BackendQueueExecutionAdaptation::None,
        );
        let outcome = match physical {
            ScheduledArtifactMetadataReadOutcome::Completed(completed) => {
                PhysicalExecutorOutcome::MetadataCompleted {
                    physical: completed.physical(),
                    scheduler: execute_ready_queue_plan(plan, completed.queue()),
                }
            }
            ScheduledArtifactMetadataReadOutcome::DeniedBeforeEffect(failure) => {
                PhysicalExecutorOutcome::DeniedBeforeEffect {
                    failure,
                    retry: crate::physical_runtime::work::PhysicalRetryPayload::Metadata,
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
