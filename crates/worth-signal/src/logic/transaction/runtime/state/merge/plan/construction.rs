use super::super::adoption::{SourceNodeAdoptionCarryPolicy, SourceNodeAdoptionPlanCore};
use super::super::candidate_scope::LoweredScopedMergeCandidateSet;
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
use super::super::proof::lowered_strategy_bundle_digest;
use super::super::scoped_proof::ScopedMergeProofPacket;
use super::super::semantics::SelectedMergeSemanticsBundle;
use super::super::source_only_policy_registry::{
    SourceOnlyPolicyName, SourceOnlyPolicySelectionBasis,
};
use super::super::strategy_registry::{MergeStrategyName, MergeStrategySelectionBasis};
use super::super::{aspect_policy_inventory, SignalMergeStrategyWitness};

use super::candidates::{
    ConservativeOverlapExpansion, LoweredAspectMergeDecisionPlan, LoweredAspectMergePolicyPlan,
    LoweredConflictIsolationPlan, LoweredDeletionPolicyPlan, LoweredIdentityCorrespondencePlan,
    LoweredMergeBasePlan, PlannedMergeCandidateSet, ProofMinimalOverlapBasis,
};
use super::{LoweredMergePlan, NodeMergePlan};

impl LoweredMergePlan {
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
}
