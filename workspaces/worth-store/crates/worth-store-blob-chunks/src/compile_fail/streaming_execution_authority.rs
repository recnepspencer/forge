//! Blob streaming and compaction execution authority remains move-owned at
//! each public consumer boundary.
//!
//! Ingest pressure admission cannot be cloned.
//!
//! ```compile_fail
//! use worth_store_blob_chunks::BlobStreamingPressureAdmission;
//!
//! fn duplicate(admission: BlobStreamingPressureAdmission) {
//!     let _duplicate = admission.clone();
//! }
//! ```
//!
//! Ingest pressure admission cannot be consumed twice.
//!
//! ```compile_fail
//! use worth_store_blob_chunks::BlobStreamingPressureAdmission;
//!
//! fn consume(_: BlobStreamingPressureAdmission) {}
//!
//! fn consume_twice(admission: BlobStreamingPressureAdmission) {
//!     consume(admission);
//!     consume(admission);
//! }
//! ```
//!
//! Verification-read admission cannot be cloned.
//!
//! ```compile_fail
//! use worth_store_blob_chunks::BlobStreamingReadAdmission;
//!
//! fn duplicate(admission: BlobStreamingReadAdmission) {
//!     let _duplicate = admission.clone();
//! }
//! ```
//!
//! Verification-read admission cannot be consumed twice.
//!
//! ```compile_fail
//! use worth_store_blob_chunks::BlobStreamingReadAdmission;
//!
//! fn consume(_: BlobStreamingReadAdmission) {}
//!
//! fn consume_twice(admission: BlobStreamingReadAdmission) {
//!     consume(admission);
//!     consume(admission);
//! }
//! ```
//!
//! A paced compaction intent cannot be cloned.
//!
//! ```compile_fail
//! use worth_store_blob_chunks::BlobCompactionIntent;
//!
//! fn duplicate(intent: BlobCompactionIntent) {
//!     let _duplicate = intent.clone();
//! }
//! ```
//!
//! A paced compaction intent cannot be consumed twice.
//!
//! ```compile_fail
//! use worth_store_blob_chunks::BlobCompactionIntent;
//!
//! fn consume(_: BlobCompactionIntent) {}
//!
//! fn consume_twice(intent: BlobCompactionIntent) {
//!     consume(intent);
//!     consume(intent);
//! }
//! ```
