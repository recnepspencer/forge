use forge_store_contracts::StableDigest;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobChunkContentDigest {
    digest: StableDigest,
}

impl BlobChunkContentDigest {
    pub(crate) const fn from_integrity_parts(digest: StableDigest) -> Self {
        Self { digest }
    }

    pub const fn digest(&self) -> &StableDigest {
        &self.digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobChunkIdentity {
    chunk_digest: StableDigest,
}

impl BlobChunkIdentity {
    pub(crate) const fn from_integrity_parts(chunk_digest: StableDigest) -> Self {
        Self { chunk_digest }
    }

    pub const fn chunk_digest(&self) -> &StableDigest {
        &self.chunk_digest
    }
}
