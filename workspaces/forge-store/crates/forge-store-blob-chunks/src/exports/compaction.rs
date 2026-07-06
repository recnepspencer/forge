// --- Capabilities (admission handles, next-step types) ---
pub use crate::compaction::{
    BlobCompactionAuthority, BlobCompactionColdReadiness, BlobCompactionEquivalence,
    BlobCompactionIntent, BlobCompactionPhysicalInterlock, BlobCompactionReadHold,
    BlobCompactionRewriteExecution, BlobCompactionRewritePlan, BlobCompactionS6Pacing,
};
// --- Outcomes (transition receipts) ---
pub use crate::compaction::{
    BlobCompactionPublishedObservation, BlobCompactionResidue, BlobCompactionRestartOutcome,
};
// --- Denials (classified failure enums) ---
pub use crate::compaction::BlobCompactionDenial;
// --- Counter witnesses (read-only snapshots) ---
pub use crate::compaction::BlobCompactionCounterSnapshot;