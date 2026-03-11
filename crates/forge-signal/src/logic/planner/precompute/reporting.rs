use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::diagnostics::failure::{ExecutionFailureContext, ExecutionFailurePhase};

use super::StageExecutionData;
use super::super::execution::task_reporting::record_execution_failure;
#[cfg(feature = "parallel")]
use super::admission::StageParallelAdmission;
use super::super::types::{ExecutionReport, PlanSummary, StageExecutor};

pub(in crate::logic::planner) fn record_stage_precompute_telemetry(
    graph: &mut SignalGraph,
    execution: &StageExecutionData,
    snapshot_nanos: u128,
    precompute_nanos: u128,
    executor: StageExecutor,
    #[cfg(feature = "parallel")] parallel_admission: StageParallelAdmission,
) {
    graph.telemetry_mut().execution_snapshot_nanos += snapshot_nanos;
    graph.telemetry_mut().stage_precompute_nanos += precompute_nanos;
    graph.telemetry_mut().prepared_evaluations_produced += execution.len() as u64;
    match executor {
        StageExecutor::Serial => {
            graph.telemetry_mut().serial_precompute_task_count += execution.len() as u64;
        }
        #[cfg(feature = "parallel")]
        _ if parallel_admission.use_parallel => {
            graph.telemetry_mut().parallel_stage_dispatch_count += 1;
            graph.telemetry_mut().parallel_precompute_task_count += execution.len() as u64;
        }
        #[cfg(feature = "parallel")]
        StageExecutor::StagedParallelPrecompute { .. } | StageExecutor::FullParallel { .. } => {
            graph.telemetry_mut().serial_precompute_task_count += execution.len() as u64;
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
    record_execution_failure(
        graph,
        ExecutionFailureContext::new(
            ExecutionFailurePhase::Precompute,
            Some(stage_index),
            None,
            Some(executor),
            None,
            Some(summary.clone()),
            err.to_string(),
        ),
    );
}
