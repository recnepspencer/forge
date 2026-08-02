use worth_store_io_scheduler::QueueExecutionOutcome;

use crate::physical_runtime::work::{
    CompletedPhysicalCheckpointAction, DispatchedPhysicalWork,
    IndeterminatePhysicalCheckpointAction, PhysicalCheckpointRecoveryAction,
    PhysicalWorkEffectFate, PhysicalWorkPublicationResiduePosture, PhysicalWorkRecoveryDisposition,
    PhysicalWorkRecoveryTarget, PhysicalWorkSchedulerPosture, PhysicalWorkSettlementEvidence,
    PhysicalWorkTerminalCause, PhysicalWorkTerminalFailure,
};

pub(super) fn matches_completed(
    dispatched: &DispatchedPhysicalWork,
    physical: &CompletedPhysicalCheckpointAction,
) -> bool {
    expected_action(dispatched) == Some(physical.action())
        && expected_completed_bytes(dispatched) == Some(physical.completed_bytes())
}

pub(super) fn matches_indeterminate(
    dispatched: &DispatchedPhysicalWork,
    physical: &IndeterminatePhysicalCheckpointAction,
) -> bool {
    expected_action(dispatched) == Some(physical.action())
}

pub(super) fn classify_completed(
    dispatched: &DispatchedPhysicalWork,
    physical: CompletedPhysicalCheckpointAction,
    scheduler: QueueExecutionOutcome,
) -> PhysicalWorkSettlementEvidence {
    if matches!(scheduler, QueueExecutionOutcome::Executed(_)) {
        return PhysicalWorkSettlementEvidence::Checkpoint {
            physical,
            scheduler,
        };
    }
    PhysicalWorkSettlementEvidence::TerminalFailure(PhysicalWorkTerminalFailure {
        identity: dispatched.intent().identity(),
        effect_fate: PhysicalWorkEffectFate::WrittenButSchedulerRejected,
        target: recovery_target(dispatched),
        completed_bytes: physical.completed_bytes(),
        backend_operation: physical.operation(),
        backend_role: physical.role(),
        scheduler: PhysicalWorkSchedulerPosture::RejectedAfterEffect,
        publication_residue: PhysicalWorkPublicationResiduePosture::MayExist,
        recovery: PhysicalWorkRecoveryDisposition::InspectionRequired,
        cause: PhysicalWorkTerminalCause::SchedulerRejectedAfterEffect,
    })
}

pub(super) fn classify_indeterminate(
    dispatched: &DispatchedPhysicalWork,
    physical: IndeterminatePhysicalCheckpointAction,
) -> PhysicalWorkSettlementEvidence {
    PhysicalWorkSettlementEvidence::TerminalFailure(PhysicalWorkTerminalFailure {
        identity: dispatched.intent().identity(),
        effect_fate: PhysicalWorkEffectFate::Indeterminate,
        target: recovery_target(dispatched),
        completed_bytes: physical.completed_bytes(),
        backend_operation: physical.operation(),
        backend_role: physical.role(),
        scheduler: PhysicalWorkSchedulerPosture::NotObserved,
        publication_residue: PhysicalWorkPublicationResiduePosture::MayExist,
        recovery: PhysicalWorkRecoveryDisposition::InspectionRequired,
        cause: PhysicalWorkTerminalCause::Backend(physical.failure()),
    })
}

fn recovery_target(dispatched: &DispatchedPhysicalWork) -> PhysicalWorkRecoveryTarget {
    let scope = dispatched
        .intent()
        .scope()
        .checkpoint_target()
        .expect("checkpoint work carries checkpoint scope");
    PhysicalWorkRecoveryTarget::Checkpoint {
        sequence: scope.checkpoint().sequence().get(),
        action: PhysicalCheckpointRecoveryAction::from(scope.action()),
    }
}

fn expected_action(
    dispatched: &DispatchedPhysicalWork,
) -> Option<PhysicalCheckpointRecoveryAction> {
    dispatched
        .intent()
        .scope()
        .checkpoint_target()
        .map(|scope| PhysicalCheckpointRecoveryAction::from(scope.action()))
}

fn expected_completed_bytes(dispatched: &DispatchedPhysicalWork) -> Option<u64> {
    let scope = dispatched.intent().scope().checkpoint_target()?;
    Some(match scope.action() {
        crate::physical_runtime::work::PhysicalCheckpointWorkAction::CreateCandidate {
            byte_count,
        }
        | crate::physical_runtime::work::PhysicalCheckpointWorkAction::AppendCandidate {
            byte_count,
            ..
        } => byte_count,
        crate::physical_runtime::work::PhysicalCheckpointWorkAction::SynchronizeCandidate
        | crate::physical_runtime::work::PhysicalCheckpointWorkAction::RemoveCandidate
        | crate::physical_runtime::work::PhysicalCheckpointWorkAction::PublishCandidate
        | crate::physical_runtime::work::PhysicalCheckpointWorkAction::SynchronizeNamespace => 0,
    })
}
