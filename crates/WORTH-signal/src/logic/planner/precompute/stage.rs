use crate::data::comparator::ComparatorPolicyResolver;
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;

use super::super::execution::StageSlice;
use super::super::types::{PlanSummary, StageExecutor};
#[cfg(feature = "parallel")]
use super::admission::{decide_stage_parallel_admission, StageParallelAdmission};
use super::dispatch::dispatch_stage_precompute;
use super::reporting::{record_stage_precompute_failure, record_stage_precompute_telemetry};
use super::{StageExecutionData, TemporalLoweringContext};

pub(in crate::logic::planner) struct StagePrecomputeResult {
    pub(in crate::logic::planner) execution: StageExecutionData,
    pub(in crate::logic::planner) snapshot_nanos: u128,
    pub(in crate::logic::planner) precompute_nanos: u128,
    #[cfg(feature = "parallel")]
    pub(in crate::logic::planner) parallel_admission: StageParallelAdmission,
}

struct SnapshotPass {
    snapshot_nanos: u128,
}

struct PrecomputeDispatchPass {
    execution: StageExecutionData,
    precompute_nanos: u128,
}

pub(in crate::logic::planner) fn perform_stage_precompute(
    graph: &mut SignalGraph,
    summary: &PlanSummary,
    stage: &StageSlice<'_>,
    precompute: &(impl Fn(
        crate::data::handle::NodeId,
        &crate::logic::prepared::ExecutionReadView<'_>,
    ) -> Result<crate::logic::prepared::PreparedEvaluation, SignalError>
          + Sync),
    comparator_resolver: &mut impl ComparatorPolicyResolver,
    temporal_lowering: TemporalLoweringContext,
    executor: StageExecutor,
) -> Result<StagePrecomputeResult, SignalError> {
    #[cfg(feature = "parallel")]
    let parallel_admission = decide_stage_parallel_admission(graph, stage, executor);

    let snapshot_pass = run_snapshot_pass(graph);
    let dispatch_pass = run_precompute_dispatch_pass(
        graph,
        summary,
        stage.index,
        stage.tasks,
        precompute,
        comparator_resolver,
        temporal_lowering,
        executor,
        #[cfg(feature = "parallel")]
        parallel_admission,
    )?;
    record_stage_precompute_telemetry(
        graph,
        &dispatch_pass.execution,
        snapshot_pass.snapshot_nanos,
        dispatch_pass.precompute_nanos,
        executor,
        #[cfg(feature = "parallel")]
        parallel_admission,
    );

    Ok(StagePrecomputeResult {
        execution: dispatch_pass.execution,
        snapshot_nanos: snapshot_pass.snapshot_nanos,
        precompute_nanos: dispatch_pass.precompute_nanos,
        #[cfg(feature = "parallel")]
        parallel_admission,
    })
}

fn run_snapshot_pass(graph: &mut SignalGraph) -> SnapshotPass {
    let snapshot_start = crate::clock::RuntimeInstant::now();
    graph.telemetry_mut().execution.execution_snapshots_built += 1;
    SnapshotPass {
        snapshot_nanos: snapshot_start.elapsed().as_nanos(),
    }
}

#[allow(clippy::too_many_arguments)]
fn run_precompute_dispatch_pass(
    graph: &mut SignalGraph,
    summary: &PlanSummary,
    stage_index: u32,
    tasks: &[crate::logic::planner::EligibleTask],
    precompute: &(impl Fn(
        crate::data::handle::NodeId,
        &crate::logic::prepared::ExecutionReadView<'_>,
    ) -> Result<crate::logic::prepared::PreparedEvaluation, SignalError>
          + Sync),
    comparator_resolver: &mut impl ComparatorPolicyResolver,
    temporal_lowering: TemporalLoweringContext,
    executor: StageExecutor,
    #[cfg(feature = "parallel")] parallel_admission: StageParallelAdmission,
) -> Result<PrecomputeDispatchPass, SignalError> {
    let precompute_start = crate::clock::RuntimeInstant::now();
    let execution = dispatch_stage_precompute(
        graph,
        tasks,
        precompute,
        comparator_resolver,
        temporal_lowering,
        executor,
        #[cfg(feature = "parallel")]
        parallel_admission,
    )
    .map_err(|err| {
        record_stage_precompute_failure(graph, summary, stage_index, executor, &err);
        err
    })?;
    Ok(PrecomputeDispatchPass {
        execution,
        precompute_nanos: precompute_start.elapsed().as_nanos(),
    })
}
