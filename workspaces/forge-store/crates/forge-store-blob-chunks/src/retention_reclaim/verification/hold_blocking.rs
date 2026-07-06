use crate::retention_reclaim::classification::{
    assemble_retention_denial, RetentionReclaimEligibilityCase,
};
use crate::retention_reclaim::denial::BlobRetentionReclaimDenial;
use crate::retention_reclaim::holds::BlobRetentionHold;
use crate::BlobChunkReachabilityRegistry;

pub(crate) fn verify_no_live_reachability_holds(
    reachability: &BlobChunkReachabilityRegistry,
) -> Result<(), BlobRetentionReclaimDenial> {
    if let Some(hold) = reachability.first_retention_hold_for_reclaim() {
        return Err(assemble_retention_denial(
            RetentionReclaimEligibilityCase::BlockedByReachabilityHold {
                kind: hold.kind(),
            },
        ));
    }
    Ok(())
}

pub(crate) fn deny_retention_hold(hold: &BlobRetentionHold) -> BlobRetentionReclaimDenial {
    assemble_retention_denial(RetentionReclaimEligibilityCase::BlockedByReachabilityHold {
        kind: hold.kind(),
    })
}