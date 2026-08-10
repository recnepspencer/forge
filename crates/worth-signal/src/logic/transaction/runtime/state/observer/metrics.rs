use crate::data::telemetry::{EvaluationTelemetry, InvalidationTelemetry};

pub(super) fn merge_evaluation_telemetry(
    graph: EvaluationTelemetry,
    runtime: EvaluationTelemetry,
) -> EvaluationTelemetry {
    let mut merged = EvaluationTelemetry::default();
    merge_evaluation_counts(&mut merged, graph, runtime);
    merge_evaluation_rejection_counts(&mut merged, graph, runtime);
    merge_evaluation_outcome_counts(&mut merged, graph, runtime);
    merged
}

fn merge_evaluation_counts(
    merged: &mut EvaluationTelemetry,
    graph: EvaluationTelemetry,
    runtime: EvaluationTelemetry,
) {
    merged.evaluation_calls = graph.evaluation_calls + runtime.evaluation_calls;
    merged.evaluation_nanos = graph.evaluation_nanos + runtime.evaluation_nanos;
    merged.nodes_evaluated = graph.nodes_evaluated + runtime.nodes_evaluated;
    merged.nodes_recomputed = graph.nodes_recomputed + runtime.nodes_recomputed;
    merged.reuse_eligibility_checks_attempted =
        graph.reuse_eligibility_checks_attempted + runtime.reuse_eligibility_checks_attempted;
    merged.fresh_compute_count = graph.fresh_compute_count + runtime.fresh_compute_count;
    merged.output_suppressed_count =
        graph.output_suppressed_count + runtime.output_suppressed_count;
    merged.memoized_reuse_count = graph.memoized_reuse_count + runtime.memoized_reuse_count;
    merged.snapshot_restore_reuse_count =
        graph.snapshot_restore_reuse_count + runtime.snapshot_restore_reuse_count;
    merged.reconciliation_adoption_count =
        graph.reconciliation_adoption_count + runtime.reconciliation_adoption_count;
    merged.cross_identity_reuse_count =
        graph.cross_identity_reuse_count + runtime.cross_identity_reuse_count;
    merged.partial_artifact_splice_count =
        graph.partial_artifact_splice_count + runtime.partial_artifact_splice_count;
}

fn merge_evaluation_rejection_counts(
    merged: &mut EvaluationTelemetry,
    graph: EvaluationTelemetry,
    runtime: EvaluationTelemetry,
) {
    merged.reuse_rejected_unsupported_strategy_count = graph
        .reuse_rejected_unsupported_strategy_count
        + runtime.reuse_rejected_unsupported_strategy_count;
    merged.reuse_rejected_contract_strategy_count = graph.reuse_rejected_contract_strategy_count
        + runtime.reuse_rejected_contract_strategy_count;
    merged.reuse_rejected_boundary_mismatch_count = graph.reuse_rejected_boundary_mismatch_count
        + runtime.reuse_rejected_boundary_mismatch_count;
    merged.reuse_rejected_missing_prior_context_count = graph
        .reuse_rejected_missing_prior_context_count
        + runtime.reuse_rejected_missing_prior_context_count;
    merged.reuse_rejected_persistent_correspondence_missing_count = graph
        .reuse_rejected_persistent_correspondence_missing_count
        + runtime.reuse_rejected_persistent_correspondence_missing_count;
    merged.reuse_rejected_persistent_correspondence_invalid_count = graph
        .reuse_rejected_persistent_correspondence_invalid_count
        + runtime.reuse_rejected_persistent_correspondence_invalid_count;
    merged.reuse_rejected_composition_region_count = graph.reuse_rejected_composition_region_count
        + runtime.reuse_rejected_composition_region_count;
    merged.reuse_rejected_mixed_basis_insufficiency_count = graph
        .reuse_rejected_mixed_basis_insufficiency_count
        + runtime.reuse_rejected_mixed_basis_insufficiency_count;
    merged.reuse_dependency_comparison_breadth =
        graph.reuse_dependency_comparison_breadth + runtime.reuse_dependency_comparison_breadth;
    merged.reuse_cold_certification_materialization_count = graph
        .reuse_cold_certification_materialization_count
        + runtime.reuse_cold_certification_materialization_count;
}

fn merge_evaluation_outcome_counts(
    merged: &mut EvaluationTelemetry,
    graph: EvaluationTelemetry,
    runtime: EvaluationTelemetry,
) {
    merged.skipped_by_comparator = graph.skipped_by_comparator + runtime.skipped_by_comparator;
    merged.suppressed_downstream_propagations =
        graph.suppressed_downstream_propagations + runtime.suppressed_downstream_propagations;
    merged.output_identity_unchanged_count =
        graph.output_identity_unchanged_count + runtime.output_identity_unchanged_count;
    merged.memoization_hits = graph.memoization_hits + runtime.memoization_hits;
    merged.memoization_misses = graph.memoization_misses + runtime.memoization_misses;
    merged.condition_skip_count = graph.condition_skip_count + runtime.condition_skip_count;
    merged.ondemand_deferred_count =
        graph.ondemand_deferred_count + runtime.ondemand_deferred_count;
    merged.debounce_deferred_count =
        graph.debounce_deferred_count + runtime.debounce_deferred_count;
    merged.evaluation_stack_peak = graph
        .evaluation_stack_peak
        .max(runtime.evaluation_stack_peak);
}

pub(super) fn merge_invalidation_telemetry(
    graph: InvalidationTelemetry,
    runtime: InvalidationTelemetry,
) -> InvalidationTelemetry {
    InvalidationTelemetry {
        batch_width: graph.batch_width + runtime.batch_width,
        dirty_delta_breadth: graph.dirty_delta_breadth + runtime.dirty_delta_breadth,
        partition_aware_recomputations: graph.partition_aware_recomputations
            + runtime.partition_aware_recomputations,
        keyed_evaluation_count: graph.keyed_evaluation_count + runtime.keyed_evaluation_count,
        partition_scoped_invalidation_checks: graph.partition_scoped_invalidation_checks
            + runtime.partition_scoped_invalidation_checks,
        partition_match_dirty_count: graph.partition_match_dirty_count
            + runtime.partition_match_dirty_count,
        detail_match_dirty_count: graph.detail_match_dirty_count + runtime.detail_match_dirty_count,
        partition_scope_revert_clean_count: graph.partition_scope_revert_clean_count
            + runtime.partition_scope_revert_clean_count,
        partition_interner_growth_delta: graph.partition_interner_growth_delta
            + runtime.partition_interner_growth_delta,
        invalidation_nodes_visited: graph.invalidation_nodes_visited
            + runtime.invalidation_nodes_visited,
        narrowed_frontier_width: graph.narrowed_frontier_width + runtime.narrowed_frontier_width,
        transitive_frontier_width: graph.transitive_frontier_width
            + runtime.transitive_frontier_width,
        frontier_seed_count: graph.frontier_seed_count + runtime.frontier_seed_count,
        frontier_group_count: graph.frontier_group_count + runtime.frontier_group_count,
        frontier_direct_wave_count: graph.frontier_direct_wave_count
            + runtime.frontier_direct_wave_count,
        frontier_transitive_wave_count: graph.frontier_transitive_wave_count
            + runtime.frontier_transitive_wave_count,
        frontier_direct_dirty_count: graph.frontier_direct_dirty_count
            + runtime.frontier_direct_dirty_count,
        frontier_maybe_stale_count: graph.frontier_maybe_stale_count
            + runtime.frontier_maybe_stale_count,
        frontier_partition_match_count: graph.frontier_partition_match_count
            + runtime.frontier_partition_match_count,
        frontier_detail_match_count: graph.frontier_detail_match_count
            + runtime.frontier_detail_match_count,
        frontier_cycle_check_candidate_count: graph.frontier_cycle_check_candidate_count
            + runtime.frontier_cycle_check_candidate_count,
        frontier_cycle_check_visited_count: graph.frontier_cycle_check_visited_count
            + runtime.frontier_cycle_check_visited_count,
        frontier_trace_retained_count: graph.frontier_trace_retained_count
            + runtime.frontier_trace_retained_count,
        subscriber_repair_breadth: graph.subscriber_repair_breadth
            + runtime.subscriber_repair_breadth,
    }
}
