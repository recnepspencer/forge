mod adoption;
mod conflict;
mod core;
mod execute;
mod journal;
mod plan;
mod policy;
mod result;

#[allow(unused_imports)]
pub use adoption::{
    AdoptedNodeContract, AdoptedNodeMaterialization, AdoptionDependencySnapshotRef,
    AdoptionDependencyTopology, CausalityCarryPolicy, RetainedArtifactCarryPolicy,
    RuntimeArtifactCarryPolicy, SourceNodeAdoptionCarryPolicy, SourceNodeAdoptionPlanCore,
    TargetNodeIdentityIntent,
};
#[allow(unused_imports)]
pub use conflict::{
    BranchConflictResolutionPlan, BranchMergeConflictEvidence, BranchMergeConflictKind,
    BranchMergeConflictRecord, BranchMergeConflictSummary, BranchMergeResolutionRequirement,
    ConflictResolutionRecord, ConflictResolutionStrategy,
};
#[allow(unused_imports)]
pub use core::{
    BranchMergeBase, BranchMergeDivergence, BranchMergeFailureKind, BranchMergeKind,
    BranchMergeRequest, BranchMergeStrategy, MergeBoundaryWitness, MergeBoundaryWitnessKind,
};
pub(crate) use execute::{
    adopt_source_node_into_target, merge_comparable, remap_dependency_snapshot,
};
#[allow(unused_imports)]
pub use journal::{
    BranchMutationJournalSlice, BranchMutationLedger, MergeNodeMap, StructuralMergeCandidateRecord,
    StructuralMergeJournalSlice,
};
#[allow(unused_imports)]
pub use plan::{
    ArtifactMergeComparable, BranchMergePlan, ConservativeOverlapExpansion, DependencyFingerprint,
    LoweredMergePlan, NodeMergeInputState, NodeMergePlan, NodeReconciliationDecision,
    NodeReconciliationShape, PlannedMergeCandidateSet, ProofMinimalOverlapBasis,
};
#[allow(unused_imports)]
pub use policy::{
    BranchMergeReconciliationPolicy, ConflictMergePolicy, ExistingTargetMergePolicy,
    SourceOnlyMergePolicy,
};
#[allow(unused_imports)]
pub use result::{
    ArtifactMergeAction, BranchMergeCounters, BranchMergeExecutionSummary, BranchMergeResult,
    DependencyRemapRecord, MergeDecisionBasis, MergeTouchedNodeSet, MergedArtifactRecord,
    TopologyRepairSummary,
};
