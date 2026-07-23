pub(super) fn add_conditional_counters(
    target: &mut super::WorthQueryWorkflowRunCounters,
    source: crate::domain_installation::WorthQueryOperationExecutionCounters,
) {
    target.conditional_request_admission_checks += source.conditional_request_admission_checks;
    target.conditional_contract_lookups += source.conditional_contract_lookups;
    target.conditional_dependency_observation_reads +=
        source.conditional_dependency_observation_reads;
    target.conditional_dependency_checks += source.conditional_dependency_checks;
    target.conditional_semantic_reads += source.conditional_semantic_reads;
    target.conditional_condition_checks += source.conditional_condition_checks;
    target.conditional_condition_deferrals += source.conditional_condition_deferrals;
    target.conditional_temporal_deferrals += source.conditional_temporal_deferrals;
    target.conditional_on_demand_deferrals += source.conditional_on_demand_deferrals;
    target.conditional_comparator_checks += source.conditional_comparator_checks;
    target.conditional_compute_contacts += source.conditional_compute_contacts;
    target.conditional_output_version_reads += source.conditional_output_version_reads;
    target.conditional_runtime_dependency_edges_captured +=
        source.conditional_runtime_dependency_edges_captured;
    target.conditional_application_contacts += source.conditional_application_contacts;
    target.conditional_semantic_classifications += source.conditional_semantic_classifications;
    target.conditional_reverted_clean_outcomes += source.conditional_reverted_clean_outcomes;
    target.conditional_semantic_changes += source.conditional_semantic_changes;
    target.conditional_reuse_checks += source.conditional_reuse_checks;
    target.conditional_decisions_delivered += source.conditional_decisions_delivered;
}
