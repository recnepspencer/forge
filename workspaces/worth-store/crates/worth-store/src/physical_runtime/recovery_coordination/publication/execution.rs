use worth_store_io_scheduler::{execute_ready_queue_plan, QueueExecutionOutcome};
use worth_store_physical_backend::ScheduledArtifactTreePublicationEffectOutcome;

use crate::physical_runtime::recovery_coordination::settlement::{
    settle, signal_completion_is_terminal,
};
#[cfg(feature = "certification-test-authority")]
use crate::physical_runtime::recovery_coordination::settlement::{
    settle_with_certification, PhysicalRecoverySettlementCertificationStage,
};
use crate::physical_runtime::work::{
    IndeterminatePhysicalPublicationEffect, PhysicalEffectRecoveryObligation,
    PhysicalExecutorDispatch, PhysicalExecutorOutcome, PhysicalRetryPayload,
};
use crate::physical_runtime::{
    PhysicalPublicationEffect, PhysicalWorkSchedulerPosture, PhysicalWorkScope,
};

use super::{
    CompletedPhysicalRecoveryPublicationCommand, PhysicalRecoveryPublicationCommand,
    PhysicalRecoveryPublicationCommandDenial, PhysicalRecoveryPublicationCommandDenialKind,
    PhysicalRecoveryPublicationCommandIndeterminate, PhysicalRecoveryPublicationCommandOutcome,
    PhysicalRecoveryPublicationCommandStage,
};
use crate::physical_runtime::recovery_coordination::{
    PerformedRecoveryPhysicalEffect, PhysicalRecoveryCoordination, RecoveryPublicationOccurrence,
    RecoveryRootProtocolReplacementAction,
};

mod admission;
mod candidate;

pub(super) fn execute(
    coordination: &PhysicalRecoveryCoordination,
    media: &worth_store_physical_backend::AdmittedRecoveryFilesystemMedia,
    command: PhysicalRecoveryPublicationCommand,
) -> PhysicalRecoveryPublicationCommandOutcome {
    let candidates = match candidate::materialize_all(coordination, media, &command) {
        Ok(candidates) => candidates,
        Err(outcome) => return outcome,
    };
    let (candidates, root_protocol) =
        match replace_root_protocol(coordination, media, &command, candidates) {
            Ok(completed) => completed,
            Err(outcome) => return outcome,
        };
    synchronize_record_namespace(coordination, media, command, candidates, root_protocol)
}

fn replace_root_protocol(
    coordination: &PhysicalRecoveryCoordination,
    media: &worth_store_physical_backend::AdmittedRecoveryFilesystemMedia,
    command: &PhysicalRecoveryPublicationCommand,
    candidates: Box<[super::CompletedPhysicalRecoveryPublicationCandidate]>,
) -> Result<
    (
        Box<[super::CompletedPhysicalRecoveryPublicationCandidate]>,
        PerformedRecoveryPhysicalEffect<RecoveryRootProtocolReplacementAction>,
    ),
    PhysicalRecoveryPublicationCommandOutcome,
> {
    let stage = PhysicalRecoveryPublicationCommandStage::RootProtocolReplacement;
    let artifact = command.protocol.catalog_candidate();
    let work = match admission::admit(coordination, stage, PhysicalWorkScope::artifact(artifact)) {
        Ok(work) => work,
        Err(outcome) => return Err(attach_candidates(outcome, candidates)),
    };
    let work_identity = work.intent().identity();
    let (dispatched, plan) = match work.into_execution_parts(None) {
        Ok(parts) => parts,
        Err(denial) => return Err(pre_effect(stage, denial, candidates, None)),
    };
    let physical = media.replace_recovery_root_protocol_scheduled(
        command.protocol,
        plan.backend_completion_binding()
            .backend_execution_binding(),
    );
    let completed = complete_effect(
        coordination,
        dispatched,
        plan,
        physical,
        stage,
        artifact,
        PhysicalPublicationEffect::ReplaceCatalog,
        candidates,
        None,
    )?;
    let performed =
        PerformedRecoveryPhysicalEffect::record_root_protocol(RecoveryPublicationOccurrence::new(
            coordination.session_identity(),
            command.plan,
            command.staging_generation,
            command.protocol.publication(),
            completed.physical,
            work_identity,
            completed.posture,
            completed.signal,
        ));
    Ok((completed.candidates, performed))
}

fn synchronize_record_namespace(
    coordination: &PhysicalRecoveryCoordination,
    media: &worth_store_physical_backend::AdmittedRecoveryFilesystemMedia,
    command: PhysicalRecoveryPublicationCommand,
    candidates: Box<[super::CompletedPhysicalRecoveryPublicationCandidate]>,
    root_protocol: PerformedRecoveryPhysicalEffect<RecoveryRootProtocolReplacementAction>,
) -> PhysicalRecoveryPublicationCommandOutcome {
    let stage = PhysicalRecoveryPublicationCommandStage::RecordNamespaceSynchronization;
    let artifact = command.protocol.catalog_candidate();
    let work = match admission::admit(coordination, stage, PhysicalWorkScope::artifact(artifact)) {
        Ok(work) => work,
        Err(outcome) => return attach_effects(outcome, candidates, root_protocol),
    };
    let work_identity = work.intent().identity();
    let (dispatched, plan) = match work.into_execution_parts(None) {
        Ok(parts) => parts,
        Err(denial) => {
            return pre_effect(stage, denial, candidates, Some(root_protocol));
        }
    };
    let physical = media.synchronize_recovery_record_namespace_scheduled(
        plan.backend_completion_binding()
            .backend_execution_binding(),
    );
    let completed = match complete_effect(
        coordination,
        dispatched,
        plan,
        physical,
        stage,
        artifact,
        PhysicalPublicationEffect::SynchronizeRecordFamily,
        candidates,
        Some(root_protocol),
    ) {
        Ok(completed) => completed,
        Err(outcome) => return outcome,
    };
    let root_protocol = completed
        .root_protocol
        .expect("namespace completion retains root-protocol authority");
    let record_namespace = PerformedRecoveryPhysicalEffect::record_record_namespace(
        RecoveryPublicationOccurrence::new(
            coordination.session_identity(),
            command.plan,
            command.staging_generation,
            command.protocol.publication(),
            completed.physical,
            work_identity,
            completed.posture,
            completed.signal,
        ),
    );
    PhysicalRecoveryPublicationCommandOutcome::Completed(
        CompletedPhysicalRecoveryPublicationCommand::new(
            completed.candidates,
            root_protocol,
            record_namespace,
        ),
    )
}

struct CompletedPublicationEffect {
    physical: worth_store_physical_backend::CompletedArtifactTreePublicationEffect,
    posture: PhysicalWorkSchedulerPosture,
    signal: crate::physical_runtime::PhysicalSignalSettlementOutcome,
    candidates: Box<[super::CompletedPhysicalRecoveryPublicationCandidate]>,
    root_protocol: Option<PerformedRecoveryPhysicalEffect<RecoveryRootProtocolReplacementAction>>,
}

#[allow(clippy::too_many_arguments)]
fn complete_effect(
    coordination: &PhysicalRecoveryCoordination,
    dispatched: crate::physical_runtime::DispatchedPhysicalWork,
    plan: worth_store_io_scheduler::QueueExecutionReadyPlan,
    physical: ScheduledArtifactTreePublicationEffectOutcome,
    stage: PhysicalRecoveryPublicationCommandStage,
    artifact: worth_store_physical_format::RecordArtifactFile,
    effect: PhysicalPublicationEffect,
    candidates: Box<[super::CompletedPhysicalRecoveryPublicationCandidate]>,
    root_protocol: Option<PerformedRecoveryPhysicalEffect<RecoveryRootProtocolReplacementAction>>,
) -> Result<CompletedPublicationEffect, PhysicalRecoveryPublicationCommandOutcome> {
    match physical {
        ScheduledArtifactTreePublicationEffectOutcome::Completed(completed) => {
            let physical = completed.physical().clone();
            let queue = completed.queue();
            #[cfg(feature = "certification-test-authority")]
            let queue = if coordination.take_certification_publication_scheduler_failure(stage) {
                queue.with_foreign_plan_binding_for_certification()
            } else {
                queue
            };
            let scheduler = execute_ready_queue_plan(plan, queue);
            let posture = scheduler_posture(&scheduler);
            let dispatch = PhysicalExecutorDispatch::new(
                dispatched,
                PhysicalExecutorOutcome::PublicationEffectCompleted {
                    physical: crate::physical_runtime::CompletedPhysicalPublicationEffect::new(
                        physical.clone(),
                        artifact,
                        effect,
                    ),
                    scheduler,
                },
                PhysicalEffectRecoveryObligation::Cleared,
            );
            #[cfg(feature = "certification-test-authority")]
            let signal = settle_with_certification(
                coordination,
                dispatch,
                PhysicalRecoverySettlementCertificationStage::Publication(stage),
            );
            #[cfg(not(feature = "certification-test-authority"))]
            let signal = settle(coordination, dispatch);
            if posture != PhysicalWorkSchedulerPosture::Executed {
                return Err(PhysicalRecoveryPublicationCommandOutcome::Indeterminate(
                    PhysicalRecoveryPublicationCommandIndeterminate::Scheduler {
                        stage,
                        physical,
                        candidates,
                        root_protocol,
                        posture,
                    },
                ));
            }
            if !signal_completion_is_terminal(signal) {
                return Err(PhysicalRecoveryPublicationCommandOutcome::Indeterminate(
                    PhysicalRecoveryPublicationCommandIndeterminate::Signal {
                        stage,
                        physical,
                        candidates,
                        root_protocol,
                        outcome: signal,
                    },
                ));
            }
            Ok(CompletedPublicationEffect {
                physical,
                posture,
                signal,
                candidates,
                root_protocol,
            })
        }
        ScheduledArtifactTreePublicationEffectOutcome::DeniedBeforeEffect(failure) => {
            let _ = settle(
                coordination,
                PhysicalExecutorDispatch::new(
                    dispatched,
                    PhysicalExecutorOutcome::DeniedBeforeEffect {
                        failure,
                        retry: PhysicalRetryPayload::RootPublicationEffect,
                    },
                    PhysicalEffectRecoveryObligation::Cleared,
                ),
            );
            Err(denied(
                stage,
                PhysicalRecoveryPublicationCommandDenialKind::Media(failure),
                candidates,
                root_protocol,
                None,
            ))
        }
        ScheduledArtifactTreePublicationEffectOutcome::Indeterminate(physical) => {
            let retained = physical.clone();
            let _ = settle(
                coordination,
                PhysicalExecutorDispatch::new(
                    dispatched,
                    PhysicalExecutorOutcome::PublicationEffectIndeterminate(
                        IndeterminatePhysicalPublicationEffect::new(physical, artifact, effect),
                    ),
                    PhysicalEffectRecoveryObligation::Retained,
                ),
            );
            Err(PhysicalRecoveryPublicationCommandOutcome::Indeterminate(
                PhysicalRecoveryPublicationCommandIndeterminate::Media {
                    stage,
                    physical: retained,
                    candidates,
                    root_protocol,
                },
            ))
        }
    }
}

fn scheduler_posture(outcome: &QueueExecutionOutcome) -> PhysicalWorkSchedulerPosture {
    if matches!(outcome, QueueExecutionOutcome::Executed(_)) {
        PhysicalWorkSchedulerPosture::Executed
    } else {
        PhysicalWorkSchedulerPosture::RejectedAfterEffect
    }
}

fn pre_effect(
    stage: PhysicalRecoveryPublicationCommandStage,
    denial: crate::physical_runtime::PhysicalWorkPreEffectDenial,
    candidates: Box<[super::CompletedPhysicalRecoveryPublicationCandidate]>,
    root_protocol: Option<PerformedRecoveryPhysicalEffect<RecoveryRootProtocolReplacementAction>>,
) -> PhysicalRecoveryPublicationCommandOutcome {
    denied(
        stage,
        PhysicalRecoveryPublicationCommandDenialKind::PreEffect(denial),
        candidates,
        root_protocol,
        None,
    )
}

fn denied(
    stage: PhysicalRecoveryPublicationCommandStage,
    denial: PhysicalRecoveryPublicationCommandDenialKind,
    candidates: Box<[super::CompletedPhysicalRecoveryPublicationCandidate]>,
    root_protocol: Option<PerformedRecoveryPhysicalEffect<RecoveryRootProtocolReplacementAction>>,
    scheduler: Option<PhysicalWorkSchedulerPosture>,
) -> PhysicalRecoveryPublicationCommandOutcome {
    PhysicalRecoveryPublicationCommandOutcome::DeniedBeforeEffect(
        PhysicalRecoveryPublicationCommandDenial::new(
            stage,
            denial,
            candidates,
            None,
            root_protocol,
            scheduler,
        ),
    )
}

fn attach_effects(
    outcome: PhysicalRecoveryPublicationCommandOutcome,
    candidates: Box<[super::CompletedPhysicalRecoveryPublicationCandidate]>,
    root_protocol: PerformedRecoveryPhysicalEffect<RecoveryRootProtocolReplacementAction>,
) -> PhysicalRecoveryPublicationCommandOutcome {
    match outcome {
        PhysicalRecoveryPublicationCommandOutcome::DeniedBeforeEffect(denial) => denied(
            denial.stage(),
            denial.denial(),
            candidates,
            Some(root_protocol),
            denial.scheduler_posture(),
        ),
        other => other,
    }
}

fn attach_candidates(
    outcome: PhysicalRecoveryPublicationCommandOutcome,
    candidates: Box<[super::CompletedPhysicalRecoveryPublicationCandidate]>,
) -> PhysicalRecoveryPublicationCommandOutcome {
    match outcome {
        PhysicalRecoveryPublicationCommandOutcome::DeniedBeforeEffect(denial) => {
            let scheduler = denial.scheduler_posture();
            denied(
                denial.stage(),
                denial.denial(),
                candidates,
                denial.root_protocol,
                scheduler,
            )
        }
        other => other,
    }
}
