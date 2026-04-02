use serde::{Deserialize, Serialize};

use crate::state::{SignalBranchHandle, SignalBranchId, SignalSnapshotId};

use super::aspect_policy_registry::AspectMergePolicyBinding;
use super::conflict_isolation_registry::ConflictIsolationPolicyName;
use super::conflict_policy_registry::ConflictPolicyName;
use super::deletion_policy_registry::DeletionPolicyName;
use super::identity_matcher_registry::IdentityMatcherName;
use super::merge_base_registry::MergeBaseStrategyName;
use super::source_only_policy_registry::SourceOnlyPolicyName;
use super::strategy_registry::MergeStrategyName;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BranchMergeRequest {
    pub source_branch: SignalBranchHandle,
    pub target_branch: SignalBranchHandle,
    #[serde(default)]
    pub strategy_name: Option<MergeStrategyName>,
    #[serde(default)]
    pub strategy_hint: Option<BranchMergeStrategy>,
    #[serde(default)]
    pub merge_base_name: Option<MergeBaseStrategyName>,
    #[serde(default)]
    pub conflict_policy_name: Option<ConflictPolicyName>,
    #[serde(default)]
    pub identity_matcher_name: Option<IdentityMatcherName>,
    #[serde(default)]
    pub source_only_policy_name: Option<SourceOnlyPolicyName>,
    #[serde(default)]
    pub deletion_policy_name: Option<DeletionPolicyName>,
    #[serde(default)]
    pub conflict_isolation_policy_name: Option<ConflictIsolationPolicyName>,
    #[serde(default)]
    pub aspect_policy_bindings: Vec<AspectMergePolicyBinding>,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum MergeBoundaryWitnessKind {
    #[default]
    MutationJournalBoundary,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergeBoundaryWitness {
    pub source_branch_id: SignalBranchId,
    pub target_branch_id: SignalBranchId,
    pub kind: MergeBoundaryWitnessKind,
    pub forked_from_snapshot_id: Option<SignalSnapshotId>,
    pub source_snapshot_id: Option<SignalSnapshotId>,
    pub target_snapshot_id_before: Option<SignalSnapshotId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BranchMergeBase {
    pub source_branch_id: SignalBranchId,
    pub target_branch_id: SignalBranchId,
    pub forked_from_snapshot_id: Option<SignalSnapshotId>,
    pub source_snapshot_id: Option<SignalSnapshotId>,
    pub target_snapshot_id_before: Option<SignalSnapshotId>,
}
