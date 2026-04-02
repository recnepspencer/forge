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
pub use merge::{branch_state_proof_report, canonical_digest};
pub use merge::{
    lowered_strategy_bundle_digest, merge_lineage_digest, replay_artifact_proof_report,
    replay_parity_proof_report,
};
#[allow(unused_imports)]
pub use merge::{
    merge_plan_proof_report, merge_result_proof_report, runtime_proof_report, ArtifactMergeAction,
    ArtifactMergeComparable, AspectMergeDecisionOutcome, AspectMergePolicy,
    AspectMergePolicyBinding, AspectMergePolicyDescriptor, AspectMergePolicyId,
    AspectMergePolicyName, AspectMergePolicyRegistration, AspectMergePolicySelectionBasis,
    AspectMergePolicyVersion, BranchConflictResolutionPlan, BranchMergeBase,
    BranchMergeConflictEvidence, BranchMergeConflictKind, BranchMergeConflictRecord,
    BranchMergeConflictSummary, BranchMergeCounters, BranchMergeDeletionFailureEvidence,
    BranchMergeDivergence, BranchMergeExecutionSummary, BranchMergeFailureEvidence,
    BranchMergeFailureKind, BranchMergeIdentityFailureEvidence, BranchMergeKind, BranchMergePlan,
    BranchMergeReconciliationPolicy, BranchMergeRequest, BranchMergeResolutionRequirement,
    BranchMergeResult, BranchMergeStrategy, BranchMutationJournalSlice, BranchMutationLedger,
    BranchStateDenseGridProofBasis, BranchStateProofBasis, BranchStateProofReport,
    CausalityCarryPolicy, ConflictIsolationGranularity, ConflictIsolationPolicyDescriptor,
    ConflictIsolationPolicyId, ConflictIsolationPolicyName, ConflictIsolationPolicyRegistration,
    ConflictIsolationPolicyVersion, ConflictIsolationSelectionBasis, ConflictMergePolicy,
    ConflictPolicyDescriptor, ConflictPolicyId, ConflictPolicyName, ConflictPolicyRegistration,
    ConflictPolicySelectionBasis, ConflictPolicyVersion, ConflictResolutionRecord,
    ConflictResolutionStrategy, ConservativeOverlapExpansion, DeletionMergePolicy,
    DeletionPolicyDescriptor, DeletionPolicyId, DeletionPolicyName, DeletionPolicyRegistration,
    DeletionPolicySelectionBasis, DeletionPolicyVersion, DependencyFingerprint,
    DependencyRemapRecord, DuplicateAspectMergePolicyRegistration,
    DuplicateConflictIsolationPolicyRegistration, DuplicateConflictPolicyRegistration,
    DuplicateDeletionPolicyRegistration, DuplicateIdentityMatcherRegistration,
    DuplicateMergeBaseStrategyRegistration, DuplicateMergeStrategyRegistration,
    DuplicateSourceOnlyPolicyRegistration, ExistingTargetMergePolicy,
    FrozenAspectMergePolicyRegistry, FrozenConflictIsolationRegistry, FrozenConflictPolicyRegistry,
    FrozenDeletionPolicyRegistry, FrozenIdentityMatcherRegistry, FrozenMergeBaseStrategyRegistry,
    FrozenMergeStrategyRegistry, FrozenSourceOnlyPolicyRegistry, IdentityCorrespondenceBasis,
    IdentityCorrespondenceRecord, IdentityCorrespondenceStatus, IdentityMatchPolicy,
    IdentityMatcherDescriptor, IdentityMatcherId, IdentityMatcherName, IdentityMatcherRegistration,
    IdentityMatcherSelectionBasis, IdentityMatcherVersion, LoweredAspectMergeDecisionPlan,
    LoweredAspectMergeDecisionRecord, LoweredConflictIsolationPlan, LoweredConflictIsolationRecord,
    LoweredDeletionPolicyPlan, LoweredIdentityCorrespondencePlan, LoweredMergeBasePlan,
    LoweredMergePlan, MergeBaseSelectionBasis, MergeBaseSelectionPolicy,
    MergeBaseStrategyDescriptor, MergeBaseStrategyId, MergeBaseStrategyName,
    MergeBaseStrategyRegistration, MergeBaseStrategyVersion, MergeBoundaryWitness,
    MergeBoundaryWitnessKind, MergeDecisionBasis, MergeNodeMap, MergePlanProofReport,
    MergeResultProofReport, MergeStrategyDescriptor, MergeStrategyId, MergeStrategyName,
    MergeStrategyRegistration, MergeStrategySelectionBasis, MergeStrategyVersion,
    MergeTouchedNodeSet, MergedArtifactRecord, NodeMergeInputState, NodeMergePlan,
    NodeReconciliationDecision, NodeReconciliationShape, PlannedMergeCandidateSet,
    ProofMinimalOverlapBasis, ReplayArtifactProofInput, ReplayArtifactProofReport,
    ReplayMismatchClass, ReplayParityProofReport, RetainedArtifactCarryPolicy,
    RuntimeArtifactCarryPolicy, RuntimeProofReport, SelectedMergeSemanticsBundle,
    SourceNodeAdoptionCarryPolicy, SourceNodeAdoptionPlanCore, SourceOnlyMergePolicy,
    SourceOnlyPolicyDescriptor, SourceOnlyPolicyId, SourceOnlyPolicyName,
    SourceOnlyPolicyRegistration, SourceOnlyPolicySelectionBasis, SourceOnlyPolicyVersion,
    StructuralMergeCandidateRecord, StructuralMergeJournalSlice, TargetNodeIdentityIntent,
    TopologyRepairSummary, BRANCH_STATE_PROOF_BASIS_VERSION, MERGE_PROOF_SCHEMA_VERSION,
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
