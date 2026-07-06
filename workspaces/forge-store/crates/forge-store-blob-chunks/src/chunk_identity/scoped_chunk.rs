use forge_store_security::StoreSecurityScopeIdentity;

use crate::{
    BlobChunkContentDigest, BlobChunkIdentity, BlobChunkIntegrityProof,
    BlobChunkSecurityMetadataWitness, StoredChunkDigest,
};

#[derive(Debug, PartialEq, Eq)]
pub struct ScopedBlobChunk {
    identity: BlobChunkIdentity,
    stored_digest: StoredChunkDigest,
    content_digest: BlobChunkContentDigest,
    security_metadata: BlobChunkSecurityMetadataWitness,
    bytes_observed: u64,
}

impl ScopedBlobChunk {
    pub fn from_integrity_proof(proof: BlobChunkIntegrityProof) -> Self {
        let content_digest = proof.content_digest().clone();
        Self {
            identity: proof.identity().clone(),
            stored_digest: proof.stored_digest().clone(),
            content_digest,
            security_metadata: proof.security_metadata(),
            bytes_observed: proof.byte_range().len(),
        }
    }

    pub const fn identity(&self) -> &BlobChunkIdentity {
        &self.identity
    }

    pub const fn stored_digest(&self) -> &StoredChunkDigest {
        &self.stored_digest
    }

    pub const fn content_digest(&self) -> &forge_store_contracts::StableDigest {
        self.content_digest.digest()
    }

    pub const fn security_scope(&self) -> StoreSecurityScopeIdentity {
        self.security_metadata.identity()
    }

    pub const fn security_metadata(&self) -> BlobChunkSecurityMetadataWitness {
        self.security_metadata
    }

    pub const fn bytes_observed(&self) -> u64 {
        self.bytes_observed
    }
}
