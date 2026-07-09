use crate::retention_reclaim::classification::retention_reclaim_case::RetentionReclaimEligibilityCase;
use crate::retention_reclaim::counters::BlobRetentionReclaimCounterSnapshot;
use crate::retention_reclaim::denial::BlobRetentionReclaimDenial;

fn base_denial_counters() -> BlobRetentionReclaimCounterSnapshot {
    BlobRetentionReclaimCounterSnapshot::start()
        .with_orphan_candidate()
        .record_replay_convergence_check()
}

pub(crate) fn assemble_retention_denial(
    case: RetentionReclaimEligibilityCase,
) -> BlobRetentionReclaimDenial {
    match case {
        RetentionReclaimEligibilityCase::BlockedByReachabilityHold { kind } => {
            BlobRetentionReclaimDenial::ReclaimBlockedByRetentionHold {
                kind,
                counters: base_denial_counters().record_hold_denial(kind),
            }
        }
        RetentionReclaimEligibilityCase::ReachabilityDenied => {
            BlobRetentionReclaimDenial::ReachabilityReclaimDenied {
                counters: base_denial_counters().record_reachability_denial(),
            }
        }
        RetentionReclaimEligibilityCase::S6ScopeMismatch => {
            BlobRetentionReclaimDenial::S6ReclaimPostureScopeMismatch {
                counters: base_denial_counters().record_s6_posture_denial(),
            }
        }
        RetentionReclaimEligibilityCase::BarrierMismatch => {
            BlobRetentionReclaimDenial::ReclaimCandidateIdentityMismatch {
                counters: base_denial_counters().record_identity_mismatch_denial(),
            }
        }
        RetentionReclaimEligibilityCase::EligibleReachabilityOrphan
        | RetentionReclaimEligibilityCase::EligibleAbandonedResumeOrphan => {
            unreachable!("eligible cases do not assemble denials")
        }
    }
}
