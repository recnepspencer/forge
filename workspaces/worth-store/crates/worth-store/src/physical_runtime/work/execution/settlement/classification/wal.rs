use worth_store_io_scheduler::QueueExecutionOutcome;
use worth_store_physical_backend::{
    CompletedArtifactAppend, CompletedArtifactNewWrite, IndeterminateArtifactAppend,
    IndeterminateArtifactNewWrite, MediaOperationRole,
};

pub(super) fn classify_completed_wal_append(
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
    PhysicalWorkSettlementEvidence::TerminalFailure(PhysicalWorkTerminalFailure {
        identity: dispatched.intent().identity(),
        effect_fate: PhysicalWorkEffectFate::WrittenButSchedulerRejected,
        target: wal_append_recovery_target(dispatched),
        completed_bytes: physical.range().byte_count(),
        backend_operation: physical.operation(),
        backend_role: MediaOperationRole::PositionedWrite,
        scheduler: PhysicalWorkSchedulerPosture::RejectedAfterEffect,
        publication_residue: PhysicalWorkPublicationResiduePosture::NotApplicable,
        recovery: PhysicalWorkRecoveryDisposition::InspectionRequired,
        cause: PhysicalWorkTerminalCause::SchedulerRejectedAfterEffect,
    })
}

pub(super) fn indeterminate_wal_append(
    dispatched: &DispatchedPhysicalWork,
    physical: IndeterminateArtifactAppend,
) -> PhysicalWorkSettlementEvidence {
    PhysicalWorkSettlementEvidence::TerminalFailure(PhysicalWorkTerminalFailure {
        identity: dispatched.intent().identity(),
        effect_fate: PhysicalWorkEffectFate::Indeterminate,
        target: wal_append_recovery_target(dispatched),
        completed_bytes: physical.completed_bytes(),
        backend_operation: physical.operation(),
        backend_role: MediaOperationRole::PositionedWrite,
        scheduler: PhysicalWorkSchedulerPosture::NotObserved,
        publication_residue: PhysicalWorkPublicationResiduePosture::NotApplicable,
        recovery: PhysicalWorkRecoveryDisposition::InspectionRequired,
        cause: PhysicalWorkTerminalCause::Backend(physical.failure()),
    })
}

pub(super) fn classify_completed_wal_barrier(
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

pub(super) fn indeterminate_wal_barrier(
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

fn wal_append_recovery_target(dispatched: &DispatchedPhysicalWork) -> PhysicalWorkRecoveryTarget {
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

use crate::physical_runtime::{
    DispatchedPhysicalWork, PhysicalWorkEffectFate, PhysicalWorkPublicationResiduePosture,
    PhysicalWorkRecoveryDisposition, PhysicalWorkRecoveryTarget, PhysicalWorkSchedulerPosture,
    PhysicalWorkSettlementEvidence, PhysicalWorkTerminalCause, PhysicalWorkTerminalFailure,
};

pub(super) fn classify_completed_wal_segment_create(
    dispatched: &DispatchedPhysicalWork,
    physical: CompletedArtifactNewWrite,
    scheduler: QueueExecutionOutcome,
) -> PhysicalWorkSettlementEvidence {
    if matches!(scheduler, QueueExecutionOutcome::Executed(_)) {
        return PhysicalWorkSettlementEvidence::WalSegmentCreate {
            physical,
            scheduler,
        };
    }
    PhysicalWorkSettlementEvidence::TerminalFailure(PhysicalWorkTerminalFailure {
        identity: dispatched.intent().identity(),
        effect_fate: PhysicalWorkEffectFate::WrittenButSchedulerRejected,
        target: recovery_target(dispatched),
        completed_bytes: physical.completed_bytes(),
        backend_operation: physical.write_operation(),
        backend_role: MediaOperationRole::PositionedWrite,
        scheduler: PhysicalWorkSchedulerPosture::RejectedAfterEffect,
        publication_residue: PhysicalWorkPublicationResiduePosture::NotApplicable,
        recovery: PhysicalWorkRecoveryDisposition::InspectionRequired,
        cause: PhysicalWorkTerminalCause::SchedulerRejectedAfterEffect,
    })
}

pub(super) fn indeterminate_wal_segment_create(
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
        target: recovery_target(dispatched),
        completed_bytes: physical.completed_bytes(),
        backend_operation: physical
            .write_operation()
            .unwrap_or_else(|| physical.create_operation()),
        backend_role,
        scheduler: PhysicalWorkSchedulerPosture::NotObserved,
        publication_residue: PhysicalWorkPublicationResiduePosture::NotApplicable,
        recovery: PhysicalWorkRecoveryDisposition::InspectionRequired,
        cause: PhysicalWorkTerminalCause::Backend(physical.failure()),
    })
}

fn recovery_target(dispatched: &DispatchedPhysicalWork) -> PhysicalWorkRecoveryTarget {
    let scope = dispatched
        .intent()
        .scope()
        .wal_append_target()
        .expect("WAL segment creation work carries WAL scope");
    PhysicalWorkRecoveryTarget::WalArtifactInterval {
        segment: scope.segment(),
        generation: scope.generation(),
        offset: 0,
        byte_count: scope.byte_count(),
    }
}
