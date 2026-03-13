use crate::data::graph::SignalGraph;

use super::super::execution::task_reporting::accumulate_report_counters;
use super::super::types::{ExecutionReport, TaskExecutionRecord};
pub(super) fn record_semantic_update(
    graph: &mut SignalGraph,
    report: &mut ExecutionReport,
    task_record: &TaskExecutionRecord,
    dependency_updates: u32,
    recomputed: bool,
    partition_aware: bool,
) {
    accumulate_report_counters(report, task_record);
    graph.telemetry_mut().execution.prepared_evaluations_applied += 1;
    graph.telemetry_mut().execution.dependency_capture_updates += dependency_updates as u64;
    if dependency_updates > 0 {
        graph.telemetry_mut().execution.rewiring_apply_count += 1;
    }
    if recomputed {
        graph.telemetry_mut().evaluation.nodes_recomputed += 1;
    }
    if partition_aware {
        graph
            .telemetry_mut()
            .invalidation
            .partition_aware_recomputations += 1;
    }
    report.prepared_evaluations_applied += 1;
    report.dependency_capture_updates += dependency_updates;
}
