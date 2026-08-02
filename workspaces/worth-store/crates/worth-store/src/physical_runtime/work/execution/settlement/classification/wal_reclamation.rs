use worth_store_io_scheduler::QueueExecutionOutcome;
use worth_store_physical_backend::MediaOperationRole;

use crate::physical_runtime::work::{
    CompletedPhysicalWalReclamationAction, DispatchedPhysicalWork,
    IndeterminatePhysicalWalReclamationAction, PhysicalWorkEffectFate,
    PhysicalWorkPublicationResiduePosture, PhysicalWorkRecoveryDisposition,
    PhysicalWorkRecoveryTarget, PhysicalWorkSchedulerPosture, PhysicalWorkSettlementEvidence,
    PhysicalWorkTerminalCause, PhysicalWorkTerminalFailure,
};

pub(super) fn matches_completed(
    dispatched: &DispatchedPhysicalWork,
    physical: &CompletedPhysicalWalReclamationAction,
) -> bool {
    dispatched
        .intent()
        .scope()
        .wal_reclamation_target()
        .is_some_and(|scope| {
            scope.checkpoint() == physical.checkpoint()
                && scope.segment() == physical.segment()
                && scope.lsn_range() == physical.lsn_range()
                && scope.byte_count() == physical.byte_count()
        })
}

pub(super) fn matches_indeterminate(
    dispatched: &DispatchedPhysicalWork,
    physical: &IndeterminatePhysicalWalReclamationAction,
) -> bool {
    dispatched
        .intent()
        .scope()
        .wal_reclamation_target()
        .is_some_and(|scope| {
            scope.checkpoint() == physical.checkpoint() && scope.segment() == physical.segment()
        })
}

pub(super) fn classify_completed(
    dispatched: &DispatchedPhysicalWork,
    physical: CompletedPhysicalWalReclamationAction,
    scheduler: QueueExecutionOutcome,
) -> PhysicalWorkSettlementEvidence {
    if matches!(scheduler, QueueExecutionOutcome::Executed(_)) {
        return PhysicalWorkSettlementEvidence::WalReclamation {
            physical,
            scheduler,
        };
    }
    PhysicalWorkSettlementEvidence::TerminalFailure(PhysicalWorkTerminalFailure {
        identity: dispatched.intent().identity(),
        effect_fate: PhysicalWorkEffectFate::WrittenButSchedulerRejected,
        target: recovery_target(dispatched),
        completed_bytes: 0,
        backend_operation: physical.operation(),
        backend_role: MediaOperationRole::Delete,
        scheduler: PhysicalWorkSchedulerPosture::RejectedAfterEffect,
        publication_residue: PhysicalWorkPublicationResiduePosture::DeletionMayHaveOccurred,
        recovery: PhysicalWorkRecoveryDisposition::InspectionRequired,
        cause: PhysicalWorkTerminalCause::SchedulerRejectedAfterEffect,
    })
}

pub(super) fn classify_indeterminate(
    dispatched: &DispatchedPhysicalWork,
    physical: IndeterminatePhysicalWalReclamationAction,
) -> PhysicalWorkSettlementEvidence {
    PhysicalWorkSettlementEvidence::TerminalFailure(PhysicalWorkTerminalFailure {
        identity: dispatched.intent().identity(),
        effect_fate: PhysicalWorkEffectFate::Indeterminate,
        target: recovery_target(dispatched),
        completed_bytes: 0,
        backend_operation: physical.operation(),
        backend_role: MediaOperationRole::Delete,
        scheduler: PhysicalWorkSchedulerPosture::NotObserved,
        publication_residue: PhysicalWorkPublicationResiduePosture::DeletionMayHaveOccurred,
        recovery: PhysicalWorkRecoveryDisposition::InspectionRequired,
        cause: PhysicalWorkTerminalCause::Backend(physical.failure()),
    })
}

fn recovery_target(dispatched: &DispatchedPhysicalWork) -> PhysicalWorkRecoveryTarget {
    let scope = dispatched
        .intent()
        .scope()
        .wal_reclamation_target()
        .expect("WAL reclamation work carries exact scope");
    PhysicalWorkRecoveryTarget::WalSegmentReclamation {
        segment: scope.segment().segment().get(),
        generation: scope.segment().generation().get(),
    }
}
