use crate::blob_orphan_reclaim::classification::OrphanReclaimCase;
use crate::blob_orphan_reclaim::denial::BlobOrphanReclaimDenial;
use crate::blob_orphan_reclaim::receipt_construction::construct_coverage;
use crate::blob_orphan_reclaim::types::barrier::BlobOrphanReclaimBarrier;
use crate::blob_orphan_reclaim::types::coverage::BlobOrphanReclaimCoverage;
use crate::blob_orphan_reclaim::types::identity::BlobOrphanReclaimIdentity;
use crate::blob_orphan_reclaim::verification::identity_coverage::refine_coverage_case;
use crate::blob_orphan_reclaim::verification::reachability_removal::classify_orphan_reclaim_coverage;
use crate::ReclaimEligibilityProof;

pub(crate) fn assemble_coverage_or_denial(
    barrier: BlobOrphanReclaimBarrier,
    identity: BlobOrphanReclaimIdentity,
    case: OrphanReclaimCase,
) -> Result<BlobOrphanReclaimCoverage, BlobOrphanReclaimDenial> {
    match case {
        OrphanReclaimCase::CoverageAdmitted { receipt } => {
            Ok(construct_coverage(barrier, identity, &receipt))
        }
        OrphanReclaimCase::DeniedMissingRemovalEvidence
        | OrphanReclaimCase::DeniedIdentityNotCovered => {
            Err(BlobOrphanReclaimDenial::MissingS7ReclaimBarrier)
        }
    }
}

pub(crate) fn transition_admit_coverage(
    barrier: BlobOrphanReclaimBarrier,
    eligibility: ReclaimEligibilityProof,
) -> Result<BlobOrphanReclaimCoverage, BlobOrphanReclaimDenial> {
    let identity = barrier.reclaim_identity();
    let case = classify_orphan_reclaim_coverage(eligibility);
    let case = refine_coverage_case(case, &identity);
    assemble_coverage_or_denial(barrier, identity, case)
}
