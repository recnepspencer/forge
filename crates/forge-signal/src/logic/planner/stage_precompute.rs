use crate::data::comparator::ComparatorPolicyResolver;
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::logic::prepared::ExecutionSnapshot;

use super::execution::StageSlice;
use super::precompute::StageExecutionData;
use super::precompute_dispatch::dispatch_stage_precompute;
use super::precompute_reporting::{
    record_stage_precompute_failure, record_stage_precompute_telemetry,
};
#[cfg(feature = "parallel")]
use super::stage_admission::{decide_stage_parallel_admission, StageParallelAdmission};
use super::types::{PlanSummary, StageExecutor};

pub(crate) struct StagePrecomputeResult {
    pub(crate) execution: StageExecutionData,
    pub(crate) snapshot_nanos: u128,
    pub(crate) precompute_nanos: u128,
    #[cfg(feature = "parallel")]
    pub(crate) parallel_admission: StageParallelAdmission,
}

pub(crate) fn perform_stage_precompute(
    graph: &mut SignalGraph,
    summary: &PlanSummary,
    stage: &StageSlice<'_>,
    precompute: &(impl Fn(
        crate::data::handle::NodeId,
        &crate::logic::prepared::ExecutionReadView<'_>,
    ) -> Result<crate::logic::prepared::PreparedEvaluation, SignalError>
        + Sync),
    comparator_resolver: &mut impl ComparatorPolicyResolver,
    executor: StageExecutor,
) -> Result<StagePrecomputeResult, SignalError> {
    #[cfg(feature = "parallel")]
    let parallel_admission = decide_stage_parallel_admission(graph, stage, executor);

    let snapshot_start = std::time::Instant::now();
    graph.telemetry_mut().execution_snapshots_built += 1;
    let precompute_start = std::time::Instant::now();
    let execution = (|| {
        let snapshot = ExecutionSnapshot::new(&*graph);
        dispatch_stage_precompute(
            stage.tasks,
            &snapshot,
            precompute,
            comparator_resolver,
            executor,
            #[cfg(feature = "parallel")]
            parallel_admission,
        )
    })()
    .map_err(|err| {
        record_stage_precompute_failure(graph, summary, stage.index, executor, &err);
        err
    })?;

    let snapshot_nanos = snapshot_start.elapsed().as_nanos();
    let precompute_nanos = precompute_start.elapsed().as_nanos();
    record_stage_precompute_telemetry(
        graph,
        &execution,
        snapshot_nanos,
        precompute_nanos,
        executor,
        #[cfg(feature = "parallel")]
        parallel_admission,
    );

    Ok(StagePrecomputeResult {
        execution,
        snapshot_nanos,
        precompute_nanos,
        #[cfg(feature = "parallel")]
        parallel_admission,
    })
}
