mod adoption;
mod conflict;
mod core;
mod execute;
mod journal;
mod plan;
mod policy;
mod result;

pub(crate) use execute::{
    adopt_source_node_into_target, merge_comparable, remap_dependency_snapshot,
};
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
    BranchMergeRequest, BranchMergeStrategy, MergeCandidateScope,
};
#[allow(unused_imports)]
pub use journal::{
    BranchMutationJournalSlice, BranchMutationLedger, MergeNodeMap, StructuralMergeCandidateRecord,
};
#[allow(unused_imports)]
pub use plan::{
    ArtifactMergeComparable, BranchMergePlan, DependencyFingerprint, NodeMergeInputState,
    NodeMergePlan, NodeReconciliationDecision, NodeReconciliationShape,
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
