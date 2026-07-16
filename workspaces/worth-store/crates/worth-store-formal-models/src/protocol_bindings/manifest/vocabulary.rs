#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ProtocolFamily {
    DurabilityRecovery,
    RecoverySourcePrecedence,
    CompactionVisibility,
    LeaseReclaim,
    QuarantineReadmission,
    ImportPublication,
    ReplicationAdmission,
    SharedFrontiers,
}

impl ProtocolFamily {
    pub const fn all() -> [Self; 8] {
        [
            Self::DurabilityRecovery,
            Self::RecoverySourcePrecedence,
            Self::CompactionVisibility,
            Self::LeaseReclaim,
            Self::QuarantineReadmission,
            Self::ImportPublication,
            Self::ReplicationAdmission,
            Self::SharedFrontiers,
        ]
    }

    pub const fn admits_model_action_family(self, action: ModelActionFamily) -> bool {
        match self {
            Self::DurabilityRecovery => matches!(
                action,
                ModelActionFamily::DurabilityAdmission
                    | ModelActionFamily::DurabilityFrontier
                    | ModelActionFamily::RecoverySourcePrecedence
                    | ModelActionFamily::RecoveryRedo
                    | ModelActionFamily::BackendCapability
            ),
            Self::RecoverySourcePrecedence => matches!(
                action,
                ModelActionFamily::RecoverySourcePrecedence | ModelActionFamily::RecoveryRedo
            ),
            Self::CompactionVisibility => matches!(
                action,
                ModelActionFamily::LsmMembership
                    | ModelActionFamily::LsmExecution
                    | ModelActionFamily::LsmMaintenance
                    | ModelActionFamily::PhysicalCompaction
            ),
            Self::LeaseReclaim => matches!(
                action,
                ModelActionFamily::LeaseReclaim | ModelActionFamily::GenerationReuse
            ),
            Self::QuarantineReadmission => {
                matches!(action, ModelActionFamily::QuarantineReadmission)
            }
            Self::ImportPublication => matches!(
                action,
                ModelActionFamily::ImportPublication | ModelActionFamily::TrustBoundaryReadmission
            ),
            Self::ReplicationAdmission => {
                matches!(action, ModelActionFamily::ReplicationAdmission)
            }
            Self::SharedFrontiers => false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ProductionOwner {
    Wal,
    RecoveryPhysics,
    PhysicalBackend,
    LayoutIndexes,
    LsmAuthority,
    PhysicalIsolation,
    PhysicalIntegrity,
    Operations,
    Security,
    Replication,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ModelActionFamily {
    DurabilityAdmission,
    DurabilityFrontier,
    RecoverySourcePrecedence,
    RecoveryRedo,
    LsmMembership,
    LsmExecution,
    LsmMaintenance,
    PhysicalCompaction,
    LeaseReclaim,
    GenerationReuse,
    QuarantineReadmission,
    ImportPublication,
    TrustBoundaryReadmission,
    BackendCapability,
    ReplicationAdmission,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OwnerOperationFamily {
    WalAppendAdmission,
    CheckpointPublicationAdmission,
    DurablePublicationDeclaration,
    WalReplayTailInspection,
    WalAppendPlanning,
    WalAppendProgress,
    WalAppendExecution,
    DurableAcknowledgement,
    PageFlushRecovery,
    BackendDurabilityExecution,
    RecoveryCandidateDiscovery,
    RecoverySourceSelection,
    RecoverySourceAdmission,
    RecoveryCheckpointBase,
    RecoveryWalTailSource,
    RecoveryRedoPlanning,
    RedoExecution,
    RecoveryCompletion,
    RecoveryReopenObservation,
    RecoveryDeterminism,
    ReopenedArtifactAdmission,
    LsmMembership,
    LsmCompactionExecution,
    LsmMaintenanceAdmission,
    PhysicalCompactionCutover,
    CompactionMutationOutcome,
    CompactionStability,
    CompactionPublication,
    PhysicalPublication,
    PublicationCrashRecovery,
    ReclaimEligibility,
    DeferredReclaim,
    ReclaimDrain,
    ReclaimReuseFence,
    GenerationAdvance,
    QuarantineEntry,
    QuarantineFinding,
    QuarantineRecord,
    QuarantineHandoff,
    CorruptionReadmission,
    LayoutReadmission,
    ImportCustodyReadmission,
    ExportCustodyAdmission,
    RestoredLayoutOutcome,
    RestoredLayoutMaterialization,
    ImportPublicationReadiness,
    ImportPublicationCompletion,
    TrustBoundaryReadmission,
    SecurityScopeReadmission,
    BackendCapabilityAdmission,
    BackendCapabilityClaim,
    BackendAccessPolicyExecution,
    BackendQueueCompletion,
    ReplicationSourceAdmission,
    ReplicationProgressObservation,
    ReplicationPublicationReadiness,
    ReplicationPublicationCompletion,
    ReplicationDurablePublication,
    ReplicationPeerProgress,
}

impl OwnerOperationFamily {
    pub const fn model_action_family(self) -> ModelActionFamily {
        match self {
            Self::WalAppendAdmission | Self::CheckpointPublicationAdmission => {
                ModelActionFamily::DurabilityAdmission
            }
            Self::DurablePublicationDeclaration
            | Self::WalAppendPlanning
            | Self::WalAppendProgress
            | Self::WalAppendExecution
            | Self::DurableAcknowledgement
            | Self::PageFlushRecovery
            | Self::BackendDurabilityExecution => ModelActionFamily::DurabilityFrontier,
            Self::WalReplayTailInspection
            | Self::RecoveryCandidateDiscovery
            | Self::RecoverySourceSelection
            | Self::RecoverySourceAdmission
            | Self::RecoveryCheckpointBase
            | Self::RecoveryWalTailSource
            | Self::ReopenedArtifactAdmission => ModelActionFamily::RecoverySourcePrecedence,
            Self::RecoveryRedoPlanning
            | Self::RedoExecution
            | Self::RecoveryCompletion
            | Self::RecoveryReopenObservation
            | Self::RecoveryDeterminism => ModelActionFamily::RecoveryRedo,
            Self::LsmMembership => ModelActionFamily::LsmMembership,
            Self::LsmCompactionExecution => ModelActionFamily::LsmExecution,
            Self::LsmMaintenanceAdmission => ModelActionFamily::LsmMaintenance,
            Self::PhysicalCompactionCutover
            | Self::CompactionMutationOutcome
            | Self::CompactionStability
            | Self::CompactionPublication
            | Self::PhysicalPublication
            | Self::PublicationCrashRecovery => ModelActionFamily::PhysicalCompaction,
            Self::ReclaimEligibility | Self::DeferredReclaim | Self::ReclaimDrain => {
                ModelActionFamily::LeaseReclaim
            }
            Self::ReclaimReuseFence | Self::GenerationAdvance => ModelActionFamily::GenerationReuse,
            Self::QuarantineEntry
            | Self::QuarantineFinding
            | Self::QuarantineRecord
            | Self::QuarantineHandoff
            | Self::CorruptionReadmission
            | Self::LayoutReadmission => ModelActionFamily::QuarantineReadmission,
            Self::ImportCustodyReadmission
            | Self::ExportCustodyAdmission
            | Self::RestoredLayoutOutcome
            | Self::RestoredLayoutMaterialization
            | Self::ImportPublicationReadiness
            | Self::ImportPublicationCompletion => ModelActionFamily::ImportPublication,
            Self::TrustBoundaryReadmission | Self::SecurityScopeReadmission => {
                ModelActionFamily::TrustBoundaryReadmission
            }
            Self::BackendCapabilityAdmission
            | Self::BackendCapabilityClaim
            | Self::BackendAccessPolicyExecution
            | Self::BackendQueueCompletion => ModelActionFamily::BackendCapability,
            Self::ReplicationSourceAdmission
            | Self::ReplicationProgressObservation
            | Self::ReplicationPublicationReadiness
            | Self::ReplicationPublicationCompletion
            | Self::ReplicationDurablePublication
            | Self::ReplicationPeerProgress => ModelActionFamily::ReplicationAdmission,
        }
    }
}
