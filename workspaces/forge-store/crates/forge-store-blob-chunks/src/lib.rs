//! S.3 chunk integrity reports cannot satisfy S.7 dedupe receipts:
//! ```compile_fail
//! use forge_store_blob_chunks::BlobDedupeReceipt;
//! use forge_store_physical_integrity::ChunkIntegrityReport;
//!
//! fn requires_dedupe_receipt(_: BlobDedupeReceipt) {}
//!
//! let report: ChunkIntegrityReport = todo!();
//! requires_dedupe_receipt(report);
//! ```
//! S.3 chunk integrity reports cannot satisfy S.7 reachability receipts:
//! ```compile_fail
//! use forge_store_blob_chunks::BlobReachabilityReceipt;
//! use forge_store_physical_integrity::ChunkIntegrityReport;
//!
//! fn requires_reachability_receipt(_: BlobReachabilityReceipt) {}
//!
//! let report: ChunkIntegrityReport = todo!();
//! requires_reachability_receipt(report);
//! ```
//! S.3 chunk integrity reports cannot satisfy S.7 resumability receipts:
//! ```compile_fail
//! use forge_store_blob_chunks::BlobResumabilityReceipt;
//! use forge_store_physical_integrity::ChunkIntegrityReport;
//!
//! fn requires_resumability_receipt(_: BlobResumabilityReceipt) {}
//!
//! let report: ChunkIntegrityReport = todo!();
//! requires_resumability_receipt(report);
//! ```
//! S.3 chunk integrity reports cannot satisfy S.7 retention receipts:
//! ```compile_fail
//! use forge_store_blob_chunks::BlobRetentionReceipt;
//! use forge_store_physical_integrity::ChunkIntegrityReport;
//!
//! fn requires_retention_receipt(_: BlobRetentionReceipt) {}
//!
//! let report: ChunkIntegrityReport = todo!();
//! requires_retention_receipt(report);
//! ```
//! S.7 digest-derived blob identity cannot satisfy S.3 chunk integrity:
//! ```compile_fail
//! use forge_store_blob_chunks::BlobChunkIdentity;
//! use forge_store_physical_integrity::ChunkIntegrityReport;
//!
//! fn requires_chunk_integrity(_: ChunkIntegrityReport) {}
//!
//! let identity: BlobChunkIdentity = todo!();
//! requires_chunk_integrity(identity);
//! ```

#![forbid(unsafe_code)]

mod blob_lifecycle_receipts;
mod large_record_streaming_envelope;

pub use blob_lifecycle_receipts::{
    BlobDedupeReceipt, BlobReachabilityReceipt, BlobResumabilityReceipt, BlobRetentionReceipt,
};
use forge_store_contracts::StableDigest;
pub use large_record_streaming_envelope::{
    LargeRecordStreamingEnvelope, LargeRecordStreamingEnvelopeDenial,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobChunkIdentity {
    content_digest: StableDigest,
}

impl BlobChunkIdentity {
    pub const fn from_digest(content_digest: StableDigest) -> Self {
        Self { content_digest }
    }
}
