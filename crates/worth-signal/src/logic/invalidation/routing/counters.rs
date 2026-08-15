use crate::data::graph::SignalGraph;
use crate::data::proof::FrontierDiagnosticsProjection;

pub(super) fn record_diagnostic_projection(
    graph: &mut SignalGraph,
    counters: &FrontierDiagnosticsProjection,
) {
    graph.telemetry_mut().invalidation.frontier_seed_count += counters.frontier_seed_count;
    graph.telemetry_mut().invalidation.frontier_group_count += counters.frontier_group_count;
    graph
        .telemetry_mut()
        .invalidation
        .frontier_direct_wave_count += counters.frontier_direct_wave_count;
    graph
        .telemetry_mut()
        .invalidation
        .frontier_transitive_wave_count += counters.frontier_transitive_wave_count;
    graph
        .telemetry_mut()
        .invalidation
        .frontier_direct_dirty_count += counters.frontier_direct_dirty_count;
    graph
        .telemetry_mut()
        .invalidation
        .frontier_maybe_stale_count += counters.frontier_maybe_stale_count;
    graph
        .telemetry_mut()
        .invalidation
        .frontier_partition_match_count += counters.frontier_partition_match_count;
    graph
        .telemetry_mut()
        .invalidation
        .frontier_detail_match_count += counters.frontier_detail_match_count;
    graph
        .telemetry_mut()
        .invalidation
        .partition_match_dirty_count += counters.frontier_partition_match_count;
    graph.telemetry_mut().invalidation.detail_match_dirty_count +=
        counters.frontier_detail_match_count;
    graph
        .telemetry_mut()
        .invalidation
        .frontier_cycle_check_candidate_count += counters.frontier_cycle_check_candidate_count;
    graph
        .telemetry_mut()
        .invalidation
        .frontier_cycle_check_visited_count += counters.frontier_cycle_check_visited_count;
}
