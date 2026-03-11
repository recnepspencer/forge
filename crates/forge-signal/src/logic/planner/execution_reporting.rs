use crate::data::graph::SignalGraph;

use super::types::{ExecutionReport, PlanSummary, StageExecutionRecord, StageExecutor};

pub(crate) fn begin_execution_report(
    graph: &mut SignalGraph,
    summary: &PlanSummary,
    stage_count: usize,
    maybe_stale_validation_tasks: u64,
    executor: StageExecutor,
) -> ExecutionReport {
    graph.telemetry_mut().plans_built += 1;
    graph.telemetry_mut().stages_built += stage_count as u64;
    graph.telemetry_mut().tasks_scheduled += summary.task_count as u64;
    graph.telemetry_mut().max_tasks_in_stage = graph
        .telemetry()
        .max_tasks_in_stage
        .max(summary.max_stage_width as u64);
    graph.telemetry_mut().maybe_stale_validation_tasks += maybe_stale_validation_tasks;

    #[cfg(feature = "parallel")]
    match executor {
        StageExecutor::StagedParallelPrecompute { .. } | StageExecutor::FullParallel { .. } => {
            graph.telemetry_mut().parallel_executor_usage_count += 1;
        }
        StageExecutor::Serial => {
            graph.telemetry_mut().serial_executor_usage_count += 1;
        }
    }
    #[cfg(not(feature = "parallel"))]
    {
        let _ = executor;
        graph.telemetry_mut().serial_executor_usage_count += 1;
    }

    ExecutionReport {
        plan_summary: summary.clone(),
        stage_count: summary.stage_count,
        task_count: summary.task_count,
        tasks_executed: 0,
        tasks_pruned: 0,
        tasks_validated_clean: 0,
        tasks_deferred_by_condition: 0,
        tasks_reverted_clean_by_condition: 0,
        tasks_satisfied_by_memoization: 0,
        tasks_with_suppressed_propagation: 0,
        execution_snapshots_built: 0,
        prepared_evaluations_produced: 0,
        prepared_evaluations_applied: 0,
        dependency_capture_updates: 0,
        execution_snapshot_nanos: 0,
        stage_precompute_nanos: 0,
        stage_apply_nanos: 0,
        semantic_finalize_nanos: 0,
        semantic_segment_count: 0,
        stages: Vec::new(),
    }
}

pub(crate) fn record_stage_execution_completion(
    graph: &mut SignalGraph,
    report: &mut ExecutionReport,
    mut stage_record: StageExecutionRecord,
    apply_elapsed_nanos: u128,
    stage_elapsed_nanos: u128,
) {
    stage_record.apply_duration_nanos = apply_elapsed_nanos
        .saturating_sub(stage_record.semantic_finalize_duration_nanos);
    report.stage_apply_nanos += stage_record.apply_duration_nanos;
    graph.telemetry_mut().stage_apply_nanos += stage_record.apply_duration_nanos;
    report.semantic_finalize_nanos += stage_record.semantic_finalize_duration_nanos;

    stage_record.duration_nanos = stage_elapsed_nanos;
    graph.telemetry_mut().stage_execution_count += 1;
    graph.telemetry_mut().stage_execution_nanos += stage_record.duration_nanos;
    report.stages.push(stage_record);
}
