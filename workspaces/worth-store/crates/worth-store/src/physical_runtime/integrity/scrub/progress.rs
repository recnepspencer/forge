#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::physical_runtime) enum ManagedPhysicalIntegrityScrubProgress {
    WindowInspected { ordinal: u64 },
    DeferredAllocation { ordinal: u64, requested_bytes: u64 },
    RejectedStoreScope { ordinal: u64 },
    Paused,
    Cancelled,
    Closed,
    StaleRuntimeGeneration,
    Completed,
}
