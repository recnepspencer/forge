use super::{OwnerBoundaryBinding, OwnerOperationFamily, ProductionOwner, ProtocolFamily};
use crate::protocol_bindings::OwnerEvidenceClass;

pub(super) fn current() -> Vec<OwnerBoundaryBinding> {
    use OwnerEvidenceClass::{
        DurableAuthoritativeReceipt, EphemeralDiagnosticTrace, ForbiddenAuthoritySubstitute,
        ReopenedObservedReceipt,
    };
    use OwnerOperationFamily::*;
    use ProductionOwner::RecoveryPhysics;
    use ProtocolFamily::{DurabilityRecovery, RecoverySourcePrecedence};

    vec![
        OwnerBoundaryBinding::to::<worth_store_recovery_physics::RecoveryCandidateDiscoveryTrace>(
            RecoverySourcePrecedence,
            RecoveryPhysics,
            RecoveryCandidateDiscovery,
            EphemeralDiagnosticTrace,
        ),
        OwnerBoundaryBinding::to::<worth_store_recovery_physics::RecoverySourceDecisionTrace>(
            RecoverySourcePrecedence,
            RecoveryPhysics,
            RecoverySourceSelection,
            EphemeralDiagnosticTrace,
        ),
        OwnerBoundaryBinding::to::<worth_store_recovery_physics::AdmittedRecoverySource>(
            RecoverySourcePrecedence,
            RecoveryPhysics,
            RecoverySourceAdmission,
            ReopenedObservedReceipt,
        ),
        OwnerBoundaryBinding::to::<worth_store_recovery_physics::CheckpointBaseAdmission>(
            RecoverySourcePrecedence,
            RecoveryPhysics,
            RecoveryCheckpointBase,
            ReopenedObservedReceipt,
        ),
        OwnerBoundaryBinding::to::<worth_store_recovery_physics::WalTailRedoSource>(
            RecoverySourcePrecedence,
            RecoveryPhysics,
            RecoveryWalTailSource,
            ReopenedObservedReceipt,
        ),
        OwnerBoundaryBinding::to::<worth_store_recovery_physics::RecoveryRedoPlan>(
            DurabilityRecovery,
            RecoveryPhysics,
            RecoveryRedoPlanning,
            ForbiddenAuthoritySubstitute,
        ),
        OwnerBoundaryBinding::to::<worth_store_recovery_physics::RedoExecutionReceipt>(
            DurabilityRecovery,
            RecoveryPhysics,
            RedoExecution,
            EphemeralDiagnosticTrace,
        ),
        OwnerBoundaryBinding::to::<worth_store_recovery_physics::RecoveryCompletion>(
            DurabilityRecovery,
            RecoveryPhysics,
            RecoveryCompletion,
            DurableAuthoritativeReceipt,
        ),
        OwnerBoundaryBinding::to::<worth_store_recovery_physics::ReopenedRecoveryArtifactAdmission>(
            DurabilityRecovery,
            RecoveryPhysics,
            RecoveryReopenObservation,
            ReopenedObservedReceipt,
        ),
        OwnerBoundaryBinding::to::<worth_store_recovery_physics::RecoveryDeterminismReport>(
            RecoverySourcePrecedence,
            RecoveryPhysics,
            RecoveryDeterminism,
            EphemeralDiagnosticTrace,
        ),
        OwnerBoundaryBinding::to::<worth_store_recovery_physics::ReopenedRecoveryArtifactAdmission>(
            RecoverySourcePrecedence,
            RecoveryPhysics,
            ReopenedArtifactAdmission,
            ReopenedObservedReceipt,
        ),
    ]
}
