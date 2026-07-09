use super::shape::S8AccessShape;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8BatchPointBasis {
    DeclaredBatchSet,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8SortedBatchBasis {
    CanonicallySortedBatch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8RangeBasis {
    CanonicalRangeBounds,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8MultiRangeBasis {
    DeclaredDisjointRangeSet,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8PrefixBasis {
    CanonicalPrefixBounds,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8GroupedPrefixBasis {
    CanonicalGroupedPrefixes,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8CoalescedPageReadBasis {
    AdjacentPageWindow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8ChunkTreeWalkBasis {
    RootedChunkTraversal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8ManifestGraphWalkBasis {
    ManifestAuthorityGraph,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8BoundedScanBasis {
    LocalityBoundedTraversal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8FullDeclaredScanBasis {
    DeclaredFullTraversal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8StreamingReadBasis {
    SequentialStreamTraversal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8StreamingContinuationBasis {
    ResumeCursorContinuation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8MutationAccessBasis {
    WalBeforeDataAppend,
    CompactionRewriteTraversal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8MaintenanceReadBasis {
    RebuildTraversal,
    VerifierTraversal,
    RepairTraversal,
    QuarantineTraversal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8DegradedExactScanBasis {
    BudgetedCounterBoundedTraversal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8AccessShapeDetail {
    PointLookup,
    BatchPointLookup(S8BatchPointBasis),
    SortedBatchLookup(S8SortedBatchBasis),
    RangeLookup(S8RangeBasis),
    MultiRangeLookup(S8MultiRangeBasis),
    PrefixLookup(S8PrefixBasis),
    GroupedPrefixLookup(S8GroupedPrefixBasis),
    CoalescedPageRead(S8CoalescedPageReadBasis),
    ChunkTreeWalk(S8ChunkTreeWalkBasis),
    ManifestGraphWalk(S8ManifestGraphWalkBasis),
    BoundedScan(S8BoundedScanBasis),
    FullDeclaredScan(S8FullDeclaredScanBasis),
    StreamingRead(S8StreamingReadBasis),
    StreamingContinuationRead(S8StreamingContinuationBasis),
    Append(S8MutationAccessBasis),
    CompactionRead(S8MutationAccessBasis),
    RebuildRead(S8MaintenanceReadBasis),
    VerifierRead(S8MaintenanceReadBasis),
    RepairRead(S8MaintenanceReadBasis),
    QuarantineRead(S8MaintenanceReadBasis),
    DegradedExactScan(S8DegradedExactScanBasis),
}

impl S8AccessShapeDetail {
    pub const fn shape(self) -> S8AccessShape {
        match self {
            Self::PointLookup => S8AccessShape::PointLookup,
            Self::BatchPointLookup(_) => S8AccessShape::BatchPointLookup,
            Self::SortedBatchLookup(_) => S8AccessShape::SortedBatchLookup,
            Self::RangeLookup(_) => S8AccessShape::RangeLookup,
            Self::MultiRangeLookup(_) => S8AccessShape::MultiRangeLookup,
            Self::PrefixLookup(_) => S8AccessShape::PrefixLookup,
            Self::GroupedPrefixLookup(_) => S8AccessShape::GroupedPrefixLookup,
            Self::CoalescedPageRead(_) => S8AccessShape::CoalescedPageRead,
            Self::ChunkTreeWalk(_) => S8AccessShape::ChunkTreeWalk,
            Self::ManifestGraphWalk(_) => S8AccessShape::ManifestGraphWalk,
            Self::BoundedScan(_) => S8AccessShape::BoundedScan,
            Self::FullDeclaredScan(_) => S8AccessShape::FullDeclaredScan,
            Self::StreamingRead(_) => S8AccessShape::StreamingRead,
            Self::StreamingContinuationRead(_) => S8AccessShape::StreamingContinuationRead,
            Self::Append(_) => S8AccessShape::Append,
            Self::CompactionRead(_) => S8AccessShape::CompactionRead,
            Self::RebuildRead(_) => S8AccessShape::RebuildRead,
            Self::VerifierRead(_) => S8AccessShape::VerifierRead,
            Self::RepairRead(_) => S8AccessShape::RepairRead,
            Self::QuarantineRead(_) => S8AccessShape::QuarantineRead,
            Self::DegradedExactScan(_) => S8AccessShape::DegradedExactScan,
        }
    }
}
