use crate::blob_orphan_reclaim::classification::OrphanReclaimCase;
use crate::blob_orphan_reclaim::types::identity::BlobOrphanReclaimIdentity;
use crate::ReclaimReachabilityRemovalReceipt;

pub(crate) fn verify_identity_covered_by_receipt(
    receipt: &ReclaimReachabilityRemovalReceipt,
    identity: &BlobOrphanReclaimIdentity,
) -> bool {
    receipt.covers_reclaimed_identity(identity.physical_reference())
}

pub(crate) fn refine_coverage_case(
    case: OrphanReclaimCase,
    identity: &BlobOrphanReclaimIdentity,
) -> OrphanReclaimCase {
    match case {
        OrphanReclaimCase::CoverageAdmitted { receipt } => {
            if verify_identity_covered_by_receipt(&receipt, identity) {
                OrphanReclaimCase::CoverageAdmitted { receipt }
            } else {
                OrphanReclaimCase::DeniedIdentityNotCovered
            }
        }
        other => other,
    }
}
