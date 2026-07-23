use super::*;

pub(super) fn retain_lookup_rows<D, O, F, L: crate::basis_lifecycle::BasisOperationLane>(
    settled: &crate::domain_installation::WorthQuerySettledDomainProjection<D, O, F, L>,
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

pub(super) fn retain_binding_rows<D, O, F, L: crate::basis_lifecycle::BasisOperationLane>(
    settled: &crate::domain_installation::WorthQuerySettledDomainProjection<D, O, F, L>,
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

pub(super) fn retain_support_rows<D, O, F, L: crate::basis_lifecycle::BasisOperationLane>(
    settled: &crate::domain_installation::WorthQuerySettledDomainProjection<D, O, F, L>,
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

pub(super) fn retain_execution_rows<D, O, F, L: crate::basis_lifecycle::BasisOperationLane>(
    settled: &crate::domain_installation::WorthQuerySettledDomainProjection<D, O, F, L>,
    rows: &mut Vec<WorthQueryConsumptionCostRow>,
) {
    let counters = settled.counters();
    let work_class = execution_work_class(settled.bound_operation().commit_posture());
    retain_rows!(
        rows,
        "query.execution",
        work_class,
        counters,
        [
            runtime_authority_checks,
            input_contract_checks,
            graph_provider_contacts,
            primary_read_contacts,
            executor_contacts,
            terminal_posture_checks,
            publication_checks,
            consumption_contacts,
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

pub(super) fn retain_dependency_rows<D, O, F, L: crate::basis_lifecycle::BasisOperationLane>(
    settled: &crate::domain_installation::WorthQuerySettledDomainProjection<D, O, F, L>,
    rows: &mut Vec<WorthQueryConsumptionCostRow>,
) {
    let counters = settled.semantic_aspect_dependency_closure().counters();
    retain_dependency_counter_rows(counters, rows);
}

pub(super) fn retain_dependency_counter_rows(
    counters: crate::domain_installation::WorthQuerySemanticAspectDependencyCompilationCounters,
    rows: &mut Vec<WorthQueryConsumptionCostRow>,
) {
    retain_rows!(
        rows,
        "query.dependency",
        FoundationalPerformanceWorkClass::ValidationPlanning,
        counters,
        [
            semantic_contract_checks,
            execution_receipt_checks,
            graph_receipt_checks,
            conditional_authority_checks,
            workflow_trace_checks,
            installed_definition_visits,
            graph_read_role_visits,
            native_projection_edges,
            collection_membership_edges,
            collection_ordering_edges,
            collection_grouping_edges,
            collection_window_edges,
            result_shape_edges,
            touch_edges,
            effect_contract_edges,
            invariant_contract_edges,
            replay_contract_edges,
            lineage_contract_edges,
            support_contract_edges,
            workflow_stage_read_edges,
            conditional_node_visits,
            conditional_truth_edges,
            realized_graph_call_edges,
            realized_direct_output_edges,
            realized_workflow_read_edges,
            realized_conditional_outcome_edges,
            realized_effect_edges,
            realized_invariant_edges,
            realized_lineage_edges,
            realized_workflow_output_edges,
            conditional_observations_retained,
            canonical_traversal_edges,
            uniqueness_hash_checks,
            compiled_dependency_count,
            closure_edges_traversed,
            workflow_graph_edges_traversed,
            impact_index_entries,
            impact_index_dependency_visits,
            impact_mask_propagation_edges,
            unrelated_definition_scans,
            unrelated_runtime_scans,
            consumer_registry_scans,
        ]
    );
}

pub(super) fn retain_native_binding_rows<D, O, F, L: crate::basis_lifecycle::BasisOperationLane>(
    settled: &crate::domain_installation::WorthQuerySettledDomainProjection<D, O, F, L>,
    rows: &mut Vec<WorthQueryConsumptionCostRow>,
) {
    let Some(counters) = settled.native_access_binding_counters() else {
        return;
    };
    retain_native_binding_counter_rows(counters, rows);
}

pub(super) fn retain_native_binding_counter_rows(
    counters: crate::domain_installation::WorthQueryNativeAccessBindingCounters,
    rows: &mut Vec<WorthQueryConsumptionCostRow>,
) {
    retain_rows!(
        rows,
        "query.native_binding",
        FoundationalPerformanceWorkClass::ValidationPlanning,
        counters,
        [
            declared_key_routes,
            declared_key_layout_checks,
            lane_shape_checks,
            fact_scans,
            row_scans,
            path_parses,
            view_registry_inspections,
            domain_registry_inspections,
        ]
    );
}
