use forge_store_contracts::StableDigest;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobDedupeReceipt {
    content_digest: StableDigest,
}

impl BlobDedupeReceipt {
    pub const fn content_digest(&self) -> &StableDigest {
        &self.content_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobReachabilityReceipt {
    content_digest: StableDigest,
}

impl BlobReachabilityReceipt {
    pub const fn content_digest(&self) -> &StableDigest {
        &self.content_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobResumabilityReceipt {
    content_digest: StableDigest,
}

impl BlobResumabilityReceipt {
    pub const fn content_digest(&self) -> &StableDigest {
        &self.content_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobRetentionReceipt {
    content_digest: StableDigest,
}

impl BlobRetentionReceipt {
    pub const fn content_digest(&self) -> &StableDigest {
        &self.content_digest
    }
}
