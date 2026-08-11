use worth_store_io_scheduler::execute_ready_queue_plan;
use worth_store_physical_backend::{
    ArtifactRangeWriteDurabilityRequirement, RecoveryStagingSynchronizationOutcome,
    RecoveryStagingWriteDisposition, RecoveryStagingWriteOutcome,
};

use crate::physical_runtime::work::{
    PhysicalEffectRecoveryObligation, PhysicalExecutorDispatch, PhysicalExecutorOutcome,
    PhysicalRetryPayload,
};
use crate::physical_runtime::{
    PhysicalPublicationEffect, PhysicalWorkSchedulerPosture, PhysicalWorkScope,
};

use super::{
    CompletedPhysicalRecoveryStagingCommand, PhysicalRecoveryStagingCommand,
    PhysicalRecoveryStagingCommandDenialKind, PhysicalRecoveryStagingCommandIndeterminate,
    PhysicalRecoveryStagingCommandOutcome, PhysicalRecoveryStagingCommandStage,
    PhysicalRecoveryStagingMaterialization, PhysicalRecoveryStagingMaterializationEvidence,
};
use crate::physical_runtime::recovery_coordination::{
    PhysicalRecoveryCoordination, RecoveryStagingSynchronizationOccurrence,
    RecoveryStagingWriteOccurrence,
};

mod admission;
mod outcome;
use crate::physical_runtime::recovery_coordination::settlement::{
    scheduler_posture, settle, signal_completion_is_terminal,
};
#[cfg(feature = "certification-test-authority")]
use crate::physical_runtime::recovery_coordination::settlement::{
    settle_with_certification, PhysicalRecoverySettlementCertificationStage,
};
use outcome::{attach_materialization, denied, pre_effect};

pub(super) fn execute(
    coordination: &PhysicalRecoveryCoordination,
    media: &worth_store_physical_backend::AdmittedRecoveryFilesystemMedia,
    command: PhysicalRecoveryStagingCommand<'_>,
) -> PhysicalRecoveryStagingCommandOutcome {
    let materialization = match materialize(coordination, media, &command) {
        Ok(materialization) => materialization,
        Err(outcome) => return outcome,
    };
    synchronize(coordination, media, command, materialization)
}

fn materialize(
    coordination: &PhysicalRecoveryCoordination,
    media: &worth_store_physical_backend::AdmittedRecoveryFilesystemMedia,
    command: &PhysicalRecoveryStagingCommand<'_>,
) -> Result<PhysicalRecoveryStagingMaterialization, PhysicalRecoveryStagingCommandOutcome> {
    let coordinate = command_coordinate(command).ok_or_else(|| {
        denied(
            PhysicalRecoveryStagingCommandStage::Materialization,
            PhysicalRecoveryStagingCommandDenialKind::Submission,
            None,
            None,
        )
    })?;
    let work = admission::admit(
        coordination,
        PhysicalRecoveryStagingCommandStage::Materialization,
        PhysicalWorkScope::one(coordinate),
        ArtifactRangeWriteDurabilityRequirement::BufferedWrite,
        command.bytes.len() as u64,
        false,
    )?;
    let work_identity = work.intent().identity();
    let (dispatched, plan) = work
        .into_execution_parts(Some(command.payload_digest))
        .map_err(|denial| {
            pre_effect(PhysicalRecoveryStagingCommandStage::Materialization, denial)
        })?;
    match media.stage_recovery_artifact_scheduled(
        command.artifact,
        command.bytes,
        plan.backend_completion_binding()
            .backend_execution_binding(),
    ) {
        RecoveryStagingWriteOutcome::Completed(completed) => {
            let physical = completed.physical().clone();
            let scheduler = execute_ready_queue_plan(plan, completed.queue());
            let posture = scheduler_posture(&scheduler);
            let dispatch = PhysicalExecutorDispatch::new(
                dispatched,
                PhysicalExecutorOutcome::RecoveryStagingCompleted {
                    physical: physical.clone(),
                    scheduler,
                },
                PhysicalEffectRecoveryObligation::Cleared,
            );
            #[cfg(feature = "certification-test-authority")]
            let signal = settle_with_certification(
                coordination,
                dispatch,
                PhysicalRecoverySettlementCertificationStage::Staging(
                    PhysicalRecoveryStagingCommandStage::Materialization,
                ),
            );
            #[cfg(not(feature = "certification-test-authority"))]
            let signal = settle(coordination, dispatch);
            if posture != PhysicalWorkSchedulerPosture::Executed {
                return Err(PhysicalRecoveryStagingCommandOutcome::Indeterminate(
                    PhysicalRecoveryStagingCommandIndeterminate::Scheduler {
                        stage: PhysicalRecoveryStagingCommandStage::Materialization,
                        materialization: Some(
                            PhysicalRecoveryStagingMaterializationEvidence::PhysicallyCompleted(
                                physical,
                            ),
                        ),
                        synchronization: None,
                        posture,
                    },
                ));
            }
            if !signal_completion_is_terminal(signal) {
                return Err(PhysicalRecoveryStagingCommandOutcome::Indeterminate(
                    PhysicalRecoveryStagingCommandIndeterminate::Signal {
                        stage: PhysicalRecoveryStagingCommandStage::Materialization,
                        materialization: Some(
                            PhysicalRecoveryStagingMaterializationEvidence::PhysicallyCompleted(
                                physical,
                            ),
                        ),
                        synchronization: None,
                        outcome: signal,
                    },
                ));
            }
            Ok(match physical.disposition() {
                RecoveryStagingWriteDisposition::Created => {
                    PhysicalRecoveryStagingMaterialization::Created(
                        super::super::PerformedRecoveryPhysicalEffect::record_write(
                            RecoveryStagingWriteOccurrence::new(
                                coordination.session_identity(),
                                command.plan,
                                command.staging_generation,
                                command.ordinal,
                                physical,
                                work_identity,
                                posture,
                                signal,
                            ),
                        ),
                    )
                }
                RecoveryStagingWriteDisposition::AlreadyMaterialized => {
                    PhysicalRecoveryStagingMaterialization::AlreadyMaterialized(physical)
                }
            })
        }
        RecoveryStagingWriteOutcome::DeniedBeforeEffect(failure) => {
            let scheduler = failure
                .queue()
                .map(|queue| execute_ready_queue_plan(plan, queue));
            let posture = scheduler.as_ref().map(scheduler_posture);
            let physical_failure = failure.failure();
            let _ = settle(
                coordination,
                PhysicalExecutorDispatch::new(
                    dispatched,
                    PhysicalExecutorOutcome::DeniedBeforeEffect {
                        failure: physical_failure,
                        retry: PhysicalRetryPayload::NewArtifact(command.bytes.into()),
                    },
                    PhysicalEffectRecoveryObligation::Cleared,
                ),
            );
            Err(denied(
                PhysicalRecoveryStagingCommandStage::Materialization,
                PhysicalRecoveryStagingCommandDenialKind::Media(physical_failure),
                None,
                posture,
            ))
        }
        RecoveryStagingWriteOutcome::Indeterminate(physical) => {
            let scheduler = execute_ready_queue_plan(plan, physical.queue());
            let posture = scheduler_posture(&scheduler);
            let retained = physical.physical().clone();
            let _ = settle(
                coordination,
                PhysicalExecutorDispatch::new(
                    dispatched,
                    PhysicalExecutorOutcome::RecoveryStagingIndeterminate(
                        physical.physical().clone(),
                    ),
                    PhysicalEffectRecoveryObligation::Retained,
                ),
            );
            Err(PhysicalRecoveryStagingCommandOutcome::Indeterminate(
                PhysicalRecoveryStagingCommandIndeterminate::Materialization {
                    physical: retained,
                    scheduler: Some(posture),
                },
            ))
        }
    }
}

fn synchronize(
    coordination: &PhysicalRecoveryCoordination,
    media: &worth_store_physical_backend::AdmittedRecoveryFilesystemMedia,
    command: PhysicalRecoveryStagingCommand<'_>,
    materialization: PhysicalRecoveryStagingMaterialization,
) -> PhysicalRecoveryStagingCommandOutcome {
    let work = match admission::admit(
        coordination,
        PhysicalRecoveryStagingCommandStage::Synchronization,
        PhysicalWorkScope::artifact(command.artifact),
        ArtifactRangeWriteDurabilityRequirement::FileDataSynchronization,
        0,
        true,
    ) {
        Ok(work) => work,
        Err(outcome) => return attach_materialization(outcome, materialization),
    };
    let work_identity = work.intent().identity();
    let (dispatched, plan) = match work.into_execution_parts(None) {
        Ok(parts) => parts,
        Err(denial) => {
            return attach_materialization(
                pre_effect(PhysicalRecoveryStagingCommandStage::Synchronization, denial),
                materialization,
            )
        }
    };
    match media.synchronize_recovery_artifact_scheduled(
        command.artifact,
        plan.backend_completion_binding()
            .backend_execution_binding(),
    ) {
        RecoveryStagingSynchronizationOutcome::Completed(completed) => {
            let physical = completed.physical().clone();
            let scheduler = execute_ready_queue_plan(plan, completed.queue());
            let posture = scheduler_posture(&scheduler);
            let dispatch = PhysicalExecutorDispatch::new(
                dispatched,
                PhysicalExecutorOutcome::PublicationEffectCompleted {
                    physical: crate::physical_runtime::CompletedPhysicalPublicationEffect::new(
                        physical.clone(),
                        command.artifact,
                        PhysicalPublicationEffect::SynchronizeArtifact,
                    ),
                    scheduler,
                },
                PhysicalEffectRecoveryObligation::Cleared,
            );
            #[cfg(feature = "certification-test-authority")]
            let signal = settle_with_certification(
                coordination,
                dispatch,
                PhysicalRecoverySettlementCertificationStage::Staging(
                    PhysicalRecoveryStagingCommandStage::Synchronization,
                ),
            );
            #[cfg(not(feature = "certification-test-authority"))]
            let signal = settle(coordination, dispatch);
            if posture != PhysicalWorkSchedulerPosture::Executed {
                return PhysicalRecoveryStagingCommandOutcome::Indeterminate(
                    PhysicalRecoveryStagingCommandIndeterminate::Scheduler {
                        stage: PhysicalRecoveryStagingCommandStage::Synchronization,
                        materialization: Some(
                            PhysicalRecoveryStagingMaterializationEvidence::Performed(
                                materialization,
                            ),
                        ),
                        synchronization: Some(physical),
                        posture,
                    },
                );
            }
            if !signal_completion_is_terminal(signal) {
                return PhysicalRecoveryStagingCommandOutcome::Indeterminate(
                    PhysicalRecoveryStagingCommandIndeterminate::Signal {
                        stage: PhysicalRecoveryStagingCommandStage::Synchronization,
                        materialization: Some(
                            PhysicalRecoveryStagingMaterializationEvidence::Performed(
                                materialization,
                            ),
                        ),
                        synchronization: Some(physical),
                        outcome: signal,
                    },
                );
            }
            let performed = super::super::PerformedRecoveryPhysicalEffect::record_synchronization(
                RecoveryStagingSynchronizationOccurrence::new(
                    coordination.session_identity(),
                    command.plan,
                    command.staging_generation,
                    command.ordinal,
                    physical,
                    work_identity,
                    posture,
                    signal,
                ),
            );
            PhysicalRecoveryStagingCommandOutcome::Completed(
                CompletedPhysicalRecoveryStagingCommand::new(materialization, performed),
            )
        }
        RecoveryStagingSynchronizationOutcome::DeniedBeforeEffect(failure) => {
            let scheduler = failure
                .queue()
                .map(|queue| execute_ready_queue_plan(plan, queue));
            let posture = scheduler.as_ref().map(scheduler_posture);
            let physical_failure = failure.failure();
            let _ = settle(
                coordination,
                PhysicalExecutorDispatch::new(
                    dispatched,
                    PhysicalExecutorOutcome::DeniedBeforeEffect {
                        failure: physical_failure,
                        retry: PhysicalRetryPayload::PublicationEffect(
                            PhysicalPublicationEffect::SynchronizeArtifact,
                        ),
                    },
                    PhysicalEffectRecoveryObligation::Cleared,
                ),
            );
            denied(
                PhysicalRecoveryStagingCommandStage::Synchronization,
                PhysicalRecoveryStagingCommandDenialKind::Media(physical_failure),
                Some(materialization),
                posture,
            )
        }
        RecoveryStagingSynchronizationOutcome::Indeterminate(physical) => {
            let scheduler = execute_ready_queue_plan(plan, physical.queue());
            let posture = scheduler_posture(&scheduler);
            let retained = physical.physical().clone();
            let _ = settle(
                coordination,
                PhysicalExecutorDispatch::new(
                    dispatched,
                    PhysicalExecutorOutcome::PublicationEffectIndeterminate(
                        crate::physical_runtime::work::IndeterminatePhysicalPublicationEffect::new(
                            physical.physical().clone(),
                            command.artifact,
                            PhysicalPublicationEffect::SynchronizeArtifact,
                        ),
                    ),
                    PhysicalEffectRecoveryObligation::Retained,
                ),
            );
            PhysicalRecoveryStagingCommandOutcome::Indeterminate(
                PhysicalRecoveryStagingCommandIndeterminate::Synchronization {
                    physical: retained,
                    materialization,
                    scheduler: Some(posture),
                },
            )
        }
    }
}

fn command_coordinate(
    command: &PhysicalRecoveryStagingCommand<'_>,
) -> Option<worth_store_physical_format::RecordFrameCoordinate> {
    u32::try_from(command.bytes.len()).ok().and_then(|length| {
        worth_store_physical_format::RecordFrameCoordinate::new(command.artifact, 0, length)
    })
}
