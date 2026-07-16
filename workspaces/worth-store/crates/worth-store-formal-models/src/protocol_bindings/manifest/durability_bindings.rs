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
        OwnerBoundaryBinding::to::<worth_store_wal::AdmittedWalAppendReceipt>(
            DurabilityRecovery,
            Wal,
            WalAppendAdmission,
            DurableAuthoritativeReceipt,
        ),
        OwnerBoundaryBinding::to::<worth_store_wal::AdmittedCheckpointPublicationReceipt>(
            DurabilityRecovery,
            Wal,
            CheckpointPublicationAdmission,
            DurableAuthoritativeReceipt,
        ),
        OwnerBoundaryBinding::to::<worth_store_wal::DurablePublicationDeclaration>(
            DurabilityRecovery,
            Wal,
            DurablePublicationDeclaration,
            ForbiddenAuthoritySubstitute,
        ),
        OwnerBoundaryBinding::to::<worth_store_wal::WalReplayTailRecordReport>(
            DurabilityRecovery,
            Wal,
            WalReplayTailInspection,
            ReopenedObservedReceipt,
        ),
        OwnerBoundaryBinding::to_polymorphic::<
            worth_store_recovery_physics::WalAppendPlan<
                worth_store_physical_backend::PosixFileFsyncDirFsyncProfile,
            >,
        >(
            DurabilityRecovery,
            RecoveryPhysics,
            WalAppendPlanning,
            ForbiddenAuthoritySubstitute,
            OwnerSourcePolymorphism::AcrossBackendDurabilityProfiles,
        ),
        OwnerBoundaryBinding::to_polymorphic::<
            worth_store_recovery_physics::WalAppendProgress<
                worth_store_physical_backend::PosixFileFsyncDirFsyncProfile,
            >,
        >(
            DurabilityRecovery,
            RecoveryPhysics,
            WalAppendProgress,
            EphemeralDiagnosticTrace,
            OwnerSourcePolymorphism::AcrossBackendDurabilityProfiles,
        ),
        OwnerBoundaryBinding::to_polymorphic::<
            worth_store_recovery_physics::WalAppendReceipt<
                worth_store_physical_backend::PosixFileFsyncDirFsyncProfile,
            >,
        >(
            DurabilityRecovery,
            RecoveryPhysics,
            WalAppendExecution,
            DurableAuthoritativeReceipt,
            OwnerSourcePolymorphism::AcrossBackendDurabilityProfiles,
        ),
        OwnerBoundaryBinding::to_polymorphic::<
            worth_store_recovery_physics::DurableAckReceipt<
                worth_store_physical_backend::PosixFileFsyncDirFsyncProfile,
            >,
        >(
            DurabilityRecovery,
            RecoveryPhysics,
            DurableAcknowledgement,
            DurableAuthoritativeReceipt,
            OwnerSourcePolymorphism::AcrossBackendDurabilityProfiles,
        ),
        OwnerBoundaryBinding::to::<worth_store_recovery_physics::PageFlushRecoveryReceipt>(
            DurabilityRecovery,
            RecoveryPhysics,
            PageFlushRecovery,
            DurableAuthoritativeReceipt,
        ),
        OwnerBoundaryBinding::to_polymorphic::<
            worth_store_physical_backend::StoreDurabilityExecutionProof<&'static str>,
        >(
            DurabilityRecovery,
            PhysicalBackend,
            BackendDurabilityExecution,
            DurableAuthoritativeReceipt,
            OwnerSourcePolymorphism::AcrossOwnerScopeTypes,
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
