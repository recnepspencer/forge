use crate::{
    BlobChunkContentDigest, BlobChunkIdentity, BlobChunkIntegrityProof,
    BlobChunkSecurityMetadataWitness,
};
use worth_store_contracts::StableDigest;

#[derive(Debug, PartialEq, Eq)]
pub struct BlobChunkDedupeCandidate {
    pub(crate) proof: BlobChunkIntegrityProof,
    pub(crate) identity: BlobChunkIdentity,
    pub(crate) content_digest: BlobChunkContentDigest,
    pub(crate) security_metadata: BlobChunkSecurityMetadataWitness,
}

impl BlobChunkDedupeCandidate {
    pub fn from_integrity_proof(proof: BlobChunkIntegrityProof) -> Self {
        Self {
            identity: proof.identity().clone(),
            content_digest: proof.content_digest().clone(),
            security_metadata: proof.security_metadata(),
            proof,
        }
    }

    #[cfg(test)]
    pub(crate) fn with_forced_content_digest_for_collision_fixture(
        mut self,
        content_digest: StableDigest,
    ) -> Self {
        self.content_digest = BlobChunkContentDigest::from_integrity_parts(content_digest);
        self
    }

    pub const fn identity(&self) -> &BlobChunkIdentity {
        &self.identity
    }

    pub const fn content_digest(&self) -> &StableDigest {
        self.content_digest.digest()
    }

    pub(crate) const fn content_digest_witness(&self) -> &BlobChunkContentDigest {
        &self.content_digest
    }

    pub const fn proof(&self) -> &BlobChunkIntegrityProof {
        &self.proof
    }

    pub const fn security_metadata(&self) -> BlobChunkSecurityMetadataWitness {
        self.security_metadata
    }

    pub const fn security_scope(&self) -> worth_store_security::StoreSecurityScopeIdentity {
        self.security_metadata.identity()
    }
}
