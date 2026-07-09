use crate::{
    BlobAuthorityClassification, BlobGeneration, BlobGenerationRegistryCounterSnapshot,
    BlobGenerationRegistryDenial, BlobObjectId, ChunkTreeRoot, DerivedBlobRebuildAuthority,
    LifecycleReceipt, LogicalContentDigest,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AuthoritativeBlob;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DerivedBlob;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobObjectClassification {
    Authoritative(AuthoritativeBlob),
    Derived(DerivedBlob),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlobCorruptionClassification {
    classification: BlobObjectClassification,
    counters: BlobGenerationRegistryCounterSnapshot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DerivedBlobRebuildPosture {
    counters: BlobGenerationRegistryCounterSnapshot,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobObjectClassificationAdmission {
    object_id: BlobObjectId,
    generation: BlobGeneration,
    chunk_tree_root: ChunkTreeRoot,
    logical_content_digest: LogicalContentDigest,
    classification: BlobObjectClassification,
    counters: BlobGenerationRegistryCounterSnapshot,
}

impl BlobObjectClassification {
    pub const fn authoritative() -> Self {
        Self::Authoritative(AuthoritativeBlob)
    }

    pub const fn derived() -> Self {
        Self::Derived(DerivedBlob)
    }

    pub const fn is_authoritative(self) -> bool {
        matches!(self, Self::Authoritative(_))
    }

    pub const fn is_derived(self) -> bool {
        matches!(self, Self::Derived(_))
    }
}

impl BlobObjectClassificationAdmission {
    pub fn from_executed_lifecycle(receipt: &LifecycleReceipt) -> Self {
        let declaration = receipt.declaration();
        Self {
            object_id: declaration.object_id().clone(),
            generation: declaration.generation(),
            chunk_tree_root: declaration.chunk_tree_root().clone(),
            logical_content_digest: declaration.logical_content_digest().clone(),
            classification: classification_from_lifecycle_authority(
                declaration.authority_classification(),
            ),
            counters: BlobGenerationRegistryCounterSnapshot::start(),
        }
    }

    pub(crate) const fn object_id(&self) -> &BlobObjectId {
        &self.object_id
    }

    pub(crate) const fn generation(&self) -> BlobGeneration {
        self.generation
    }

    pub(crate) const fn chunk_tree_root(&self) -> &ChunkTreeRoot {
        &self.chunk_tree_root
    }

    pub(crate) const fn logical_content_digest(&self) -> &LogicalContentDigest {
        &self.logical_content_digest
    }

    pub const fn classification(&self) -> BlobObjectClassification {
        self.classification
    }

    pub const fn counters(&self) -> BlobGenerationRegistryCounterSnapshot {
        self.counters
    }
}

impl BlobCorruptionClassification {
    pub(crate) const fn new(
        classification: BlobObjectClassification,
        counters: BlobGenerationRegistryCounterSnapshot,
    ) -> Self {
        Self {
            classification,
            counters,
        }
    }

    pub fn admit_rebuild(
        self,
        authority: DerivedBlobRebuildAuthority,
    ) -> Result<DerivedBlobRebuildPosture, BlobGenerationRegistryDenial> {
        let _current_authority = authority.into_current_authority();
        match self.classification {
            BlobObjectClassification::Derived(_) => Ok(DerivedBlobRebuildPosture {
                counters: self.counters.record_rebuild_admission(),
            }),
            BlobObjectClassification::Authoritative(_) => Err(
                BlobGenerationRegistryDenial::AuthoritativeBlobRequiresAuthoritativeRepair {
                    counters: self.counters.record_denial(),
                },
            ),
        }
    }

    pub const fn deny_rebuild_without_authority(self) -> BlobGenerationRegistryDenial {
        BlobGenerationRegistryDenial::DerivedRebuildAuthorityRequired {
            counters: self.counters.record_denial(),
        }
    }

    pub const fn classification(self) -> BlobObjectClassification {
        self.classification
    }

    pub const fn counters(self) -> BlobGenerationRegistryCounterSnapshot {
        self.counters
    }
}

impl DerivedBlobRebuildPosture {
    pub const fn counters(self) -> BlobGenerationRegistryCounterSnapshot {
        self.counters
    }
}

const fn classification_from_lifecycle_authority(
    authority: BlobAuthorityClassification,
) -> BlobObjectClassification {
    match authority {
        BlobAuthorityClassification::StoreOwnedPhysicalBlob
        | BlobAuthorityClassification::StoreOwnedExternalPlacement => {
            BlobObjectClassification::Authoritative(AuthoritativeBlob)
        }
        BlobAuthorityClassification::StoreOwnedDerivedBlob => {
            BlobObjectClassification::Derived(DerivedBlob)
        }
    }
}
