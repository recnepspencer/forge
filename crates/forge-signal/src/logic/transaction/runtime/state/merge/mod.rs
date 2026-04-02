mod adoption;
mod aspect_policy_registry;
mod conflict;
mod conflict_isolation_registry;
mod conflict_policy_registry;
mod core;
mod deletion_policy_registry;
mod execute;
mod identity_matcher_registry;
mod journal;
mod merge_base_registry;
mod plan;
mod policy;
mod proof;
mod result;
mod semantics;
mod source_only_policy_registry;
mod strategy_registry;

#[allow(unused_imports)]
pub use adoption::{
    AdoptedNodeContract, AdoptedNodeMaterialization, AdoptionDependencySnapshotRef,
    AdoptionDependencyTopology, CausalityCarryPolicy, RetainedArtifactCarryPolicy,
    RuntimeArtifactCarryPolicy, SourceNodeAdoptionCarryPolicy, SourceNodeAdoptionPlanCore,
    TargetNodeIdentityIntent,
};
#[allow(unused_imports)]
pub use aspect_policy_registry::{
    AspectMergePolicyBinding, AspectMergePolicyDescriptor, AspectMergePolicyId,
    AspectMergePolicyName, AspectMergePolicyRegistration, AspectMergePolicySelectionBasis,
    AspectMergePolicyVersion, DuplicateAspectMergePolicyRegistration,
    FrozenAspectMergePolicyRegistry,
};
#[allow(unused_imports)]
pub use conflict::{
    BranchConflictResolutionPlan, BranchMergeConflictEvidence, BranchMergeConflictKind,
    BranchMergeConflictRecord, BranchMergeConflictSummary, BranchMergeDeletionFailureEvidence,
    BranchMergeFailureEvidence, BranchMergeIdentityFailureEvidence,
    BranchMergeResolutionRequirement, ConflictResolutionRecord, ConflictResolutionStrategy,
};
#[allow(unused_imports)]
pub use conflict_isolation_registry::{
    ConflictIsolationPolicyDescriptor, ConflictIsolationPolicyId, ConflictIsolationPolicyName,
    ConflictIsolationPolicyRegistration, ConflictIsolationPolicyVersion,
    ConflictIsolationSelectionBasis, DuplicateConflictIsolationPolicyRegistration,
    FrozenConflictIsolationRegistry,
};
#[allow(unused_imports)]
pub use conflict_policy_registry::{
    ConflictPolicyDescriptor, ConflictPolicyId, ConflictPolicyName, ConflictPolicyRegistration,
    ConflictPolicySelectionBasis, ConflictPolicyVersion, DuplicateConflictPolicyRegistration,
    FrozenConflictPolicyRegistry,
};
#[allow(unused_imports)]
pub use core::{
    BranchMergeBase, BranchMergeDivergence, BranchMergeFailureKind, BranchMergeKind,
    BranchMergeRequest, BranchMergeStrategy, MergeBoundaryWitness, MergeBoundaryWitnessKind,
};
#[allow(unused_imports)]
pub use deletion_policy_registry::{
    DeletionPolicyDescriptor, DeletionPolicyId, DeletionPolicyName, DeletionPolicyRegistration,
    DeletionPolicySelectionBasis, DeletionPolicyVersion, DuplicateDeletionPolicyRegistration,
    FrozenDeletionPolicyRegistry,
};
pub(crate) use execute::{adopt_source_node_into_target, remap_dependency_snapshot};
#[allow(unused_imports)]
pub use identity_matcher_registry::{
    DuplicateIdentityMatcherRegistration, FrozenIdentityMatcherRegistry, IdentityMatchPolicy,
    IdentityMatcherDescriptor, IdentityMatcherId, IdentityMatcherName, IdentityMatcherRegistration,
    IdentityMatcherSelectionBasis, IdentityMatcherVersion,
};
#[allow(unused_imports)]
pub use journal::{
    BranchMutationJournalSlice, BranchMutationLedger, MergeNodeMap, StructuralMergeCandidateRecord,
    StructuralMergeJournalSlice,
};
#[allow(unused_imports)]
pub use merge_base_registry::{
    DuplicateMergeBaseStrategyRegistration, FrozenMergeBaseStrategyRegistry,
    MergeBaseSelectionBasis, MergeBaseSelectionPolicy, MergeBaseStrategyDescriptor,
    MergeBaseStrategyId, MergeBaseStrategyName, MergeBaseStrategyRegistration,
    MergeBaseStrategyVersion,
};
#[allow(unused_imports)]
pub use plan::{
    ArtifactMergeComparable, AspectMergeDecisionOutcome, BranchMergePlan, ConflictIsolationWitness,
    ConservativeIsolationExpansion, ConservativeOverlapExpansion, DependencyFingerprint,
    IdentityCorrespondenceBasis, IdentityCorrespondenceRecord, IdentityCorrespondenceStatus,
    LoweredAspectMergeDecisionPlan, LoweredAspectMergeDecisionRecord, LoweredAspectMergePolicyPlan,
    LoweredAspectMergePolicyRecord, LoweredConflictIsolationPlan, LoweredConflictIsolationRecord,
    LoweredDeletionPolicyPlan, LoweredIdentityCorrespondencePlan, LoweredMergeBasePlan,
    LoweredMergePlan, NodeMergeInputState, NodeMergePlan, NodeReconciliationDecision,
    NodeReconciliationShape, PlannedMergeCandidateSet, ProofMinimalOverlapBasis,
    RegionIsolationSummary,
};
#[allow(unused_imports)]
pub use policy::{
    AspectMergePolicy, BranchMergeReconciliationPolicy, ConflictIsolationGranularity,
    ConflictMergePolicy, DeletionMergePolicy, ExistingTargetMergePolicy, SourceOnlyMergePolicy,
};
#[allow(unused_imports)]
pub use proof::{
    branch_state_proof_report, canonical_digest, lowered_strategy_bundle_digest,
    merge_lineage_digest, merge_plan_proof_report, merge_result_proof_report,
    replay_artifact_proof_report, replay_parity_proof_report, runtime_proof_report,
    BranchStateDenseGridProofBasis, BranchStateProofBasis, BranchStateProofReport,
    MergePlanProofReport, MergeResultProofReport, ReplayArtifactProofInput,
    ReplayArtifactProofReport, ReplayMismatchClass, ReplayParityProofReport, RuntimeProofReport,
    BRANCH_STATE_PROOF_BASIS_VERSION, MERGE_PROOF_SCHEMA_VERSION,
};
#[allow(unused_imports)]
pub use result::{
    ArtifactMergeAction, BranchMergeCounters, BranchMergeExecutionSummary, BranchMergeResult,
    DependencyRemapRecord, MergeDecisionBasis, MergeTouchedNodeSet, MergedArtifactRecord,
    TopologyRepairSummary,
};
#[allow(unused_imports)]
pub use semantics::SelectedMergeSemanticsBundle;
#[allow(unused_imports)]
pub use source_only_policy_registry::{
    DuplicateSourceOnlyPolicyRegistration, FrozenSourceOnlyPolicyRegistry,
    SourceOnlyPolicyDescriptor, SourceOnlyPolicyId, SourceOnlyPolicyName,
    SourceOnlyPolicyRegistration, SourceOnlyPolicySelectionBasis, SourceOnlyPolicyVersion,
};
#[allow(unused_imports)]
pub use strategy_registry::{
    DuplicateMergeStrategyRegistration, FrozenMergeStrategyRegistry, MergeStrategyDescriptor,
    MergeStrategyId, MergeStrategyName, MergeStrategyRegistration, MergeStrategySelectionBasis,
    MergeStrategyVersion,
};
