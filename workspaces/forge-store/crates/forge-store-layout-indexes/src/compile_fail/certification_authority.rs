//! ```compile_fail
//! use forge_store_layout_indexes::certification_test_authority::bridge_s7_export_trust_boundary;
//! ```
//!
//! ```compile_fail
//! use forge_store_layout_indexes::certification_test_authority::{
//!     materialize_s7_executed_lifecycle_evidence, BlobHarnessExecutedWitness,
//! };
//!
//! fn replay_only(witness: BlobHarnessExecutedWitness) {
//!     let _ = materialize_s7_executed_lifecycle_evidence(witness);
//! }
//! ```
