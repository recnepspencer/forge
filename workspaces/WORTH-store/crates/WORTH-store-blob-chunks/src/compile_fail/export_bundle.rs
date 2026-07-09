//! Exported chunk byte witnesses cannot be synthesized from copied leaves and bytes:
//! ```compile_fail
//! use worth_store_blob_chunks::{BlobExportedChunkBytes, BlobChunkByteWindow, BlobChunkProofLeaf};
//!
//! let leaf: BlobChunkProofLeaf = todo!();
//! let bytes = BlobChunkByteWindow::borrowed(0, b"chunk").unwrap();
//! let _WORTHd = BlobExportedChunkBytes { leaf, bytes };
//! ```
//! Export publication cannot consume raw proof leaves as exported byte authority:
//! ```compile_fail
//! use worth_store_blob_chunks::{BlobExportIntent, BlobChunkProofLeaf};
//!
//! let leaf: BlobChunkProofLeaf = todo!();
//! let intent: BlobExportIntent<'static> = todo!();
//! let _WORTHd = intent.with_exported_chunks([leaf]);
//! ```
