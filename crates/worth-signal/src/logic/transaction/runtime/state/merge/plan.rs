use serde::{Deserialize, Serialize};

use crate::data::aspect::Aspect;
use crate::data::core_profile::StableHashValue;
use crate::data::handle::NodeId;
use crate::data::output::{ArtifactContinuityToken, OutputIdentity};
use crate::data::reuse::ReuseBasis;
use crate::data::trace::ArtifactMergeAuthority;
use crate::diagnostics::lineage::LineageArtifactId;

use super::adoption::{SourceNodeAdoptionCarryPolicy, SourceNodeAdoptionPlanCore};
use super::aspect_policy_registry::{AspectMergePolicyName, AspectMergePolicySelectionBasis};
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
use super::merge_base_registry::{MergeBaseSelectionBasis, MergeBaseStrategyName};
use super::policy::{BranchMergeReconciliationPolicy, ConflictIsolationGranularity};
use super::proof::lowered_strategy_bundle_digest;
use super::scoped_proof::ScopedMergeProofPacket;
use super::semantics::SelectedMergeSemanticsBundle;
use super::source_only_policy_registry::{SourceOnlyPolicyName, SourceOnlyPolicySelectionBasis};
use super::strategy_registry::{MergeStrategyName, MergeStrategySelectionBasis};
use super::{aspect_policy_inventory, SignalMergeStrategyWitness};

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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProofMinimalOverlapBasis {
    pub shared_nodes: Vec<NodeId>,
}

impl ProofMinimalOverlapBasis {
    pub fn breadth(&self) -> u64 {
        self.shared_nodes.len() as u64
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConservativeOverlapExpansion {
    pub expanded_nodes: Vec<NodeId>,
    pub support_nodes: Vec<NodeId>,
}

impl ConservativeOverlapExpansion {
    pub fn breadth(&self) -> u64 {
        self.expanded_nodes.len() as u64
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlannedMergeCandidateSet {
    pub nodes: Vec<NodeId>,
}

impl PlannedMergeCandidateSet {
    pub fn breadth(&self) -> u64 {
        self.nodes.len() as u64
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IdentityCorrespondenceBasis {
    ExactNodeId,
    OutputIdentityTargetJournal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IdentityCorrespondenceStatus {
    Matched,
    UnmatchedNoCandidate,
    UnmatchedRejectedAdmissibility,
    AmbiguousCandidates,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IdentityCorrespondenceRecord {
    pub source_node: NodeId,
    pub target_node: Option<NodeId>,
    pub basis: Option<IdentityCorrespondenceBasis>,
    pub status: IdentityCorrespondenceStatus,
    pub source_output_identity: Option<OutputIdentity>,
    pub target_output_identity: Option<OutputIdentity>,
    pub candidate_count: u32,
    #[serde(default)]
    pub candidate_target_nodes: Vec<NodeId>,
    pub admissibility_rejection: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct LoweredIdentityCorrespondencePlan {
    pub target_candidate_count: u64,
    pub source_lookup_count: u64,
    pub ambiguous_match_count: u64,
    pub rejected_admissibility_count: u64,
    pub records: Vec<IdentityCorrespondenceRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct LoweredDeletionPolicyPlan {
    pub target_only_nodes: Vec<NodeId>,
    pub target_only_count: u64,
    pub rejected_target_only_count: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LoweredConflictIsolationRecord {
    pub source_node: NodeId,
    pub target_node: Option<NodeId>,
    pub granularity: ConflictIsolationGranularity,
    pub isolated_aspects: Vec<Aspect>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConflictIsolationWitness {
    pub granularity: ConflictIsolationGranularity,
    pub conflict_record_count: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RegionIsolationSummary {
    pub isolated_region_count: u64,
    pub host_declared_region_count: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ConservativeIsolationExpansion {
    pub expanded_node_count: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct LoweredConflictIsolationPlan {
    pub selected_policy_name: Option<ConflictIsolationPolicyName>,
    pub selected_policy_digest: Option<String>,
    pub selected_policy_basis: Option<ConflictIsolationSelectionBasis>,
    pub expansion_breadth: u64,
    pub witness: Option<ConflictIsolationWitness>,
    pub region_summary: RegionIsolationSummary,
    pub conservative_expansion: ConservativeIsolationExpansion,
    pub records: Vec<LoweredConflictIsolationRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LoweredMergeBasePlan {
    pub resolved_base: BranchMergeBase,
    pub selected_merge_base_name: MergeBaseStrategyName,
    pub selected_merge_base_digest: String,
    pub selected_merge_base_basis: MergeBaseSelectionBasis,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LoweredAspectMergePolicyRecord {
    pub aspect: Aspect,
    pub selected_policy_name: AspectMergePolicyName,
    pub selected_policy_digest: String,
    pub selected_policy_basis: AspectMergePolicySelectionBasis,
    pub affected_source_nodes: Vec<NodeId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct LoweredAspectMergePolicyPlan {
    pub records: Vec<LoweredAspectMergePolicyRecord>,
}

impl LoweredAspectMergePolicyPlan {
    pub fn aspect_count(&self) -> u64 {
        self.records.len() as u64
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AspectMergeDecisionOutcome {
    SourceAuthorityAdopted,
    SourceIntroducedIntoTarget,
    EquivalentUnchanged,
    TargetPreserved,
    SourceSkippedNonAdoptable,
    ConflictRequired,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LoweredAspectMergeDecisionRecord {
    pub aspect: Aspect,
    pub source_node: NodeId,
    pub target_node: Option<NodeId>,
    pub selected_policy_name: AspectMergePolicyName,
    pub selected_policy_digest: String,
    pub selected_policy_basis: AspectMergePolicySelectionBasis,
    pub outcome: AspectMergeDecisionOutcome,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct LoweredAspectMergeDecisionPlan {
    pub records: Vec<LoweredAspectMergeDecisionRecord>,
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
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        source_branch_id: crate::state::SignalBranchId,
        target_branch_id: crate::state::SignalBranchId,
        schema_registry_digest: String,
        registry_bundle_digest: String,
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
    ) -> Self {
        let selected_semantics = SelectedMergeSemanticsBundle::new(
            selected_strategy_name.clone(),
            selected_strategy_digest.clone(),
            selected_strategy_basis,
            lowered_merge_base
                .as_ref()
                .map(|base| base.selected_merge_base_name.clone())
                .expect("merge-base plan"),
            lowered_merge_base
                .as_ref()
                .map(|base| base.selected_merge_base_digest.clone())
                .expect("merge-base plan"),
            lowered_merge_base
                .as_ref()
                .map(|base| base.selected_merge_base_basis)
                .expect("merge-base plan"),
            selected_conflict_policy_name.clone(),
            selected_conflict_policy_digest.clone(),
            selected_conflict_policy_basis,
            selected_conflict_isolation_name.clone(),
            selected_conflict_isolation_digest.clone(),
            selected_conflict_isolation_basis,
            selected_identity_matcher_name.clone(),
            selected_identity_matcher_digest.clone(),
            selected_identity_matcher_basis,
            selected_source_only_policy_name.clone(),
            selected_source_only_policy_digest.clone(),
            selected_source_only_policy_basis,
            selected_deletion_policy_name.clone(),
            selected_deletion_policy_digest.clone(),
            selected_deletion_policy_basis,
        );
        let lowered_strategy_bundle_digest = lowered_strategy_bundle_digest(
            &selected_semantics,
            lowered_merge_base.as_ref(),
            &deletion_plan,
            &conflict_isolation_plan,
            &aspect_policy_plan,
            &aspect_decision_plan,
        );
        let strategy_witness = SignalMergeStrategyWitness::from_admitted_plan_components(
            &selected_semantics,
            merge_strategy,
            &lowered_strategy_bundle_digest,
            &boundary_witness,
            aspect_policy_inventory(&aspect_policy_plan),
            &adoption_policy,
        );
        Self {
            source_branch_id,
            target_branch_id,
            schema_registry_digest,
            registry_bundle_digest,
            lowered_strategy_bundle_digest,
            merge_kind,
            divergence,
            merge_strategy,
            selected_strategy_name,
            selected_strategy_digest,
            selected_strategy_basis,
            selected_conflict_policy_name,
            selected_conflict_policy_digest,
            selected_conflict_policy_basis,
            selected_conflict_isolation_name,
            selected_conflict_isolation_digest,
            selected_conflict_isolation_basis,
            selected_identity_matcher_name: selected_identity_matcher_name.clone(),
            selected_identity_matcher_digest: selected_identity_matcher_digest.clone(),
            selected_identity_matcher_basis,
            selected_source_only_policy_name,
            selected_source_only_policy_digest,
            selected_source_only_policy_basis,
            selected_deletion_policy_name,
            selected_deletion_policy_digest,
            selected_deletion_policy_basis,
            selected_semantics,
            strategy_witness,
            reconciliation_policy,
            boundary_witness,
            source_journal,
            target_overlap_journal,
            identity_correspondence,
            deletion_plan,
            conflict_isolation_plan,
            aspect_policy_plan,
            aspect_decision_plan,
            scoped_candidates,
            scoped_merge_proof,
            proof_minimal_overlap,
            conservative_overlap,
            planned_candidates,
            source_snapshot_id,
            target_snapshot_id_before,
            merge_base,
            lowered_merge_base,
            resolution_plan,
            node_map,
            node_plan,
            adoption_core,
            adoption_policy,
        }
    }

    pub fn source_branch_id(&self) -> crate::state::SignalBranchId {
        self.source_branch_id
    }

    pub fn target_branch_id(&self) -> crate::state::SignalBranchId {
        self.target_branch_id
    }

    pub fn merge_kind(&self) -> BranchMergeKind {
        self.merge_kind
    }

    pub fn schema_registry_digest(&self) -> &str {
        &self.schema_registry_digest
    }

    pub fn registry_bundle_digest(&self) -> &str {
        &self.registry_bundle_digest
    }

    pub fn lowered_strategy_bundle_digest(&self) -> &str {
        &self.lowered_strategy_bundle_digest
    }

    pub fn divergence(&self) -> BranchMergeDivergence {
        self.divergence
    }

    pub fn merge_strategy(&self) -> BranchMergeStrategy {
        self.merge_strategy
    }

    pub fn selected_strategy_name(&self) -> &MergeStrategyName {
        &self.selected_strategy_name
    }

    pub fn selected_strategy_digest(&self) -> &str {
        &self.selected_strategy_digest
    }

    pub fn selected_strategy_basis(&self) -> MergeStrategySelectionBasis {
        self.selected_strategy_basis
    }

    pub fn selected_conflict_policy_name(&self) -> &ConflictPolicyName {
        &self.selected_conflict_policy_name
    }

    pub fn selected_conflict_policy_digest(&self) -> &str {
        &self.selected_conflict_policy_digest
    }

    pub fn selected_conflict_policy_basis(&self) -> ConflictPolicySelectionBasis {
        self.selected_conflict_policy_basis
    }

    pub fn selected_identity_matcher_name(&self) -> &IdentityMatcherName {
        &self.selected_identity_matcher_name
    }

    pub fn selected_conflict_isolation_name(&self) -> &ConflictIsolationPolicyName {
        &self.selected_conflict_isolation_name
    }

    pub fn selected_conflict_isolation_digest(&self) -> &str {
        &self.selected_conflict_isolation_digest
    }

    pub fn selected_conflict_isolation_basis(&self) -> ConflictIsolationSelectionBasis {
        self.selected_conflict_isolation_basis
    }

    pub fn selected_identity_matcher_digest(&self) -> &str {
        &self.selected_identity_matcher_digest
    }

    pub fn selected_identity_matcher_basis(&self) -> IdentityMatcherSelectionBasis {
        self.selected_identity_matcher_basis
    }

    pub fn selected_source_only_policy_name(&self) -> &SourceOnlyPolicyName {
        &self.selected_source_only_policy_name
    }

    pub fn selected_source_only_policy_digest(&self) -> &str {
        &self.selected_source_only_policy_digest
    }

    pub fn selected_source_only_policy_basis(&self) -> SourceOnlyPolicySelectionBasis {
        self.selected_source_only_policy_basis
    }

    pub fn selected_deletion_policy_name(&self) -> &DeletionPolicyName {
        &self.selected_deletion_policy_name
    }

    pub fn selected_deletion_policy_digest(&self) -> &str {
        &self.selected_deletion_policy_digest
    }

    pub fn selected_deletion_policy_basis(&self) -> DeletionPolicySelectionBasis {
        self.selected_deletion_policy_basis
    }

    pub fn selected_semantics(&self) -> &SelectedMergeSemanticsBundle {
        &self.selected_semantics
    }

    pub fn strategy_witness(&self) -> &SignalMergeStrategyWitness {
        &self.strategy_witness
    }

    pub fn reconciliation_policy(&self) -> &BranchMergeReconciliationPolicy {
        &self.reconciliation_policy
    }

    pub fn boundary_witness(&self) -> &MergeBoundaryWitness {
        &self.boundary_witness
    }

    pub fn source_journal(&self) -> &StructuralMergeJournalSlice {
        &self.source_journal
    }

    pub fn target_overlap_journal(&self) -> &BranchMutationJournalSlice {
        &self.target_overlap_journal
    }

    pub fn identity_correspondence(&self) -> &LoweredIdentityCorrespondencePlan {
        &self.identity_correspondence
    }

    pub fn deletion_plan(&self) -> &LoweredDeletionPolicyPlan {
        &self.deletion_plan
    }

    pub fn conflict_isolation_plan(&self) -> &LoweredConflictIsolationPlan {
        &self.conflict_isolation_plan
    }

    pub fn aspect_policy_plan(&self) -> &LoweredAspectMergePolicyPlan {
        &self.aspect_policy_plan
    }

    pub fn aspect_decision_plan(&self) -> &LoweredAspectMergeDecisionPlan {
        &self.aspect_decision_plan
    }

    pub fn scoped_candidates(&self) -> &LoweredScopedMergeCandidateSet {
        &self.scoped_candidates
    }

    pub fn scoped_merge_proof(&self) -> &ScopedMergeProofPacket {
        &self.scoped_merge_proof
    }

    pub fn proof_minimal_overlap(&self) -> &ProofMinimalOverlapBasis {
        &self.proof_minimal_overlap
    }

    pub fn conservative_overlap(&self) -> &ConservativeOverlapExpansion {
        &self.conservative_overlap
    }

    pub fn planned_candidates(&self) -> &PlannedMergeCandidateSet {
        &self.planned_candidates
    }

    pub fn source_snapshot_id(&self) -> Option<crate::state::SignalSnapshotId> {
        self.source_snapshot_id
    }

    pub fn target_snapshot_id_before(&self) -> Option<crate::state::SignalSnapshotId> {
        self.target_snapshot_id_before
    }

    pub fn merge_base(&self) -> Option<&BranchMergeBase> {
        self.merge_base.as_ref()
    }

    pub fn lowered_merge_base(&self) -> Option<&LoweredMergeBasePlan> {
        self.lowered_merge_base.as_ref()
    }

    pub fn resolution_plan(&self) -> Option<&BranchConflictResolutionPlan> {
        self.resolution_plan.as_ref()
    }

    pub fn node_map(&self) -> &MergeNodeMap {
        &self.node_map
    }

    pub fn node_plan(&self) -> &[NodeMergePlan] {
        &self.node_plan
    }

    pub fn adoption_core(&self) -> &[SourceNodeAdoptionPlanCore] {
        &self.adoption_core
    }

    pub fn adoption_policy(&self) -> &[SourceNodeAdoptionCarryPolicy] {
        &self.adoption_policy
    }
}

pub type BranchMergePlan = LoweredMergePlan;
