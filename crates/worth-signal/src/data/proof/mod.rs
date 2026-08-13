//! Proof-bearing forms crossing Signal's planning, invalidation, and commit phases.

mod contracts;
mod delta;
mod dependency_batch;
mod dirty_batch;
pub(crate) mod invalidation;
mod locality;
mod ordered_stream;
mod snapshot_commit;
mod subscriber_repair;

pub use contracts::{
    CanonicalForm, DeltaForm, DesiredState, LoweredForm, ResolvedForm, SingleConsumer, SummaryForm,
};
pub use delta::{DirtyDelta, PatchPlan, StructuralDelta};
pub use dependency_batch::{DependencyBatchEdit, DependencySetEdit};
#[allow(deprecated)]
pub use dirty_batch::{DirtyBatch, DirtyBatchEntry, SemanticBatchCommit, SourceRecomputeAdmission};
pub use invalidation::{
    FrontierEntryClassification, FrontierExecutionCounters, FrontierExecutionSummary,
    FrontierInclusionBasis, FrontierPlan, FrontierPredictedCounters, FrontierSeedCause,
    FrontierValidationDecision, FrontierWaveEntryPlan, FrontierWaveEntrySummary, FrontierWavePlan,
    FrontierWaveSummary, InvalidationSeed, InvalidationSeedBatch, InvalidationTraceRecord,
    TransitiveFrontierEntrySummary, TransitiveFrontierWaveSummary,
};
pub use locality::{
    DedupedNodeBatch, LocalityFootprint, PartitionScopeSet, SortedSourceBatch, TouchedScopeSummary,
};
pub use ordered_stream::{
    LocallyOrderedShard, MergeableOrderedStream, OrderedStreamItem, OrderedStreamMergeError,
};
pub use snapshot_commit::{
    ClassifiedSnapshotBatchCommit, MixedSnapshotBatchCommit, PendingSnapshotBatch,
    PendingSnapshotCommit, SnapshotBatchCommit, StableShapeSnapshotBatchCommit,
};
pub use subscriber_repair::{SubscriberRepair, SubscriberRepairBatch};
