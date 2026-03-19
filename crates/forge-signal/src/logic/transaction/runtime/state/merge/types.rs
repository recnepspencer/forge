use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::data::core_profile::StableHashValue;
use crate::data::dependency::{DependencyEdge, DependencySnapshot};
use crate::data::graph::BranchMutationRecord;
use crate::data::handle::NodeId;
use crate::data::node::NodeEvaluationConfig;
use crate::data::output::{ArtifactContinuityToken, OutputIdentity};
use crate::data::reuse::ReuseBasis;
use crate::data::trace::ArtifactMergeAuthority;
use crate::diagnostics::lineage::LineageArtifactId;
use crate::state::{SignalBranchHandle, SignalBranchId, SignalSnapshotId};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BranchMergeRequest {
    pub source_branch: SignalBranchHandle,
    pub target_branch: SignalBranchHandle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BranchMergeKind {
    FastForward,
    Applied,
    ConflictResolved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BranchMergeStrategy {
    AdoptSourceHead,
    AdoptSourceSubset,
    ReplaySourceDeltaOntoTarget,
    RebaseSourceOntoTarget,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BranchMergeDivergence {
    None,
    TargetAdvanced,
    SharedStateConflict,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BranchMergeFailureKind {
    SelfMergeRejected,
    UnknownSourceBranch,
    UnknownTargetBranch,
    MissingMergeBase,
    DivergenceRequiresConflictResolution,
    UnsupportedMergeStrategy,
    UnresolvedDependencyRemap,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExistingTargetMergePolicy {
    PreserveEquivalentOtherwiseAdoptSource,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SourceOnlyMergePolicy {
    IntroduceAdoptableSkipNonAdoptable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConflictMergePolicy {
    RejectSharedStateConflict,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct BranchMergeReconciliationPolicy {
    pub existing_target: ExistingTargetMergePolicy,
    pub source_only: SourceOnlyMergePolicy,
    pub conflict: ConflictMergePolicy,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BranchMergeConflictRecord {
    pub source_node: NodeId,
    pub target_node: NodeId,
    pub conflict_kinds: Vec<BranchMergeConflictKind>,
    pub source_comparable: Option<ArtifactMergeComparable>,
    pub target_comparable: Option<ArtifactMergeComparable>,
    pub source_structural_record: Option<StructuralMergeCandidateRecord>,
    pub target_structural_record: Option<StructuralMergeCandidateRecord>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BranchMergeConflictKind {
    ComparableMismatch,
    DependencyTopologyMismatch,
    DependencySnapshotMismatch,
    RuntimeArtifactMismatch,
    MergeAuthorityMismatch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BranchMergeResolutionRequirement {
    ReconcileComparableState,
    ReconcileDependencyTopology,
    ReconcileDependencySnapshot,
    ReconcileRuntimeArtifactState,
    ReconcileMergeAuthority,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct BranchMergeConflictSummary {
    pub total_conflict_count: u64,
    pub comparable_mismatch_count: u64,
    pub dependency_topology_mismatch_count: u64,
    pub dependency_snapshot_mismatch_count: u64,
    pub runtime_artifact_mismatch_count: u64,
    pub merge_authority_mismatch_count: u64,
    pub primary_conflict_kind: Option<BranchMergeConflictKind>,
    pub required_resolution: Vec<BranchMergeResolutionRequirement>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BranchMergeConflictEvidence {
    pub divergence: BranchMergeDivergence,
    pub reconciliation_policy: BranchMergeReconciliationPolicy,
    pub summary: BranchMergeConflictSummary,
    pub records: Vec<BranchMergeConflictRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StructuralMergeCandidateRecord {
    pub node: NodeId,
    pub introduced: bool,
    pub state_changed: bool,
    pub dependencies_changed: bool,
    pub dependency_snapshot_changed: bool,
    pub runtime_artifact_changed: bool,
    pub retained_artifact_changed: bool,
    pub causality_changed: bool,
    pub structural_deltas: Vec<crate::data::graph::BranchStructuralDelta>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct BranchMutationJournalSlice {
    pub records: Vec<StructuralMergeCandidateRecord>,
}

impl BranchMutationJournalSlice {
    pub fn candidate_nodes(&self) -> Vec<NodeId> {
        self.records.iter().map(|record| record.node).collect()
    }

    pub fn contains_node(&self, node: NodeId) -> bool {
        self.records.iter().any(|record| record.node == node)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MergeCandidateScope {
    WholeLiveAuthoritySurface,
    CandidateNodeSet(Vec<NodeId>),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BranchMergeBase {
    pub source_branch_id: SignalBranchId,
    pub target_branch_id: SignalBranchId,
    pub forked_from_snapshot_id: Option<SignalSnapshotId>,
    pub source_snapshot_id: Option<SignalSnapshotId>,
    pub target_snapshot_id_before: Option<SignalSnapshotId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct BranchMutationLedger {
    pub(crate) pending: BTreeMap<NodeId, BranchMutationRecord>,
    pub(crate) baseline_snapshot_id: Option<SignalSnapshotId>,
    pub(crate) boundary_established: bool,
}

impl BranchMutationLedger {
    pub fn with_baseline_snapshot(mut self, snapshot_id: Option<SignalSnapshotId>) -> Self {
        self.baseline_snapshot_id = snapshot_id;
        self.boundary_established = true;
        self
    }

    pub(crate) fn absorb_records(
        &mut self,
        records: impl IntoIterator<Item = (NodeId, BranchMutationRecord)>,
    ) {
        for (node, record) in records {
            let entry = self.pending.entry(node).or_default();
            entry.introduced |= record.introduced;
            entry.state_changed |= record.state_changed;
            entry.dependencies_changed |= record.dependencies_changed;
            entry.dependency_snapshot_changed |= record.dependency_snapshot_changed;
            entry.runtime_artifact_changed |= record.runtime_artifact_changed;
            entry.retained_artifact_changed |= record.retained_artifact_changed;
            entry.causality_changed |= record.causality_changed;
            entry
                .structural_deltas
                .extend(record.structural_deltas.into_iter());
        }
    }

    pub fn structural_merge_journal(&self) -> BranchMutationJournalSlice {
        BranchMutationJournalSlice {
            records: self
                .pending
                .iter()
                .filter(|(_, record)| record.merge_relevant())
                .map(|(node, record)| StructuralMergeCandidateRecord {
                    node: *node,
                    introduced: record.introduced,
                    state_changed: record.state_changed,
                    dependencies_changed: record.dependencies_changed,
                    dependency_snapshot_changed: record.dependency_snapshot_changed,
                    runtime_artifact_changed: record.runtime_artifact_changed,
                    retained_artifact_changed: record.retained_artifact_changed,
                    causality_changed: record.causality_changed,
                    structural_deltas: record.structural_deltas.clone(),
                })
                .collect(),
        }
    }

    pub fn clear_all(&mut self, baseline_snapshot_id: Option<SignalSnapshotId>) {
        self.pending.clear();
        self.baseline_snapshot_id = baseline_snapshot_id;
        self.boundary_established = true;
    }

    pub fn clear_merged_nodes(
        &mut self,
        merged_nodes: impl IntoIterator<Item = NodeId>,
        baseline_snapshot_id: Option<SignalSnapshotId>,
    ) {
        for node in merged_nodes {
            self.pending.remove(&node);
        }
        self.baseline_snapshot_id = baseline_snapshot_id;
        self.boundary_established = true;
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct MergeNodeMap {
    pub source_to_target: BTreeMap<NodeId, NodeId>,
}

impl MergeNodeMap {
    pub fn insert(&mut self, source: NodeId, target: NodeId) {
        self.source_to_target.insert(source, target);
    }

    pub fn resolve(&self, source: NodeId) -> Option<NodeId> {
        self.source_to_target.get(&source).copied()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TargetNodeIdentityIntent {
    ExistingMapping { mapped_target_node: NodeId },
    AllocateTargetNode,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdoptedNodeContract {
    pub eval_config: NodeEvaluationConfig,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdoptionDependencyTopology {
    pub dependencies: Vec<DependencyEdge>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdoptionDependencySnapshotRef {
    pub snapshot: DependencySnapshot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RuntimeArtifactCarryPolicy {
    CarryMergeAdoptable,
    RebuildAfterAdoption,
    DoNotCarry,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RetainedArtifactCarryPolicy {
    CarryIfPolicyAllows,
    ReconstructIfNeeded,
    Drop,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CausalityCarryPolicy {
    CarryIfPolicyAllows,
    Drop,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SourceNodeAdoptionPlanCore {
    pub source_node: NodeId,
    pub target_identity: TargetNodeIdentityIntent,
    pub authority: ArtifactMergeAuthority,
    pub entry_contract: AdoptedNodeContract,
    pub dependency_topology: AdoptionDependencyTopology,
    pub dependency_snapshot_ref: AdoptionDependencySnapshotRef,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceNodeAdoptionCarryPolicy {
    pub runtime_artifact: RuntimeArtifactCarryPolicy,
    pub retained_artifact: RetainedArtifactCarryPolicy,
    pub causality: CausalityCarryPolicy,
}

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
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BranchMergePlan {
    pub source_branch_id: SignalBranchId,
    pub target_branch_id: SignalBranchId,
    pub merge_kind: BranchMergeKind,
    pub divergence: BranchMergeDivergence,
    pub merge_strategy: BranchMergeStrategy,
    pub reconciliation_policy: BranchMergeReconciliationPolicy,
    pub candidate_scope: MergeCandidateScope,
    pub source_journal: BranchMutationJournalSlice,
    pub target_overlap_journal: BranchMutationJournalSlice,
    pub source_snapshot_id: Option<SignalSnapshotId>,
    pub target_snapshot_id_before: Option<SignalSnapshotId>,
    pub merge_base: Option<BranchMergeBase>,
    pub node_map: MergeNodeMap,
    pub node_plan: Vec<NodeMergePlan>,
    pub adoption_core: Vec<SourceNodeAdoptionPlanCore>,
    pub adoption_policy: Vec<SourceNodeAdoptionCarryPolicy>,
}

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
    pub branch_wide_scan_performed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BranchMergeResult {
    pub source_branch: SignalBranchId,
    pub target_branch: SignalBranchId,
    pub merge_kind: BranchMergeKind,
    pub divergence: BranchMergeDivergence,
    pub merge_strategy: BranchMergeStrategy,
    pub reconciliation_policy: BranchMergeReconciliationPolicy,
    pub candidate_scope: MergeCandidateScope,
    pub merged_snapshot_id: Option<SignalSnapshotId>,
    pub target_snapshot_id_before: Option<SignalSnapshotId>,
    pub target_snapshot_id_after: Option<SignalSnapshotId>,
    pub source_snapshot_id: Option<SignalSnapshotId>,
    pub records: Vec<MergedArtifactRecord>,
    pub counters: BranchMergeCounters,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BranchMergeExecutionSummary {
    pub source_branch_id: SignalBranchId,
    pub target_branch_id: SignalBranchId,
    pub merge_kind: BranchMergeKind,
    pub divergence: BranchMergeDivergence,
    pub merge_strategy: BranchMergeStrategy,
    pub reconciliation_policy: BranchMergeReconciliationPolicy,
    pub candidate_scope: MergeCandidateScope,
    pub merge_base: Option<BranchMergeBase>,
    pub source_snapshot_id: Option<SignalSnapshotId>,
    pub target_snapshot_id_before: Option<SignalSnapshotId>,
    pub target_snapshot_id_after: Option<SignalSnapshotId>,
    pub node_map: MergeNodeMap,
    pub records: Vec<MergedArtifactRecord>,
    pub dependency_remaps: Vec<DependencyRemapRecord>,
    pub topology_repair: TopologyRepairSummary,
    pub counters: BranchMergeCounters,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdoptedNodeMaterialization {
    pub target_node: NodeId,
    pub dependency_count: usize,
}
