use super::*;

pub(super) fn retain_workflow_lookup_rows<
    D,
    O,
    F,
    L: crate::basis_lifecycle::BasisOperationLane,
>(
    settled: &crate::domain_installation::WorthQuerySettledWorkflowProjection<D, O, F, L>,
    rows: &mut Vec<WorthQueryConsumptionCostRow>,
) {
    let counters = settled.bound_operation().operation().lookup_counters();
    retain_rows!(
        rows,
        "query.lookup",
        FoundationalPerformanceWorkClass::ValidationPlanning,
        counters,
        [
            authority_checks,
            indexed_operation_lookups,
            graph_binding_lookups,
            graph_bindings_retained,
            package_content_scans,
            planning_steps,
            lower_runtime_contacts,
        ]
    );
}

pub(super) fn retain_workflow_binding_rows<
    D,
    O,
    F,
    L: crate::basis_lifecycle::BasisOperationLane,
>(
    settled: &crate::domain_installation::WorthQuerySettledWorkflowProjection<D, O, F, L>,
    rows: &mut Vec<WorthQueryConsumptionCostRow>,
) {
    let counters = settled.bound_operation().binding_counters();
    retain_rows!(
        rows,
        "query.binding",
        FoundationalPerformanceWorkClass::ValidationPlanning,
        counters,
        [
            authority_checks,
            operation_lookups,
            required_domain_lookups,
            graph_binding_lookups,
            graph_participation_lookups,
            graph_provider_contacts,
            conditional_lowering_lookups,
            conditional_lowerings_retained,
            conditional_declarations_inspected,
            conditional_workflow_stages_inspected,
            conditional_lowering_checks,
            graph_contract_checks,
            graph_read_role_checks,
            touched_graph_role_checks,
            commit_graph_checks,
            commit_authority_checks,
            planning_steps,
            authority_shape_admissions,
            commit_posture_classifications,
            executor_route_lookups,
            workflow_executor_route_lookups,
            parallel_admission_route_lookups,
        ]
    );
}

pub(super) fn retain_workflow_support_rows<
    D,
    O,
    F,
    L: crate::basis_lifecycle::BasisOperationLane,
>(
    settled: &crate::domain_installation::WorthQuerySettledWorkflowProjection<D, O, F, L>,
    rows: &mut Vec<WorthQueryConsumptionCostRow>,
) {
    let counters = settled.consumer_contract().counters();
    retain_rows!(
        rows,
        "query.support",
        FoundationalPerformanceWorkClass::ValidationPlanning,
        counters,
        [
            installation_generation_checks,
            mint_guard_checks,
            dimensions_evaluated,
            reporting_digest_comparisons,
            downstream_hook_inspections,
        ]
    );
}

pub(super) fn retain_workflow_execution_rows<
    D,
    O,
    F,
    L: crate::basis_lifecycle::BasisOperationLane,
>(
    settled: &crate::domain_installation::WorthQuerySettledWorkflowProjection<D, O, F, L>,
    rows: &mut Vec<WorthQueryConsumptionCostRow>,
) {
    let counters = settled.counters();
    let work_class = execution_work_class(settled.bound_operation().commit_posture());
    retain_rows!(
        rows,
        "query.workflow_execution",
        work_class,
        counters,
        [
            runtime_authority_checks,
            stage_index_lookups,
            stage_admission_checks,
            predecessor_checks,
            predecessor_receipt_lookups,
            required_capability_checks,
            required_domain_checks,
            graph_read_contacts,
            touch_effect_contacts,
            effect_receipt_checks,
            commit_admission_contacts,
            invariant_checks,
            parallel_admission_checks,
            stage_executor_contacts,
            output_contract_checks,
            terminal_contract_checks,
            consumption_contacts,
            unrelated_run_scans,
            conditional_request_admission_checks,
            conditional_contract_lookups,
            conditional_dependency_observation_reads,
            conditional_dependency_checks,
            conditional_semantic_reads,
            conditional_condition_checks,
            conditional_condition_deferrals,
            conditional_temporal_deferrals,
            conditional_on_demand_deferrals,
            conditional_comparator_checks,
            conditional_compute_contacts,
            conditional_output_version_reads,
            conditional_runtime_dependency_edges_captured,
            conditional_application_contacts,
            conditional_semantic_classifications,
            conditional_reverted_clean_outcomes,
            conditional_semantic_changes,
            conditional_reuse_checks,
            conditional_decisions_delivered,
        ]
    );
}
