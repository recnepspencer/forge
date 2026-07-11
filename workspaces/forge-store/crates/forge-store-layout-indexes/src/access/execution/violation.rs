#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8ObservedCounterMetric {
    PointLookups,
    RangeLookups,
    WalReplays,
    Publications,
    MaintenanceReads,
    PageTouches,
    IndexProbes,
    KeyComparisons,
    RangeSteps,
    PrefixSteps,
    ChunkTreeNodeReads,
    ManifestReads,
    BytesRead,
    BytesWritten,
    WriteFanout,
    ReadAmplification,
    WriteAmplification,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8CostEnvelopeViolationOutcome {
    ObservedExceededPlanned {
        metric: S8ObservedCounterMetric,
        planned: u64,
        observed: u64,
    },
}
