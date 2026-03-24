use serde::{Deserialize, Serialize};

use crate::data::handle::NodeId;

use super::core::BranchMergeDivergence;
use super::journal::StructuralMergeCandidateRecord;
use super::plan::ArtifactMergeComparable;
use super::policy::BranchMergeReconciliationPolicy;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum BranchMergeResolutionRequirement {
    ReconcileComparableState,
    ReconcileDependencyTopology,
    ReconcileDependencySnapshot,
    ReconcileRuntimeArtifactState,
    ReconcileMergeAuthority,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ConflictResolutionStrategy {
    AdoptSourceComparableState,
    PreserveTargetComparableState,
    ReplaySourceDependencyTopology,
    PreserveTargetDependencyTopology,
    ReplaySourceDependencySnapshot,
    PreserveTargetDependencySnapshot,
    AdoptSourceRuntimeArtifactState,
    PreserveTargetRuntimeArtifactState,
    AdoptSourceMergeAuthority,
    PreserveTargetMergeAuthority,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConflictResolutionRecord {
    pub source_node: NodeId,
    pub target_node: NodeId,
    pub required_resolution: Vec<BranchMergeResolutionRequirement>,
    pub supported_strategies: Vec<ConflictResolutionStrategy>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BranchConflictResolutionPlan {
    pub source_branch_id: crate::state::SignalBranchId,
    pub target_branch_id: crate::state::SignalBranchId,
    pub divergence: BranchMergeDivergence,
    pub records: Vec<ConflictResolutionRecord>,
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
    pub resolution_plan: BranchConflictResolutionPlan,
    pub records: Vec<BranchMergeConflictRecord>,
}
