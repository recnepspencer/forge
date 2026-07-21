use crate::runtime::{
    WorthUiCandidateAdmissionCounters, WorthUiDurableStateReconciliationCounters,
    WorthUiExecutionPlanEquivalenceCounters, WorthUiFrameCostCounter, WorthUiIdentityMatchCounters,
    WorthUiImpactLookupCounters, WorthUiMeasurementBoundary, WorthUiPlanLoweringCounters,
    WorthUiPlanTopologyCounters, WorthUiQueryLiveRebindCounters,
    WorthUiRuntimeArtifactComparisonCounters, WorthUiRuntimeCounterFamily,
    WorthUiRuntimeHandleAllocationCounters,
};

pub(crate) fn admission_rows(
    counters: WorthUiCandidateAdmissionCounters,
) -> Vec<WorthUiFrameCostCounter> {
    vec![
        count(
            "reload.candidate_admission.candidate_proof_checks",
            counters.candidate_proof_checks(),
        ),
        count(
            "reload.candidate_admission.snapshot_compatibility_checks",
            counters.snapshot_compatibility_checks(),
        ),
        count(
            "reload.candidate_admission.runtime_posture_checks",
            counters.runtime_posture_checks(),
        ),
    ]
}

pub(crate) fn artifact_comparison_rows(
    counters: WorthUiRuntimeArtifactComparisonCounters,
) -> Vec<WorthUiFrameCostCounter> {
    vec![count(
        "reload.artifact_comparison.artifact_comparisons",
        counters.artifact_comparisons(),
    )]
}

pub(crate) fn impact_narrowing_rows(
    counters: WorthUiImpactLookupCounters,
) -> Vec<WorthUiFrameCostCounter> {
    vec![
        count(
            "reload.impact_narrowing.impact_classifications_consumed",
            counters.impact_classifications_consumed(),
        ),
        count(
            "reload.impact_narrowing.dependency_metadata_reads",
            counters.dependency_metadata_reads(),
        ),
        count(
            "reload.impact_narrowing.module_impact_lookups",
            counters.module_impact_lookups(),
        ),
        count(
            "reload.impact_narrowing.subtree_impact_lookups",
            counters.subtree_impact_lookups(),
        ),
        count(
            "reload.impact_narrowing.runtime_hook_lookups",
            counters.runtime_hook_lookups(),
        ),
        count(
            "reload.impact_narrowing.subtree_digest_lookups",
            counters.subtree_digest_lookups(),
        ),
        count(
            "reload.impact_narrowing.full_artifact_scans",
            counters.full_artifact_scans(),
        ),
    ]
}

pub(crate) fn identity_rows(
    counters: WorthUiIdentityMatchCounters,
) -> Vec<WorthUiFrameCostCounter> {
    vec![
        count(
            "reload.identity_replacement.active_nodes_indexed",
            counters.active_nodes_indexed(),
        ),
        count(
            "reload.identity_replacement.candidate_nodes_indexed",
            counters.candidate_nodes_indexed(),
        ),
        count(
            "reload.identity_replacement.stable_seed_lookups",
            counters.stable_seed_lookups(),
        ),
        count(
            "reload.identity_replacement.matches_emitted",
            counters.matches_emitted(),
        ),
    ]
}

pub(crate) fn reconciliation_rows(
    counters: WorthUiDurableStateReconciliationCounters,
) -> Vec<WorthUiFrameCostCounter> {
    vec![
        count(
            "reload.durable_state_reconciliation.families_reconciled",
            counters.reconciled_family_count(),
        ),
        count(
            "reload.durable_state_reconciliation.nodes_reconciled",
            counters.reconciled_node_count(),
        ),
        count(
            "reload.durable_state_reconciliation.receipts",
            counters.receipt_count(),
        ),
        count(
            "reload.durable_state_reconciliation.query_posture_required",
            counters.query_posture_required_count(),
        ),
    ]
}

pub(crate) fn query_rebind_rows(
    counters: WorthUiQueryLiveRebindCounters,
) -> Vec<WorthUiFrameCostCounter> {
    vec![
        count(
            "reload.query_rebind_planning.bindings_planned",
            counters.bindings_planned(),
        ),
        count(
            "reload.query_rebind_planning.bindings_preserved",
            counters.preserved_binding_count(),
        ),
        count(
            "reload.query_rebind_planning.bindings_rebound",
            counters.rebound_binding_count(),
        ),
        count(
            "reload.query_rebind_planning.bindings_retired",
            counters.retired_binding_count(),
        ),
    ]
}

pub(crate) fn plan_lowering_rows(
    counters: WorthUiPlanLoweringCounters,
) -> Vec<WorthUiFrameCostCounter> {
    vec![
        count(
            "plan.lowering.staged_node_inputs",
            counters.staged_node_input_count(),
        ),
        count(
            "plan.lowering.query_binding_inputs",
            counters.query_binding_input_count(),
        ),
        count(
            "plan.lowering.reconciliation_receipt_inputs",
            counters.reconciliation_receipt_input_count(),
        ),
        count(
            "plan.lowering.component_hook_inputs",
            counters.component_hook_input_count(),
        ),
        count(
            "plan.lowering.readiness_verifications",
            counters.readiness_verification_count(),
        ),
        count(
            "plan.lowering.epoch_verifications",
            counters.epoch_verification_count(),
        ),
        count(
            "plan.lowering.source_parse_count",
            counters.source_parse_count(),
        ),
        count(
            "plan.lowering.registry_string_lookup_count",
            counters.registry_string_lookup_count(),
        ),
    ]
}

pub(crate) fn plan_assembly_rows(
    handle: WorthUiRuntimeHandleAllocationCounters,
    topology: WorthUiPlanTopologyCounters,
    equivalence: WorthUiExecutionPlanEquivalenceCounters,
) -> Vec<WorthUiFrameCostCounter> {
    vec![
        count(
            "plan.assembly.handle_plan_node_inputs",
            handle.plan_node_input_count(),
        ),
        count(
            "plan.assembly.component_handles",
            handle.component_handle_count(),
        ),
        count(
            "plan.assembly.command_handles",
            handle.command_handle_count(),
        ),
        count("plan.assembly.token_handles", handle.token_handle_count()),
        count(
            "plan.assembly.collision_checks",
            handle.collision_check_count(),
        ),
        count(
            "plan.assembly.handle_broad_registry_scans",
            handle.broad_registry_scan_count(),
        ),
        count(
            "plan.assembly.topology_nodes",
            topology.topology_node_count(),
        ),
        count(
            "plan.assembly.lookup_entries",
            topology.lookup_entry_count(),
        ),
        count(
            "plan.assembly.topology_validations",
            topology.topology_validation_count(),
        ),
        count(
            "plan.assembly.topology_artifact_tree_scans",
            topology.artifact_tree_scan_count(),
        ),
        count(
            "plan.assembly.topology_broad_registry_scans",
            topology.broad_registry_scan_count(),
        ),
        count(
            "plan.assembly.plan_digest_count",
            equivalence.plan_digest_count(),
        ),
        count(
            "plan.assembly.plan_node_digest_count",
            equivalence.plan_node_digest_count(),
        ),
        count(
            "plan.assembly.plan_equivalence_comparisons",
            equivalence.equivalence_comparison_count(),
        ),
        count(
            "plan.assembly.equivalence_artifact_tree_scans",
            equivalence.artifact_tree_scan_count(),
        ),
    ]
}

pub(crate) fn boundary_for_family(
    family: WorthUiRuntimeCounterFamily,
) -> WorthUiMeasurementBoundary {
    family.allowed_boundary()
}

fn count(name: &'static str, value: usize) -> WorthUiFrameCostCounter {
    WorthUiFrameCostCounter::count(name, value as u64)
}
