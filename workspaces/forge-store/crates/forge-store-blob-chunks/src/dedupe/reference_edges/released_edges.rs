use forge_store_contracts::StableDigest;

use crate::{BlobChunkDedupeCounterSnapshot, BlobChunkIdentity, BlobChunkSecurityMetadataWitness};

use super::reference_set::BlobChunkDedupeReferenceSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobChunkDedupeReferenceRelease {
    shared_identity: BlobChunkIdentity,
    security_metadata: BlobChunkSecurityMetadataWitness,
    counters: BlobChunkDedupeCounterSnapshot,
    released_edges: u64,
    released_reference_identities: Vec<StableDigest>,
}

impl BlobChunkDedupeReferenceRelease {
    pub(super) fn from_denied_set(set: BlobChunkDedupeReferenceSet) -> Self {
        Self::snapshot(&set)
    }

    pub(super) fn snapshot(set: &BlobChunkDedupeReferenceSet) -> Self {
        Self {
            shared_identity: set.shared_identity().clone(),
            security_metadata: set.security_metadata(),
            counters: set.counters(),
            released_edges: set.denied_edges(),
            released_reference_identities: set.released_reference_identities(),
        }
    }

    pub const fn shared_identity(&self) -> &BlobChunkIdentity {
        &self.shared_identity
    }

    pub const fn security_metadata(&self) -> BlobChunkSecurityMetadataWitness {
        self.security_metadata
    }

    pub const fn counters(&self) -> BlobChunkDedupeCounterSnapshot {
        self.counters
    }

    pub const fn released_edges(&self) -> u64 {
        self.released_edges
    }

    pub(crate) fn contains_reference_identity(&self, identity: &StableDigest) -> bool {
        self.released_reference_identities
            .iter()
            .any(|released| released == identity)
    }
}
