mod adoption;
mod aspect_policy_registry;
mod candidate_scope;
mod canonical_basis;
mod compatibility;
mod conflict;
mod conflict_isolation_registry;
mod conflict_policy_registry;
mod core;
mod deletion_policy_registry;
mod diagnostic_surfaces;
mod execute;
mod foundational_scope;
mod identity_matcher_registry;
mod journal;
mod locator;
mod merge_base_registry;
mod plan;
mod policy;
mod proof;
mod request;
mod result;
mod scoped_admission;
mod scoped_failure;
mod scoped_proof;
mod semantics;
mod source_only_policy_registry;
mod strategy_identity;
mod strategy_registry;
mod strategy_witness;
mod strategy_witness_denial;

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
pub use candidate_scope::{LoweredScopedMergeCandidateSet, ScopedMergeCandidateBreadthSummary};
pub use canonical_basis::SignalScopedMergeCanonicalBasisBundle;
#[allow(unused_imports)]
pub use compatibility::{
    bridge_signal_merge_compatibility_trust_boundary, bridged_compatibility_posture_kind,
    compatibility_posture_kind, BoundaryBridgedSignalMergeCompatibilityArtifact,
    SignalMergeCompatibilityArtifact, SignalMergeCompatibilityAuthority,
    SignalMergeCompatibilityBasis, SignalMergeCompatibilityDenial,
    SignalMergeCompatibilityDenialKind, SignalMergeCompatibilityFactInventory,
    SignalMergeCompatibilityPostureKind, SignalMergeCompatibilityReadmissionAuthority,
    SignalMergeCompatibilityReady, SignalMergeCompatibilityWitness,
    SIGNAL_MERGE_COMPATIBILITY_SCHEMA_VERSION,
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
    BranchMergeStrategy, MergeBoundaryWitness, MergeBoundaryWitnessKind,
};
#[allow(unused_imports)]
pub use deletion_policy_registry::{
    DeletionPolicyDescriptor, DeletionPolicyId, DeletionPolicyName, DeletionPolicyRegistration,
    DeletionPolicySelectionBasis, DeletionPolicyVersion, DuplicateDeletionPolicyRegistration,
    FrozenDeletionPolicyRegistry,
};
pub use diagnostic_surfaces::SignalScopedMergeCanonicalLocatorBundle;
pub(crate) use execute::{adopt_source_node_into_target, remap_dependency_snapshot};
pub(crate) use foundational_scope::{
    foundational_branch_id, foundational_denied_aspect_locus, foundational_denied_node_locus,
};
pub use foundational_scope::{
    signal_scope_family_matches_foundational_family, FoundationalScopeLoweringDenial,
    LoweredFoundationalMergeRequest,
};
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
pub use locator::{SignalScopedMergeDiagnosticRow, SignalScopedMergeLocatorBundle};
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
pub(crate) use plan::{
    LoweredMergePlanConstruction, LoweredMergePlanDecisions, LoweredMergePlanJournals,
    LoweredMergePlanLineage, LoweredMergePlanNodes, LoweredMergePlanScope, LoweredMergePlanWorld,
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
pub use request::{
    BranchMergeRequest, BranchMergeRequestDenial, BranchMergeRequestScope,
    BranchMergeRequestScopeFamily, NormalizedBranchMergeRequest, NormalizedBranchMergeRequestScope,
    SignalSelectedAspectRequestEntry,
};
#[allow(unused_imports)]
pub use result::{
    ArtifactMergeAction, BranchMergeCounters, BranchMergeExecutionSummary, BranchMergeResult,
    DependencyRemapRecord, MergeDecisionBasis, MergeTouchedNodeSet, MergedArtifactRecord,
    TopologyRepairSummary,
};
pub(crate) use scoped_admission::{
    classify_initial_scoped_merge_admission, deny_selected_node_non_adoptable,
    deny_selected_target_rejected_by_declaration,
};
pub(crate) use scoped_failure::{
    rewrite_identity_scoped_admission_error, scoped_admission_outcome_to_signal_error,
};
pub use scoped_failure::{
    BranchMergeScopedDenialFailureEvidence, BranchMergeScopedDenialKind,
    BranchMergeScopedDeniedLocus, BranchMergeScopedUnavailableFailureEvidence,
    BranchMergeScopedUnavailableOutcomeKind, BranchMergeScopedUnavailableReason,
};
pub use scoped_proof::ScopedMergeProofPacket;
#[allow(unused_imports)]
pub use semantics::SelectedMergeSemanticsBundle;
#[allow(unused_imports)]
pub use source_only_policy_registry::{
    DuplicateSourceOnlyPolicyRegistration, FrozenSourceOnlyPolicyRegistry,
    SourceOnlyPolicyDescriptor, SourceOnlyPolicyId, SourceOnlyPolicyName,
    SourceOnlyPolicyRegistration, SourceOnlyPolicySelectionBasis, SourceOnlyPolicyVersion,
};
pub(crate) use strategy_identity::aspect_policy_inventory;
pub use strategy_identity::{
    SignalAspectPolicyInventoryEntry, SignalDeliveryStrategyIdentity,
    SignalInvalidationStrategyIdentity, SignalMergeStrategyIdentity,
};
#[allow(unused_imports)]
pub use strategy_registry::{
    DuplicateMergeStrategyRegistration, FrozenMergeStrategyRegistry, MergeStrategyDescriptor,
    MergeStrategyId, MergeStrategyName, MergeStrategyRegistration, MergeStrategySelectionBasis,
    MergeStrategyVersion,
};
pub use strategy_witness::SignalMergeStrategyWitness;
pub use strategy_witness_denial::{
    SignalMergeStrategyWitnessDenial, SignalMergeStrategyWitnessDenialKind,
};
