//! Root publication is Store-owned chunk-tree evidence, not copied scalar data.
//!
//! ```compile_fail
//! use forge_store_blob_chunks::{BlobChunkRootPublication, ChunkTreeRoot};
//!
//! let copied_root: ChunkTreeRoot = todo!();
//! let _forged = BlobChunkRootPublication::from(copied_root);
//! ```
//!
//! ```compile_fail
//! use forge_store_blob_chunks::{BlobChunkRootCanonicalBasis, LogicalContentDigest};
//!
//! let copied_digest: LogicalContentDigest = todo!();
//! let _forged = BlobChunkRootCanonicalBasis::from(copied_digest);
//! ```
//!
//! ```compile_fail
//! use forge_store_blob_chunks::BlobChunkRootPublication;
//! use forge_store_physical_format::PhysicalChunkChecksumWitness;
//!
//! let checksum: PhysicalChunkChecksumWitness = todo!();
//! let _forged = BlobChunkRootPublication::publish(checksum);
//! ```
//!
//! ```compile_fail
//! use forge_store_blob_chunks::BlobChunkRootPublication;
//! use forge_store_contracts::StableDigest;
//!
//! let digest = StableDigest::new("sha256:copied").unwrap();
//! let _forged = BlobChunkRootPublication::publish(digest);
//! ```
//!
//! ```compile_fail
//! use forge_store_blob_chunks::AdmittedBlobChunkSequence;
//!
//! let sequence: AdmittedBlobChunkSequence = todo!();
//! let _root = sequence.chunk_tree_root();
//! ```
