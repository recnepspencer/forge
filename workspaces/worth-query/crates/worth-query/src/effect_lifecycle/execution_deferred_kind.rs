#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EffectExecutionDeferredKind {
    ActiveSnapshotCapacityExhausted { maximum_active_snapshots: usize },
    TransactionRetentionCapacityExhausted,
    PatchPositionReservationContended,
    RetentionBackpressure,
    CandidateLifetimeExpired { maximum_lifetime_millis: u64 },
    CandidateCapacityExhausted { maximum_candidates: usize },
    PublishedSnapshotCapacityExhausted { maximum_handles: usize },
}
