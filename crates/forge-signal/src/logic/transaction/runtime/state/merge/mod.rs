mod execute;
mod types;

pub(crate) use execute::{adopt_source_node_into_target, merge_comparable};
#[allow(unused_imports)]
pub use types::{
    AdoptedNodeContract, AdoptedNodeMaterialization, AdoptionDependencySnapshotRef,
    AdoptionDependencyTopology, ArtifactMergeAction, ArtifactMergeComparable, BranchMergeBase,
    BranchMergeConflictEvidence, BranchMergeConflictKind, BranchMergeConflictRecord,
    BranchMergeConflictSummary, BranchMergeCounters, BranchMergeDivergence,
    BranchMergeExecutionSummary, BranchMergeKind, BranchMergePlan,
    BranchMergeReconciliationPolicy, BranchMergeRequest, BranchMergeResolutionRequirement,
    BranchMergeResult,
    BranchMergeStrategy, BranchMergeFailureKind, ConflictMergePolicy,
    BranchMutationJournalSlice, BranchMutationLedger, CausalityCarryPolicy,
    DependencyFingerprint, DependencyRemapRecord, ExistingTargetMergePolicy,
    MergeCandidateScope, MergeDecisionBasis, MergeNodeMap, MergeTouchedNodeSet,
    MergedArtifactRecord, NodeMergeInputState, NodeMergePlan, NodeReconciliationDecision,
    NodeReconciliationShape,
    RetainedArtifactCarryPolicy, RuntimeArtifactCarryPolicy,
    SourceNodeAdoptionCarryPolicy, SourceNodeAdoptionPlanCore, SourceOnlyMergePolicy,
    StructuralMergeCandidateRecord, TargetNodeIdentityIntent, TopologyRepairSummary,
};
