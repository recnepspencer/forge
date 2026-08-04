use worth_store_io_scheduler::execute_ready_queue_plan;
use worth_store_physical_backend::{
    BackendQueueExecutionAdaptation, ScheduledArtifactAppendOutcome,
};

use super::{recovery_obligation::scheduler_recovery, PhysicalWorkExecutor};
use crate::physical_runtime::work::{PhysicalRetryPayload, PhysicalWalAppendExecutorCommand};
use crate::physical_runtime::{
    PhysicalEffectRecoveryObligation, PhysicalExecutorDispatch, PhysicalExecutorOutcome,
};

impl PhysicalWorkExecutor {
    pub(super) fn dispatch_wal_append(
        &self,
        command: PhysicalWalAppendExecutorCommand,
    ) -> Result<PhysicalExecutorDispatch, crate::physical_runtime::PhysicalWorkPreEffectDenial>
    {
        let PhysicalWalAppendExecutorCommand {
            work,
            artifact,
            range,
            payload,
            payload_digest,
        } = command;
        let scope = work
            .intent()
            .scope()
            .wal_append_target()
            .expect("WAL append commands require WAL scope");
        let target = crate::physical_runtime::PhysicalWorkRecoveryTarget::WalArtifactInterval {
            segment: scope.segment(),
            generation: scope.generation(),
            offset: scope.offset(),
            byte_count: scope.byte_count(),
        };
        let (dispatched, plan) = work.into_execution_parts(Some(payload_digest))?;
        let prepared = self.prepare_effect_recovery(&dispatched, target, Some(payload_digest))?;
        let physical = self
            .media
            .artifact_tree()
            .append_scheduled_artifact_exact_at(
                &artifact,
                range,
                &payload,
                plan.backend_completion_binding()
                    .backend_execution_binding(),
                BackendQueueExecutionAdaptation::None,
            );
        let (outcome, physical_recovery) = match physical {
            ScheduledArtifactAppendOutcome::Completed(completed) => {
                let physical = completed.physical().clone();
                let scheduler = execute_ready_queue_plan(plan, completed.queue());
                let recovery = scheduler_recovery(&scheduler);
                (
                    PhysicalExecutorOutcome::WalAppendCompleted {
                        physical,
                        scheduler,
                    },
                    recovery,
                )
            }
            ScheduledArtifactAppendOutcome::DeniedBeforeEffect(failure) => (
                PhysicalExecutorOutcome::DeniedBeforeEffect {
                    failure,
                    retry: PhysicalRetryPayload::WalAppend {
                        artifact,
                        range,
                        payload,
                    },
                },
                PhysicalEffectRecoveryObligation::Cleared,
            ),
            ScheduledArtifactAppendOutcome::Indeterminate(failure) => (
                PhysicalExecutorOutcome::WalAppendIndeterminate(failure),
                PhysicalEffectRecoveryObligation::Retained,
            ),
        };
        let recovery = self.finish_effect_recovery(prepared, physical_recovery);
        Ok(PhysicalExecutorDispatch::new(dispatched, outcome, recovery))
    }
}
