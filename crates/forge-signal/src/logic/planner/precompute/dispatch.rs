use crate::data::comparator::ComparatorPolicyResolver;
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;

#[cfg(feature = "parallel")]
use super::{build_parallel_stage_patches, precompute_stage_parallel};
use super::{precompute_stage_serial, StageExecutionData};
#[cfg(feature = "parallel")]
use super::admission::StageParallelAdmission;
use super::super::types::{EvaluationTask, StageExecutor};

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
    match executor {
        StageExecutor::Serial => Ok(StageExecutionData::Prepared(precompute_stage_serial(
            graph,
            tasks,
            precompute,
            comparator_resolver,
        )?)),
        #[cfg(feature = "parallel")]
        _ if parallel_admission.use_parallel => {
            if executor.is_full_parallel() {
                Ok(StageExecutionData::Patched(build_parallel_stage_patches(
                    graph,
                    tasks,
                    precompute,
                    executor
                        .parallel_policy()
                        .expect("parallel policy should exist"),
                    comparator_resolver,
                )?))
            } else {
                Ok(StageExecutionData::Prepared(precompute_stage_parallel(
                    graph,
                    tasks,
                    precompute,
                    executor
                        .parallel_policy()
                        .expect("parallel policy should exist"),
                    comparator_resolver,
                )?))
            }
        }
        #[cfg(feature = "parallel")]
        StageExecutor::StagedParallelPrecompute { .. } | StageExecutor::FullParallel { .. } => {
            Ok(StageExecutionData::Prepared(precompute_stage_serial(
                graph,
                tasks,
                precompute,
                comparator_resolver,
            )?))
        }
    }
}
