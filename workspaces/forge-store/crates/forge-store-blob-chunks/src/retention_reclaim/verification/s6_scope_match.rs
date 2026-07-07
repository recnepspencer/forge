use crate::retention_reclaim::classification::{
    assemble_retention_denial, RetentionReclaimEligibilityCase,
};
use crate::retention_reclaim::denial::BlobRetentionReclaimDenial;
use crate::{BlobReachabilityReclaimRelease, S6BlobReclaimNonClaimHandoff};

pub(crate) fn verify_s6_posture_matches_release(
    release: &BlobReachabilityReclaimRelease,
    s6_posture: S6BlobReclaimNonClaimHandoff,
) -> Result<(), BlobRetentionReclaimDenial> {
    if s6_posture.carries_blob_lifecycle_claim()
        || s6_posture.security_metadata() != release.released_edges()[0].security_metadata()
    {
        return Err(assemble_retention_denial(
            RetentionReclaimEligibilityCase::S6ScopeMismatch,
        ));
    }
    Ok(())
}
