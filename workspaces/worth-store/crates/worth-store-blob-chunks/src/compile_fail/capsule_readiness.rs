//! Capsule readiness witnesses cannot be synthesized from raw fields:
//! ```compile_fail
//! use worth_store_blob_chunks::BlobCapsuleReadinessWitness;
//!
//! let _forged = BlobCapsuleReadinessWitness {
//!     object_id: todo!(),
//!     generation: todo!(),
//!     chunk_tree_root: todo!(),
//!     logical_content_digest: todo!(),
//!     selected_chunks: vec![],
//!     readiness_digest: String::new(),
//!     declared_bytes: 0,
//!     counters: todo!(),
//! };
//! ```
//! Materialized capsule bundles cannot be synthesized from copied metadata:
//! ```compile_fail
//! use worth_store_blob_chunks::MaterializedBlobCapsuleBundle;
//!
//! let _forged = MaterializedBlobCapsuleBundle {
//!     object_id: todo!(),
//!     generation: todo!(),
//!     chunk_tree_root: todo!(),
//!     logical_content_digest: todo!(),
//!     classification: todo!(),
//!     materialized_chunks: vec![],
//!     declared_bytes: 0,
//!     placement_scope: todo!(),
//!     reachability_fingerprint: String::new(),
//!     counters: todo!(),
//! };
//! ```
