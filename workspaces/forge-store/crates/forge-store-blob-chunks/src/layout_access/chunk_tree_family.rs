use forge_store_contracts::DurableArtifactFamilyId;

use super::{BlobLayoutAccessDenial, BlobLayoutAccessDenialKind, BlobLayoutAccessPathEvidence};
use crate::{
    BlobStreamingContentFrontier, BlobStreamingVerifiedRead, ChunkTreeRoot, LogicalContentDigest,
};

use super::blob_object_family::BlobObjectLayoutReport;

pub fn reject_streaming_frontier_as_chunk_tree_layout_authority(
    _frontier: &BlobStreamingContentFrontier,
) -> Result<(), BlobLayoutAccessDenial> {
    Err(BlobLayoutAccessDenial::new(
        BlobLayoutAccessDenialKind::StreamingFrontierCannotStandInForChunkTreeLayoutAuthority,
    ))
}

impl ChunkTreeLayoutReport {
    fn admit_chunk_tree(
        blob: &BlobObjectLayoutReport,
        read: &BlobStreamingVerifiedRead,
    ) -> Result<ChunkTreeLayoutReport, BlobLayoutAccessDenial> {
        if blob.object_id() != read.object_id()
            || blob.generation() != read.generation()
            || blob.chunk_tree_root() != read.chunk_tree_root()
            || blob.logical_content_digest() != read.logical_content_digest()
        {
            return Err(BlobLayoutAccessDenial::new(
                BlobLayoutAccessDenialKind::PublishedGenerationDoesNotMatchVerifiedRead,
            ));
        }
        Ok(ChunkTreeLayoutReport::from_read(read))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChunkTreeLayoutReport {
    family_id: DurableArtifactFamilyId,
    chunk_tree_root: ChunkTreeRoot,
    logical_content_digest: LogicalContentDigest,
    verified_chunks: u64,
    counter_evidence: BlobLayoutAccessPathEvidence,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoredChunkLookupLayoutReport {
    family_id: DurableArtifactFamilyId,
    chunk_tree_root: ChunkTreeRoot,
    lookup_chunks: u64,
    counter_evidence: BlobLayoutAccessPathEvidence,
}

impl ChunkTreeLayoutReport {
    fn from_read(read: &BlobStreamingVerifiedRead) -> Self {
        let family_id = DurableArtifactFamilyId::BlobChunk;
        Self {
            family_id,
            chunk_tree_root: read.chunk_tree_root().clone(),
            logical_content_digest: read.logical_content_digest().clone(),
            verified_chunks: read.counters().chunks_verified(),
            counter_evidence: BlobLayoutAccessPathEvidence::from_streaming(
                family_id,
                read.counters(),
            ),
        }
    }

    pub fn admit_stored_chunk_lookup_layout(&self) -> StoredChunkLookupLayoutReport {
        StoredChunkLookupLayoutReport {
            family_id: DurableArtifactFamilyId::BlobChunk,
            chunk_tree_root: self.chunk_tree_root.clone(),
            lookup_chunks: self.verified_chunks,
            counter_evidence: self.counter_evidence,
        }
    }

    pub const fn family_id(&self) -> DurableArtifactFamilyId {
        self.family_id
    }

    pub const fn chunk_tree_root(&self) -> &ChunkTreeRoot {
        &self.chunk_tree_root
    }

    pub const fn logical_content_digest(&self) -> &LogicalContentDigest {
        &self.logical_content_digest
    }

    pub const fn verified_chunks(&self) -> u64 {
        self.verified_chunks
    }

    pub const fn counter_evidence(&self) -> BlobLayoutAccessPathEvidence {
        self.counter_evidence
    }
}

impl StoredChunkLookupLayoutReport {
    pub const fn family_id(&self) -> DurableArtifactFamilyId {
        self.family_id
    }

    pub const fn chunk_tree_root(&self) -> &ChunkTreeRoot {
        &self.chunk_tree_root
    }

    pub const fn lookup_chunks(&self) -> u64 {
        self.lookup_chunks
    }

    pub const fn counter_evidence(&self) -> BlobLayoutAccessPathEvidence {
        self.counter_evidence
    }
}

impl BlobObjectLayoutReport {
    pub fn admit_chunk_tree_layout(
        &self,
        read: &BlobStreamingVerifiedRead,
    ) -> Result<ChunkTreeLayoutReport, BlobLayoutAccessDenial> {
        ChunkTreeLayoutReport::admit_chunk_tree(self, read)
    }
}
