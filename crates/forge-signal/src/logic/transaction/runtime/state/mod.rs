mod branching;
mod builder;
mod guided;
mod merge;
mod mutation;
mod observation;
mod observer;
mod reconstructability;
mod runtime_state;

pub(in crate::logic::transaction::runtime) use branching::BranchManager;
pub use builder::SignalRuntimeBuilder;
pub use guided::{PlannedRuntimeMerge, RuntimeHistory, RuntimeMerge};
#[allow(unused_imports)]
pub use merge::{
    ArtifactMergeAction, ArtifactMergeComparable, BranchConflictResolutionPlan, BranchMergeBase,
    BranchMergeConflictEvidence, BranchMergeConflictKind, BranchMergeConflictRecord,
    BranchMergeConflictSummary, BranchMergeCounters, BranchMergeDivergence,
    BranchMergeExecutionSummary, BranchMergeFailureKind, BranchMergeKind, BranchMergePlan,
    BranchMergeReconciliationPolicy, BranchMergeRequest, BranchMergeResolutionRequirement,
    BranchMergeResult, BranchMergeStrategy, BranchMutationJournalSlice, BranchMutationLedger,
    CausalityCarryPolicy, ConflictMergePolicy, ConflictResolutionRecord,
    ConflictResolutionStrategy, ConservativeOverlapExpansion, DependencyFingerprint,
    DependencyRemapRecord, ExistingTargetMergePolicy, LoweredMergePlan, MergeBoundaryWitness,
    MergeBoundaryWitnessKind, MergeDecisionBasis, MergeNodeMap, MergeTouchedNodeSet,
    MergedArtifactRecord, NodeMergeInputState, NodeMergePlan, NodeReconciliationDecision,
    NodeReconciliationShape, PlannedMergeCandidateSet, ProofMinimalOverlapBasis,
    RetainedArtifactCarryPolicy, RuntimeArtifactCarryPolicy, SourceNodeAdoptionCarryPolicy,
    SourceNodeAdoptionPlanCore, SourceOnlyMergePolicy, StructuralMergeCandidateRecord,
    StructuralMergeJournalSlice, TargetNodeIdentityIntent, TopologyRepairSummary,
};
pub use observer::{RuntimeMaterializer, RuntimeObserver};
#[allow(unused_imports)]
pub use reconstructability::{
    BoundedJournalSegment, CheckpointBoundary, DependencyIndexRebuildProof,
    MergeSupportRebuildProof, ReconstructabilityProof, ReplaySuffixRebuildProof,
    RequiredDerivedRebuildSet,
};
#[allow(unused_imports)]
pub use reconstructability::{CheckpointRecord, JournalSegment, ReconstructabilityRecord};
pub use runtime_state::SignalRuntime;
