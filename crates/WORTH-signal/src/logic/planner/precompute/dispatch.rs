use crate::data::comparator::ComparatorPolicyResolver;
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::proof::SingleConsumer;

use super::super::types::{EligibleTask, StageExecutor};
#[cfg(feature = "parallel")]
use super::admission::StageParallelAdmission;
#[cfg(feature = "parallel")]
use super::{build_parallel_stage_patches, precompute_stage_parallel};
use super::{precompute_stage_serial, StageExecutionData, TemporalLoweringContext};

pub(in crate::logic::planner) fn dispatch_stage_precompute(
    graph: &mut SignalGraph,
    tasks: &[EligibleTask],
    precompute: &(impl Fn(
        crate::data::handle::NodeId,
        &crate::logic::prepared::ExecutionReadView<'_>,
    ) -> Result<crate::logic::prepared::PreparedEvaluation, SignalError>
          + Sync),
    comparator_resolver: &mut impl ComparatorPolicyResolver,
    temporal_lowering: TemporalLoweringContext,
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
            temporal_lowering,
            executor,
        );
    }

    dispatch_stage_precompute_serial(
        graph,
        tasks,
        precompute,
        comparator_resolver,
        temporal_lowering,
    )
}

fn dispatch_stage_precompute_serial(
    graph: &mut SignalGraph,
    tasks: &[EligibleTask],
    precompute: &(impl Fn(
        crate::data::handle::NodeId,
        &crate::logic::prepared::ExecutionReadView<'_>,
    ) -> Result<crate::logic::prepared::PreparedEvaluation, SignalError>
          + Sync),
    comparator_resolver: &mut impl ComparatorPolicyResolver,
    temporal_lowering: TemporalLoweringContext,
) -> Result<StageExecutionData, SignalError> {
    Ok(StageExecutionData::Prepared(SingleConsumer::new(
        precompute_stage_serial(
            graph,
            tasks,
            precompute,
            comparator_resolver,
            temporal_lowering,
        )?,
    )))
}

#[cfg(feature = "parallel")]
fn dispatch_stage_precompute_parallel(
    graph: &mut SignalGraph,
    tasks: &[EligibleTask],
    precompute: &(impl Fn(
        crate::data::handle::NodeId,
        &crate::logic::prepared::ExecutionReadView<'_>,
    ) -> Result<crate::logic::prepared::PreparedEvaluation, SignalError>
          + Sync),
    comparator_resolver: &mut impl ComparatorPolicyResolver,
    temporal_lowering: TemporalLoweringContext,
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
                temporal_lowering,
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
                temporal_lowering,
            )?),
        )),
        StageExecutor::Serial => dispatch_stage_precompute_serial(
            graph,
            tasks,
            precompute,
            comparator_resolver,
            temporal_lowering,
        ),
    }
}
