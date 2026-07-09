//! Root publication is Store-owned chunk-tree evidence, not copied scalar data.
//!
//! ```compile_fail
//! use worth_store_blob_chunks::{BlobChunkRootPublication, ChunkTreeRoot};
//!
//! let copied_root: ChunkTreeRoot = todo!();
//! let _WORTHd = BlobChunkRootPublication::from(copied_root);
//! ```
//!
//! ```compile_fail
//! use worth_store_blob_chunks::{BlobChunkRootCanonicalBasis, LogicalContentDigest};
//!
//! let copied_digest: LogicalContentDigest = todo!();
//! let _WORTHd = BlobChunkRootCanonicalBasis::from(copied_digest);
//! ```
//!
//! ```compile_fail
//! use worth_store_blob_chunks::BlobChunkRootPublication;
//! use worth_store_physical_format::PhysicalChunkChecksumWitness;
//!
//! let checksum: PhysicalChunkChecksumWitness = todo!();
//! let _WORTHd = BlobChunkRootPublication::publish(checksum);
//! ```
//!
//! ```compile_fail
//! use worth_store_blob_chunks::BlobChunkRootPublication;
//! use worth_store_contracts::StableDigest;
//!
//! let digest = StableDigest::new("sha256:copied").unwrap();
//! let _WORTHd = BlobChunkRootPublication::publish(digest);
//! ```
//!
//! ```compile_fail
//! use worth_store_blob_chunks::AdmittedBlobChunkSequence;
//!
//! let sequence: AdmittedBlobChunkSequence = todo!();
//! let _root = sequence.chunk_tree_root();
//! ```
