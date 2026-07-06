use super::*;

impl BlobChunkDedupeReferenceRelease {
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

impl BlobChunkRegisteredDedupeReference {
    pub(crate) const fn reference_identity(&self) -> &StableDigest {
        &self.reference_identity
    }

    pub const fn shared_identity(&self) -> &BlobChunkIdentity {
        &self.shared_identity
    }

    pub const fn candidate_identity(&self) -> &BlobChunkIdentity {
        &self.candidate_identity
    }

    pub const fn content_digest(&self) -> &StableDigest {
        &self.content_digest
    }

    pub const fn security_metadata(&self) -> BlobChunkSecurityMetadataWitness {
        self.security_metadata
    }

    pub fn contains_chunk_identity(&self, chunk_identity: &BlobChunkIdentity) -> bool {
        &self.shared_identity == chunk_identity || &self.candidate_identity == chunk_identity
    }
}
