// --- Capabilities (admission handles, next-step types) ---
pub use crate::retention_reclaim::{
    BlobRetentionReclaimAdmission, BlobRetentionReclaimAdmissionAuthority,
    BlobRetentionReclaimPermit, BlobRetentionReclaimRequest, BlobRetentionSafeReclaimPlanner,
};
// --- Outcomes (transition receipts) ---
pub use crate::retention_reclaim::{
    BlobLocalizedReclaimResidue, BlobReclaimResidueKind, BlobRetentionHold, BlobRetentionHoldKind,
    BlobRetentionHoldSet, BlobRetentionOrphanCandidate, BlobRetentionOrphanSource,
    BlobRetentionPhysicalOrphanIdentity, BlobRetentionReclaimOutcome, BlobRetentionReclaimReceipt,
};
// --- Denials (classified failure enums) ---
pub use crate::retention_reclaim::BlobRetentionReclaimDenial;
// --- Counter witnesses (read-only snapshots) ---
pub use crate::retention_reclaim::BlobRetentionReclaimCounterSnapshot;
