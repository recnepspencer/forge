// --- Capabilities (admission handles, next-step types) ---
pub use crate::reachability::{
    BlobChunkReachabilityProofSet, BlobChunkReachabilityRegistry, BlobReachabilityEdge,
    BlobReachabilityEdgeRelease, BlobReachabilityProof, BlobReachabilityProtectedHold,
    BlobReachabilityReclaimDecision, BlobReachabilityReclaimRelease,
};
// --- Outcomes (transition receipts) ---
pub use crate::reachability::BlobReachabilityCanonicalSnapshot;
// --- Denials (classified failure enums) ---
pub use crate::reachability::{BlobReachabilityDenial, BlobReachabilityEdgeKind};
// --- Counter witnesses (read-only snapshots) ---
pub use crate::reachability::BlobReachabilityCounterSnapshot;
