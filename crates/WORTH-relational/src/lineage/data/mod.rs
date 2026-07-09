mod artifacts;
mod correspondence;
mod events;
mod graph;
mod invariants;
mod metrics;
mod resolution;

pub use artifacts::{
    LineageArtifactCounters, LineageCheckpointArtifact, LineageCheckpointCounters,
    LineageCheckpointDigestBasis, LineageDecisionKind, LineageDecisionLogDigestBasis,
    LineageDecisionRecord, LineageDigestBasis, LineageEventBatchDigestBasis,
};
pub use correspondence::{
    CorrespondenceCandidate, CorrespondenceCandidateId,
    CorrespondencePromotionExecutionFailureClass, CorrespondencePromotionOutcome,
    CorrespondencePromotionRejectionClass, CorrespondenceResolution, LineageResolutionStatus,
};
pub use events::{LineageEventKind, LineageEventRecord};
pub use graph::{
    LineageDivergenceMetrics, LineageDivergenceRequest, LineageDivergenceSummary,
    LineageDivergenceTraversalBasis, LineageGraphDigestBasis, LineageGraphDigestMode,
    LineageGraphMetrics, LineageGraphRequest, LineageGraphSnapshot, LineageGraphTraversalBasis,
    LineageNode,
};
pub use invariants::LineageInvariant;
pub use metrics::LineageFinalizationCounters;
pub use resolution::{
    HistoricalLineageResolution, HistoricalLineageResolutionDigestBasis,
    HistoricalLineageResolutionMetrics, HistoricalResolutionBoundednessBasis,
    HistoricalResolutionDigestMode, HistoricalResolutionRequest, HistoricalResolutionTrace,
    RecordHistoryRequest,
};

#[cfg(test)]
pub(crate) use artifacts::LineageRejectionArtifact;
pub(crate) use artifacts::{
    FinalizedLineageEventBatch, LineageDecisionLog, LineageFinalizationArtifact,
    PublishedLineageArtifact,
};
