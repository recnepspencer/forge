//! Proof-bearing forms crossing Signal's planning, invalidation, and commit phases.

mod contracts;
mod delta;
mod dependency_batch;
mod dirty_batch;
mod invalidation_admission;
mod invalidation_execution;
mod invalidation_frontier;
mod invalidation_plan;
mod locality;
mod ordered_stream;
mod snapshot_commit;
mod subscriber_repair;

pub use contracts::{
    CanonicalForm, DeltaForm, DesiredState, LoweredForm, ResolvedForm, SingleConsumer, SummaryForm,
};
pub use delta::{DirtyDelta, PatchPlan, StructuralDelta};
pub use dependency_batch::{DependencyBatchEdit, DependencySetEdit};
pub use dirty_batch::{DirtyBatch, DirtyBatchEntry, SemanticBatchCommit};
pub use invalidation_admission::{
    FrontierEntryClassification, FrontierInclusionBasis, FrontierSeedCause,
    FrontierValidationDecision, InvalidationSeed, InvalidationSeedBatch,
};
pub use invalidation_execution::{
    FrontierExecutionCounters, FrontierExecutionSummary, FrontierWaveEntrySummary,
    FrontierWaveSummary, InvalidationTraceRecord,
};
pub use invalidation_frontier::{FrontierWave, InvalidationFrontier, NarrowedPropagationSet};
pub use invalidation_plan::{
    FrontierPlan, FrontierPredictedCounters, FrontierWaveEntryPlan, FrontierWavePlan,
    TransitiveFrontierRoot,
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
