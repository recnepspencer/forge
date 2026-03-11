use std::time::Instant;

use crate::data::comparator::ComparatorPolicyResolver;
use crate::data::error::SignalError;
use crate::data::handle::NodeId;
use crate::logic::prepared::PreparedEvaluation;

use super::execution::StageSlice;
use super::execution_context::ExecutionContext;
use super::execution_reporting::record_stage_execution_completion;
use super::precompute_reporting::record_stage_precompute_report;
use super::stage_apply::apply_stage;
use super::stage_recording::begin_stage_record;
use super::semantic::reserve_stage_identities;
use super::stage_precompute::perform_stage_precompute;

pub(crate) fn execute_stage<F, R>(
    ctx: &mut ExecutionContext<'_, F, R>,
    stage: &StageSlice<'_>,
) -> Result<(), SignalError>
where
    F: Fn(
            NodeId,
            &crate::logic::prepared::ExecutionReadView<'_>,
        ) -> Result<PreparedEvaluation, SignalError>
        + Sync,
    R: ComparatorPolicyResolver,
{
    let stage_start = Instant::now();
    let precomputed = perform_stage_precompute(
        ctx.graph,
        ctx.summary,
        stage,
        ctx.precompute,
        ctx.comparator_resolver,
        ctx.executor,
    )?;
    record_stage_precompute_report(
        &mut ctx.report,
        &precomputed.execution,
        precomputed.snapshot_nanos,
        precomputed.precompute_nanos,
    );

    let apply_start = Instant::now();
    let mut stage_record = begin_stage_record(
        stage.index,
        precomputed.snapshot_nanos,
        precomputed.precompute_nanos,
        #[cfg(feature = "parallel")]
        precomputed.parallel_admission,
    );
    let stage_identities = reserve_stage_identities(
        &mut ctx.next_record_id,
        &mut ctx.next_segment_id,
        stage.tasks.len(),
    );

    apply_stage::<F, R>(
        ctx.graph,
        ctx.summary,
        stage,
        precomputed,
        ctx.comparator_resolver,
        ctx.executor,
        &stage_identities,
        &mut ctx.report,
        &mut stage_record,
    )?;

    record_stage_execution_completion(
        ctx.graph,
        &mut ctx.report,
        stage_record,
        apply_start.elapsed().as_nanos(),
        stage_start.elapsed().as_nanos(),
    );
    Ok(())
}
