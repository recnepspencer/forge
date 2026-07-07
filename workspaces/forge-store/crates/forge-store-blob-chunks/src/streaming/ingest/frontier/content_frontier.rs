use forge_store_contracts::StableDigest;

use crate::{
    AdmittedBlobChunkSequence, BlobChunkProofFrontier, ChunkTreeRoot, LogicalContentDigest,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobStreamingContentFrontier {
    proof_frontier: BlobChunkProofFrontier,
    chunk_identity_summary: StableDigest,
    chunk_tree_root: ChunkTreeRoot,
    logical_content_digest: LogicalContentDigest,
}

impl BlobStreamingContentFrontier {
    pub(crate) fn from_sequence(sequence: &AdmittedBlobChunkSequence) -> Self {
        Self {
            proof_frontier: sequence.proof_frontier().clone(),
            chunk_identity_summary: sequence.chunk_identity_summary().clone(),
            chunk_tree_root: sequence.chunk_tree_root().clone(),
            logical_content_digest: sequence.logical_content_digest().clone(),
        }
    }

    pub const fn proof_frontier(&self) -> &BlobChunkProofFrontier {
        &self.proof_frontier
    }

    pub const fn chunk_identity_summary(&self) -> &StableDigest {
        &self.chunk_identity_summary
    }

    pub const fn chunk_tree_root(&self) -> &ChunkTreeRoot {
        &self.chunk_tree_root
    }

    pub const fn logical_content_digest(&self) -> &LogicalContentDigest {
        &self.logical_content_digest
    }
}
