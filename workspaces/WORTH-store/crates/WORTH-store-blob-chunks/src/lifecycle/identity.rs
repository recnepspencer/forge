use worth_store_contracts::StableDigest;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobObjectId {
    digest: StableDigest,
}

impl BlobObjectId {
    #[allow(dead_code)]
    pub(crate) const fn from_declared_digest(digest: StableDigest) -> Self {
        Self { digest }
    }

    pub const fn digest(&self) -> &StableDigest {
        &self.digest
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlobGeneration {
    sequence: u64,
}

impl BlobGeneration {
    #[allow(dead_code)]
    pub(crate) const fn published(sequence: u64) -> Self {
        Self { sequence }
    }

    pub const fn sequence(self) -> u64 {
        self.sequence
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChunkTreeRoot {
    digest: StableDigest,
}

impl ChunkTreeRoot {
    #[allow(dead_code)]
    pub(crate) const fn from_declared_digest(digest: StableDigest) -> Self {
        Self { digest }
    }

    pub const fn digest(&self) -> &StableDigest {
        &self.digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogicalContentDigest {
    digest: StableDigest,
}

impl LogicalContentDigest {
    #[allow(dead_code)]
    pub(crate) const fn from_declared_digest(digest: StableDigest) -> Self {
        Self { digest }
    }

    pub const fn digest(&self) -> &StableDigest {
        &self.digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoredChunkDigest {
    digest: StableDigest,
}

impl StoredChunkDigest {
    pub(crate) const fn from_declared_digest(digest: StableDigest) -> Self {
        Self { digest }
    }

    pub const fn digest(&self) -> &StableDigest {
        &self.digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthenticatedFrameDigest {
    digest: StableDigest,
}

impl AuthenticatedFrameDigest {
    #[allow(dead_code)]
    pub(crate) const fn from_declared_digest(digest: StableDigest) -> Self {
        Self { digest }
    }

    pub const fn digest(&self) -> &StableDigest {
        &self.digest
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobAuthorityClassification {
    StoreOwnedPhysicalBlob,
    StoreOwnedExternalPlacement,
    StoreOwnedDerivedBlob,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobLifecycleDeclaration {
    object_id: BlobObjectId,
    generation: BlobGeneration,
    chunk_tree_root: ChunkTreeRoot,
    logical_content_digest: LogicalContentDigest,
    security_metadata: crate::BlobChunkSecurityMetadataWitness,
    stored_chunk_digest: StoredChunkDigest,
    authenticated_frame_digest: AuthenticatedFrameDigest,
    authority_classification: BlobAuthorityClassification,
}

impl BlobLifecycleDeclaration {
    #[allow(dead_code)]
    pub(crate) const fn new(
        object_id: BlobObjectId,
        generation: BlobGeneration,
        chunk_tree_root: ChunkTreeRoot,
        logical_content_digest: LogicalContentDigest,
        security_metadata: crate::BlobChunkSecurityMetadataWitness,
        stored_chunk_digest: StoredChunkDigest,
        authenticated_frame_digest: AuthenticatedFrameDigest,
        authority_classification: BlobAuthorityClassification,
    ) -> Self {
        Self {
            object_id,
            generation,
            chunk_tree_root,
            logical_content_digest,
            security_metadata,
            stored_chunk_digest,
            authenticated_frame_digest,
            authority_classification,
        }
    }

    pub const fn object_id(&self) -> &BlobObjectId {
        &self.object_id
    }

    pub const fn generation(&self) -> BlobGeneration {
        self.generation
    }

    pub const fn chunk_tree_root(&self) -> &ChunkTreeRoot {
        &self.chunk_tree_root
    }

    pub const fn logical_content_digest(&self) -> &LogicalContentDigest {
        &self.logical_content_digest
    }

    pub const fn security_metadata(&self) -> crate::BlobChunkSecurityMetadataWitness {
        self.security_metadata
    }

    pub const fn stored_chunk_digest(&self) -> &StoredChunkDigest {
        &self.stored_chunk_digest
    }

    pub const fn authenticated_frame_digest(&self) -> &AuthenticatedFrameDigest {
        &self.authenticated_frame_digest
    }

    pub const fn authority_classification(&self) -> BlobAuthorityClassification {
        self.authority_classification
    }
}
