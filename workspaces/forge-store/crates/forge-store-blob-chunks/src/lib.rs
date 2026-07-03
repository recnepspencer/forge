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
//! Digest-derived blob identity cannot satisfy blob security scope:
//! ```compile_fail
//! use forge_store_blob_chunks::{BlobChunkIdentity, BlobChunkSecurityScope};
//!
//! fn requires_blob_scope(_: BlobChunkSecurityScope) {}
//!
//! let identity: BlobChunkIdentity = todo!();
//! requires_blob_scope(identity);
//! ```
//! Digest-derived blob identity cannot enter dedupe admission:
//! ```compile_fail
//! use forge_store_blob_chunks::{BlobChunkDedupeCandidate, BlobChunkIdentity};
//!
//! fn requires_dedupe_candidate(_: BlobChunkDedupeCandidate) {}
//!
//! let identity: BlobChunkIdentity = todo!();
//! requires_dedupe_candidate(identity);
//! ```
//! Blob dedupe candidates are move-only proof carriers:
//! ```compile_fail
//! use forge_store_blob_chunks::BlobChunkDedupeCandidate;
//!
//! let candidate: BlobChunkDedupeCandidate = todo!();
//! let _copy = candidate.clone();
//! ```
//! Stable digests cannot satisfy candidate-bound canonical equivalence:
//! ```compile_fail
//! use forge_store_blob_chunks::BlobChunkCanonicalEquivalence;
//! use forge_store_contracts::StableDigest;
//!
//! fn requires_equivalence(_: BlobChunkCanonicalEquivalence) {}
//!
//! let digest: StableDigest = todo!();
//! requires_equivalence(digest);
//! ```
//! Chunk integrity reports cannot satisfy blob residency proof:
//! ```compile_fail
//! use forge_store_blob_chunks::BlobChunkStreamingResidencyProof;
//! use forge_store_physical_integrity::ChunkIntegrityReport;
//!
//! fn requires_residency(_: BlobChunkStreamingResidencyProof) {}
//!
//! let report: ChunkIntegrityReport = todo!();
//! requires_residency(report);
//! ```
//! Copied counters cannot satisfy blob dedupe share claims:
//! ```compile_fail
//! use forge_store_blob_chunks::{BlobChunkDedupeCounterSnapshot, BlobChunkDedupeShareClaim};
//!
//! fn requires_share_claim(_: BlobChunkDedupeShareClaim) {}
//!
//! let counters: BlobChunkDedupeCounterSnapshot = todo!();
//! requires_share_claim(counters);
//! ```
//! Copied scope counters cannot satisfy blob security scope:
//! ```compile_fail
//! use forge_store_blob_chunks::{BlobChunkScopeCounterSnapshot, BlobChunkSecurityScope};
//!
//! fn requires_scope(_: BlobChunkSecurityScope) {}
//!
//! let counters: BlobChunkScopeCounterSnapshot = todo!();
//! requires_scope(counters);
//! ```

#![forbid(unsafe_code)]

mod blob_chunk_counters;
mod blob_chunk_dedupe;
mod blob_chunk_denial;
mod blob_chunk_identity;
mod blob_chunk_scope;
#[cfg(test)]
mod blob_chunk_scope_tests;
mod blob_chunk_streaming;
#[cfg(test)]
mod blob_chunk_test_support;
mod blob_lifecycle_receipts;
mod large_record_streaming_envelope;
mod s7_blob_security_handoff;

pub use blob_chunk_counters::{
    BlobChunkDedupeCounterSnapshot, BlobChunkScopeCounterSnapshot,
    BlobChunkStreamingCounterSnapshot,
};
pub use blob_chunk_dedupe::{
    BlobChunkCanonicalComparisonBasis, BlobChunkCanonicalEquivalence, BlobChunkDedupeAdmission,
    BlobChunkDedupeAdmissionOutcome, BlobChunkDedupeCandidate, BlobChunkDedupeShareClaim,
};
pub use blob_chunk_denial::{
    BlobChunkDedupeAdmissionDenial, BlobChunkSecurityScopeDenial, BlobChunkStreamingDenial,
};
pub use blob_chunk_identity::BlobChunkIdentity;
pub use blob_chunk_scope::BlobChunkSecurityScope;
pub use blob_chunk_streaming::{
    BlobChunkStreamingObservation, BlobChunkStreamingOperation, BlobChunkStreamingOperationKind,
    BlobChunkStreamingResidencyProof, BlobChunkStreamingWindow,
};
pub use blob_lifecycle_receipts::{
    BlobDedupeReceipt, BlobReachabilityReceipt, BlobResumabilityReceipt, BlobRetentionReceipt,
};
pub use large_record_streaming_envelope::{
    LargeRecordStreamingEnvelope, LargeRecordStreamingEnvelopeDenial,
};
pub use s7_blob_security_handoff::{S7BlobChunkSecurityHandoff, S7BlobChunkSecurityPermission};
