use crate::capsule_readiness::counters::BlobCapsuleReadinessCounters;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlobCapsuleReadinessDenial {
    EmptySelection {
        counters: BlobCapsuleReadinessCounters,
    },
    DuplicateOrdinal {
        ordinal: u64,
        counters: BlobCapsuleReadinessCounters,
    },
    UnsortedOrdinal {
        previous: u64,
        next: u64,
        counters: BlobCapsuleReadinessCounters,
    },
    GenerationMismatch {
        counters: BlobCapsuleReadinessCounters,
    },
    MissingParentRootBasis {
        counters: BlobCapsuleReadinessCounters,
    },
    MissingChunk {
        ordinal: u64,
        counters: BlobCapsuleReadinessCounters,
    },
    StaleSecurityScope {
        counters: BlobCapsuleReadinessCounters,
    },
    QuarantinedChunk {
        ordinal: u64,
        counters: BlobCapsuleReadinessCounters,
    },
    ColdPlacementUnavailable {
        counters: BlobCapsuleReadinessCounters,
    },
    CrossScopeSharedChunk {
        counters: BlobCapsuleReadinessCounters,
    },
    ReachabilityChangedDuringCreation {
        counters: BlobCapsuleReadinessCounters,
    },
    CopiedCapsuleRow {
        counters: BlobCapsuleReadinessCounters,
    },
    DigestOnlyChunkReference {
        counters: BlobCapsuleReadinessCounters,
    },
}

impl BlobCapsuleReadinessDenial {
    pub const fn counters(&self) -> BlobCapsuleReadinessCounters {
        match self {
            Self::EmptySelection { counters }
            | Self::GenerationMismatch { counters }
            | Self::MissingParentRootBasis { counters }
            | Self::StaleSecurityScope { counters }
            | Self::ColdPlacementUnavailable { counters }
            | Self::CrossScopeSharedChunk { counters }
            | Self::ReachabilityChangedDuringCreation { counters }
            | Self::CopiedCapsuleRow { counters }
            | Self::DigestOnlyChunkReference { counters } => *counters,
            Self::DuplicateOrdinal { counters, .. }
            | Self::UnsortedOrdinal { counters, .. }
            | Self::MissingChunk { counters, .. }
            | Self::QuarantinedChunk { counters, .. } => *counters,
        }
    }
}
