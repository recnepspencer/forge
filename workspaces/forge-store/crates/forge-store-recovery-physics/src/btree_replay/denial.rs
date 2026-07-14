use forge_store_physical_format::{
    PhysicalBootstrapCatalogDenial, PhysicalReference, PhysicalStoreRuntimeDenial,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BTreeReplaySourceDenial {
    BootstrapCatalog(PhysicalBootstrapCatalogDenial),
    PhysicalOpen(PhysicalStoreRuntimeDenial),
    RootManifestMissing,
    AmbiguousRootManifest {
        candidates: usize,
    },
    NoAdmittedDurableSource,
    DurableSourceBlockedByIntegrity,
    WalOnlyRootNotMaterialized,
    CheckpointRootMismatch {
        expected: PhysicalReference,
        actual: PhysicalReference,
    },
    CheckpointTailIdentityMismatch,
    CheckpointTailFrontierMismatch {
        checkpoint_end: u64,
        tail_start: u64,
    },
}

impl From<PhysicalBootstrapCatalogDenial> for BTreeReplaySourceDenial {
    fn from(value: PhysicalBootstrapCatalogDenial) -> Self {
        Self::BootstrapCatalog(value)
    }
}

impl From<PhysicalStoreRuntimeDenial> for BTreeReplaySourceDenial {
    fn from(value: PhysicalStoreRuntimeDenial) -> Self {
        Self::PhysicalOpen(value)
    }
}
