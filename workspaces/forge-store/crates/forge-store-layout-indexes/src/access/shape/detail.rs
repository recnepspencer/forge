use super::shape::AccessShape;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BatchPointBasis {
    DeclaredBatchSet,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortedBatchBasis {
    CanonicallySortedBatch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RangeBasis {
    CanonicalRangeBounds,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MultiRangeBasis {
    DeclaredDisjointRangeSet,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrefixBasis {
    CanonicalPrefixBounds,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GroupedPrefixBasis {
    CanonicalGroupedPrefixes,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoalescedPageReadBasis {
    AdjacentPageWindow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChunkTreeWalkBasis {
    RootedChunkTraversal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManifestGraphWalkBasis {
    ManifestAuthorityGraph,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoundedScanBasis {
    LocalityBoundedTraversal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FullDeclaredScanBasis {
    DeclaredFullTraversal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StreamingReadBasis {
    SequentialStreamTraversal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StreamingContinuationBasis {
    ResumeCursorContinuation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MutationAccessBasis {
    WalBeforeDataAppend,
    CompactionRewriteTraversal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaintenanceReadBasis {
    RebuildTraversal,
    VerifierTraversal,
    RepairTraversal,
    QuarantineTraversal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DegradedExactScanBasis {
    BudgetedCounterBoundedTraversal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessShapeDetail {
    PointLookup,
    BatchPointLookup(BatchPointBasis),
    SortedBatchLookup(SortedBatchBasis),
    RangeLookup(RangeBasis),
    MultiRangeLookup(MultiRangeBasis),
    PrefixLookup(PrefixBasis),
    GroupedPrefixLookup(GroupedPrefixBasis),
    CoalescedPageRead(CoalescedPageReadBasis),
    ChunkTreeWalk(ChunkTreeWalkBasis),
    ManifestGraphWalk(ManifestGraphWalkBasis),
    BoundedScan(BoundedScanBasis),
    FullDeclaredScan(FullDeclaredScanBasis),
    StreamingRead(StreamingReadBasis),
    StreamingContinuationRead(StreamingContinuationBasis),
    Append(MutationAccessBasis),
    CompactionRead(MutationAccessBasis),
    RebuildRead(MaintenanceReadBasis),
    VerifierRead(MaintenanceReadBasis),
    RepairRead(MaintenanceReadBasis),
    QuarantineRead(MaintenanceReadBasis),
    DegradedExactScan(DegradedExactScanBasis),
}

impl AccessShapeDetail {
    pub const fn shape(self) -> AccessShape {
        match self {
            Self::PointLookup => AccessShape::PointLookup,
            Self::BatchPointLookup(_) => AccessShape::BatchPointLookup,
            Self::SortedBatchLookup(_) => AccessShape::SortedBatchLookup,
            Self::RangeLookup(_) => AccessShape::RangeLookup,
            Self::MultiRangeLookup(_) => AccessShape::MultiRangeLookup,
            Self::PrefixLookup(_) => AccessShape::PrefixLookup,
            Self::GroupedPrefixLookup(_) => AccessShape::GroupedPrefixLookup,
            Self::CoalescedPageRead(_) => AccessShape::CoalescedPageRead,
            Self::ChunkTreeWalk(_) => AccessShape::ChunkTreeWalk,
            Self::ManifestGraphWalk(_) => AccessShape::ManifestGraphWalk,
            Self::BoundedScan(_) => AccessShape::BoundedScan,
            Self::FullDeclaredScan(_) => AccessShape::FullDeclaredScan,
            Self::StreamingRead(_) => AccessShape::StreamingRead,
            Self::StreamingContinuationRead(_) => AccessShape::StreamingContinuationRead,
            Self::Append(_) => AccessShape::Append,
            Self::CompactionRead(_) => AccessShape::CompactionRead,
            Self::RebuildRead(_) => AccessShape::RebuildRead,
            Self::VerifierRead(_) => AccessShape::VerifierRead,
            Self::RepairRead(_) => AccessShape::RepairRead,
            Self::QuarantineRead(_) => AccessShape::QuarantineRead,
            Self::DegradedExactScan(_) => AccessShape::DegradedExactScan,
        }
    }
}
