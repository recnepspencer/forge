use serde::{Deserialize, Serialize};

use crate::data::handle::NodeId;
use crate::diagnostics::lineage::LineageArtifactId;

use super::conflict::{BranchConflictResolutionPlan, BranchMergeConflictKind};
use super::core::{BranchMergeBase, BranchMergeDivergence, BranchMergeKind, BranchMergeStrategy, MergeBoundaryWitnessKind, MergeBoundaryWitness};
use super::journal::MergeNodeMap;
use super::plan::{
    ArtifactMergeComparable, ConservativeOverlapExpansion, PlannedMergeCandidateSet,
    ProofMinimalOverlapBasis,
};
use super::policy::BranchMergeReconciliationPolicy;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArtifactMergeAction {
    Adopted,
    Replaced,
    PreservedTarget,
    EquivalentUnchanged,
    SkippedNonAdoptable,
    IntroducedIntoTarget,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MergeDecisionBasis {
    FastForwardSourceHead,
    EquivalentArtifacts,
    SourceAuthorityAdopted,
    SourceNodeIntroducedIntoTarget,
    TargetPreservedNonAdoptable,
    MissingSourceArtifact,
    MissingTargetArtifact,
    StrategyExcluded,
    ConflictUnresolved,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergedArtifactRecord {
    pub source_node: NodeId,
    pub target_node: Option<NodeId>,
    pub source_artifact_id: Option<LineageArtifactId>,
    pub target_artifact_id_before: Option<LineageArtifactId>,
    pub target_artifact_id_after: Option<LineageArtifactId>,
    pub action: ArtifactMergeAction,
    pub basis: MergeDecisionBasis,
    pub source_comparable: Option<ArtifactMergeComparable>,
    pub target_comparable: Option<ArtifactMergeComparable>,
    pub resolved_conflict_kinds: Vec<BranchMergeConflictKind>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DependencyRemapRecord {
    pub source_node: NodeId,
    pub source_dependency: NodeId,
    pub target_dependency: NodeId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct MergeTouchedNodeSet {
    pub nodes: Vec<NodeId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct TopologyRepairSummary {
    pub touched_node_count: u64,
    pub subscriber_repair_breadth: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct BranchMergeCounters {
    pub boundary_witness_kind: MergeBoundaryWitnessKind,
    pub source_slice_breadth: u64,
    pub proof_minimal_overlap_breadth: u64,
    pub conservative_overlap_expansion_breadth: u64,
    pub final_candidate_breadth: u64,
    pub reconciliation_breadth: u64,
    pub candidate_node_count: u64,
    pub examined_node_count: u64,
    pub adopted_count: u64,
    pub introduced_node_count: u64,
    pub replaced_count: u64,
    pub preserved_target_count: u64,
    pub skipped_non_adoptable_count: u64,
    pub equivalent_unchanged_count: u64,
    pub source_only_count: u64,
    pub target_only_count: u64,
    pub dependency_remap_count: u64,
    pub subscriber_repair_breadth: u64,
    pub merge_lineage_record_count: u64,
    pub replay_event_count: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BranchMergeResult {
    pub source_branch: crate::state::SignalBranchId,
    pub target_branch: crate::state::SignalBranchId,
    pub merge_kind: BranchMergeKind,
    pub divergence: BranchMergeDivergence,
    pub merge_strategy: BranchMergeStrategy,
    pub reconciliation_policy: BranchMergeReconciliationPolicy,
    pub boundary_witness: MergeBoundaryWitness,
    pub proof_minimal_overlap: ProofMinimalOverlapBasis,
    pub conservative_overlap: ConservativeOverlapExpansion,
    pub planned_candidates: PlannedMergeCandidateSet,
    pub merged_snapshot_id: Option<crate::state::SignalSnapshotId>,
    pub target_snapshot_id_before: Option<crate::state::SignalSnapshotId>,
    pub target_snapshot_id_after: Option<crate::state::SignalSnapshotId>,
    pub source_snapshot_id: Option<crate::state::SignalSnapshotId>,
    pub resolution_plan: Option<BranchConflictResolutionPlan>,
    pub records: Vec<MergedArtifactRecord>,
    pub counters: BranchMergeCounters,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BranchMergeExecutionSummary {
    pub source_branch_id: crate::state::SignalBranchId,
    pub target_branch_id: crate::state::SignalBranchId,
    pub merge_kind: BranchMergeKind,
    pub divergence: BranchMergeDivergence,
    pub merge_strategy: BranchMergeStrategy,
    pub reconciliation_policy: BranchMergeReconciliationPolicy,
    pub boundary_witness: MergeBoundaryWitness,
    pub proof_minimal_overlap: ProofMinimalOverlapBasis,
    pub conservative_overlap: ConservativeOverlapExpansion,
    pub planned_candidates: PlannedMergeCandidateSet,
    pub merge_base: Option<BranchMergeBase>,
    pub source_snapshot_id: Option<crate::state::SignalSnapshotId>,
    pub target_snapshot_id_before: Option<crate::state::SignalSnapshotId>,
    pub target_snapshot_id_after: Option<crate::state::SignalSnapshotId>,
    pub resolution_plan: Option<BranchConflictResolutionPlan>,
    pub node_map: MergeNodeMap,
    pub records: Vec<MergedArtifactRecord>,
    pub dependency_remaps: Vec<DependencyRemapRecord>,
    pub topology_repair: TopologyRepairSummary,
    pub counters: BranchMergeCounters,
}
