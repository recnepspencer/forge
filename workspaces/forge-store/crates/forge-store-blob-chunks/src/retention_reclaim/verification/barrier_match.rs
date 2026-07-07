use crate::retention_reclaim::candidate::BlobRetentionPhysicalOrphanClaim;
use crate::retention_reclaim::classification::{
    assemble_retention_denial, RetentionReclaimEligibilityCase,
};
use crate::retention_reclaim::denial::BlobRetentionReclaimDenial;
use forge_store_physical_isolation::BlobOrphanReclaimBarrier;

pub(crate) fn verify_resume_barrier_matches_claim(
    physical_claim: &BlobRetentionPhysicalOrphanClaim,
    barrier: &BlobOrphanReclaimBarrier,
) -> Result<(), BlobRetentionReclaimDenial> {
    if !physical_claim.matches_resume_barrier(barrier) {
        return Err(assemble_retention_denial(
            RetentionReclaimEligibilityCase::BarrierMismatch,
        ));
    }
    Ok(())
}
