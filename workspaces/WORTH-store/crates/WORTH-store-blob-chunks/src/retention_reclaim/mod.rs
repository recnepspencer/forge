mod admission;
mod candidate;
mod classification;
mod counters;
mod denial;
mod holds;
mod orchestration;
mod permit;
mod residue;
mod transitions;
mod types;
mod verification;

#[cfg(test)]
mod test_support;
#[cfg(test)]
mod tests;

pub use candidate::{
    BlobRetentionOrphanCandidate, BlobRetentionOrphanSource, BlobRetentionPhysicalOrphanIdentity,
};
pub use counters::BlobRetentionReclaimCounterSnapshot;
pub use denial::{
    reject_backend_residue_as_retention_reclaim_authority,
    reject_copied_counter_as_retention_reclaim_authority,
    reject_copied_receipt_as_retention_reclaim_authority,
    reject_s6_reclaim_handoff_as_retention_reclaim_authority,
    reject_terminal_projection_as_retention_reclaim_authority, BlobRetentionReclaimDenial,
};
pub use holds::{BlobRetentionHold, BlobRetentionHoldKind, BlobRetentionHoldSet};
pub use orchestration::{BlobRetentionReclaimAdmissionAuthority, BlobRetentionSafeReclaimPlanner};
pub use permit::{BlobRetentionReclaimPermit, BlobRetentionReclaimReceipt};
pub use residue::{BlobLocalizedReclaimResidue, BlobReclaimResidueKind};
pub use types::{
    BlobRetentionReclaimAdmission, BlobRetentionReclaimOutcome, BlobRetentionReclaimRequest,
};
