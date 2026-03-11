use crate::data::graph::SignalGraph;

use super::super::execution::task_reporting::accumulate_report_counters;
use super::super::types::{ExecutionReport, TaskExecutionRecord};
use super::SemanticTaskUpdate;

pub(super) fn record_semantic_update(
    graph: &mut SignalGraph,
    report: &mut ExecutionReport,
    task_record: &TaskExecutionRecord,
    update: &SemanticTaskUpdate,
) {
    accumulate_report_counters(report, task_record);
    graph.telemetry_mut().prepared_evaluations_applied += 1;
    graph.telemetry_mut().dependency_capture_updates += update.dependency_updates as u64;
    if update.dependency_updates > 0 {
        graph.telemetry_mut().rewiring_apply_count += 1;
    }
    if update.recomputed {
        graph.telemetry_mut().nodes_recomputed += 1;
    }
    if update.partition_aware {
        graph.telemetry_mut().partition_aware_recomputations += 1;
    }
    report.prepared_evaluations_applied += 1;
    report.dependency_capture_updates += update.dependency_updates;
}
