use crate::{
    AdmittedBlobChunkSequence, BlobChunkRootCanonicalBasis, BlobChunkRootCounterSnapshot,
    BlobChunkRootPublicationDenial, ChunkTreeRoot, LogicalContentDigest,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobChunkRootPublication {
    chunk_tree_root: ChunkTreeRoot,
    logical_content_digest: LogicalContentDigest,
    canonical_basis: BlobChunkRootCanonicalBasis,
    source_counters: crate::BlobChunkIntegrityCounterSnapshot,
    counters: BlobChunkRootCounterSnapshot,
}

impl BlobChunkRootPublication {
    pub fn publish(
        sequence: AdmittedBlobChunkSequence,
    ) -> Result<Self, BlobChunkRootPublicationDenial> {
        let canonical_basis = BlobChunkRootCanonicalBasis::from_sequence(&sequence)?;
        Ok(Self {
            chunk_tree_root: sequence.chunk_tree_root().clone(),
            logical_content_digest: sequence.logical_content_digest().clone(),
            source_counters: sequence.counters(),
            counters: canonical_basis.counters(),
            canonical_basis,
        })
    }

    pub const fn chunk_tree_root(&self) -> &ChunkTreeRoot {
        &self.chunk_tree_root
    }

    pub const fn logical_content_digest(&self) -> &LogicalContentDigest {
        &self.logical_content_digest
    }

    pub const fn canonical_basis(&self) -> &BlobChunkRootCanonicalBasis {
        &self.canonical_basis
    }

    pub const fn source_counters(&self) -> crate::BlobChunkIntegrityCounterSnapshot {
        self.source_counters
    }

    pub const fn counters(&self) -> BlobChunkRootCounterSnapshot {
        self.counters
    }
}
