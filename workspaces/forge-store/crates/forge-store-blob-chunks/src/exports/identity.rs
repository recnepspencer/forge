// --- Capabilities (admission handles, next-step types) ---
pub use crate::chunk_identity::{
    BlobChunkByteRange, BlobChunkByteWindow, BlobChunkContentDigest, BlobChunkIdentity,
    BlobChunkOrdinal, BlobChunkSecurityMetadataWitness, BlobChunkSecurityScope, ScopedBlobChunk,
};
// --- Outcomes (transition receipts) ---
// (security scope admission is the terminal capability for this stage)
// --- Denials (classified failure enums) ---
pub use crate::chunk_identity::BlobChunkSecurityScopeDenial;
// --- Counter witnesses (read-only snapshots) ---
pub use crate::chunk_identity::BlobChunkScopeCounterSnapshot;