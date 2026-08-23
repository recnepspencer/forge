use super::{OwnerBoundaryBinding, OwnerOperationFamily, ProductionOwner, ProtocolFamily};
use crate::protocol_bindings::OwnerEvidenceClass;

pub(super) fn current() -> Vec<OwnerBoundaryBinding> {
    use OwnerEvidenceClass::{
        EphemeralDiagnosticTrace, ForbiddenAuthoritySubstitute, ReopenedObservedReceipt,
    };
    use OwnerOperationFamily::*;
    use ProductionOwner::{OfflineVerifier, RecoveryPhysics, RecoveryRuntime};
    use ProtocolFamily::{DurabilityRecovery, RecoverySourcePrecedence};

    vec![
        OwnerBoundaryBinding::to::<worth_store_recovery_physics::PhysicalRootSourceCandidate>(
            RecoverySourcePrecedence,
            RecoveryPhysics,
            RecoveryCandidateDiscovery,
            ReopenedObservedReceipt,
        ),
        OwnerBoundaryBinding::to::<worth_store_recovery_physics::PhysicalSourceSelection>(
            RecoverySourcePrecedence,
            RecoveryPhysics,
            RecoverySourceSelection,
            ReopenedObservedReceipt,
        ),
        OwnerBoundaryBinding::to::<worth_store_recovery_physics::PhysicalCheckpointBase>(
            RecoverySourcePrecedence,
            RecoveryPhysics,
            RecoveryCheckpointBase,
            ReopenedObservedReceipt,
        ),
        OwnerBoundaryBinding::to::<worth_store_recovery_physics::SelectedPhysicalWalTail>(
            RecoverySourcePrecedence,
            RecoveryPhysics,
            RecoveryWalTailSource,
            ReopenedObservedReceipt,
        ),
        OwnerBoundaryBinding::to::<worth_store_recovery_physics::ImmutablePhysicalRedoPlan>(
            DurabilityRecovery,
            RecoveryPhysics,
            RecoveryRedoPlanning,
            ForbiddenAuthoritySubstitute,
        ),
        OwnerBoundaryBinding::to::<worth_store_recovery_physics::PageRedoEligibility>(
            DurabilityRecovery,
            RecoveryPhysics,
            RedoExecution,
            EphemeralDiagnosticTrace,
        ),
        OwnerBoundaryBinding::to::<worth_store_recovery_physics::ReconciledOperationFates>(
            DurabilityRecovery,
            RecoveryPhysics,
            RecoveryDeterminism,
            ReopenedObservedReceipt,
        ),
        OwnerBoundaryBinding::to::<worth_store_recovery_runtime::ReopenedPhysicalRecovery>(
            DurabilityRecovery,
            RecoveryRuntime,
            RecoveryReopenObservation,
            ReopenedObservedReceipt,
        ),
        OwnerBoundaryBinding::to::<worth_store_recovery_runtime::RecoveredPhysicalRuntimeHandoff>(
            DurabilityRecovery,
            RecoveryRuntime,
            RecoveryCompletion,
            ReopenedObservedReceipt,
        ),
        OwnerBoundaryBinding::to::<worth_store_offline_verifier::RecoveryObserverReport>(
            RecoverySourcePrecedence,
            OfflineVerifier,
            RecoveryDeterminism,
            EphemeralDiagnosticTrace,
        ),
        OwnerBoundaryBinding::to::<worth_store_recovery_runtime::RecoveryReportEnvelope>(
            DurabilityRecovery,
            RecoveryRuntime,
            RecoveryDeterminism,
            EphemeralDiagnosticTrace,
        ),
    ]
}
