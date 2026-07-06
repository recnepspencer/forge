// --- Capabilities (admission handles, next-step types) ---
pub use crate::chunk_integrity::{
    AdmittedBlobChunkSequence, BlobChunkIntegrityProof, BlobChunkProofFrontier,
    BlobChunkProofLeaf, BlobChunkReferenceAccountingRegistry, BlobChunkRootPublication,
    BlobChunkSequenceAdmission, BlobChunkSize, BlobChunkingRuleAdmission,
};
// --- Outcomes (transition receipts) ---
pub use crate::chunk_integrity::{
    BlobChunkCanonicalComparisonBasis, BlobChunkCollisionVerificationReceipt,
    BlobChunkRootCanonicalBasis, BlobChunkRootCanonicalComparison,
};
// --- Denials (classified failure enums) ---
pub use crate::chunk_integrity::{
    BlobChunkIntegrityDenial, BlobChunkReferenceAccountingDenial, BlobChunkRootPublicationDenial,
};
// --- Counter witnesses (read-only snapshots) ---
pub use crate::chunk_integrity::{
    BlobChunkIntegrityCounterSnapshot, BlobChunkRootCounterSnapshot,
};