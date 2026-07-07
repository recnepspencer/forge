use crate::retention_reclaim::holds::BlobRetentionHoldKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RetentionReclaimEligibilityCase {
    BlockedByReachabilityHold { kind: BlobRetentionHoldKind },
    ReachabilityDenied,
    S6ScopeMismatch,
    BarrierMismatch,
    EligibleReachabilityOrphan,
    EligibleAbandonedResumeOrphan,
}
