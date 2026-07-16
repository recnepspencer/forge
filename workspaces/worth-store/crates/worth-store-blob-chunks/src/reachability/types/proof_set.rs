use worth_store_contracts::StableDigest;

use crate::reachability::counters::BlobReachabilityCounterSnapshot;
use crate::reachability::edges::BlobReachabilityAuthorityKey;
use crate::reachability::receipt_construction::BlobReachabilityCanonicalSnapshot;
use crate::{
    BlobChunkIdentity, BlobChunkSecurityMetadataWitness, BlobLifecycleDeclaration,
    BlobPublicationIntent, StoredChunkDigest,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobChunkReachabilityProofSet {
    pub(super) authority: BlobReachabilityAuthorityKey,
    pub(super) reachable_chunks: Vec<BlobChunkIdentity>,
    pub(super) stored_digest: StoredChunkDigest,
    pub(super) security_metadata: BlobChunkSecurityMetadataWitness,
    pub(super) reference_edges: Vec<StableDigest>,
    pub(super) protected_holds: Vec<StableDigest>,
    pub(super) orphan_candidates: Vec<BlobChunkIdentity>,
    pub(super) counters: BlobReachabilityCounterSnapshot,
}

pub(crate) struct BlobReachabilityProofSetParts {
    pub(crate) authority: BlobReachabilityAuthorityKey,
    pub(crate) reachable_chunks: Vec<BlobChunkIdentity>,
    pub(crate) stored_digest: StoredChunkDigest,
    pub(crate) security_metadata: BlobChunkSecurityMetadataWitness,
    pub(crate) reference_edges: Vec<StableDigest>,
    pub(crate) protected_holds: Vec<StableDigest>,
    pub(crate) orphan_candidates: Vec<BlobChunkIdentity>,
    pub(crate) counters: BlobReachabilityCounterSnapshot,
}

impl BlobChunkReachabilityProofSet {
    pub(crate) fn construct(parts: BlobReachabilityProofSetParts) -> Self {
        Self {
            authority: parts.authority,
            reachable_chunks: parts.reachable_chunks,
            stored_digest: parts.stored_digest,
            security_metadata: parts.security_metadata,
            reference_edges: parts.reference_edges,
            protected_holds: parts.protected_holds,
            orphan_candidates: parts.orphan_candidates,
            counters: parts.counters,
        }
    }

    pub(crate) fn matches_lifecycle_declaration(
        &self,
        declaration: &BlobLifecycleDeclaration,
    ) -> bool {
        self.authority.matches_declaration(declaration)
    }

    pub(crate) fn matches_publication_intent(&self, intent: &BlobPublicationIntent) -> bool {
        self.authority.matches_publication_intent(intent)
    }

    pub fn reachable_chunks(&self) -> &[BlobChunkIdentity] {
        &self.reachable_chunks
    }

    pub fn reference_edges(&self) -> &[StableDigest] {
        &self.reference_edges
    }

    pub const fn stored_digest(&self) -> &StoredChunkDigest {
        &self.stored_digest
    }

    pub const fn security_metadata(&self) -> BlobChunkSecurityMetadataWitness {
        self.security_metadata
    }

    pub fn protected_holds(&self) -> &[StableDigest] {
        &self.protected_holds
    }

    pub fn orphan_candidates(&self) -> &[BlobChunkIdentity] {
        &self.orphan_candidates
    }

    pub const fn counters(&self) -> BlobReachabilityCounterSnapshot {
        self.counters
    }

    pub(crate) fn into_canonical_snapshot(
        self,
        counters: BlobReachabilityCounterSnapshot,
    ) -> BlobReachabilityCanonicalSnapshot {
        let counters = counters
            .with_reachable_chunks(self.reachable_chunks.len() as u64)
            .with_orphan_candidates(self.orphan_candidates.len() as u64);
        BlobReachabilityCanonicalSnapshot::from_parts(
            self.reachable_chunks
                .into_iter()
                .map(|chunk| chunk.chunk_digest().as_str().to_owned())
                .collect(),
            self.reference_edges
                .into_iter()
                .map(|edge| edge.as_str().to_owned())
                .collect(),
            self.protected_holds
                .into_iter()
                .map(|hold| hold.as_str().to_owned())
                .collect(),
            counters,
        )
    }
}
