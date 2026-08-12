use worth_store_io_scheduler::execute_ready_queue_plan;
use worth_store_physical_backend::RecoveryCleanupRemovalOutcome as BackendRemovalOutcome;
use worth_store_wal::WalSegmentArtifactIdentity;

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
    artifact: WalSegmentArtifactIdentity,
}

struct CompletedRemovalSettlement {
    physical: worth_store_physical_backend::CompletedArtifactTreePublicationEffect,
    authorization: [u8; 32],
    revalidation: worth_store_physical_backend::RecoveryCleanupArtifactRevalidationProgress,
    work: crate::physical_runtime::PhysicalWorkIdentity,
    artifact: WalSegmentArtifactIdentity,
    posture: PhysicalWorkSchedulerPosture,
    signal: crate::physical_runtime::PhysicalSignalSettlementOutcome,
}

pub(in crate::physical_runtime::recovery_coordination::cleanup) fn execute(
    coordination: &PhysicalRecoveryCoordination,
    media: &worth_store_physical_backend::AdmittedRecoveryFilesystemMedia,
    command: PhysicalRecoveryCleanupRemovalCommand<'_>,
) -> PhysicalRecoveryCleanupRemovalOutcome {
    let execution = match admit_execution(coordination, media, &command) {
        Ok(execution) => execution,
        Err(outcome) => return outcome,
    };
    let authorization = authorize_cleanup_effect(coordination, &command, execution.work);
    match media.remove_recovery_wal_artifact_scheduled(
        &command.selector_read,
        command.checkpoint_stream,
        &command.verified_wal,
        authorization,
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

fn authorize_cleanup_effect(
    coordination: &PhysicalRecoveryCoordination,
    command: &PhysicalRecoveryCleanupRemovalCommand<'_>,
    work: crate::physical_runtime::PhysicalWorkIdentity,
) -> worth_store_authority::RecoveryCleanupEffectAuthorization {
    let inspection = command.verified_wal.inspection();
    #[cfg(feature = "certification-test-authority")]
    let artifact_generation =
        if coordination.take_certification_cleanup_authorization_substitution() {
            command.artifact.generation().get().saturating_add(1)
        } else {
            command.artifact.generation().get()
        };
    #[cfg(not(feature = "certification-test-authority"))]
    let artifact_generation = command.artifact.generation().get();
    let binding = worth_store_authority::RecoveryCleanupEffectBinding::new(
        command.store.bytes(),
        command.session,
        command.plan,
        command.published_generation,
        command.checkpoint.store_identity().bytes(),
        command.checkpoint.sequence().get(),
        command.artifact.segment().get(),
        artifact_generation,
        command.lsn_range.start().get(),
        command.lsn_range.end_exclusive().get(),
        command.byte_count,
        inspection.artifact_digest(),
        work.runtime().get(),
        work.generation().lifecycle().get(),
        work.operation().get(),
    )
    .expect("Store-admitted cleanup command has one complete nonzero effect binding");
    coordination.cleanup_effect_authority.authorize(binding)
}

fn admit_execution(
    coordination: &PhysicalRecoveryCoordination,
    media: &worth_store_physical_backend::AdmittedRecoveryFilesystemMedia,
    command: &PhysicalRecoveryCleanupRemovalCommand<'_>,
) -> Result<RemovalExecution, PhysicalRecoveryCleanupRemovalOutcome> {
    let scope =
        validated_scope(coordination, media, command).ok_or_else(invalid_command_outcome)?;
    let work =
        super::super::admission::removal(coordination, scope).map_err(admission_denial_outcome)?;
    let work_identity = work.intent().identity();
    let (dispatched, plan) = work
        .into_execution_parts(None)
        .map_err(|denial| execution_denial_outcome(denial, work_identity))?;
    Ok(RemovalExecution {
        dispatched,
        plan,
        work: work_identity,
        artifact: command.artifact,
    })
}

fn validated_scope(
    coordination: &PhysicalRecoveryCoordination,
    media: &worth_store_physical_backend::AdmittedRecoveryFilesystemMedia,
    command: &PhysicalRecoveryCleanupRemovalCommand<'_>,
) -> Option<PhysicalWalReclamationScope> {
    if command.store != media.store_identity()
        || command.media_generation != media.media_generation()
        || command.session != coordination.session_identity()
    {
        return None;
    }
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
    command: PhysicalRecoveryCleanupRemovalCommand<'_>,
    execution: RemovalExecution,
    completed: worth_store_physical_backend::CompletedScheduledRecoveryCleanupRemoval,
) -> PhysicalRecoveryCleanupRemovalOutcome {
    let settlement = settle_completed_removal(coordination, &command, execution, completed);
    if settlement.posture != PhysicalWorkSchedulerPosture::Executed {
        return PhysicalRecoveryCleanupRemovalOutcome::Indeterminate(
            PhysicalRecoveryCleanupRemovalIndeterminate::Scheduler {
                physical: settlement.physical,
                revalidation: settlement.revalidation,
                posture: settlement.posture,
                signal: settlement.signal,
            },
        );
    }
    if !signal_completion_is_terminal(settlement.signal) {
        return PhysicalRecoveryCleanupRemovalOutcome::Indeterminate(
            PhysicalRecoveryCleanupRemovalIndeterminate::Signal {
                physical: settlement.physical,
                revalidation: settlement.revalidation,
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
                settlement.artifact,
                command.lsn_range,
                command.byte_count,
            ),
            RecoveryCleanupRemovalSettlement::new(
                settlement.physical,
                settlement.authorization,
                settlement.work,
                settlement.posture,
                settlement.signal,
            ),
        ),
    );
    PhysicalRecoveryCleanupRemovalOutcome::Completed(CompletedPhysicalRecoveryCleanupRemoval {
        performed,
        revalidation: settlement.revalidation,
    })
}

fn settle_completed_removal(
    coordination: &PhysicalRecoveryCoordination,
    command: &PhysicalRecoveryCleanupRemovalCommand<'_>,
    execution: RemovalExecution,
    completed: worth_store_physical_backend::CompletedScheduledRecoveryCleanupRemoval,
) -> CompletedRemovalSettlement {
    let physical = completed.physical().clone();
    let authorization = completed.authorization();
    let revalidation = completed.revalidation();
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
        authorization,
        revalidation,
        work: execution.work,
        artifact: execution.artifact,
        posture,
        signal: settle(coordination, dispatch),
    }
}

fn deny_removal(
    coordination: &PhysicalRecoveryCoordination,
    execution: RemovalExecution,
    denied: Box<worth_store_physical_backend::DeniedScheduledRecoveryCleanupRemoval>,
) -> PhysicalRecoveryCleanupRemovalOutcome {
    let cause = denied.cause();
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
            kind: PhysicalRecoveryCleanupRemovalDenialKind::Media(cause),
            physical: Some(*denied),
            work: Some(execution.work),
            scheduler,
            signal: Some(signal),
        },
    )
}

fn indeterminate_removal(
    coordination: &PhysicalRecoveryCoordination,
    command: PhysicalRecoveryCleanupRemovalCommand<'_>,
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
