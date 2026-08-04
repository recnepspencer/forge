use super::{
    OwnerBoundaryBinding, OwnerOperationFamily, OwnerSourcePolymorphism, ProductionOwner,
    ProtocolFamily,
};
use crate::protocol_bindings::OwnerEvidenceClass;

pub(super) fn current() -> Vec<OwnerBoundaryBinding> {
    use OwnerEvidenceClass::{
        DurableAuthoritativeReceipt, EphemeralDiagnosticTrace, ForbiddenAuthoritySubstitute,
        ReopenedObservedReceipt,
    };
    use OwnerOperationFamily::*;
    use ProductionOwner::*;
    use ProtocolFamily::DurabilityRecovery;

    vec![
        OwnerBoundaryBinding::to::<worth_store_wal::PublicationDeclaration>(
            DurabilityRecovery,
            Wal,
            PublicationDeclaration,
            ForbiddenAuthoritySubstitute,
        ),
        OwnerBoundaryBinding::to::<worth_store_wal::WalReplayTailRecordReport>(
            DurabilityRecovery,
            Wal,
            WalReplayTailInspection,
            ReopenedObservedReceipt,
        ),
        OwnerBoundaryBinding::to_polymorphic::<
            worth_store_recovery_physics::WalAppendReceipt<
                worth_store_physical_backend::PosixFileFsyncDirFsyncProfile,
            >,
        >(
            DurabilityRecovery,
            RecoveryPhysics,
            WalAppendObservation,
            ReopenedObservedReceipt,
            OwnerSourcePolymorphism::AcrossBackendDurabilityProfiles,
        ),
        OwnerBoundaryBinding::to_polymorphic::<
            worth_store_recovery_physics::WalDurabilityObservation<
                worth_store_physical_backend::PosixFileFsyncDirFsyncProfile,
            >,
        >(
            DurabilityRecovery,
            RecoveryPhysics,
            WalDurabilityObservation,
            ReopenedObservedReceipt,
            OwnerSourcePolymorphism::AcrossBackendDurabilityProfiles,
        ),
        OwnerBoundaryBinding::to::<worth_store_physical_backend::AdmittedBackendCapabilityWitness>(
            DurabilityRecovery,
            PhysicalBackend,
            BackendCapabilityAdmission,
            DurableAuthoritativeReceipt,
        ),
        OwnerBoundaryBinding::to::<worth_store_physical_backend::BackendCapabilityClaimWitness>(
            DurabilityRecovery,
            PhysicalBackend,
            BackendCapabilityClaim,
            ForbiddenAuthoritySubstitute,
        ),
        OwnerBoundaryBinding::to::<worth_store_physical_backend::AccessPolicyExecutionReceipt>(
            DurabilityRecovery,
            PhysicalBackend,
            BackendAccessPolicyExecution,
            EphemeralDiagnosticTrace,
        ),
        OwnerBoundaryBinding::to::<worth_store_physical_backend::BackendQueueExecutionCompletion>(
            DurabilityRecovery,
            PhysicalBackend,
            BackendQueueCompletion,
            EphemeralDiagnosticTrace,
        ),
    ]
}
