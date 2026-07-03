use forge_store_contracts::StableDigest;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobChunkIdentity {
    content_digest: StableDigest,
}

impl BlobChunkIdentity {
    pub const fn from_digest(content_digest: StableDigest) -> Self {
        Self { content_digest }
    }

    pub const fn content_digest(&self) -> &StableDigest {
        &self.content_digest
    }
}
