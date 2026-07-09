use crate::blob_orphan_reclaim::denial::BlobOrphanReclaimDenial;
use crate::blob_orphan_reclaim::types::identity::BlobOrphanReclaimIdentity;
use crate::CurrentGenerationPhysicalReference;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobPartialChunkOrphan {
    session_digest: String,
    chunk_ordinal: u64,
    chunk_digest: String,
    durable_bytes: u64,
    physical_reference: CurrentGenerationPhysicalReference,
}

impl BlobPartialChunkOrphan {
    pub fn unreached(
        session_digest: impl Into<String>,
        chunk_ordinal: u64,
        chunk_digest: impl Into<String>,
        durable_bytes: u64,
        physical_reference: CurrentGenerationPhysicalReference,
    ) -> Result<Self, BlobOrphanReclaimDenial> {
        let session_digest = session_digest.into();
        let chunk_digest = chunk_digest.into();
        if session_digest.is_empty() {
            return Err(BlobOrphanReclaimDenial::MissingSessionDigest);
        }
        if chunk_digest.is_empty() {
            return Err(BlobOrphanReclaimDenial::MissingChunkDigest);
        }
        if durable_bytes == 0 {
            return Err(BlobOrphanReclaimDenial::EmptyPartialChunk);
        }
        Ok(Self {
            session_digest,
            chunk_ordinal,
            chunk_digest,
            durable_bytes,
            physical_reference,
        })
    }

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

    pub fn reclaim_identity(&self) -> BlobOrphanReclaimIdentity {
        BlobOrphanReclaimIdentity {
            session_digest: self.session_digest.clone(),
            chunk_ordinal: self.chunk_ordinal,
            chunk_digest: self.chunk_digest.clone(),
            durable_bytes: self.durable_bytes,
            physical_reference: self.physical_reference,
        }
    }
}