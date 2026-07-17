use super::BlobImportPlacementPlan;

use crate::{
    BlobChunkIdentity, BlobChunkSecurityMetadataWitness, BlobGeneration, BlobObjectId,
    ChunkTreeRoot, LogicalContentDigest, StoredChunkDigest,
};

use super::counters::BlobImportReadmissionCounters;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobImportReadmissionReceipt {
    security_metadata: BlobChunkSecurityMetadataWitness,
    authority_identity: worth_store_authority::StoreCurrentAuthorityIdentity,
    counters: BlobImportReadmissionCounters,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportedBlobWitness {
    object_id: BlobObjectId,
    generation: BlobGeneration,
    chunk_tree_root: ChunkTreeRoot,
    logical_content_digest: LogicalContentDigest,
    security_metadata: BlobChunkSecurityMetadataWitness,
    authority_identity: worth_store_authority::StoreCurrentAuthorityIdentity,
    reachable_chunks: Vec<BlobChunkIdentity>,
    stored_digest: StoredChunkDigest,
    placement_plan: BlobImportPlacementPlan,
    counters: BlobImportReadmissionCounters,
}

pub(crate) struct ImportedBlobWitnessParts {
    pub(super) object_id: BlobObjectId,
    pub(super) generation: BlobGeneration,
    pub(super) chunk_tree_root: ChunkTreeRoot,
    pub(super) logical_content_digest: LogicalContentDigest,
    pub(super) security_metadata: BlobChunkSecurityMetadataWitness,
    pub(super) authority_identity: worth_store_authority::StoreCurrentAuthorityIdentity,
    pub(super) reachable_chunks: Vec<BlobChunkIdentity>,
    pub(super) stored_digest: StoredChunkDigest,
    pub(super) placement_plan: BlobImportPlacementPlan,
    pub(super) counters: BlobImportReadmissionCounters,
}

impl BlobImportReadmissionReceipt {
    pub(crate) const fn new(
        security_metadata: BlobChunkSecurityMetadataWitness,
        authority_identity: worth_store_authority::StoreCurrentAuthorityIdentity,
        counters: BlobImportReadmissionCounters,
    ) -> Self {
        Self {
            security_metadata,
            authority_identity,
            counters,
        }
    }

    pub const fn security_metadata(&self) -> BlobChunkSecurityMetadataWitness {
        self.security_metadata
    }

    pub const fn counters(&self) -> BlobImportReadmissionCounters {
        self.counters
    }

    pub const fn authority_identity(&self) -> worth_store_authority::StoreCurrentAuthorityIdentity {
        self.authority_identity
    }
}

impl ImportedBlobWitness {
    pub(crate) fn new(parts: ImportedBlobWitnessParts) -> Self {
        Self {
            object_id: parts.object_id,
            generation: parts.generation,
            chunk_tree_root: parts.chunk_tree_root,
            logical_content_digest: parts.logical_content_digest,
            security_metadata: parts.security_metadata,
            authority_identity: parts.authority_identity,
            reachable_chunks: parts.reachable_chunks,
            stored_digest: parts.stored_digest,
            placement_plan: parts.placement_plan,
            counters: parts.counters,
        }
    }

    pub fn object_id(&self) -> &BlobObjectId {
        &self.object_id
    }

    pub const fn generation(&self) -> BlobGeneration {
        self.generation
    }

    pub fn chunk_tree_root(&self) -> &ChunkTreeRoot {
        &self.chunk_tree_root
    }

    pub fn logical_content_digest(&self) -> &LogicalContentDigest {
        &self.logical_content_digest
    }

    pub const fn security_metadata(&self) -> BlobChunkSecurityMetadataWitness {
        self.security_metadata
    }

    pub const fn authority_identity(&self) -> worth_store_authority::StoreCurrentAuthorityIdentity {
        self.authority_identity
    }

    pub fn reachable_chunks(&self) -> &[BlobChunkIdentity] {
        &self.reachable_chunks
    }

    pub const fn stored_digest(&self) -> &StoredChunkDigest {
        &self.stored_digest
    }

    pub const fn placement_plan(&self) -> BlobImportPlacementPlan {
        self.placement_plan
    }

    pub const fn counters(&self) -> BlobImportReadmissionCounters {
        self.counters
    }
}
