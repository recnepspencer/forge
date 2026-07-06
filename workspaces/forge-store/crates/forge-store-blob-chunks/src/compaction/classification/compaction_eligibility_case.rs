#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CompactionEligibilityCase {
    Admit,
    PhysicalInterlockDenied,
    MissingReachabilityProof,
    ActiveReadHold,
    ReadHoldPlanMismatch,
    UnsupportedS6Pacing,
    UnavailableColdChunk,
    QuarantineHold,
    EquivalenceBasisMismatch,
    LifecycleReachabilityMismatch,
    LifecyclePlacementMismatch,
    DedupeScopeMismatch,
    StaleDedupeReference,
}