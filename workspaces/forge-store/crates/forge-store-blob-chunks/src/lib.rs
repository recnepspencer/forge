//! Roadmap 2 S.7 native content-addressed blob and object chunk store.
//!
//! # Legal blob lifecycle order
//!
//! Callers follow proof flow through facade-returned capabilities in this order:
//!
//! ```text
//! identity → integrity → dedupe → streaming → lifecycle → publication
//! → reachability → placement → compaction → corruption → recovery → retention/reclaim
//! ```
//!
//! Each lifecycle stage exports capabilities, outcomes, denials, and counter
//! witnesses in that authority order. Hostile-lane `reject_*` constructors are
//! grouped separately and must not be mixed with production admission handles.
//!
//! Construction-boundary compile-fail proofs live in [`compile_fail`] and the
//! `#[doc(hidden)]` compile-fail modules re-exported below.
#![forbid(unsafe_code)]

mod capsule_readiness;
mod chunk_identity;
mod chunk_integrity;
mod closeout_bundle;
mod compaction;
mod compile_fail;
mod corruption;
mod dedupe;
mod export_bundle;
mod exports;
mod heavy_fixture;
mod harness_execution;
mod handoffs;
mod import_readmission;
mod lifecycle;
mod placement;
mod publication;
mod reachability;
mod recovery;
mod retention_reclaim;
mod streaming;
#[cfg(test)]
mod test_support;

#[cfg(test)]
pub use exports::hostile_lane::*;
pub use exports::*;
pub use heavy_fixture::*;
pub use closeout_bundle::S7ExecutedLifecycleEvidenceBundle;

// --- Construction-boundary compile-fail evidence ---
#[path = "compile_fail/capsule_readiness.rs"]
#[doc(hidden)]
pub mod blob_capsule_readiness_compile_fail;
#[path = "compile_fail/integrity.rs"]
#[doc(hidden)]
pub mod blob_chunk_integrity_compile_fail;
#[path = "compile_fail/root.rs"]
#[doc(hidden)]
pub mod blob_chunk_root_compile_fail;
#[path = "compile_fail/corruption.rs"]
#[doc(hidden)]
pub mod blob_corruption_compile_fail;
#[path = "compile_fail/export_bundle.rs"]
#[doc(hidden)]
pub mod blob_export_bundle_compile_fail;
#[path = "compile_fail/generation_registry.rs"]
#[doc(hidden)]
pub mod blob_generation_registry_compile_fail;
#[path = "compile_fail/import_readmission.rs"]
#[doc(hidden)]
pub mod blob_import_readmission_compile_fail;
#[path = "compile_fail/placement_movement.rs"]
#[doc(hidden)]
pub mod blob_placement_movement_compile_fail;
#[path = "compile_fail/publication.rs"]
#[doc(hidden)]
pub mod blob_publication_commit_compile_fail;
#[path = "compile_fail/reachability.rs"]
#[doc(hidden)]
pub mod blob_reachability_compile_fail;
#[path = "compile_fail/recovery_records.rs"]
#[doc(hidden)]
pub mod blob_recovery_records_compile_fail;
#[path = "compile_fail/retention_reclaim.rs"]
#[doc(hidden)]
pub mod blob_retention_reclaim_compile_fail;
#[path = "compile_fail/streaming_read.rs"]
#[doc(hidden)]
pub mod blob_streaming_read_compile_fail;
#[path = "compile_fail/security_metadata.rs"]
#[doc(hidden)]
pub mod security_metadata_compile_fail;
