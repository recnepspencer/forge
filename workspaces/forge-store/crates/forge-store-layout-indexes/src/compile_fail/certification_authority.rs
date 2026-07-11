//! ```compile_fail
//! use forge_store_layout_indexes::certification_test_authority::bridge_blob_export_trust_boundary;
//! ```
//!
//! ```compile_fail
//! use forge_store_layout_indexes::certification_test_authority::{
//!     materialize_blob_executed_lifecycle_evidence, BlobHarnessExecutedWitness,
//! };
//!
//! fn replay_only(witness: BlobHarnessExecutedWitness) {
//!     let _ = materialize_blob_executed_lifecycle_evidence(witness);
//! }
//! ```
