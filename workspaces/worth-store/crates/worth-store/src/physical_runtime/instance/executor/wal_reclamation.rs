use worth_store_io_scheduler::execute_ready_queue_plan;
use worth_store_physical_backend::{
    ArtifactTreeDirectory, ArtifactTreeFile, BackendQueueExecutionAdaptation,
    ScheduledArtifactTreePublicationEffectOutcome,
};

use super::{recovery_obligation::scheduler_recovery, PhysicalWorkExecutor};
use crate::physical_runtime::work::{
    CompletedPhysicalWalReclamationAction, IndeterminatePhysicalWalReclamationAction,
    PhysicalRetryPayload, PhysicalWalReclamationExecutorCommand,
};
use crate::physical_runtime::{
    PhysicalEffectRecoveryObligation, PhysicalExecutorDispatch, PhysicalExecutorOutcome,
    PhysicalWorkRecoveryTarget,
};

impl PhysicalWorkExecutor {
    pub(super) fn dispatch_wal_reclamation(
        &self,
        command: PhysicalWalReclamationExecutorCommand,
    ) -> Result<PhysicalExecutorDispatch, crate::physical_runtime::PhysicalWorkPreEffectDenial>
    {
        let scope = reclamation_scope(&command);
        let target = recovery_target(scope);
        let (dispatched, plan) = command.work.into_execution_parts(None)?;
        let prepared = self.prepare_effect_recovery(&dispatched, target, None)?;
        let artifact = wal_artifact(scope);
        let physical = self.media.artifact_tree().remove_scheduled_file_durably(
            &artifact,
            plan.backend_completion_binding()
                .backend_execution_binding(),
            BackendQueueExecutionAdaptation::None,
        );
        let (outcome, recovery) = match physical {
            ScheduledArtifactTreePublicationEffectOutcome::Completed(completed) => {
                let operation = completed.physical().operation();
                let scheduler = execute_ready_queue_plan(plan, completed.queue());
                let recovery = scheduler_recovery(&scheduler);
                (
                    PhysicalExecutorOutcome::WalReclamationCompleted {
                        physical: CompletedPhysicalWalReclamationAction::new(
                            scope.checkpoint(),
                            scope.segment(),
                            scope.lsn_range(),
                            scope.byte_count(),
                            operation,
                        ),
                        scheduler,
                    },
                    recovery,
                )
            }
            ScheduledArtifactTreePublicationEffectOutcome::DeniedBeforeEffect(failure) => (
                PhysicalExecutorOutcome::DeniedBeforeEffect {
                    failure,
                    retry: PhysicalRetryPayload::WalReclamation,
                },
                PhysicalEffectRecoveryObligation::Cleared,
            ),
            ScheduledArtifactTreePublicationEffectOutcome::Indeterminate(physical) => (
                PhysicalExecutorOutcome::WalReclamationIndeterminate(
                    IndeterminatePhysicalWalReclamationAction::new(
                        scope.checkpoint(),
                        scope.segment(),
                        physical.operation(),
                        physical.failure(),
                    ),
                ),
                PhysicalEffectRecoveryObligation::Retained,
            ),
        };
        let recovery = self.finish_effect_recovery(prepared, recovery);
        Ok(PhysicalExecutorDispatch::new(dispatched, outcome, recovery))
    }
}

fn reclamation_scope(
    command: &PhysicalWalReclamationExecutorCommand,
) -> crate::physical_runtime::work::PhysicalWalReclamationScope {
    command
        .work
        .intent()
        .scope()
        .wal_reclamation_target()
        .expect("WAL reclamation commands carry exact reclamation scope")
}

fn recovery_target(
    scope: crate::physical_runtime::work::PhysicalWalReclamationScope,
) -> PhysicalWorkRecoveryTarget {
    PhysicalWorkRecoveryTarget::WalSegmentReclamation {
        segment: scope.segment().segment().get(),
        generation: scope.segment().generation().get(),
    }
}

fn wal_artifact(
    scope: crate::physical_runtime::work::PhysicalWalReclamationScope,
) -> ArtifactTreeFile {
    ArtifactTreeDirectory::families()
        .child("wal")
        .expect("the Store-owned WAL directory is portable")
        .file(&scope.segment().file_name())
        .expect("canonical WAL artifact names are portable")
}
