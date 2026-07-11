use forge_store_operations_vocabulary::ImportPlacementPlan;

use crate::{
    BlobChunkIdentity, BlobChunkSecurityMetadataWitness, BlobGeneration, BlobObjectId,
    ChunkTreeRoot, LogicalContentDigest, StoredChunkDigest,
};

use super::counters::BlobImportReadmissionCounters;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobImportReadmissionReceipt {
    security_metadata: BlobChunkSecurityMetadataWitness,
    counters: BlobImportReadmissionCounters,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportedBlobWitness {
    object_id: BlobObjectId,
    generation: BlobGeneration,
    chunk_tree_root: ChunkTreeRoot,
    logical_content_digest: LogicalContentDigest,
    security_metadata: BlobChunkSecurityMetadataWitness,
    reachable_chunks: Vec<BlobChunkIdentity>,
    stored_digest: StoredChunkDigest,
    placement_plan: ImportPlacementPlan,
    counters: BlobImportReadmissionCounters,
}

impl BlobImportReadmissionReceipt {
    pub(crate) const fn new(
        security_metadata: BlobChunkSecurityMetadataWitness,
        counters: BlobImportReadmissionCounters,
    ) -> Self {
        Self {
            security_metadata,
            counters,
        }
    }

    pub const fn security_metadata(&self) -> BlobChunkSecurityMetadataWitness {
        self.security_metadata
    }

    pub const fn counters(&self) -> BlobImportReadmissionCounters {
        self.counters
    }
}

impl ImportedBlobWitness {
    pub(crate) fn new(
        object_id: BlobObjectId,
        generation: BlobGeneration,
        chunk_tree_root: ChunkTreeRoot,
        logical_content_digest: LogicalContentDigest,
        security_metadata: BlobChunkSecurityMetadataWitness,
        reachable_chunks: Vec<BlobChunkIdentity>,
        stored_digest: StoredChunkDigest,
        placement_plan: ImportPlacementPlan,
        counters: BlobImportReadmissionCounters,
    ) -> Self {
        Self {
            object_id,
            generation,
            chunk_tree_root,
            logical_content_digest,
            security_metadata,
            reachable_chunks,
            stored_digest,
            placement_plan,
            counters,
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

    pub fn reachable_chunks(&self) -> &[BlobChunkIdentity] {
        &self.reachable_chunks
    }

    pub const fn stored_digest(&self) -> &StoredChunkDigest {
        &self.stored_digest
    }

    pub const fn placement_plan(&self) -> ImportPlacementPlan {
        self.placement_plan
    }

    pub const fn counters(&self) -> BlobImportReadmissionCounters {
        self.counters
    }
}
