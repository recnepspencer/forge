use worth_store::physical_runtime::{ArtifactTreeFailureKind, RecoveryDiscoveryArtifact};
use worth_store_physical_format::{
    store_namespace::StableStoreIdentity, BootstrapCatalogDenial, CheckpointStreamDecodeDenial,
    ManifestBlockReference, PhysicalRecordFormatDeclaration, RootRoutingBlockDenial,
    RootSelectorRole,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalManifestObservationDenial {
    DuplicateReference {
        reference: ManifestBlockReference,
    },
    MissingArtifact {
        reference: ManifestBlockReference,
    },
    Decode {
        reference: ManifestBlockReference,
        denial: RootRoutingBlockDenial,
    },
    FormatIdentity {
        reference: ManifestBlockReference,
        expected: PhysicalRecordFormatDeclaration,
        observed: PhysicalRecordFormatDeclaration,
    },
    TreeIdentity {
        reference: ManifestBlockReference,
        expected: u64,
        observed: u64,
    },
    ReferenceIntegrity {
        expected: ManifestBlockReference,
        observed: ManifestBlockReference,
    },
}
use worth_store_recovery_physics::{
    PhysicalCheckpointBaseDenial, PhysicalPageFactDenial, PhysicalRootCandidateDenial,
    PhysicalRootSelectionDenial, PhysicalSourceSelectionDenial, PhysicalWalArtifactCorruption,
    SelectedPhysicalWalTailDenial,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PhysicalRecoverySourceDenial {
    MediaObservation {
        artifact: RecoveryDiscoveryArtifact,
        failure: PhysicalRecoveryMediaObservationFailure,
    },
    RootSlot {
        slot: RootSelectorRole,
        denial: PhysicalRootCandidateDenial,
        observed_store: Option<StableStoreIdentity>,
        observed_role: Option<RootSelectorRole>,
        observed_generation: Option<u64>,
    },
    RootSelection(PhysicalRootSelectionDenial),
    BootstrapCatalog(BootstrapCatalogDenial),
    ManifestObservation(PhysicalManifestObservationDenial),
    ManifestFacts(PhysicalPageFactDenial),
    CheckpointFormat(CheckpointStreamDecodeDenial),
    CheckpointBinding(PhysicalCheckpointBaseDenial),
    WalArtifact(PhysicalWalArtifactCorruption),
    WalTail(SelectedPhysicalWalTailDenial),
    FinalSelection(PhysicalSourceSelectionDenial),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalRecoveryMediaObservationFailure {
    InvalidAddress,
    Backend {
        kind: ArtifactTreeFailureKind,
        io_kind: Option<std::io::ErrorKind>,
    },
}
