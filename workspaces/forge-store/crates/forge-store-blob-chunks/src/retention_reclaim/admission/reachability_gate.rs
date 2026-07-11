use forge_store_physical_isolation::BlobOrphanReclaimBarrier;

use crate::retention_reclaim::candidate::{
    BlobRetentionOrphanCandidate, BlobRetentionPhysicalOrphanClaim,
};
use crate::retention_reclaim::classification::{
    assemble_retention_denial, RetentionReclaimEligibilityCase,
};
use crate::retention_reclaim::denial::BlobRetentionReclaimDenial;
use crate::retention_reclaim::types::admission::BlobRetentionReclaimAdmission;
use crate::retention_reclaim::verification::barrier_match::verify_resume_barrier_matches_claim;
use crate::retention_reclaim::verification::hold_blocking::verify_no_live_reachability_holds;
use crate::retention_reclaim::verification::security_scope::verify_reclaim_policy_scope;
use crate::{
    BlobChunkIdentity, BlobChunkReachabilityRegistry, BlobReachabilityReclaimDecision,
    BlobReachabilityReclaimRelease, BlobReclaimPolicyEvidence,
};

pub(crate) fn admit_via_reachability_gate(
    reachability: &BlobChunkReachabilityRegistry,
    chunk_identity: &BlobChunkIdentity,
    reclaim_policy_evidence: BlobReclaimPolicyEvidence,
    resume_barrier: Option<&BlobOrphanReclaimBarrier>,
) -> Result<BlobRetentionReclaimAdmission, BlobRetentionReclaimDenial> {
    verify_no_live_reachability_holds(reachability)?;
    let release = collect_reachability_release(reachability, chunk_identity)?;
    verify_reclaim_policy_scope(&release, &reclaim_policy_evidence)?;
    let physical_claim = BlobRetentionPhysicalOrphanClaim::from_admitted_reclaim_policy_evidence(
        &release,
        &reclaim_policy_evidence,
    )?;
    let eligibility = match resume_barrier {
        None => RetentionReclaimEligibilityCase::EligibleReachabilityOrphan,
        Some(barrier) => {
            verify_resume_barrier_matches_claim(&physical_claim, barrier)?;
            RetentionReclaimEligibilityCase::EligibleAbandonedResumeOrphan
        }
    };
    construct_admission(
        release,
        reclaim_policy_evidence,
        physical_claim,
        resume_barrier,
        eligibility,
    )
}

fn collect_reachability_release(
    reachability: &BlobChunkReachabilityRegistry,
    chunk_identity: &BlobChunkIdentity,
) -> Result<BlobReachabilityReclaimRelease, BlobRetentionReclaimDenial> {
    match reachability.reclaim_decision_for(chunk_identity) {
        BlobReachabilityReclaimDecision::ReclaimPermitted(release) => Ok(release),
        BlobReachabilityReclaimDecision::ReclaimDenied(_) => Err(assemble_retention_denial(
            RetentionReclaimEligibilityCase::ReachabilityDenied,
        )),
    }
}

fn construct_admission(
    release: BlobReachabilityReclaimRelease,
    reclaim_policy_evidence: BlobReclaimPolicyEvidence,
    physical_claim: BlobRetentionPhysicalOrphanClaim,
    resume_barrier: Option<&BlobOrphanReclaimBarrier>,
    eligibility: RetentionReclaimEligibilityCase,
) -> Result<BlobRetentionReclaimAdmission, BlobRetentionReclaimDenial> {
    let candidate = match eligibility {
        RetentionReclaimEligibilityCase::EligibleReachabilityOrphan => {
            BlobRetentionOrphanCandidate::from_reachability_release(release, physical_claim)?
        }
        RetentionReclaimEligibilityCase::EligibleAbandonedResumeOrphan => {
            let barrier = resume_barrier.expect("resume barrier required for resume orphan");
            BlobRetentionOrphanCandidate::from_abandoned_resume_barrier(release, barrier)?
        }
        _ => unreachable!("denials are assembled before construct_admission"),
    };
    Ok(BlobRetentionReclaimAdmission::construct(
        candidate,
        reclaim_policy_evidence,
    ))
}
