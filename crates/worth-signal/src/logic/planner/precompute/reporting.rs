use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::diagnostics::failure::{ExecutionFailureContext, ExecutionFailurePhase};

use super::super::execution::task_reporting::record_execution_failure_if_enabled;
use super::super::types::{ExecutionReport, PlanSummary, StageExecutor};
#[cfg(feature = "parallel")]
use super::admission::StageParallelAdmission;
use super::StageExecutionData;

pub(in crate::logic::planner) fn record_stage_precompute_telemetry(
    graph: &mut SignalGraph,
    execution: &StageExecutionData,
    snapshot_nanos: u128,
    precompute_nanos: u128,
    executor: StageExecutor,
    #[cfg(feature = "parallel")] parallel_admission: StageParallelAdmission,
) {
    let execution_len = execution.len() as u64;
    graph.with_telemetry(|telemetry| {
        telemetry.execution.execution_snapshot_nanos += snapshot_nanos;
        telemetry.execution.stage_precompute_nanos += precompute_nanos;
        telemetry.execution.prepared_evaluations_produced += execution_len;
    });
    match executor {
        StageExecutor::Serial => {
            graph.with_telemetry(|telemetry| {
                telemetry.execution.serial_precompute_task_count += execution_len;
            });
        }
        #[cfg(feature = "parallel")]
        _ if parallel_admission.use_parallel => {
            graph.with_telemetry(|telemetry| {
                telemetry.execution.parallel_stage_dispatch_count += 1;
                telemetry.execution.parallel_precompute_task_count += execution_len;
            });
        }
        #[cfg(feature = "parallel")]
        StageExecutor::StagedParallelPrecompute { .. } | StageExecutor::FullParallel { .. } => {
            graph.with_telemetry(|telemetry| {
                telemetry.execution.serial_precompute_task_count += execution_len;
            });
        }
    }
}

pub(in crate::logic::planner) fn record_stage_precompute_report(
    report: &mut ExecutionReport,
    execution: &StageExecutionData,
    snapshot_nanos: u128,
    precompute_nanos: u128,
) {
    report.execution_snapshots_built += 1;
    report.execution_snapshot_nanos += snapshot_nanos;
    report.prepared_evaluations_produced += execution.len() as u32;
    report.stage_precompute_nanos += precompute_nanos;
}

pub(crate) fn record_stage_precompute_failure(
    graph: &mut SignalGraph,
    summary: &PlanSummary,
    stage_index: u32,
    executor: StageExecutor,
    err: &SignalError,
) {
    record_execution_failure_if_enabled(graph, || {
        ExecutionFailureContext::new(
            ExecutionFailurePhase::Precompute,
            Some(stage_index),
            None,
            Some(executor),
            None,
            Some(*summary),
            err.to_string(),
        )
    });
}
