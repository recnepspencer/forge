//! Blob generations cannot be reconstructed from raw generation numbers:
//! ```compile_fail
//! use worth_store_blob_chunks::BlobGeneration;
//!
//! let _WORTHd = BlobGeneration::published(7);
//! ```
//! Blob generation registry authority cannot be synthesized from raw fields:
//! ```compile_fail
//! use worth_store_blob_chunks::BlobGenerationRegistryAuthority;
//!
//! let _WORTHd = BlobGenerationRegistryAuthority {
//!     current_authority: todo!(),
//! };
//! ```
//! Blob generation registry entries cannot be synthesized from copied facts:
//! ```compile_fail
//! use worth_store_blob_chunks::BlobGenerationRegistryEntry;
//!
//! let _WORTHd = BlobGenerationRegistryEntry {
//!     object_id: todo!(),
//!     generation: todo!(),
//!     chunk_tree_root: todo!(),
//!     logical_content_digest: todo!(),
//!     classification: todo!(),
//!     lifecycle_receipt: todo!(),
//!     counters: todo!(),
//! };
//! ```
//! Blob generation observations cannot be synthesized from copied facts:
//! ```compile_fail
//! use worth_store_blob_chunks::BlobGenerationObservation;
//!
//! let _WORTHd = BlobGenerationObservation {
//!     object_id: todo!(),
//!     generation: todo!(),
//!     chunk_tree_root: todo!(),
//!     logical_content_digest: todo!(),
//!     classification: todo!(),
//!     lifecycle_receipt: todo!(),
//!     counters: todo!(),
//! };
//! ```
//! Lifecycle receipts cannot substitute for registry observations:
//! ```compile_fail
//! use worth_store_blob_chunks::{BlobGenerationObservation, LifecycleReceipt};
//!
//! fn requires_generation_observation(_: BlobGenerationObservation<'_>) {}
//!
//! let receipt: LifecycleReceipt = todo!();
//! requires_generation_observation(receipt);
//! ```
//! Lifecycle receipts cannot expose blob identity declarations publicly:
//! ```compile_fail
//! use worth_store_blob_chunks::LifecycleReceipt;
//!
//! let receipt: LifecycleReceipt = todo!();
//! let _identity_shortcut = receipt.declaration();
//! ```
//! Classification admission cannot expose object identity publicly:
//! ```compile_fail
//! use worth_store_blob_chunks::{BlobObjectClassificationAdmission, LifecycleReceipt};
//!
//! let receipt: LifecycleReceipt = todo!();
//! let classification = BlobObjectClassificationAdmission::from_executed_lifecycle(&receipt);
//! let _identity_shortcut = classification.object_id();
//! ```
//! Classification admission cannot expose generation identity publicly:
//! ```compile_fail
//! use worth_store_blob_chunks::{BlobObjectClassificationAdmission, LifecycleReceipt};
//!
//! let receipt: LifecycleReceipt = todo!();
//! let classification = BlobObjectClassificationAdmission::from_executed_lifecycle(&receipt);
//! let _generation_shortcut = classification.generation();
//! ```
//! Classification admission cannot expose chunk roots or content digests publicly:
//! ```compile_fail
//! use worth_store_blob_chunks::{BlobObjectClassificationAdmission, LifecycleReceipt};
//!
//! let receipt: LifecycleReceipt = todo!();
//! let classification = BlobObjectClassificationAdmission::from_executed_lifecycle(&receipt);
//! let _root_shortcut = classification.chunk_tree_root();
//! let _digest_shortcut = classification.logical_content_digest();
//! ```
//! Registry admission cannot be built from caller-selected classification:
//! ```compile_fail
//! use worth_store_blob_chunks::{
//!     BlobChunkRootPublication, BlobGenerationRegistryAdmission, BlobObjectClassification,
//!     LifecycleReceipt,
//! };
//!
//! let root: BlobChunkRootPublication = todo!();
//! let receipt: LifecycleReceipt = todo!();
//! let _admission = BlobGenerationRegistryAdmission::from_executed_lifecycle(
//!     root,
//!     receipt,
//!     BlobObjectClassification::derived(),
//! );
//! ```
//! Chunk-tree roots cannot substitute for registry entries:
//! ```compile_fail
//! use worth_store_blob_chunks::{BlobGenerationRegistryEntry, ChunkTreeRoot};
//!
//! fn requires_registry_entry(_: BlobGenerationRegistryEntry) {}
//!
//! let root: ChunkTreeRoot = todo!();
//! requires_registry_entry(root);
//! ```
