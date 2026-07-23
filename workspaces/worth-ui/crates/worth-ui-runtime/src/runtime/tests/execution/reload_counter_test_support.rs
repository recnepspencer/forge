use super::{
    WorthUiCandidateAdmissionCounters, WorthUiDurableStateReconciliationCounters,
    WorthUiExecutionPlanEquivalenceCounters, WorthUiIdentityMatchCounters,
    WorthUiImpactLookupCounters, WorthUiPlanLoweringCounters, WorthUiPlanTopologyCounters,
    WorthUiQueryLiveRebindCounters, WorthUiReloadCounterBoundary,
    WorthUiReloadCounterBoundaryDenial, WorthUiReloadLoweringCounterReceipt,
    WorthUiRuntimeArtifactComparisonCounters, WorthUiRuntimeHandleAllocationCounters,
};

pub(super) fn complete_receipt(
) -> Result<WorthUiReloadLoweringCounterReceipt, WorthUiReloadCounterBoundaryDenial> {
    WorthUiReloadCounterBoundary::reload_completed()
        .record_admission_counters(admission_counters())
        .record_artifact_comparison_counters(artifact_comparison_counters())
        .record_impact_narrowing_counters(impact_counters())
        .record_identity_match_counters(identity_counters())
        .record_reconciliation_counters(reconciliation_counters())
        .record_query_rebind_counters(query_rebind_counters())
        .record_plan_lowering_counters(plan_lowering_counters())
        .record_plan_assembly_counters(
            handle_allocation_counters(),
            topology_counters(),
            plan_equivalence_counters(),
        )
        .seal()
}

pub(super) fn admission_counters() -> WorthUiCandidateAdmissionCounters {
    let mut counters = WorthUiCandidateAdmissionCounters::default();
    counters.record_candidate_proof_check();
    counters.record_snapshot_compatibility_check();
    counters.record_runtime_posture_check();
    counters
}

pub(super) fn artifact_comparison_counters() -> WorthUiRuntimeArtifactComparisonCounters {
    let mut counters = WorthUiRuntimeArtifactComparisonCounters::default();
    counters.record_artifact_comparison();
    counters
}

pub(super) fn impact_counters() -> WorthUiImpactLookupCounters {
    let mut counters = WorthUiImpactLookupCounters::default();
    counters.record_impact_classification_consumed();
    counters.record_dependency_metadata_read();
    counters.record_module_impact_lookup();
    counters.record_subtree_impact_lookup();
    counters.record_runtime_hook_lookup();
    counters.record_subtree_digest_lookup();
    counters
}

pub(super) fn identity_counters() -> WorthUiIdentityMatchCounters {
    let mut counters = WorthUiIdentityMatchCounters::default();
    counters.record_active_node_indexed();
    counters.record_candidate_node_indexed();
    counters.record_stable_seed_lookup();
    counters.record_match_emitted();
    counters
}

pub(super) fn reconciliation_counters() -> WorthUiDurableStateReconciliationCounters {
    let mut counters = WorthUiDurableStateReconciliationCounters::default();
    counters.record_family();
    counters.record_node();
    counters.record_query_posture_required();
    counters
}

pub(super) fn query_rebind_counters() -> WorthUiQueryLiveRebindCounters {
    let mut counters = WorthUiQueryLiveRebindCounters::default();
    counters.record_preserved_binding_for_test();
    counters
}

pub(super) fn plan_lowering_counters() -> WorthUiPlanLoweringCounters {
    let mut counters = WorthUiPlanLoweringCounters::default();
    counters.record_epoch_verification();
    counters.record_readiness_verification();
    counters.record_staged_node_input();
    counters.record_query_binding_input();
    counters.record_reconciliation_receipts(1);
    counters.record_component_hook_input();
    counters
}

pub(super) fn handle_allocation_counters() -> WorthUiRuntimeHandleAllocationCounters {
    let mut counters = WorthUiRuntimeHandleAllocationCounters::default();
    counters.record_plan_node_input();
    counters.record_component_handle();
    counters.record_command_handle();
    counters.record_token_handle();
    counters.record_collision_check();
    counters
}

pub(super) fn topology_counters() -> WorthUiPlanTopologyCounters {
    let mut counters = WorthUiPlanTopologyCounters::default();
    counters.record_plan_node_input();
    counters.record_topology_node();
    counters.record_lookup_entry();
    counters.record_validation();
    counters
}

pub(super) fn plan_equivalence_counters() -> WorthUiExecutionPlanEquivalenceCounters {
    let mut counters = WorthUiExecutionPlanEquivalenceCounters::default();
    counters.record_plan_digest();
    counters.record_plan_node_digest();
    counters.record_equivalence_comparison();
    counters
}
