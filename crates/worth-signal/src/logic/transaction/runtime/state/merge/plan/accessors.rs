use super::super::adoption::{SourceNodeAdoptionCarryPolicy, SourceNodeAdoptionPlanCore};
use super::super::conflict::BranchConflictResolutionPlan;
use super::super::conflict_isolation_registry::{
    ConflictIsolationPolicyName, ConflictIsolationSelectionBasis,
};
use super::super::conflict_policy_registry::{ConflictPolicyName, ConflictPolicySelectionBasis};
use super::super::core::{
    BranchMergeBase, BranchMergeDivergence, BranchMergeKind, BranchMergeStrategy,
    MergeBoundaryWitness,
};
use super::super::deletion_policy_registry::{DeletionPolicyName, DeletionPolicySelectionBasis};
use super::super::identity_matcher_registry::{IdentityMatcherName, IdentityMatcherSelectionBasis};
use super::super::journal::{
    BranchMutationJournalSlice, MergeNodeMap, StructuralMergeJournalSlice,
};
use super::super::policy::BranchMergeReconciliationPolicy;
use super::super::semantics::SelectedMergeSemanticsBundle;
use super::super::source_only_policy_registry::{
    SourceOnlyPolicyName, SourceOnlyPolicySelectionBasis,
};
use super::super::strategy_registry::{MergeStrategyName, MergeStrategySelectionBasis};

use super::candidates::{
    ConservativeOverlapExpansion, LoweredAspectMergeDecisionPlan, LoweredAspectMergePolicyPlan,
    LoweredConflictIsolationPlan, LoweredDeletionPolicyPlan, LoweredIdentityCorrespondencePlan,
    LoweredMergeBasePlan, PlannedMergeCandidateSet, ProofMinimalOverlapBasis,
};
use super::{LoweredMergePlan, NodeMergePlan};

impl LoweredMergePlan {
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
