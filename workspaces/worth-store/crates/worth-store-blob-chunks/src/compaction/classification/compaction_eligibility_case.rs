#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CompactionEligibilityCase {
    Admit,
    PhysicalInterlockDenied,
    MissingReachabilityProof,
    ActiveReadHold,
    ReadHoldPlanMismatch,
    UnavailableColdChunk,
    QuarantineHold,
    EquivalenceBasisMismatch,
    LifecycleReachabilityMismatch,
    LifecyclePlacementMismatch,
    DedupeScopeMismatch,
    StaleDedupeReference,
}
