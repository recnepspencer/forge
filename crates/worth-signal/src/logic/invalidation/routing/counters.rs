use crate::data::graph::SignalGraph;
use crate::data::proof::FrontierDiagnosticsProjection;

pub(super) fn record_diagnostic_projection(
    graph: &mut SignalGraph,
    counters: &FrontierDiagnosticsProjection,
) {
    graph.with_telemetry(|telemetry| {
        telemetry.invalidation.frontier_seed_count += counters.frontier_seed_count;
        telemetry.invalidation.frontier_group_count += counters.frontier_group_count;
        telemetry.invalidation.frontier_direct_wave_count += counters.frontier_direct_wave_count;
        telemetry.invalidation.frontier_transitive_wave_count +=
            counters.frontier_transitive_wave_count;
        telemetry.invalidation.frontier_direct_dirty_count += counters.frontier_direct_dirty_count;
        telemetry.invalidation.frontier_maybe_stale_count += counters.frontier_maybe_stale_count;
        telemetry.invalidation.frontier_partition_match_count +=
            counters.frontier_partition_match_count;
        telemetry.invalidation.frontier_detail_match_count += counters.frontier_detail_match_count;
        telemetry.invalidation.partition_match_dirty_count +=
            counters.frontier_partition_match_count;
        telemetry.invalidation.detail_match_dirty_count += counters.frontier_detail_match_count;
        telemetry.invalidation.frontier_cycle_check_candidate_count +=
            counters.frontier_cycle_check_candidate_count;
        telemetry.invalidation.frontier_cycle_check_visited_count +=
            counters.frontier_cycle_check_visited_count;
    });
}
