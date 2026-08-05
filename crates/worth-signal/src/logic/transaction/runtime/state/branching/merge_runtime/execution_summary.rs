use crate::logic::transaction::runtime::state::TopologyRepairSummary;
use crate::logic::transaction::runtime::{
    ArtifactMergeAction, BranchMergeCounters, BranchMergeExecutionSummary, BranchMergePlan,
};

use super::execution_finalization::BranchFinalization;

pub(super) fn build_execution_summary(
    plan: &BranchMergePlan,
    finalization: BranchFinalization,
) -> BranchMergeExecutionSummary {
    let records = finalization.records.clone();
    let counters = BranchMergeCounters {
        boundary_witness_kind: plan.boundary_witness().kind,
        source_slice_breadth: plan.source_journal().breadth(),
        proof_minimal_overlap_breadth: plan.proof_minimal_overlap().breadth(),
        conservative_overlap_expansion_breadth: plan.conservative_overlap().breadth(),
        final_candidate_breadth: plan.planned_candidates().breadth(),
        reconciliation_breadth: plan.node_plan().len() as u64,
        candidate_node_count: plan.node_plan().len() as u64,
        examined_node_count: plan.node_plan().len() as u64,
        adopted_count: records
            .iter()
            .filter(|record| matches!(record.action, ArtifactMergeAction::Adopted))
            .count() as u64,
        introduced_node_count: records
            .iter()
            .filter(|record| matches!(record.action, ArtifactMergeAction::IntroducedIntoTarget))
            .count() as u64,
        replaced_count: records
            .iter()
            .filter(|record| matches!(record.action, ArtifactMergeAction::Replaced))
            .count() as u64,
        preserved_target_count: records
            .iter()
            .filter(|record| matches!(record.action, ArtifactMergeAction::PreservedTarget))
            .count() as u64,
        skipped_non_adoptable_count: records
            .iter()
            .filter(|record| matches!(record.action, ArtifactMergeAction::SkippedNonAdoptable))
            .count() as u64,
        equivalent_unchanged_count: records
            .iter()
            .filter(|record| matches!(record.action, ArtifactMergeAction::EquivalentUnchanged))
            .count() as u64,
        source_only_count: plan
            .node_plan()
            .iter()
            .filter(|node| {
                matches!(
                    node.shape(),
                    crate::logic::transaction::runtime::NodeReconciliationShape::SourceOnlyIntroduction
                )
            })
            .count() as u64,
        target_only_count: plan.deletion_plan().target_only_count,
        dependency_remap_count: finalization.dependency_remaps.len() as u64,
        identity_target_candidates_indexed: plan
            .identity_correspondence()
            .target_candidate_count,
        identity_source_lookups: plan.identity_correspondence().source_lookup_count,
        identity_ambiguous_match_count: plan.identity_correspondence().ambiguous_match_count,
        identity_rejected_admissibility_count: plan
            .identity_correspondence()
            .rejected_admissibility_count,
        conflict_isolation_record_count: plan.conflict_isolation_plan().records.len() as u64,
        conflict_isolation_expansion_breadth: plan.conflict_isolation_plan().expansion_breadth,
        subscriber_repair_breadth: finalization.subscriber_repair_breadth,
        merge_lineage_record_count: (records.len() + 1) as u64,
        replay_event_count: 1,
    };
    let target_snapshot_after = finalization.target_snapshot_after;
    BranchMergeExecutionSummary {
        source_branch_id: plan.source_branch_id(),
        target_branch_id: plan.target_branch_id(),
        schema_registry_digest: plan.schema_registry_digest().to_owned(),
        registry_bundle_digest: plan.registry_bundle_digest().to_owned(),
        lowered_strategy_bundle_digest: plan.lowered_strategy_bundle_digest().to_owned(),
        merge_kind: plan.merge_kind(),
        divergence: plan.divergence(),
        merge_strategy: plan.merge_strategy(),
        selected_strategy_name: plan.selected_strategy_name().clone(),
        selected_strategy_digest: plan.selected_strategy_digest().to_string(),
        selected_strategy_basis: plan.selected_strategy_basis(),
        selected_conflict_policy_name: plan.selected_conflict_policy_name().clone(),
        selected_conflict_policy_digest: plan.selected_conflict_policy_digest().to_string(),
        selected_conflict_policy_basis: plan.selected_conflict_policy_basis(),
        selected_conflict_isolation_name: plan.selected_conflict_isolation_name().clone(),
        selected_conflict_isolation_digest: plan.selected_conflict_isolation_digest().to_string(),
        selected_conflict_isolation_basis: plan.selected_conflict_isolation_basis(),
        selected_identity_matcher_name: plan.selected_identity_matcher_name().clone(),
        selected_identity_matcher_digest: plan.selected_identity_matcher_digest().to_string(),
        selected_identity_matcher_basis: plan.selected_identity_matcher_basis(),
        selected_source_only_policy_name: plan.selected_source_only_policy_name().clone(),
        selected_source_only_policy_digest: plan.selected_source_only_policy_digest().to_string(),
        selected_source_only_policy_basis: plan.selected_source_only_policy_basis(),
        selected_deletion_policy_name: plan.selected_deletion_policy_name().clone(),
        selected_deletion_policy_digest: plan.selected_deletion_policy_digest().to_string(),
        selected_deletion_policy_basis: plan.selected_deletion_policy_basis(),
        selected_merge_base_name: plan
            .lowered_merge_base()
            .map(|base| base.selected_merge_base_name.clone())
            .expect("merge-base plan"),
        selected_merge_base_digest: plan
            .lowered_merge_base()
            .map(|base| base.selected_merge_base_digest.clone())
            .expect("merge-base plan"),
        selected_merge_base_basis: plan
            .lowered_merge_base()
            .map(|base| base.selected_merge_base_basis)
            .expect("merge-base plan"),
        selected_semantics: plan.selected_semantics().clone(),
        strategy_witness: plan.strategy_witness().clone(),
        compatibility_witness: finalization.compatibility_witness,
        reconciliation_policy: *plan.reconciliation_policy(),
        boundary_witness: plan.boundary_witness().clone(),
        identity_correspondence: plan.identity_correspondence().clone(),
        deletion_plan: plan.deletion_plan().clone(),
        conflict_isolation_plan: plan.conflict_isolation_plan().clone(),
        aspect_policy_plan: plan.aspect_policy_plan().clone(),
        aspect_decision_plan: plan.aspect_decision_plan().clone(),
        proof_minimal_overlap: plan.proof_minimal_overlap().clone(),
        conservative_overlap: plan.conservative_overlap().clone(),
        planned_candidates: plan.planned_candidates().clone(),
        scoped_merge_proof: plan.scoped_merge_proof().clone(),
        merge_base: plan.merge_base().cloned(),
        lowered_merge_base: plan.lowered_merge_base().cloned(),
        source_snapshot_id: plan.source_snapshot_id(),
        target_snapshot_id_before: plan.target_snapshot_id_before(),
        target_snapshot_id_after: target_snapshot_after,
        resolution_plan: plan.resolution_plan().cloned(),
        node_map: finalization.node_map,
        records,
        dependency_remaps: finalization.dependency_remaps,
        topology_repair: TopologyRepairSummary {
            touched_node_count: finalization.touched_set.nodes.len() as u64,
            subscriber_repair_breadth: finalization.subscriber_repair_breadth,
        },
        counters,
    }
}
