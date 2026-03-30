use crate::clock::RuntimeInstant;
use crate::data::comparator::ComparatorPolicyResolver;
use crate::data::error::SignalError;
use crate::data::handle::NodeId;
use crate::logic::prepared::PreparedEvaluation;

use super::super::precompute_reporting::record_stage_precompute_report;
use super::super::semantic::reserve_stage_identities;
use super::super::stage_apply::apply_stage;
use super::super::stage_precompute::perform_stage_precompute;
use super::super::stage_recording::begin_stage_record;
use super::context::ExecutionContext;
use super::reporting::record_stage_execution_completion;
use super::StageSlice;

struct StagePreparedPass {
    precomputed: crate::logic::planner::precompute::stage::StagePrecomputeResult,
    snapshot_nanos: u128,
    precompute_nanos: u128,
}

struct StageAppliedPass {
    stage_record: crate::logic::planner::types::StageExecutionRecord,
    apply_elapsed_nanos: u128,
    stage_elapsed_nanos: u128,
}

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
    let stage_start = RuntimeInstant::now();
    let prepared_pass = run_stage_precompute_pass(ctx, stage)?;
    let applied_pass = run_stage_apply_pass(ctx, stage, prepared_pass, stage_start)?;
    complete_stage_reporting_pass(ctx, applied_pass);
    Ok(())
}

fn run_stage_precompute_pass<F, R>(
    ctx: &mut ExecutionContext<'_, F, R>,
    stage: &StageSlice<'_>,
) -> Result<StagePreparedPass, SignalError>
where
    F: Fn(
            NodeId,
            &crate::logic::prepared::ExecutionReadView<'_>,
        ) -> Result<PreparedEvaluation, SignalError>
        + Sync,
    R: ComparatorPolicyResolver,
{
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
    Ok(StagePreparedPass {
        snapshot_nanos: precomputed.snapshot_nanos,
        precompute_nanos: precomputed.precompute_nanos,
        precomputed,
    })
}

fn run_stage_apply_pass<F, R>(
    ctx: &mut ExecutionContext<'_, F, R>,
    stage: &StageSlice<'_>,
    prepared_pass: StagePreparedPass,
    stage_start: RuntimeInstant,
) -> Result<StageAppliedPass, SignalError>
where
    F: Fn(
            NodeId,
            &crate::logic::prepared::ExecutionReadView<'_>,
        ) -> Result<PreparedEvaluation, SignalError>
        + Sync,
    R: ComparatorPolicyResolver,
{
    let apply_start = RuntimeInstant::now();
    let mut stage_record = begin_stage_record(
        stage.index,
        prepared_pass.snapshot_nanos,
        prepared_pass.precompute_nanos,
        #[cfg(feature = "parallel")]
        prepared_pass.precomputed.parallel_admission,
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
        prepared_pass.precomputed,
        ctx.comparator_resolver,
        ctx.executor,
        &stage_identities,
        &mut ctx.report,
        &mut stage_record,
    )?;
    Ok(StageAppliedPass {
        stage_record,
        apply_elapsed_nanos: apply_start.elapsed().as_nanos(),
        stage_elapsed_nanos: stage_start.elapsed().as_nanos(),
    })
}

fn complete_stage_reporting_pass<F, R>(
    ctx: &mut ExecutionContext<'_, F, R>,
    applied_pass: StageAppliedPass,
) where
    F: Fn(
            NodeId,
            &crate::logic::prepared::ExecutionReadView<'_>,
        ) -> Result<PreparedEvaluation, SignalError>
        + Sync,
    R: ComparatorPolicyResolver,
{
    record_stage_execution_completion(
        ctx.graph,
        &mut ctx.report,
        applied_pass.stage_record,
        applied_pass.apply_elapsed_nanos,
        applied_pass.stage_elapsed_nanos,
    );
}
