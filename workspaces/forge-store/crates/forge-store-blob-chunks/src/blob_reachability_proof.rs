use forge_store_security::StoreSecurityScopeIdentity;

use crate::{
    BlobChunkIdentity, BlobChunkSecurityMetadataWitness, ScopedBlobChunk, StoredChunkDigest,
};

#[derive(Debug, PartialEq, Eq)]
pub struct BlobReachabilityProof {
    chunk_identity: BlobChunkIdentity,
    stored_digest: StoredChunkDigest,
    security_metadata: BlobChunkSecurityMetadataWitness,
    reachable_bytes: u64,
}

impl BlobReachabilityProof {
    pub fn from_scoped_chunk(scoped_chunk: ScopedBlobChunk) -> Self {
        Self {
            chunk_identity: scoped_chunk.identity().clone(),
            stored_digest: scoped_chunk.stored_digest().clone(),
            security_metadata: scoped_chunk.security_metadata(),
            reachable_bytes: scoped_chunk.bytes_observed(),
        }
    }

    pub const fn chunk_identity(&self) -> &BlobChunkIdentity {
        &self.chunk_identity
    }

    pub const fn stored_digest(&self) -> &StoredChunkDigest {
        &self.stored_digest
    }

    pub const fn security_scope(&self) -> StoreSecurityScopeIdentity {
        self.security_metadata.identity()
    }

    pub const fn security_metadata(&self) -> BlobChunkSecurityMetadataWitness {
        self.security_metadata
    }

    pub const fn reachable_bytes(&self) -> u64 {
        self.reachable_bytes
    }
}
