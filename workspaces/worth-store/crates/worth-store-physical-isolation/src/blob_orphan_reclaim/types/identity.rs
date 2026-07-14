use crate::CurrentGenerationPhysicalReference;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobOrphanReclaimIdentity {
    pub(super) session_digest: String,
    pub(super) chunk_ordinal: u64,
    pub(super) chunk_digest: String,
    pub(super) durable_bytes: u64,
    pub(super) physical_reference: CurrentGenerationPhysicalReference,
}

impl BlobOrphanReclaimIdentity {
    pub fn session_digest(&self) -> &str {
        &self.session_digest
    }

    pub const fn chunk_ordinal(&self) -> u64 {
        self.chunk_ordinal
    }

    pub fn chunk_digest(&self) -> &str {
        &self.chunk_digest
    }

    pub const fn durable_bytes(&self) -> u64 {
        self.durable_bytes
    }

    pub const fn physical_reference(&self) -> CurrentGenerationPhysicalReference {
        self.physical_reference
    }
}
