mod accessors;
mod candidates;
mod construction;

pub use candidates::{
    AspectMergeDecisionOutcome, ConflictIsolationWitness, ConservativeIsolationExpansion,
    ConservativeOverlapExpansion, IdentityCorrespondenceBasis, IdentityCorrespondenceRecord,
    IdentityCorrespondenceStatus, LoweredAspectMergeDecisionPlan, LoweredAspectMergeDecisionRecord,
    LoweredAspectMergePolicyPlan, LoweredAspectMergePolicyRecord, LoweredConflictIsolationPlan,
    LoweredConflictIsolationRecord, LoweredDeletionPolicyPlan, LoweredIdentityCorrespondencePlan,
    LoweredMergeBasePlan, PlannedMergeCandidateSet, ProofMinimalOverlapBasis,
    RegionIsolationSummary,
};

use serde::{Deserialize, Serialize};

use crate::data::core_profile::StableHashValue;
use crate::data::handle::NodeId;
use crate::data::output::{ArtifactContinuityToken, OutputIdentity};
use crate::data::reuse::ReuseBasis;
use crate::data::trace::ArtifactMergeAuthority;
use crate::diagnostics::lineage::LineageArtifactId;

use super::adoption::{SourceNodeAdoptionCarryPolicy, SourceNodeAdoptionPlanCore};
use super::candidate_scope::LoweredScopedMergeCandidateSet;
use super::conflict::{BranchConflictResolutionPlan, BranchMergeConflictKind};
use super::conflict_isolation_registry::{
    ConflictIsolationPolicyName, ConflictIsolationSelectionBasis,
};
use super::conflict_policy_registry::{ConflictPolicyName, ConflictPolicySelectionBasis};
use super::core::{
    BranchMergeBase, BranchMergeDivergence, BranchMergeKind, BranchMergeStrategy,
    MergeBoundaryWitness,
};
use super::deletion_policy_registry::{DeletionPolicyName, DeletionPolicySelectionBasis};
use super::identity_matcher_registry::{IdentityMatcherName, IdentityMatcherSelectionBasis};
use super::journal::{BranchMutationJournalSlice, MergeNodeMap, StructuralMergeJournalSlice};
use super::policy::BranchMergeReconciliationPolicy;
use super::scoped_proof::ScopedMergeProofPacket;
use super::semantics::SelectedMergeSemanticsBundle;
use super::source_only_policy_registry::{SourceOnlyPolicyName, SourceOnlyPolicySelectionBasis};
use super::strategy_registry::{MergeStrategyName, MergeStrategySelectionBasis};
use super::SignalMergeStrategyWitness;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeReconciliationShape {
    ExistingTargetNode { target_node: NodeId },
    SourceOnlyIntroduction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeReconciliationDecision {
    PreserveTarget,
    AdoptSourceAuthority,
    ReplaceTargetAuthority,
    MarkEquivalentUnchanged,
    SkipNonAdoptableSource,
    RejectRequiresConflictResolution,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DependencyFingerprint {
    pub dependency_count: u32,
    pub meaningful_input_changes: u32,
    pub output_hash: StableHashValue,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactMergeComparable {
    pub output_identity: Option<OutputIdentity>,
    pub continuity_token: Option<ArtifactContinuityToken>,
    pub reuse_basis: ReuseBasis,
    pub dependency_fingerprint: DependencyFingerprint,
    pub authority: ArtifactMergeAuthority,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NodeMergeInputState {
    current_artifact_id: Option<LineageArtifactId>,
    comparable: Option<ArtifactMergeComparable>,
    authority: Option<ArtifactMergeAuthority>,
    exists_in_branch: bool,
}

impl NodeMergeInputState {
    pub fn new(
        current_artifact_id: Option<LineageArtifactId>,
        comparable: Option<ArtifactMergeComparable>,
        authority: Option<ArtifactMergeAuthority>,
        exists_in_branch: bool,
    ) -> Self {
        Self {
            current_artifact_id,
            comparable,
            authority,
            exists_in_branch,
        }
    }

    pub fn current_artifact_id(&self) -> Option<LineageArtifactId> {
        self.current_artifact_id
    }

    pub fn comparable(&self) -> Option<&ArtifactMergeComparable> {
        self.comparable.as_ref()
    }

    pub fn authority(&self) -> Option<&ArtifactMergeAuthority> {
        self.authority.as_ref()
    }

    pub fn exists_in_branch(&self) -> bool {
        self.exists_in_branch
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NodeMergePlan {
    source_node: NodeId,
    shape: NodeReconciliationShape,
    source_state: NodeMergeInputState,
    target_state: NodeMergeInputState,
    decision: NodeReconciliationDecision,
    resolved_conflict_kinds: Vec<BranchMergeConflictKind>,
}

impl NodeMergePlan {
    pub fn new(
        source_node: NodeId,
        shape: NodeReconciliationShape,
        source_state: NodeMergeInputState,
        target_state: NodeMergeInputState,
        decision: NodeReconciliationDecision,
        resolved_conflict_kinds: Vec<BranchMergeConflictKind>,
    ) -> Self {
        Self {
            source_node,
            shape,
            source_state,
            target_state,
            decision,
            resolved_conflict_kinds,
        }
    }

    pub fn source_node(&self) -> NodeId {
        self.source_node
    }

    pub fn shape(&self) -> NodeReconciliationShape {
        self.shape
    }

    pub fn source_state(&self) -> &NodeMergeInputState {
        &self.source_state
    }

    pub fn target_state(&self) -> &NodeMergeInputState {
        &self.target_state
    }

    pub fn decision(&self) -> NodeReconciliationDecision {
        self.decision
    }

    pub fn resolved_conflict_kinds(&self) -> &[BranchMergeConflictKind] {
        &self.resolved_conflict_kinds
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct LoweredMergePlan {
    source_branch_id: crate::state::SignalBranchId,
    target_branch_id: crate::state::SignalBranchId,
    schema_registry_digest: String,
    registry_bundle_digest: String,
    lowered_strategy_bundle_digest: String,
    merge_kind: BranchMergeKind,
    divergence: BranchMergeDivergence,
    merge_strategy: BranchMergeStrategy,
    selected_strategy_name: MergeStrategyName,
    selected_strategy_digest: String,
    selected_strategy_basis: MergeStrategySelectionBasis,
    selected_conflict_policy_name: ConflictPolicyName,
    selected_conflict_policy_digest: String,
    selected_conflict_policy_basis: ConflictPolicySelectionBasis,
    selected_conflict_isolation_name: ConflictIsolationPolicyName,
    selected_conflict_isolation_digest: String,
    selected_conflict_isolation_basis: ConflictIsolationSelectionBasis,
    selected_identity_matcher_name: IdentityMatcherName,
    selected_identity_matcher_digest: String,
    selected_identity_matcher_basis: IdentityMatcherSelectionBasis,
    selected_source_only_policy_name: SourceOnlyPolicyName,
    selected_source_only_policy_digest: String,
    selected_source_only_policy_basis: SourceOnlyPolicySelectionBasis,
    selected_deletion_policy_name: DeletionPolicyName,
    selected_deletion_policy_digest: String,
    selected_deletion_policy_basis: DeletionPolicySelectionBasis,
    selected_semantics: SelectedMergeSemanticsBundle,
    strategy_witness: SignalMergeStrategyWitness,
    reconciliation_policy: BranchMergeReconciliationPolicy,
    boundary_witness: MergeBoundaryWitness,
    source_journal: StructuralMergeJournalSlice,
    target_overlap_journal: BranchMutationJournalSlice,
    identity_correspondence: LoweredIdentityCorrespondencePlan,
    deletion_plan: LoweredDeletionPolicyPlan,
    conflict_isolation_plan: LoweredConflictIsolationPlan,
    aspect_policy_plan: LoweredAspectMergePolicyPlan,
    aspect_decision_plan: LoweredAspectMergeDecisionPlan,
    scoped_candidates: LoweredScopedMergeCandidateSet,
    scoped_merge_proof: ScopedMergeProofPacket,
    proof_minimal_overlap: ProofMinimalOverlapBasis,
    conservative_overlap: ConservativeOverlapExpansion,
    planned_candidates: PlannedMergeCandidateSet,
    source_snapshot_id: Option<crate::state::SignalSnapshotId>,
    target_snapshot_id_before: Option<crate::state::SignalSnapshotId>,
    merge_base: Option<BranchMergeBase>,
    lowered_merge_base: Option<LoweredMergeBasePlan>,
    resolution_plan: Option<BranchConflictResolutionPlan>,
    node_map: MergeNodeMap,
    node_plan: Vec<NodeMergePlan>,
    adoption_core: Vec<SourceNodeAdoptionPlanCore>,
    adoption_policy: Vec<SourceNodeAdoptionCarryPolicy>,
}

impl LoweredMergePlan {
    pub fn strategy_witness(&self) -> &SignalMergeStrategyWitness {
        &self.strategy_witness
    }

    pub fn scoped_candidates(&self) -> &LoweredScopedMergeCandidateSet {
        &self.scoped_candidates
    }

    pub fn scoped_merge_proof(&self) -> &ScopedMergeProofPacket {
        &self.scoped_merge_proof
    }
}

pub type BranchMergePlan = LoweredMergePlan;
