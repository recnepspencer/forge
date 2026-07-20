//! store-proof-required-features: certification-test-authority
//!
//! ```compile_fail
//! use worth_store_blob_chunks::{
//!     BlobHarnessExecutedWitness,
//! };
//!
//! fn forged() -> BlobHarnessExecutedWitness {
//!     BlobHarnessExecutedWitness {
//!         executed_topology: todo!(),
//!         declared_topology: todo!(),
//!         allocation_bytes: 64 * 1024,
//!         observed_yieldpoint: todo!(),
//!         export_declared_chunk_count: 2,
//!         export_declared_total_bytes: 4096,
//!         export_logical_digest_matches_lifecycle: true,
//!         export_checksum_distinct_from_stored_digest: true,
//!         reachability_reference_edges: 1,
//!         reachability_stored_digest_matches_lifecycle: true,
//!         cross_scope_dedupe_denied: true,
//!     }
//! }
//! ```
//!
//! ```compile_fail
//! use worth_store_blob_chunks::BlobHarnessExecutionRequest;
//!
//! fn leak_request_fields(request: BlobHarnessExecutionRequest) -> u64 {
//!     request.profile as u64
//! }
//! ```
