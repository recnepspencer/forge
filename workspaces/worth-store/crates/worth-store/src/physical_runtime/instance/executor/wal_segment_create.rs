use worth_store_io_scheduler::execute_ready_queue_plan;
use worth_store_physical_backend::{
    BackendQueueExecutionAdaptation, ScheduledArtifactNewWriteOutcome,
};

use super::{recovery_obligation::scheduler_recovery, PhysicalWorkExecutor};
use crate::physical_runtime::work::{
    PhysicalRetryPayload, PhysicalWalSegmentCreateExecutorCommand,
};
use crate::physical_runtime::{
    PhysicalEffectRecoveryObligation, PhysicalExecutorDispatch, PhysicalExecutorOutcome,
};

impl PhysicalWorkExecutor {
    pub(super) fn dispatch_wal_segment_create(
        &self,
        command: PhysicalWalSegmentCreateExecutorCommand,
    ) -> Result<PhysicalExecutorDispatch, crate::physical_runtime::PhysicalWorkPreEffectDenial>
    {
        let PhysicalWalSegmentCreateExecutorCommand {
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
            .expect("WAL segment creation commands require WAL scope");
        let target = crate::physical_runtime::PhysicalWorkRecoveryTarget::WalArtifactInterval {
            segment: scope.segment(),
            generation: scope.generation(),
            offset: 0,
            byte_count: scope.byte_count(),
        };
        let (dispatched, plan) = work.into_execution_parts(Some(payload_digest))?;
        let prepared = self.prepare_effect_recovery(&dispatched, target, Some(payload_digest))?;
        let physical = self.media.artifact_tree().write_scheduled_new_exact(
            &artifact,
            range,
            &payload,
            plan.backend_completion_binding()
                .backend_execution_binding(),
            BackendQueueExecutionAdaptation::None,
        );
        let (outcome, physical_recovery) = match physical {
            ScheduledArtifactNewWriteOutcome::Completed(completed) => {
                let physical = completed.physical().clone();
                let scheduler = execute_ready_queue_plan(plan, completed.queue());
                let recovery = scheduler_recovery(&scheduler);
                (
                    PhysicalExecutorOutcome::WalSegmentCreateCompleted {
                        physical,
                        scheduler,
                    },
                    recovery,
                )
            }
            ScheduledArtifactNewWriteOutcome::DeniedBeforeEffect(failure) => (
                PhysicalExecutorOutcome::DeniedBeforeEffect {
                    failure,
                    retry: PhysicalRetryPayload::WalSegmentCreate {
                        artifact,
                        range,
                        payload,
                    },
                },
                PhysicalEffectRecoveryObligation::Cleared,
            ),
            ScheduledArtifactNewWriteOutcome::Indeterminate(failure) => (
                PhysicalExecutorOutcome::WalSegmentCreateIndeterminate(failure),
                PhysicalEffectRecoveryObligation::Retained,
            ),
        };
        let recovery = self.finish_effect_recovery(prepared, physical_recovery);
        Ok(PhysicalExecutorDispatch::new(dispatched, outcome, recovery))
    }
}
