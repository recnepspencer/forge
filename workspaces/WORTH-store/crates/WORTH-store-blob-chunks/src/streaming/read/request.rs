use crate::{
    BlobCorruptionReferenceEdges, BlobGeneration, BlobObjectId, BlobStreamingContentFrontier,
    BlobStreamingReadDenial, BlobVisibleGeneration, ChunkTreeRoot, LogicalContentDigest,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlobStreamingReadWindow {
    max_resident_bytes: u64,
}

impl BlobStreamingReadWindow {
    pub const fn bounded(max_resident_bytes: u64) -> Result<Self, BlobStreamingReadDenial> {
        if max_resident_bytes == 0 {
            return Err(BlobStreamingReadDenial::EmptyReadWindow);
        }
        Ok(Self { max_resident_bytes })
    }

    pub const fn max_resident_bytes(self) -> u64 {
        self.max_resident_bytes
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobStreamingReadRequest {
    object_id: BlobObjectId,
    generation: BlobGeneration,
    chunk_tree_root: ChunkTreeRoot,
    logical_content_digest: LogicalContentDigest,
    frontier: BlobStreamingContentFrontier,
    corruption_reference_edges: BlobCorruptionReferenceEdges,
}

impl BlobStreamingReadRequest {
    pub fn from_published_generation(
        visible_generation: BlobVisibleGeneration,
        frontier: BlobStreamingContentFrontier,
        corruption_reference_edges: BlobCorruptionReferenceEdges,
    ) -> Result<Self, BlobStreamingReadDenial> {
        if visible_generation.chunk_tree_root() != frontier.chunk_tree_root() {
            return Err(BlobStreamingReadDenial::ChunkTreeRootMismatch);
        }
        if visible_generation.logical_content_digest() != frontier.logical_content_digest() {
            return Err(BlobStreamingReadDenial::LogicalContentDigestMismatch);
        }
        corruption_reference_edges
            .validated_edge_count_for_generation(
                visible_generation.object_id(),
                visible_generation.generation(),
                frontier.chunk_tree_root(),
                frontier.logical_content_digest(),
            )
            .map_err(BlobStreamingReadDenial::CorruptionReferenceEdgeMismatch)?;
        Ok(Self {
            object_id: visible_generation.object_id().clone(),
            generation: visible_generation.generation(),
            chunk_tree_root: visible_generation.chunk_tree_root().clone(),
            logical_content_digest: visible_generation.logical_content_digest().clone(),
            frontier,
            corruption_reference_edges,
        })
    }

    pub const fn object_id(&self) -> &BlobObjectId {
        &self.object_id
    }

    pub const fn generation(&self) -> BlobGeneration {
        self.generation
    }

    pub const fn frontier(&self) -> &BlobStreamingContentFrontier {
        &self.frontier
    }

    pub const fn chunk_tree_root(&self) -> &ChunkTreeRoot {
        &self.chunk_tree_root
    }

    pub const fn logical_content_digest(&self) -> &LogicalContentDigest {
        &self.logical_content_digest
    }

    pub const fn corruption_reference_edges(&self) -> &BlobCorruptionReferenceEdges {
        &self.corruption_reference_edges
    }
}
