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
    pub current_artifact_id: Option<LineageArtifactId>,
    pub comparable: Option<ArtifactMergeComparable>,
    pub authority: Option<ArtifactMergeAuthority>,
    pub exists_in_branch: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NodeMergePlan {
    pub source_node: NodeId,
    pub shape: NodeReconciliationShape,
    pub source_state: NodeMergeInputState,
    pub target_state: NodeMergeInputState,
    pub decision: NodeReconciliationDecision,
    pub resolved_conflict_kinds: Vec<BranchMergeConflictKind>,
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
    pub source_branch_id: crate::state::SignalBranchId,
    pub target_branch_id: crate::state::SignalBranchId,
    pub merge_kind: BranchMergeKind,
    pub divergence: BranchMergeDivergence,
    pub merge_strategy: BranchMergeStrategy,
    pub reconciliation_policy: BranchMergeReconciliationPolicy,
    pub boundary_witness: MergeBoundaryWitness,
    pub source_journal: StructuralMergeJournalSlice,
    pub target_overlap_journal: BranchMutationJournalSlice,
    pub proof_minimal_overlap: ProofMinimalOverlapBasis,
    pub conservative_overlap: ConservativeOverlapExpansion,
    pub planned_candidates: PlannedMergeCandidateSet,
    pub source_snapshot_id: Option<crate::state::SignalSnapshotId>,
    pub target_snapshot_id_before: Option<crate::state::SignalSnapshotId>,
    pub merge_base: Option<BranchMergeBase>,
    pub resolution_plan: Option<BranchConflictResolutionPlan>,
    pub node_map: MergeNodeMap,
    pub node_plan: Vec<NodeMergePlan>,
    pub adoption_core: Vec<SourceNodeAdoptionPlanCore>,
    pub adoption_policy: Vec<SourceNodeAdoptionCarryPolicy>,
}

pub type BranchMergePlan = LoweredMergePlan;
