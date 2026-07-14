#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CompactionEligibilityCase {
    Admit,
    PhysicalInterlockDenied,
    MissingReachabilityProof,
    ActiveReadHold,
    ReadHoldPlanMismatch,
    UnsupportedSchedulerPacing,
    UnavailableColdChunk,
    QuarantineHold,
    EquivalenceBasisMismatch,
    LifecycleReachabilityMismatch,
    LifecyclePlacementMismatch,
    DedupeScopeMismatch,
    StaleDedupeReference,
}
