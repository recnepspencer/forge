// --- Capabilities (admission handles, next-step types) ---
pub use crate::dedupe::{
    BlobChunkCanonicalEquivalence, BlobChunkDedupeAdmission, BlobChunkDedupeByteComparison,
    BlobChunkDedupeCandidate, BlobChunkDedupeDigestRewriteBasis, BlobChunkDedupeIndexPartition,
    BlobChunkDedupePolicy, BlobChunkDedupeReferenceRegistry, BlobChunkDedupeShareClaim,
    BlobChunkRegisteredDedupeReference,
};
// --- Outcomes (transition receipts) ---
pub use crate::dedupe::{
    BlobChunkDedupeAdmissionOutcome, BlobChunkDedupeReceipt, BlobChunkDedupeReclaimDecision,
    BlobChunkDedupeReferenceRelease,
};
// --- Denials (classified failure enums) ---
pub use crate::dedupe::{BlobChunkDedupeAdmissionDenial, BlobChunkDedupeCollisionPosture};
// --- Counter witnesses (read-only snapshots) ---
pub use crate::dedupe::BlobChunkDedupeCounterSnapshot;