use super::BlobCompactionCounterSnapshot;
use forge_store_physical_isolation::CompactionReadInterlockDenial;
use forge_store_tiering::ColdPlacementState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobCompactionDenial {
    ActiveReadHold {
        counters: BlobCompactionCounterSnapshot,
    },
    QuarantineHold {
        counters: BlobCompactionCounterSnapshot,
    },
    StaleDedupeReference {
        counters: BlobCompactionCounterSnapshot,
    },
    UnavailableColdChunk {
        state: ColdPlacementState,
        counters: BlobCompactionCounterSnapshot,
    },
    UnsupportedS6Pacing {
        counters: BlobCompactionCounterSnapshot,
    },
    MissingReachabilityProof {
        counters: BlobCompactionCounterSnapshot,
    },
    ReadHoldPlanMismatch {
        counters: BlobCompactionCounterSnapshot,
    },
    LifecycleReachabilityMismatch {
        counters: BlobCompactionCounterSnapshot,
    },
    LifecyclePlacementMismatch {
        counters: BlobCompactionCounterSnapshot,
    },
    DedupeScopeMismatch {
        counters: BlobCompactionCounterSnapshot,
    },
    EquivalenceBasisMismatch {
        counters: BlobCompactionCounterSnapshot,
    },
    MixedChunkTreePublication {
        counters: BlobCompactionCounterSnapshot,
    },
    PhysicalInterlockDenied {
        source: CompactionReadInterlockDenial,
        counters: BlobCompactionCounterSnapshot,
    },
}
