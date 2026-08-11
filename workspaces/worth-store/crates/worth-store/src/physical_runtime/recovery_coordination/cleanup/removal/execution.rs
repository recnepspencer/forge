use worth_store_io_scheduler::execute_ready_queue_plan;
use worth_store_physical_backend::{
    RecoveryCleanupRemovalOutcome as BackendRemovalOutcome, RecoveryWalArtifactCoordinate,
};

use crate::physical_runtime::recovery_coordination::settlement::{
    scheduler_posture, settle, signal_completion_is_terminal,
};
use crate::physical_runtime::work::{
    CompletedPhysicalWalReclamationAction, IndeterminatePhysicalWalReclamationAction,
    PhysicalEffectRecoveryObligation, PhysicalExecutorDispatch, PhysicalExecutorOutcome,
    PhysicalRetryPayload, PhysicalWalReclamationScope,
};
use crate::physical_runtime::{PhysicalRecoveryCoordination, PhysicalWorkSchedulerPosture};

use super::*;
use crate::physical_runtime::recovery_coordination::{
    RecoveryCleanupRemovalBinding, RecoveryCleanupRemovalOccurrence,
    RecoveryCleanupRemovalSettlement, RecoveryCleanupRemovalTarget,
};

struct RemovalExecution {
    dispatched: crate::physical_runtime::DispatchedPhysicalWork,
    plan: worth_store_io_scheduler::QueueExecutionReadyPlan,
    work: crate::physical_runtime::PhysicalWorkIdentity,
    coordinate: RecoveryWalArtifactCoordinate,
}

struct CompletedRemovalSettlement {
    physical: worth_store_physical_backend::CompletedArtifactTreePublicationEffect,
    work: crate::physical_runtime::PhysicalWorkIdentity,
    coordinate: RecoveryWalArtifactCoordinate,
    posture: PhysicalWorkSchedulerPosture,
    signal: crate::physical_runtime::PhysicalSignalSettlementOutcome,
}

pub(in crate::physical_runtime::recovery_coordination::cleanup) fn execute(
    coordination: &PhysicalRecoveryCoordination,
    media: &worth_store_physical_backend::AdmittedRecoveryFilesystemMedia,
    command: PhysicalRecoveryCleanupRemovalCommand,
) -> PhysicalRecoveryCleanupRemovalOutcome {
    let execution = match admit_execution(coordination, &command) {
        Ok(execution) => execution,
        Err(outcome) => return outcome,
    };
    match media.remove_recovery_wal_artifact_scheduled(
        execution.coordinate,
        execution
            .plan
            .backend_completion_binding()
            .backend_execution_binding(),
    ) {
        BackendRemovalOutcome::Completed(completed) => {
            complete_removal(coordination, command, execution, *completed)
        }
        BackendRemovalOutcome::DeniedBeforeEffect(denied) => {
            deny_removal(coordination, execution, denied)
        }
        BackendRemovalOutcome::Indeterminate(indeterminate) => {
            indeterminate_removal(coordination, command, execution, indeterminate)
        }
    }
}

fn admit_execution(
    coordination: &PhysicalRecoveryCoordination,
    command: &PhysicalRecoveryCleanupRemovalCommand,
) -> Result<RemovalExecution, PhysicalRecoveryCleanupRemovalOutcome> {
    let scope = validated_scope(command).ok_or_else(invalid_command_outcome)?;
    let work =
        super::super::admission::removal(coordination, scope).map_err(admission_denial_outcome)?;
    let work_identity = work.intent().identity();
    let (dispatched, plan) = work
        .into_execution_parts(None)
        .map_err(|denial| execution_denial_outcome(denial, work_identity))?;
    let coordinate = RecoveryWalArtifactCoordinate::new(
        command.artifact.segment().get(),
        command.artifact.generation().get(),
    )
    .expect("admitted WAL identity");
    Ok(RemovalExecution {
        dispatched,
        plan,
        work: work_identity,
        coordinate,
    })
}

fn validated_scope(
    command: &PhysicalRecoveryCleanupRemovalCommand,
) -> Option<PhysicalWalReclamationScope> {
    PhysicalWalReclamationScope::new(
        command.checkpoint,
        command.compaction_generation,
        command.compaction_digest,
        command.retained_boundary,
        command.artifact,
        command.lsn_range,
        command.byte_count,
    )
}

fn invalid_command_outcome() -> PhysicalRecoveryCleanupRemovalOutcome {
    denied_without_physical(
        PhysicalRecoveryCleanupRemovalDenialKind::InvalidCommand,
        None,
    )
}

fn admission_denial_outcome(
    denial: super::super::admission::PhysicalRecoveryCleanupAdmissionDenial,
) -> PhysicalRecoveryCleanupRemovalOutcome {
    denied_without_physical(
        PhysicalRecoveryCleanupRemovalDenialKind::Admission(denial),
        None,
    )
}

fn execution_denial_outcome(
    denial: crate::physical_runtime::PhysicalWorkPreEffectDenial,
    work: crate::physical_runtime::PhysicalWorkIdentity,
) -> PhysicalRecoveryCleanupRemovalOutcome {
    denied_without_physical(
        PhysicalRecoveryCleanupRemovalDenialKind::Execution(denial),
        Some(work),
    )
}

fn denied_without_physical(
    kind: PhysicalRecoveryCleanupRemovalDenialKind,
    work: Option<crate::physical_runtime::PhysicalWorkIdentity>,
) -> PhysicalRecoveryCleanupRemovalOutcome {
    PhysicalRecoveryCleanupRemovalOutcome::DeniedBeforeEffect(
        PhysicalRecoveryCleanupRemovalDenial {
            kind,
            physical: None,
            work,
            scheduler: None,
            signal: None,
        },
    )
}

fn complete_removal(
    coordination: &PhysicalRecoveryCoordination,
    command: PhysicalRecoveryCleanupRemovalCommand,
    execution: RemovalExecution,
    completed: worth_store_physical_backend::CompletedScheduledRecoveryCleanupRemoval,
) -> PhysicalRecoveryCleanupRemovalOutcome {
    let settlement = settle_completed_removal(coordination, &command, execution, completed);
    if settlement.posture != PhysicalWorkSchedulerPosture::Executed {
        return PhysicalRecoveryCleanupRemovalOutcome::Indeterminate(
            PhysicalRecoveryCleanupRemovalIndeterminate::Scheduler {
                physical: settlement.physical,
                posture: settlement.posture,
                signal: settlement.signal,
            },
        );
    }
    if !signal_completion_is_terminal(settlement.signal) {
        return PhysicalRecoveryCleanupRemovalOutcome::Indeterminate(
            PhysicalRecoveryCleanupRemovalIndeterminate::Signal {
                physical: settlement.physical,
                posture: settlement.posture,
                outcome: settlement.signal,
            },
        );
    }
    let performed = PerformedRecoveryPhysicalEffect::record_cleanup_removal(
        RecoveryCleanupRemovalOccurrence::new(
            RecoveryCleanupRemovalBinding::new(
                coordination.session_identity(),
                command.plan,
                command.published_generation,
                command.checkpoint,
            ),
            RecoveryCleanupRemovalTarget::new(
                settlement.coordinate,
                command.lsn_range,
                command.byte_count,
            ),
            RecoveryCleanupRemovalSettlement::new(
                settlement.physical,
                settlement.work,
                settlement.posture,
                settlement.signal,
            ),
        ),
    );
    PhysicalRecoveryCleanupRemovalOutcome::Completed(CompletedPhysicalRecoveryCleanupRemoval {
        performed,
    })
}

fn settle_completed_removal(
    coordination: &PhysicalRecoveryCoordination,
    command: &PhysicalRecoveryCleanupRemovalCommand,
    execution: RemovalExecution,
    completed: worth_store_physical_backend::CompletedScheduledRecoveryCleanupRemoval,
) -> CompletedRemovalSettlement {
    let physical = completed.physical().clone();
    let queue = completed.queue();
    #[cfg(feature = "certification-test-authority")]
    let queue = if coordination.take_certification_cleanup_scheduler_failure(
        super::super::PhysicalRecoveryCleanupCommandStage::Removal,
    ) {
        queue.with_foreign_plan_binding_for_certification()
    } else {
        queue
    };
    let scheduler = execute_ready_queue_plan(execution.plan, queue);
    let posture = scheduler_posture(&scheduler);
    let dispatch = PhysicalExecutorDispatch::new(
        execution.dispatched,
        PhysicalExecutorOutcome::WalReclamationCompleted {
            physical: CompletedPhysicalWalReclamationAction::new(
                command.checkpoint,
                command.artifact,
                command.lsn_range,
                command.byte_count,
                physical.operation(),
            ),
            scheduler,
        },
        PhysicalEffectRecoveryObligation::Cleared,
    );
    CompletedRemovalSettlement {
        physical,
        work: execution.work,
        coordinate: execution.coordinate,
        posture,
        signal: settle(coordination, dispatch),
    }
}

fn deny_removal(
    coordination: &PhysicalRecoveryCoordination,
    execution: RemovalExecution,
    denied: Box<worth_store_physical_backend::DeniedScheduledRecoveryCleanupRemoval>,
) -> PhysicalRecoveryCleanupRemovalOutcome {
    let scheduler = denied
        .queue()
        .map(|queue| scheduler_posture(&execute_ready_queue_plan(execution.plan, queue)));
    let signal = settle(
        coordination,
        PhysicalExecutorDispatch::new(
            execution.dispatched,
            PhysicalExecutorOutcome::DeniedBeforeEffect {
                failure: denied.failure(),
                retry: PhysicalRetryPayload::WalReclamation,
            },
            PhysicalEffectRecoveryObligation::Cleared,
        ),
    );
    PhysicalRecoveryCleanupRemovalOutcome::DeniedBeforeEffect(
        PhysicalRecoveryCleanupRemovalDenial {
            kind: PhysicalRecoveryCleanupRemovalDenialKind::Media,
            physical: Some(*denied),
            work: Some(execution.work),
            scheduler,
            signal: Some(signal),
        },
    )
}

fn indeterminate_removal(
    coordination: &PhysicalRecoveryCoordination,
    command: PhysicalRecoveryCleanupRemovalCommand,
    execution: RemovalExecution,
    indeterminate: Box<worth_store_physical_backend::IndeterminateScheduledRecoveryCleanupRemoval>,
) -> PhysicalRecoveryCleanupRemovalOutcome {
    let scheduler = execute_ready_queue_plan(execution.plan, indeterminate.queue());
    let posture = scheduler_posture(&scheduler);
    let physical = indeterminate.physical();
    let signal = settle(
        coordination,
        PhysicalExecutorDispatch::new(
            execution.dispatched,
            PhysicalExecutorOutcome::WalReclamationIndeterminate(
                IndeterminatePhysicalWalReclamationAction::new(
                    command.checkpoint,
                    command.artifact,
                    physical.operation(),
                    physical.failure(),
                ),
            ),
            PhysicalEffectRecoveryObligation::Cleared,
        ),
    );
    PhysicalRecoveryCleanupRemovalOutcome::Indeterminate(
        PhysicalRecoveryCleanupRemovalIndeterminate::Media {
            physical: *indeterminate,
            scheduler: posture,
            signal,
        },
    )
}
