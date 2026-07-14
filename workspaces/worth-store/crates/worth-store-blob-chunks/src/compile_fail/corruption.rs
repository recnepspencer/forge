//! Physical integrity reports cannot mint S.7 blob corruption authority:
//! ```compile_fail
//! use worth_store_blob_chunks::BlobCorruptedChunkLocalization;
//! use worth_store_physical_integrity::ChunkIntegrityReport;
//! fn requires_localization(_: BlobCorruptedChunkLocalization) {}
//! let report: ChunkIntegrityReport = todo!();
//! requires_localization(report);
//! ```
//! Physical quarantine records cannot mint S.7 blob quarantine:
//! ```compile_fail
//! use worth_store_blob_chunks::BlobChunkQuarantine;
//! use worth_store_physical_integrity::QuarantineRecord;
//! fn requires_quarantine(_: BlobChunkQuarantine) {}
//! let record: QuarantineRecord = todo!();
//! requires_quarantine(record);
//! ```
//! Copied counters cannot mint blob corruption guards:
//! ```compile_fail
//! use worth_store_blob_chunks::{BlobCorruptionCounterSnapshot, BlobCorruptionGuard};
//! fn requires_guard(_: BlobCorruptionGuard) {}
//! let counters: BlobCorruptionCounterSnapshot = todo!();
//! requires_guard(counters);
//! ```
//! Visible generations cannot mint affected reference edges:
//! ```compile_fail
//! use worth_store_blob_chunks::{BlobCorruptionReferenceEdges, BlobVisibleGeneration};
//! let visible: BlobVisibleGeneration = todo!();
//! let _edges = BlobCorruptionReferenceEdges::from_visible_generation(&visible);
//! ```
//! Published generations cannot directly mint affected reference edges:
//! ```compile_fail
//! use worth_store_blob_chunks::{BlobCorruptionReferenceEdges, BlobGenerationPublished};
//! let published: BlobGenerationPublished = todo!();
//! let _edges = BlobCorruptionReferenceEdges::from_published_generation(&published);
//! ```
//! Lifecycle receipts cannot directly mint affected reference edges:
//! ```compile_fail
//! use worth_store_blob_chunks::{BlobCorruptionReferenceEdge, LifecycleReceipt};
//! let receipt: LifecycleReceipt = todo!();
//! let _edge = BlobCorruptionReferenceEdge::from_lifecycle_receipt(&receipt);
//! ```
