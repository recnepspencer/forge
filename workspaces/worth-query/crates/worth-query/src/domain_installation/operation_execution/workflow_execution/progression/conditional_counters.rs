pub(super) fn add_conditional_counters(
    target: &mut super::WorthQueryWorkflowRunCounters,
    source: crate::domain_installation::WorthQueryOperationExecutionCounters,
) {
    target.conditional_dependency_checks += source.conditional_dependency_checks;
    target.conditional_semantic_reads += source.conditional_semantic_reads;
    target.conditional_condition_checks += source.conditional_condition_checks;
    target.conditional_comparator_checks += source.conditional_comparator_checks;
    target.conditional_compute_contacts += source.conditional_compute_contacts;
    target.conditional_semantic_changes += source.conditional_semantic_changes;
    target.conditional_reuse_checks += source.conditional_reuse_checks;
}
