//! Imported chunk evidence cannot be synthesized from copied leaves and bytes:
//! ```compile_fail
//! use forge_store_blob_chunks::{BlobChunkByteWindow, BlobChunkProofLeaf, BlobImportedChunkEvidence};
//!
//! let leaf: BlobChunkProofLeaf = todo!();
//! let bytes = BlobChunkByteWindow::borrowed(0, b"chunk").unwrap();
//! let _forged = BlobImportedChunkEvidence { leaf, bytes };
//! ```
//! Imported blob witnesses cannot be synthesized from copied declaration fields:
//! ```compile_fail
//! use forge_store_blob_chunks::ImportedBlobWitness;
//!
//! let _forged = ImportedBlobWitness {
//!     object_id: todo!(),
//!     generation: todo!(),
//!     chunk_tree_root: todo!(),
//!     logical_content_digest: todo!(),
//!     security_metadata: todo!(),
//!     reachable_chunks: vec![],
//!     stored_digest: todo!(),
//!     placement_plan: todo!(),
//!     counters: todo!(),
//! };
//! ```
