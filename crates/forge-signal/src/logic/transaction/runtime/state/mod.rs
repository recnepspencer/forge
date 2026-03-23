mod branching;
mod builder;
mod merge;
mod mutation;
mod observation;
mod observer;
mod reconstructability;
mod runtime_state;

pub use builder::SignalRuntimeBuilder;
#[allow(unused_imports)]
pub use merge::{
    ArtifactMergeAction, ArtifactMergeComparable, BranchMergeBase,
    BranchConflictResolutionPlan, BranchMergeConflictEvidence, BranchMergeConflictKind, BranchMergeConflictRecord,
    BranchMergeConflictSummary, BranchMergeCounters, BranchMergeDivergence,
    BranchMergeExecutionSummary, BranchMergeKind, BranchMergePlan,
    BranchMergeReconciliationPolicy, BranchMergeRequest, BranchMergeResolutionRequirement,
    BranchMergeResult, ConflictResolutionRecord, ConflictResolutionStrategy,
    BranchMergeStrategy, BranchMergeFailureKind, BranchMutationJournalSlice,
    BranchMutationLedger, CausalityCarryPolicy, ConflictMergePolicy,
    ConservativeOverlapExpansion, DependencyFingerprint, DependencyRemapRecord,
    ExistingTargetMergePolicy, LoweredMergePlan, MergeBoundaryWitness,
    MergeBoundaryWitnessKind, MergeDecisionBasis, MergeNodeMap, MergeTouchedNodeSet,
    MergedArtifactRecord, NodeMergeInputState, NodeMergePlan, NodeReconciliationDecision,
    NodeReconciliationShape, PlannedMergeCandidateSet, ProofMinimalOverlapBasis,
    RetainedArtifactCarryPolicy, RuntimeArtifactCarryPolicy, SourceNodeAdoptionCarryPolicy,
    SourceNodeAdoptionPlanCore, SourceOnlyMergePolicy, StructuralMergeCandidateRecord,
    StructuralMergeJournalSlice, TargetNodeIdentityIntent, TopologyRepairSummary,
};
pub use observer::{RuntimeMaterializer, RuntimeObserver};
#[allow(unused_imports)]
pub use reconstructability::{CheckpointRecord, JournalSegment, ReconstructabilityRecord};
#[allow(unused_imports)]
pub use reconstructability::{
    BoundedJournalSegment, CheckpointBoundary, DependencyIndexRebuildProof,
    MergeSupportRebuildProof, ReconstructabilityProof, ReplaySuffixRebuildProof,
    RequiredDerivedRebuildSet,
};
pub use runtime_state::SignalRuntime;
