use super::candidate::BlobChunkDedupeCandidate;
use crate::{BlobChunkContentDigest, BlobChunkIdentity, BlobChunkSecurityMetadataWitness};
use worth_foundational::CanonicalEquivalenceBasis;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobChunkCanonicalEquivalence {
    basis: CanonicalEquivalenceBasis,
    existing_identity: BlobChunkIdentity,
    candidate_identity: BlobChunkIdentity,
    content_digest: BlobChunkContentDigest,
    security_metadata: BlobChunkSecurityMetadataWitness,
}

impl BlobChunkCanonicalEquivalence {
    pub(crate) fn from_exact_canonical_basis(
        existing_identity: BlobChunkIdentity,
        candidate_identity: BlobChunkIdentity,
        content_digest: BlobChunkContentDigest,
        security_metadata: BlobChunkSecurityMetadataWitness,
    ) -> Self {
        Self {
            basis: CanonicalEquivalenceBasis::ExactCanonicalBasis,
            existing_identity,
            candidate_identity,
            content_digest,
            security_metadata,
        }
    }

    #[cfg(test)]
    pub(crate) fn forced_digest_collision_fixture(
        existing: &BlobChunkDedupeCandidate,
        candidate: &BlobChunkDedupeCandidate,
    ) -> Self {
        Self {
            basis: CanonicalEquivalenceBasis::ExactCanonicalBasis,
            existing_identity: existing.identity.clone(),
            candidate_identity: candidate.identity.clone(),
            content_digest: candidate.content_digest.clone(),
            security_metadata: candidate.security_metadata,
        }
    }

    pub const fn basis(&self) -> CanonicalEquivalenceBasis {
        self.basis
    }

    pub const fn existing_identity(&self) -> &BlobChunkIdentity {
        &self.existing_identity
    }

    pub const fn candidate_identity(&self) -> &BlobChunkIdentity {
        &self.candidate_identity
    }

    pub(crate) fn matches_candidates(
        &self,
        existing: &BlobChunkDedupeCandidate,
        candidate: &BlobChunkDedupeCandidate,
    ) -> bool {
        self.existing_identity == existing.identity
            && self.candidate_identity == candidate.identity
            && self.content_digest == candidate.content_digest
            && self.content_digest == existing.content_digest
            && self.security_metadata == candidate.security_metadata
            && self.security_metadata == existing.security_metadata
    }
}
