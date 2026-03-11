use crate::data::comparator::ComparatorPolicyResolver;
use crate::data::error::SignalError;
use crate::logic::prepared::ExecutionSnapshot;

#[cfg(feature = "parallel")]
use super::precompute::{build_parallel_stage_patches, precompute_stage_parallel};
use super::precompute::{precompute_stage_serial, StageExecutionData};
#[cfg(feature = "parallel")]
use super::stage_admission::StageParallelAdmission;
use super::types::{EvaluationTask, StageExecutor};

pub(crate) fn dispatch_stage_precompute(
    tasks: &[EvaluationTask],
    snapshot: &ExecutionSnapshot<'_>,
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
            tasks,
            snapshot,
            precompute,
            comparator_resolver,
        )?)),
        #[cfg(feature = "parallel")]
        _ if parallel_admission.use_parallel => {
            if executor.is_full_parallel() {
                Ok(StageExecutionData::Patched(build_parallel_stage_patches(
                    tasks,
                    snapshot,
                    precompute,
                    executor
                        .parallel_policy()
                        .expect("parallel policy should exist"),
                    comparator_resolver,
                )?))
            } else {
                Ok(StageExecutionData::Prepared(precompute_stage_parallel(
                    tasks,
                    snapshot,
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
                tasks,
                snapshot,
                precompute,
                comparator_resolver,
            )?))
        }
    }
}
