use super::super::adoption::{SourceNodeAdoptionCarryPolicy, SourceNodeAdoptionPlanCore};
use super::super::candidate_scope::LoweredScopedMergeCandidateSet;
use super::super::conflict::BranchConflictResolutionPlan;
use super::super::core::{
    BranchMergeBase, BranchMergeDivergence, BranchMergeKind, BranchMergeStrategy,
    MergeBoundaryWitness,
};
use super::super::journal::{
    BranchMutationJournalSlice, MergeNodeMap, StructuralMergeJournalSlice,
};
use super::super::policy::BranchMergeReconciliationPolicy;
use super::super::proof::lowered_strategy_bundle_digest;
use super::super::scoped_proof::ScopedMergeProofPacket;
use super::super::semantics::SelectedMergeSemanticsBundle;
use super::super::{aspect_policy_inventory, SignalMergeStrategyWitness};

use super::candidates::{
    ConservativeOverlapExpansion, LoweredAspectMergeDecisionPlan, LoweredAspectMergePolicyPlan,
    LoweredConflictIsolationPlan, LoweredDeletionPolicyPlan, LoweredIdentityCorrespondencePlan,
    LoweredMergeBasePlan, PlannedMergeCandidateSet, ProofMinimalOverlapBasis,
};
use super::{LoweredMergePlan, NodeMergePlan};

pub(crate) struct LoweredMergePlanConstruction {
    pub(crate) world: LoweredMergePlanWorld,
    pub(crate) selected_semantics: SelectedMergeSemanticsBundle,
    pub(crate) journals: LoweredMergePlanJournals,
    pub(crate) decisions: LoweredMergePlanDecisions,
    pub(crate) scope: LoweredMergePlanScope,
    pub(crate) lineage: LoweredMergePlanLineage,
    pub(crate) nodes: LoweredMergePlanNodes,
}

pub(crate) struct LoweredMergePlanWorld {
    pub(crate) source_branch_id: crate::state::SignalBranchId,
    pub(crate) target_branch_id: crate::state::SignalBranchId,
    pub(crate) schema_registry_digest: String,
    pub(crate) registry_bundle_digest: String,
    pub(crate) merge_kind: BranchMergeKind,
    pub(crate) divergence: BranchMergeDivergence,
    pub(crate) merge_strategy: BranchMergeStrategy,
    pub(crate) reconciliation_policy: BranchMergeReconciliationPolicy,
    pub(crate) boundary_witness: MergeBoundaryWitness,
}

pub(crate) struct LoweredMergePlanJournals {
    pub(crate) source: StructuralMergeJournalSlice,
    pub(crate) target_overlap: BranchMutationJournalSlice,
}

pub(crate) struct LoweredMergePlanDecisions {
    pub(crate) identity_correspondence: LoweredIdentityCorrespondencePlan,
    pub(crate) deletion: LoweredDeletionPolicyPlan,
    pub(crate) conflict_isolation: LoweredConflictIsolationPlan,
    pub(crate) aspect_policy: LoweredAspectMergePolicyPlan,
    pub(crate) aspect: LoweredAspectMergeDecisionPlan,
}

pub(crate) struct LoweredMergePlanScope {
    pub(crate) candidates: LoweredScopedMergeCandidateSet,
    pub(crate) proof: ScopedMergeProofPacket,
    pub(crate) proof_minimal_overlap: ProofMinimalOverlapBasis,
    pub(crate) conservative_overlap: ConservativeOverlapExpansion,
    pub(crate) planned_candidates: PlannedMergeCandidateSet,
}

pub(crate) struct LoweredMergePlanLineage {
    pub(crate) source_snapshot_id: Option<crate::state::SignalSnapshotId>,
    pub(crate) target_snapshot_id_before: Option<crate::state::SignalSnapshotId>,
    pub(crate) merge_base: Option<BranchMergeBase>,
    pub(crate) lowered_merge_base: Option<LoweredMergeBasePlan>,
    pub(crate) resolution: Option<BranchConflictResolutionPlan>,
}

pub(crate) struct LoweredMergePlanNodes {
    pub(crate) map: MergeNodeMap,
    pub(crate) decisions: Vec<NodeMergePlan>,
    pub(crate) adoption_core: Vec<SourceNodeAdoptionPlanCore>,
    pub(crate) adoption_policy: Vec<SourceNodeAdoptionCarryPolicy>,
}

impl LoweredMergePlan {
    pub(crate) fn new(input: LoweredMergePlanConstruction) -> Self {
        let LoweredMergePlanConstruction {
            world,
            selected_semantics,
            journals,
            decisions,
            scope,
            lineage,
            nodes,
        } = input;
        let lowered_strategy_bundle_digest = lowered_strategy_bundle_digest(
            &selected_semantics,
            lineage.lowered_merge_base.as_ref(),
            &decisions.deletion,
            &decisions.conflict_isolation,
            &decisions.aspect_policy,
            &decisions.aspect,
        );
        let strategy_witness = SignalMergeStrategyWitness::from_admitted_plan_components(
            &selected_semantics,
            world.merge_strategy,
            &lowered_strategy_bundle_digest,
            &world.boundary_witness,
            aspect_policy_inventory(&decisions.aspect_policy),
            &nodes.adoption_policy,
        );
        Self {
            source_branch_id: world.source_branch_id,
            target_branch_id: world.target_branch_id,
            schema_registry_digest: world.schema_registry_digest,
            registry_bundle_digest: world.registry_bundle_digest,
            lowered_strategy_bundle_digest,
            merge_kind: world.merge_kind,
            divergence: world.divergence,
            merge_strategy: world.merge_strategy,
            selected_strategy_name: selected_semantics.strategy_name.clone(),
            selected_strategy_digest: selected_semantics.strategy_digest.clone(),
            selected_strategy_basis: selected_semantics.strategy_basis,
            selected_conflict_policy_name: selected_semantics.conflict_policy_name.clone(),
            selected_conflict_policy_digest: selected_semantics.conflict_policy_digest.clone(),
            selected_conflict_policy_basis: selected_semantics.conflict_policy_basis,
            selected_conflict_isolation_name: selected_semantics.conflict_isolation_name.clone(),
            selected_conflict_isolation_digest: selected_semantics
                .conflict_isolation_digest
                .clone(),
            selected_conflict_isolation_basis: selected_semantics.conflict_isolation_basis,
            selected_identity_matcher_name: selected_semantics.identity_matcher_name.clone(),
            selected_identity_matcher_digest: selected_semantics.identity_matcher_digest.clone(),
            selected_identity_matcher_basis: selected_semantics.identity_matcher_basis,
            selected_source_only_policy_name: selected_semantics.source_only_policy_name.clone(),
            selected_source_only_policy_digest: selected_semantics
                .source_only_policy_digest
                .clone(),
            selected_source_only_policy_basis: selected_semantics.source_only_policy_basis,
            selected_deletion_policy_name: selected_semantics.deletion_policy_name.clone(),
            selected_deletion_policy_digest: selected_semantics.deletion_policy_digest.clone(),
            selected_deletion_policy_basis: selected_semantics.deletion_policy_basis,
            selected_semantics,
            strategy_witness,
            reconciliation_policy: world.reconciliation_policy,
            boundary_witness: world.boundary_witness,
            source_journal: journals.source,
            target_overlap_journal: journals.target_overlap,
            identity_correspondence: decisions.identity_correspondence,
            deletion_plan: decisions.deletion,
            conflict_isolation_plan: decisions.conflict_isolation,
            aspect_policy_plan: decisions.aspect_policy,
            aspect_decision_plan: decisions.aspect,
            scoped_candidates: scope.candidates,
            scoped_merge_proof: scope.proof,
            proof_minimal_overlap: scope.proof_minimal_overlap,
            conservative_overlap: scope.conservative_overlap,
            planned_candidates: scope.planned_candidates,
            source_snapshot_id: lineage.source_snapshot_id,
            target_snapshot_id_before: lineage.target_snapshot_id_before,
            merge_base: lineage.merge_base,
            lowered_merge_base: lineage.lowered_merge_base,
            resolution_plan: lineage.resolution,
            node_map: nodes.map,
            node_plan: nodes.decisions,
            adoption_core: nodes.adoption_core,
            adoption_policy: nodes.adoption_policy,
        }
    }
}
