use worth_store_io_scheduler::QueueExecutionOutcome;
use worth_store_physical_backend::{
    ArtifactTreeFailureKind, CompletedArtifactNewWrite, CompletedArtifactRangeRead,
    CompletedArtifactRangeWrite, IndeterminateArtifactNewWrite, IndeterminateArtifactRangeWrite,
    MediaOperationRole,
};

use super::{
    publication_effect_role, PhysicalWorkEffectFate, PhysicalWorkHealthRevocation,
    PhysicalWorkNoEffectEvidence, PhysicalWorkPublicationResiduePosture,
    PhysicalWorkSchedulerPosture, PhysicalWorkSettlementEvidence, PhysicalWorkTerminalCause,
    PhysicalWorkTerminalFailure,
};
use crate::physical_runtime::work::{
    DispatchedPhysicalWork, PhysicalExecutorOutcome, PhysicalWorkOperationFamily,
    PhysicalWorkRecoveryDisposition, PhysicalWorkRecoveryTarget,
};

pub(super) fn classify(
    dispatched: &DispatchedPhysicalWork,
    outcome: PhysicalExecutorOutcome,
) -> PhysicalWorkSettlementEvidence {
    match outcome {
        PhysicalExecutorOutcome::DeniedBeforeEffect { failure, retry } => {
            PhysicalWorkSettlementEvidence::NoEffect(PhysicalWorkNoEffectEvidence {
                failure,
                retry,
            })
        }
        PhysicalExecutorOutcome::MetadataCompleted {
            physical,
            scheduler,
        } if dispatched.matches_metadata(physical) => PhysicalWorkSettlementEvidence::Metadata {
            physical,
            scheduler,
        },
        PhysicalExecutorOutcome::ReadCompleted {
            physical,
            bytes,
            scheduler,
        } if dispatched.matches_read(physical) => {
            classify_completed_read(dispatched, physical, bytes, scheduler)
        }
        PhysicalExecutorOutcome::WriteCompleted {
            physical,
            scheduler,
        } if dispatched.matches_write(&physical) => {
            classify_completed_write(dispatched, physical, scheduler, false)
        }
        PhysicalExecutorOutcome::ResidencyWritebackCompleted {
            physical,
            scheduler,
        } if dispatched.matches_write(&physical) => PhysicalWorkSettlementEvidence::Write {
            physical,
            scheduler,
        },
        PhysicalExecutorOutcome::PublicationCompleted {
            physical,
            scheduler,
        } if dispatched.matches_write(&physical) => {
            classify_completed_write(dispatched, physical, scheduler, true)
        }
        PhysicalExecutorOutcome::NewArtifactCompleted {
            physical,
            scheduler,
        } if dispatched.matches_new_artifact(&physical) => {
            classify_completed_new_artifact(dispatched, physical, scheduler)
        }
        PhysicalExecutorOutcome::PublicationEffectCompleted {
            physical,
            scheduler,
        } if dispatched.matches_publication_effect(&physical) => {
            classify_completed_publication_effect(dispatched, physical, scheduler)
        }
        PhysicalExecutorOutcome::Indeterminate(physical)
            if dispatched.matches_indeterminate(physical) =>
        {
            indeterminate_terminal(dispatched, physical)
        }
        PhysicalExecutorOutcome::NewArtifactIndeterminate(physical)
            if dispatched.matches_new_artifact_indeterminate(physical) =>
        {
            indeterminate_new_artifact(dispatched, physical)
        }
        PhysicalExecutorOutcome::PublicationEffectIndeterminate(physical)
            if dispatched.matches_publication_effect_indeterminate(&physical) =>
        {
            indeterminate_publication_effect(dispatched, physical)
        }
        _ => PhysicalWorkSettlementEvidence::StaleOrForeign,
    }
}

fn classify_completed_read(
    dispatched: &DispatchedPhysicalWork,
    physical: CompletedArtifactRangeRead,
    bytes: Box<[u8]>,
    scheduler: QueueExecutionOutcome,
) -> PhysicalWorkSettlementEvidence {
    let expected = u64::from(physical.coordinate().length());
    if physical.completed_bytes() == expected && bytes.len() as u64 == expected {
        return PhysicalWorkSettlementEvidence::Read {
            physical,
            bytes,
            scheduler,
        };
    }
    PhysicalWorkSettlementEvidence::TerminalFailure(PhysicalWorkTerminalFailure {
        identity: dispatched.intent().identity(),
        effect_fate: PhysicalWorkEffectFate::ReadIncomplete,
        target: PhysicalWorkRecoveryTarget::Range(physical.coordinate()),
        completed_bytes: physical.completed_bytes(),
        backend_operation: physical.operation(),
        backend_role: MediaOperationRole::PositionedRead,
        scheduler: scheduler_posture(scheduler),
        publication_residue: PhysicalWorkPublicationResiduePosture::NotApplicable,
        recovery: PhysicalWorkRecoveryDisposition::InspectionRequired,
        cause: PhysicalWorkTerminalCause::IncompleteRead {
            expected,
            completed: physical.completed_bytes(),
        },
    })
}

fn indeterminate_terminal(
    dispatched: &DispatchedPhysicalWork,
    physical: IndeterminateArtifactRangeWrite,
) -> PhysicalWorkSettlementEvidence {
    PhysicalWorkSettlementEvidence::TerminalFailure(PhysicalWorkTerminalFailure {
        identity: dispatched.intent().identity(),
        effect_fate: PhysicalWorkEffectFate::Indeterminate,
        target: PhysicalWorkRecoveryTarget::Range(physical.coordinate()),
        completed_bytes: physical.completed_bytes(),
        backend_operation: physical.operation(),
        backend_role: MediaOperationRole::PositionedWrite,
        scheduler: PhysicalWorkSchedulerPosture::NotObserved,
        publication_residue: indeterminate_publication_residue(dispatched),
        recovery: PhysicalWorkRecoveryDisposition::InspectionRequired,
        cause: PhysicalWorkTerminalCause::Backend(physical.failure()),
    })
}

pub(super) fn health_revocation(
    dispatched: &DispatchedPhysicalWork,
    evidence: &PhysicalWorkSettlementEvidence,
) -> Option<PhysicalWorkHealthRevocation> {
    let fate = evidence.fate();
    requires_health_revocation(evidence, fate).then_some(PhysicalWorkHealthRevocation {
        identity: dispatched.intent().identity(),
        fate,
        recovery: PhysicalWorkRecoveryDisposition::InspectionRequired,
    })
}

fn requires_health_revocation(
    evidence: &PhysicalWorkSettlementEvidence,
    fate: PhysicalWorkEffectFate,
) -> bool {
    matches!(
        fate,
        PhysicalWorkEffectFate::Indeterminate
            | PhysicalWorkEffectFate::WrittenButSchedulerRejected
            | PhysicalWorkEffectFate::StaleOrForeignOutcome
    ) || matches!(evidence, PhysicalWorkSettlementEvidence::TerminalFailure(_))
        || matches!(
            evidence,
            PhysicalWorkSettlementEvidence::NoEffect(evidence)
                if evidence.failure().kind() == ArtifactTreeFailureKind::Damaged
        )
}

fn classify_completed_write(
    dispatched: &DispatchedPhysicalWork,
    physical: CompletedArtifactRangeWrite,
    scheduler: QueueExecutionOutcome,
    publication: bool,
) -> PhysicalWorkSettlementEvidence {
    if matches!(scheduler, QueueExecutionOutcome::Executed(_)) {
        if publication {
            PhysicalWorkSettlementEvidence::Publication {
                physical,
                scheduler,
            }
        } else {
            PhysicalWorkSettlementEvidence::Write {
                physical,
                scheduler,
            }
        }
    } else {
        terminal_scheduler_failure(dispatched, physical, publication)
    }
}

fn terminal_scheduler_failure(
    dispatched: &DispatchedPhysicalWork,
    physical: CompletedArtifactRangeWrite,
    publication: bool,
) -> PhysicalWorkSettlementEvidence {
    PhysicalWorkSettlementEvidence::TerminalFailure(PhysicalWorkTerminalFailure {
        identity: dispatched.intent().identity(),
        effect_fate: PhysicalWorkEffectFate::WrittenButSchedulerRejected,
        target: PhysicalWorkRecoveryTarget::Range(physical.coordinate()),
        completed_bytes: physical.completed_bytes(),
        backend_operation: physical.operation(),
        backend_role: MediaOperationRole::PositionedWrite,
        scheduler: PhysicalWorkSchedulerPosture::RejectedAfterEffect,
        publication_residue: if publication {
            PhysicalWorkPublicationResiduePosture::MayExist
        } else {
            PhysicalWorkPublicationResiduePosture::NotApplicable
        },
        recovery: PhysicalWorkRecoveryDisposition::InspectionRequired,
        cause: PhysicalWorkTerminalCause::SchedulerRejectedAfterEffect,
    })
}

fn classify_completed_new_artifact(
    dispatched: &DispatchedPhysicalWork,
    physical: CompletedArtifactNewWrite,
    scheduler: QueueExecutionOutcome,
) -> PhysicalWorkSettlementEvidence {
    if matches!(scheduler, QueueExecutionOutcome::Executed(_)) {
        PhysicalWorkSettlementEvidence::NewArtifact {
            physical,
            scheduler,
        }
    } else {
        PhysicalWorkSettlementEvidence::TerminalFailure(PhysicalWorkTerminalFailure {
            identity: dispatched.intent().identity(),
            effect_fate: PhysicalWorkEffectFate::WrittenButSchedulerRejected,
            target: PhysicalWorkRecoveryTarget::Range(physical.write().coordinate()),
            completed_bytes: physical.write().completed_bytes(),
            backend_operation: physical.write().operation(),
            backend_role: MediaOperationRole::PositionedWrite,
            scheduler: PhysicalWorkSchedulerPosture::RejectedAfterEffect,
            publication_residue: PhysicalWorkPublicationResiduePosture::MayExist,
            recovery: PhysicalWorkRecoveryDisposition::InspectionRequired,
            cause: PhysicalWorkTerminalCause::SchedulerRejectedAfterEffect,
        })
    }
}

fn classify_completed_publication_effect(
    dispatched: &DispatchedPhysicalWork,
    physical: crate::physical_runtime::work::CompletedPhysicalPublicationEffect,
    scheduler: QueueExecutionOutcome,
) -> PhysicalWorkSettlementEvidence {
    if matches!(scheduler, QueueExecutionOutcome::Executed(_)) {
        PhysicalWorkSettlementEvidence::PublicationEffect {
            physical,
            scheduler,
        }
    } else {
        PhysicalWorkSettlementEvidence::TerminalFailure(PhysicalWorkTerminalFailure {
            identity: dispatched.intent().identity(),
            effect_fate: PhysicalWorkEffectFate::WrittenButSchedulerRejected,
            target: physical.recovery_target(),
            completed_bytes: 0,
            backend_operation: physical.physical().operation(),
            backend_role: publication_effect_role(physical.effect()),
            scheduler: PhysicalWorkSchedulerPosture::RejectedAfterEffect,
            publication_residue: PhysicalWorkPublicationResiduePosture::MayExist,
            recovery: PhysicalWorkRecoveryDisposition::InspectionRequired,
            cause: PhysicalWorkTerminalCause::SchedulerRejectedAfterEffect,
        })
    }
}

fn indeterminate_new_artifact(
    dispatched: &DispatchedPhysicalWork,
    physical: IndeterminateArtifactNewWrite,
) -> PhysicalWorkSettlementEvidence {
    let backend_role = if physical.write_operation().is_some() {
        MediaOperationRole::PositionedWrite
    } else {
        MediaOperationRole::CreateNew
    };
    PhysicalWorkSettlementEvidence::TerminalFailure(PhysicalWorkTerminalFailure {
        identity: dispatched.intent().identity(),
        effect_fate: PhysicalWorkEffectFate::Indeterminate,
        target: PhysicalWorkRecoveryTarget::Range(physical.coordinate()),
        completed_bytes: physical.completed_bytes(),
        backend_operation: physical
            .write_operation()
            .unwrap_or_else(|| physical.create_operation()),
        backend_role,
        scheduler: PhysicalWorkSchedulerPosture::NotObserved,
        publication_residue: PhysicalWorkPublicationResiduePosture::MayExist,
        recovery: PhysicalWorkRecoveryDisposition::InspectionRequired,
        cause: PhysicalWorkTerminalCause::Backend(physical.failure()),
    })
}

fn indeterminate_publication_effect(
    dispatched: &DispatchedPhysicalWork,
    physical: crate::physical_runtime::work::IndeterminatePhysicalPublicationEffect,
) -> PhysicalWorkSettlementEvidence {
    PhysicalWorkSettlementEvidence::TerminalFailure(PhysicalWorkTerminalFailure {
        identity: dispatched.intent().identity(),
        effect_fate: PhysicalWorkEffectFate::Indeterminate,
        target: physical.recovery_target(),
        completed_bytes: 0,
        backend_operation: physical.physical().operation(),
        backend_role: publication_effect_role(physical.effect()),
        scheduler: PhysicalWorkSchedulerPosture::NotObserved,
        publication_residue: PhysicalWorkPublicationResiduePosture::MayExist,
        recovery: PhysicalWorkRecoveryDisposition::InspectionRequired,
        cause: PhysicalWorkTerminalCause::Backend(physical.physical().failure()),
    })
}

fn scheduler_posture(scheduler: QueueExecutionOutcome) -> PhysicalWorkSchedulerPosture {
    if matches!(scheduler, QueueExecutionOutcome::Executed(_)) {
        PhysicalWorkSchedulerPosture::Executed
    } else {
        PhysicalWorkSchedulerPosture::RejectedAfterEffect
    }
}

fn indeterminate_publication_residue(
    dispatched: &DispatchedPhysicalWork,
) -> PhysicalWorkPublicationResiduePosture {
    if dispatched.intent().operation() == PhysicalWorkOperationFamily::ArtifactPublication {
        PhysicalWorkPublicationResiduePosture::MayExist
    } else {
        PhysicalWorkPublicationResiduePosture::NotApplicable
    }
}
