use serde::{Deserialize, Serialize};

use crate::data::core_profile::StableHashValue;
use crate::data::handle::NodeId;
use crate::data::output::{ArtifactContinuityToken, OutputIdentity};
use crate::data::reuse::ReuseBasis;
use crate::data::trace::ArtifactMergeAuthority;
use crate::diagnostics::lineage::LineageArtifactId;

use super::adoption::{SourceNodeAdoptionCarryPolicy, SourceNodeAdoptionPlanCore};
use super::conflict::{BranchConflictResolutionPlan, BranchMergeConflictKind};
use super::core::{
    BranchMergeBase, BranchMergeDivergence, BranchMergeKind, BranchMergeStrategy,
    MergeBoundaryWitness,
};
use super::journal::{BranchMutationJournalSlice, MergeNodeMap, StructuralMergeJournalSlice};
use super::policy::BranchMergeReconciliationPolicy;

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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoweredMergePlan {
    source_branch_id: crate::state::SignalBranchId,
    target_branch_id: crate::state::SignalBranchId,
    merge_kind: BranchMergeKind,
    divergence: BranchMergeDivergence,
    merge_strategy: BranchMergeStrategy,
    reconciliation_policy: BranchMergeReconciliationPolicy,
    boundary_witness: MergeBoundaryWitness,
    source_journal: StructuralMergeJournalSlice,
    target_overlap_journal: BranchMutationJournalSlice,
    proof_minimal_overlap: ProofMinimalOverlapBasis,
    conservative_overlap: ConservativeOverlapExpansion,
    planned_candidates: PlannedMergeCandidateSet,
    source_snapshot_id: Option<crate::state::SignalSnapshotId>,
    target_snapshot_id_before: Option<crate::state::SignalSnapshotId>,
    merge_base: Option<BranchMergeBase>,
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
        merge_kind: BranchMergeKind,
        divergence: BranchMergeDivergence,
        merge_strategy: BranchMergeStrategy,
        reconciliation_policy: BranchMergeReconciliationPolicy,
        boundary_witness: MergeBoundaryWitness,
        source_journal: StructuralMergeJournalSlice,
        target_overlap_journal: BranchMutationJournalSlice,
        proof_minimal_overlap: ProofMinimalOverlapBasis,
        conservative_overlap: ConservativeOverlapExpansion,
        planned_candidates: PlannedMergeCandidateSet,
        source_snapshot_id: Option<crate::state::SignalSnapshotId>,
        target_snapshot_id_before: Option<crate::state::SignalSnapshotId>,
        merge_base: Option<BranchMergeBase>,
        resolution_plan: Option<BranchConflictResolutionPlan>,
        node_map: MergeNodeMap,
        node_plan: Vec<NodeMergePlan>,
        adoption_core: Vec<SourceNodeAdoptionPlanCore>,
        adoption_policy: Vec<SourceNodeAdoptionCarryPolicy>,
    ) -> Self {
        Self {
            source_branch_id,
            target_branch_id,
            merge_kind,
            divergence,
            merge_strategy,
            reconciliation_policy,
            boundary_witness,
            source_journal,
            target_overlap_journal,
            proof_minimal_overlap,
            conservative_overlap,
            planned_candidates,
            source_snapshot_id,
            target_snapshot_id_before,
            merge_base,
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

    pub fn divergence(&self) -> BranchMergeDivergence {
        self.divergence
    }

    pub fn merge_strategy(&self) -> BranchMergeStrategy {
        self.merge_strategy
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
