use crate::{BlobChunkByteRange, BlobChunkIdentity, BlobChunkOrdinal, StoredChunkDigest};

use super::chunk_bytes::BlobExportedChunkBytes;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobExportChunkManifestRow {
    chunk_identity: BlobChunkIdentity,
    stored_digest: StoredChunkDigest,
    checksum_digest: worth_store_contracts::StableDigest,
    ordinal: BlobChunkOrdinal,
    range: BlobChunkByteRange,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobExportManifest {
    export_name: String,
    rows: Vec<BlobExportChunkManifestRow>,
}

impl BlobExportChunkManifestRow {
    pub(crate) fn from_collected_chunk(input: &BlobExportedChunkBytes<'_>) -> Self {
        Self {
            chunk_identity: input.leaf().identity().clone(),
            stored_digest: input.leaf().stored_digest().clone(),
            checksum_digest: input.leaf().checksum_digest().clone(),
            ordinal: input.leaf().ordinal(),
            range: input.leaf().byte_range(),
        }
    }

    pub fn chunk_identity(&self) -> &BlobChunkIdentity {
        &self.chunk_identity
    }

    pub fn stored_digest(&self) -> &StoredChunkDigest {
        &self.stored_digest
    }

    #[cfg(feature = "certification-test-authority")]
    pub(crate) const fn ordinal(&self) -> BlobChunkOrdinal {
        self.ordinal
    }

    #[cfg(feature = "certification-test-authority")]
    pub(crate) const fn range(&self) -> BlobChunkByteRange {
        self.range
    }
}

impl BlobExportManifest {
    pub(crate) fn new(export_name: String, rows: Vec<BlobExportChunkManifestRow>) -> Self {
        Self { export_name, rows }
    }

    pub fn export_name(&self) -> &str {
        &self.export_name
    }

    pub fn rows(&self) -> &[BlobExportChunkManifestRow] {
        &self.rows
    }
}
