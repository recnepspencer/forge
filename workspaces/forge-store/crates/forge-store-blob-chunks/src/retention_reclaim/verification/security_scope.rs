use crate::retention_reclaim::classification::{
    assemble_retention_denial, RetentionReclaimEligibilityCase,
};
use crate::retention_reclaim::denial::BlobRetentionReclaimDenial;
use crate::{BlobReachabilityReclaimRelease, BlobReclaimPolicyEvidence};

pub(crate) fn verify_reclaim_policy_scope(
    release: &BlobReachabilityReclaimRelease,
    reclaim_policy_evidence: &BlobReclaimPolicyEvidence,
) -> Result<(), BlobRetentionReclaimDenial> {
    if reclaim_policy_evidence.carries_blob_lifecycle_claim()
        || reclaim_policy_evidence.security_metadata()
            != release.released_edges()[0].security_metadata()
    {
        return Err(assemble_retention_denial(
            RetentionReclaimEligibilityCase::ReclaimPolicyScopeMismatch,
        ));
    }
    Ok(())
}
