use crate::data::comparator::ComparatorPolicyResolver;
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::proof::SingleConsumer;

use super::super::types::{EvaluationTask, StageExecutor};
#[cfg(feature = "parallel")]
use super::admission::StageParallelAdmission;
#[cfg(feature = "parallel")]
use super::{build_parallel_stage_patches, precompute_stage_parallel};
use super::{precompute_stage_serial, StageExecutionData};

pub(in crate::logic::planner) fn dispatch_stage_precompute(
    graph: &mut SignalGraph,
    tasks: &[EvaluationTask],
    precompute: &(impl Fn(
        crate::data::handle::NodeId,
        &crate::logic::prepared::ExecutionReadView<'_>,
    ) -> Result<crate::logic::prepared::PreparedEvaluation, SignalError>
          + Sync),
    comparator_resolver: &mut impl ComparatorPolicyResolver,
    executor: StageExecutor,
    #[cfg(feature = "parallel")] parallel_admission: StageParallelAdmission,
) -> Result<StageExecutionData, SignalError> {
    #[cfg(not(feature = "parallel"))]
    let _ = executor;

    #[cfg(feature = "parallel")]
    if parallel_admission.use_parallel {
        return dispatch_stage_precompute_parallel(
            graph,
            tasks,
            precompute,
            comparator_resolver,
            executor,
        );
    }

    dispatch_stage_precompute_serial(graph, tasks, precompute, comparator_resolver)
}

fn dispatch_stage_precompute_serial(
    graph: &mut SignalGraph,
    tasks: &[EvaluationTask],
    precompute: &(impl Fn(
        crate::data::handle::NodeId,
        &crate::logic::prepared::ExecutionReadView<'_>,
    ) -> Result<crate::logic::prepared::PreparedEvaluation, SignalError>
          + Sync),
    comparator_resolver: &mut impl ComparatorPolicyResolver,
) -> Result<StageExecutionData, SignalError> {
    Ok(StageExecutionData::Prepared(SingleConsumer::new(
        precompute_stage_serial(graph, tasks, precompute, comparator_resolver)?,
    )))
}

#[cfg(feature = "parallel")]
fn dispatch_stage_precompute_parallel(
    graph: &mut SignalGraph,
    tasks: &[EvaluationTask],
    precompute: &(impl Fn(
        crate::data::handle::NodeId,
        &crate::logic::prepared::ExecutionReadView<'_>,
    ) -> Result<crate::logic::prepared::PreparedEvaluation, SignalError>
          + Sync),
    comparator_resolver: &mut impl ComparatorPolicyResolver,
    executor: StageExecutor,
) -> Result<StageExecutionData, SignalError> {
    match executor {
        StageExecutor::FullParallel { .. } => Ok(StageExecutionData::Patched(SingleConsumer::new(
            build_parallel_stage_patches(
                graph,
                tasks,
                precompute,
                executor
                    .parallel_policy()
                    .expect("parallel policy should exist"),
                comparator_resolver,
            )?,
        ))),
        StageExecutor::StagedParallelPrecompute { .. } => Ok(StageExecutionData::Prepared(
            SingleConsumer::new(precompute_stage_parallel(
                graph,
                tasks,
                precompute,
                executor
                    .parallel_policy()
                    .expect("parallel policy should exist"),
                comparator_resolver,
            )?),
        )),
        StageExecutor::Serial => {
            dispatch_stage_precompute_serial(graph, tasks, precompute, comparator_resolver)
        }
    }
}
