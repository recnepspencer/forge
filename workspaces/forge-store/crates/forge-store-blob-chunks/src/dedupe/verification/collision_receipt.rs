use crate::{
    BlobChunkContentDigest, BlobChunkDedupeCounterSnapshot, BlobChunkIdentity,
    BlobChunkIntegrityProof, BlobChunkSecurityMetadataWitness,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobChunkCollisionVerificationReceipt {
    existing_proof: BlobChunkIntegrityProof,
    candidate_proof: BlobChunkIntegrityProof,
    content_digest: BlobChunkContentDigest,
    security_metadata: BlobChunkSecurityMetadataWitness,
    bytes_compared: u64,
    counters: BlobChunkDedupeCounterSnapshot,
}

impl BlobChunkCollisionVerificationReceipt {
    pub(crate) fn from_verified_identity_mismatch(
        existing_proof: BlobChunkIntegrityProof,
        candidate_proof: BlobChunkIntegrityProof,
        content_digest: BlobChunkContentDigest,
        security_metadata: BlobChunkSecurityMetadataWitness,
        bytes_compared: u64,
        counters: BlobChunkDedupeCounterSnapshot,
    ) -> Self {
        Self {
            existing_proof,
            candidate_proof,
            content_digest,
            security_metadata,
            bytes_compared,
            counters,
        }
    }

    pub const fn existing_identity(&self) -> &BlobChunkIdentity {
        self.existing_proof.identity()
    }

    pub const fn candidate_identity(&self) -> &BlobChunkIdentity {
        self.candidate_proof.identity()
    }

    pub const fn existing_proof(&self) -> &BlobChunkIntegrityProof {
        &self.existing_proof
    }

    pub const fn candidate_proof(&self) -> &BlobChunkIntegrityProof {
        &self.candidate_proof
    }

    pub const fn content_digest(&self) -> &BlobChunkContentDigest {
        &self.content_digest
    }

    pub const fn security_metadata(&self) -> BlobChunkSecurityMetadataWitness {
        self.security_metadata
    }

    pub const fn bytes_compared(&self) -> u64 {
        self.bytes_compared
    }

    pub const fn counters(&self) -> BlobChunkDedupeCounterSnapshot {
        self.counters
    }
}