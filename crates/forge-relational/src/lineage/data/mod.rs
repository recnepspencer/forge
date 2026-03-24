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
    CorrespondenceCandidate, CorrespondenceCandidateId, CorrespondencePromotionExecutionFailureClass,
    CorrespondencePromotionOutcome, CorrespondencePromotionRejectionClass,
    CorrespondenceResolution, LineageResolutionStatus,
};
pub use events::{LineageEventKind, LineageEventRecord};
pub use graph::{
    LineageDivergenceMetrics, LineageDivergenceRequest, LineageDivergenceSummary,
    LineageDivergenceTraversalBasis, LineageGraphDigestBasis, LineageGraphDigestMode,
    LineageGraphMetrics, LineageGraphRequest, LineageGraphSnapshot,
    LineageGraphTraversalBasis, LineageNode,
};
pub use invariants::LineageInvariant;
pub use resolution::{
    HistoricalLineageResolution, HistoricalLineageResolutionDigestBasis,
    HistoricalLineageResolutionMetrics,
    HistoricalResolutionBoundednessBasis, HistoricalResolutionDigestMode,
    HistoricalResolutionRequest, HistoricalResolutionTrace, RecordHistoryRequest,
};
pub use metrics::LineageFinalizationCounters;

pub(crate) use artifacts::{
    FinalizedLineageEventBatch, LineageDecisionLog,
    LineageFinalizationArtifact, LineageRejectionArtifact, PublishedLineageArtifact,
};
