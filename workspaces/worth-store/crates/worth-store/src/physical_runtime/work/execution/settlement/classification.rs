use worth_store_io_scheduler::QueueExecutionOutcome;
use worth_store_physical_backend::{
    ArtifactTreeFailureKind, CompletedArtifactAppend, CompletedArtifactNewWrite,
    CompletedArtifactRangeRead, CompletedArtifactRangeWrite, IndeterminateArtifactAppend,
    IndeterminateArtifactNewWrite, IndeterminateArtifactRangeWrite, MediaOperationRole,
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
        PhysicalExecutorOutcome::WalAppendCompleted {
            physical,
            scheduler,
        } if dispatched.matches_wal_append(&physical) => {
            classify_completed_wal_append(dispatched, physical, scheduler)
        }
        PhysicalExecutorOutcome::WalBarrierCompleted {
            physical,
            scheduler,
        } if dispatched.matches_wal_barrier(&physical) => {
            classify_completed_wal_barrier(dispatched, physical, scheduler)
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
        PhysicalExecutorOutcome::WalAppendIndeterminate(physical)
            if dispatched.matches_wal_append_indeterminate(&physical) =>
        {
            indeterminate_wal_append(dispatched, physical)
        }
        PhysicalExecutorOutcome::WalBarrierIndeterminate(physical)
            if dispatched.matches_wal_barrier_indeterminate(&physical) =>
        {
            indeterminate_wal_barrier(dispatched, physical)
        }
        _ => PhysicalWorkSettlementEvidence::StaleOrForeign,
    }
}

fn classify_completed_wal_barrier(
    dispatched: &DispatchedPhysicalWork,
    physical: crate::physical_runtime::work::CompletedPhysicalWalBarrier,
    scheduler: QueueExecutionOutcome,
) -> PhysicalWorkSettlementEvidence {
    if matches!(scheduler, QueueExecutionOutcome::Executed(_)) {
        return PhysicalWorkSettlementEvidence::WalBarrier {
            physical,
            scheduler,
        };
    }
    PhysicalWorkSettlementEvidence::TerminalFailure(PhysicalWorkTerminalFailure {
        identity: dispatched.intent().identity(),
        effect_fate: PhysicalWorkEffectFate::WrittenButSchedulerRejected,
        target: wal_barrier_recovery_target(dispatched),
        completed_bytes: 0,
        backend_operation: physical.physical().operation(),
        backend_role: MediaOperationRole::SynchronizeFileState,
        scheduler: PhysicalWorkSchedulerPosture::RejectedAfterEffect,
        publication_residue: PhysicalWorkPublicationResiduePosture::NotApplicable,
        recovery: PhysicalWorkRecoveryDisposition::InspectionRequired,
        cause: PhysicalWorkTerminalCause::SchedulerRejectedAfterEffect,
    })
}

fn indeterminate_wal_barrier(
    dispatched: &DispatchedPhysicalWork,
    physical: crate::physical_runtime::work::IndeterminatePhysicalWalBarrier,
) -> PhysicalWorkSettlementEvidence {
    PhysicalWorkSettlementEvidence::TerminalFailure(PhysicalWorkTerminalFailure {
        identity: dispatched.intent().identity(),
        effect_fate: PhysicalWorkEffectFate::Indeterminate,
        target: wal_barrier_recovery_target(dispatched),
        completed_bytes: 0,
        backend_operation: physical.physical().operation(),
        backend_role: MediaOperationRole::SynchronizeFileState,
        scheduler: PhysicalWorkSchedulerPosture::NotObserved,
        publication_residue: PhysicalWorkPublicationResiduePosture::NotApplicable,
        recovery: PhysicalWorkRecoveryDisposition::InspectionRequired,
        cause: PhysicalWorkTerminalCause::Backend(physical.physical().failure()),
    })
}

fn wal_barrier_recovery_target(dispatched: &DispatchedPhysicalWork) -> PhysicalWorkRecoveryTarget {
    let scope = dispatched
        .intent()
        .scope()
        .wal_barrier_target()
        .expect("WAL barrier work carries WAL barrier scope");
    PhysicalWorkRecoveryTarget::WalArtifactInterval {
        segment: scope.segment(),
        generation: scope.generation(),
        offset: scope.append_offset(),
        byte_count: scope.append_byte_count(),
    }
}

fn classify_completed_wal_append(
    dispatched: &DispatchedPhysicalWork,
    physical: CompletedArtifactAppend,
    scheduler: QueueExecutionOutcome,
) -> PhysicalWorkSettlementEvidence {
    if matches!(scheduler, QueueExecutionOutcome::Executed(_)) {
        return PhysicalWorkSettlementEvidence::WalAppend {
            physical,
            scheduler,
        };
    }
    let target = wal_recovery_target(dispatched);
    PhysicalWorkSettlementEvidence::TerminalFailure(PhysicalWorkTerminalFailure {
        identity: dispatched.intent().identity(),
        effect_fate: PhysicalWorkEffectFate::WrittenButSchedulerRejected,
        target,
        completed_bytes: physical.range().byte_count(),
        backend_operation: physical.operation(),
        backend_role: MediaOperationRole::PositionedWrite,
        scheduler: PhysicalWorkSchedulerPosture::RejectedAfterEffect,
        publication_residue: PhysicalWorkPublicationResiduePosture::NotApplicable,
        recovery: PhysicalWorkRecoveryDisposition::InspectionRequired,
        cause: PhysicalWorkTerminalCause::SchedulerRejectedAfterEffect,
    })
}

fn indeterminate_wal_append(
    dispatched: &DispatchedPhysicalWork,
    physical: IndeterminateArtifactAppend,
) -> PhysicalWorkSettlementEvidence {
    PhysicalWorkSettlementEvidence::TerminalFailure(PhysicalWorkTerminalFailure {
        identity: dispatched.intent().identity(),
        effect_fate: PhysicalWorkEffectFate::Indeterminate,
        target: wal_recovery_target(dispatched),
        completed_bytes: physical.completed_bytes(),
        backend_operation: physical.operation(),
        backend_role: MediaOperationRole::PositionedWrite,
        scheduler: PhysicalWorkSchedulerPosture::NotObserved,
        publication_residue: PhysicalWorkPublicationResiduePosture::NotApplicable,
        recovery: PhysicalWorkRecoveryDisposition::InspectionRequired,
        cause: PhysicalWorkTerminalCause::Backend(physical.failure()),
    })
}

fn wal_recovery_target(dispatched: &DispatchedPhysicalWork) -> PhysicalWorkRecoveryTarget {
    let scope = dispatched
        .intent()
        .scope()
        .wal_append_target()
        .expect("WAL append work carries WAL scope");
    PhysicalWorkRecoveryTarget::WalArtifactInterval {
        segment: scope.segment(),
        generation: scope.generation(),
        offset: scope.offset(),
        byte_count: scope.byte_count(),
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
