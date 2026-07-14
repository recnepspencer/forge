use crate::blob_orphan_reclaim::classification::OrphanReclaimCase;
use crate::blob_orphan_reclaim::denial::BlobOrphanReclaimDenial;
use crate::{ReclaimEligibilityProof, ReclaimReachabilityRemovalReceipt};

pub(crate) fn verify_reachability_removal_receipt(
    eligibility: ReclaimEligibilityProof,
) -> Result<ReclaimReachabilityRemovalReceipt, BlobOrphanReclaimDenial> {
    eligibility
        .admit_reachability_removal()
        .map_err(|_| BlobOrphanReclaimDenial::MissingS7ReclaimBarrier)
}

pub(crate) fn classify_orphan_reclaim_coverage(
    eligibility: ReclaimEligibilityProof,
) -> OrphanReclaimCase {
    match verify_reachability_removal_receipt(eligibility) {
        Ok(receipt) => OrphanReclaimCase::CoverageAdmitted { receipt },
        Err(_) => OrphanReclaimCase::DeniedMissingRemovalEvidence,
    }
}
