use worth_store_io_scheduler::execute_ready_queue_plan;
use worth_store_physical_backend::{
    BackendQueueExecutionAdaptation, ScheduledArtifactTreePublicationEffectOutcome,
};

use super::{recovery_obligation::scheduler_recovery, PhysicalWorkExecutor};
use crate::physical_runtime::{
    PhysicalEffectRecoveryObligation, PhysicalExecutorDispatch, PhysicalExecutorOutcome,
    PhysicalRetryPayload, PhysicalWalBarrierExecutorCommand,
};

impl PhysicalWorkExecutor {
    pub(super) fn dispatch_wal_barrier(
        &self,
        command: PhysicalWalBarrierExecutorCommand,
    ) -> Result<PhysicalExecutorDispatch, crate::physical_runtime::PhysicalWorkPreEffectDenial>
    {
        let PhysicalWalBarrierExecutorCommand {
            work,
            artifact,
            binding_digest,
        } = command;
        let scope = work
            .intent()
            .scope()
            .wal_barrier_target()
            .expect("WAL barrier commands carry exact WAL barrier scope");
        let (dispatched, plan) = work.into_execution_parts(Some(binding_digest))?;
        let target = crate::physical_runtime::PhysicalWorkRecoveryTarget::WalArtifactInterval {
            segment: scope.segment(),
            generation: scope.generation(),
            offset: scope.append_offset(),
            byte_count: scope.append_byte_count(),
        };
        let prepared = self.prepare_effect_recovery(&dispatched, target, Some(binding_digest))?;
        let physical = self.media.artifact_tree().synchronize_scheduled_file(
            &artifact,
            plan.backend_completion_binding()
                .backend_execution_binding(),
            BackendQueueExecutionAdaptation::None,
        );
        let (outcome, recovery) = match physical {
            ScheduledArtifactTreePublicationEffectOutcome::Completed(completed) => {
                let physical = completed.physical().clone();
                let scheduler = execute_ready_queue_plan(plan, completed.queue());
                let recovery = scheduler_recovery(&scheduler);
                (
                    PhysicalExecutorOutcome::WalBarrierCompleted {
                        physical: crate::physical_runtime::work::CompletedPhysicalWalBarrier::new(
                            physical, artifact,
                        ),
                        scheduler,
                    },
                    recovery,
                )
            }
            ScheduledArtifactTreePublicationEffectOutcome::DeniedBeforeEffect(failure) => (
                PhysicalExecutorOutcome::DeniedBeforeEffect {
                    failure,
                    retry: PhysicalRetryPayload::WalBarrier {
                        artifact,
                        binding_digest,
                    },
                },
                PhysicalEffectRecoveryObligation::Cleared,
            ),
            ScheduledArtifactTreePublicationEffectOutcome::Indeterminate(failure) => (
                PhysicalExecutorOutcome::WalBarrierIndeterminate(
                    crate::physical_runtime::work::IndeterminatePhysicalWalBarrier::new(
                        failure, artifact,
                    ),
                ),
                PhysicalEffectRecoveryObligation::Retained,
            ),
        };
        let recovery = self.finish_effect_recovery(prepared, recovery);
        Ok(PhysicalExecutorDispatch::new(dispatched, outcome, recovery))
    }
}
