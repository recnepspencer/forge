mod computation;
mod config;
mod execution;
mod state;
mod transaction;

pub use computation::{DefinedComputation, DefinedKeyedComputation, Recipe};
pub use config::SignalRuntimeConfig;
pub use execution::{RuntimeExecutionRequest, TransactionExecutionRequest};
pub use state::{branch_state_proof_report, canonical_digest};
pub use state::{
    lowered_strategy_bundle_digest, merge_lineage_digest, replay_artifact_proof_report,
    replay_parity_proof_report,
};
pub use state::{
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
    ConflictIsolationGranularity, ConflictIsolationPolicyDescriptor, ConflictIsolationPolicyId,
    ConflictIsolationPolicyName, ConflictIsolationPolicyRegistration,
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
    ReplayMismatchClass, ReplayParityProofReport, RuntimeProofReport, SelectedMergeSemanticsBundle,
    SourceNodeAdoptionPlanCore, SourceOnlyMergePolicy, SourceOnlyPolicyDescriptor,
    SourceOnlyPolicyId, SourceOnlyPolicyName, SourceOnlyPolicyRegistration,
    SourceOnlyPolicySelectionBasis, SourceOnlyPolicyVersion, StructuralMergeCandidateRecord,
    StructuralMergeJournalSlice, BRANCH_STATE_PROOF_BASIS_VERSION, MERGE_PROOF_SCHEMA_VERSION,
};
pub use state::{
    BoundedJournalSegment, CheckpointBoundary, CheckpointRecord, DependencyIndexRebuildProof,
    JournalSegment, MergeSupportRebuildProof, PlannedRuntimeMerge, ReconstructabilityProof,
    ReconstructabilityRecord, ReplaySuffixRebuildProof, RequiredDerivedRebuildSet, RuntimeHistory,
    RuntimeMaterializer, RuntimeMerge, RuntimeObserver, SignalRuntime, SignalRuntimeBuilder,
};
pub use transaction::{
    AdvisoryRecord, BatchChangeSession, DecisionDetail, DecisionLog, DecisionRecord,
    DecisionSummary, EvaluationSummary, IntegrityMarkers, SignalTransaction, TransactionOutcome,
    TransactionReplayEntry, TransactionResult, TransactionTiming,
};
