mod concurrent;
#[cfg(feature = "parallel")]
mod concurrent_packets;
mod footprint;
mod lowering;
mod metrics;
mod strategy;

use crate::clock::RuntimeInstant;
use crate::data::comparator::ComparatorPolicyResolver;
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::logic::planner::execution::StageSlice;
use crate::logic::planner::precompute::stage::StagePrecomputeResult;
use crate::logic::planner::semantic::{finalize_serial_stage_batch, StageSemanticIdentity};
use crate::logic::planner::types::{
    ExecutionReport, PlanSummary, StageExecutionRecord, StageExecutor,
};

use super::serial_batch::{LoweredSerialStage, PreparedSerialStageBatch};
use super::workspace::{StageFinalizeWork, StageScratch};

pub(in crate::logic::planner) fn apply_stage<R>(
    graph: &mut SignalGraph,
    summary: &PlanSummary,
    stage: &StageSlice<'_>,
    precomputed: StagePrecomputeResult,
    comparator_resolver: &mut R,
    executor: StageExecutor,
    stage_identities: &[StageSemanticIdentity],
    report: &mut ExecutionReport,
    stage_record: &mut StageExecutionRecord,
) -> Result<(), SignalError>
where
    R: ComparatorPolicyResolver,
{
    let lowered = lowering::build_stage_execution_form(
        graph,
        stage.index,
        stage.tasks,
        precomputed.execution,
        comparator_resolver,
        executor,
        stage_identities,
    )?;
    metrics::record_stage_lowering_metrics(graph, &lowered);
    let stage_scratch = run_lowered_apply_pass(
        graph,
        summary,
        lowered,
        comparator_resolver,
        executor,
        stage_identities,
        stage_record,
    )?;
    let (finalize_work, pending_snapshots) = stage_scratch.into_parts();
    publish_pending_snapshots(graph, pending_snapshots)?;
    finalize_stage_results(graph, stage, finalize_work, report, stage_record)
}

fn publish_pending_snapshots(
    graph: &mut SignalGraph,
    pending_snapshots: crate::data::proof::ClassifiedSnapshotBatchCommit,
) -> Result<(), SignalError> {
    if pending_snapshots.is_empty() {
        return Ok(());
    }
    graph.apply_classified_snapshot_batch_commit(pending_snapshots)
}

fn finalize_stage_results(
    graph: &mut SignalGraph,
    _stage: &StageSlice<'_>,
    finalize_work: StageFinalizeWork,
    report: &mut ExecutionReport,
    stage_record: &mut StageExecutionRecord,
) -> Result<(), SignalError> {
    let semantic_finalize_start = RuntimeInstant::now();
    match finalize_work {
        StageFinalizeWork::Serial(batch) => {
            let ready = batch.into_ready_for_finalize()?;
            finalize_serial_stage_batch(graph, ready, report, stage_record)?
                .record_into(report, stage_record);
        }
        #[cfg(feature = "parallel")]
        StageFinalizeWork::Parallel(batch) => {
            crate::logic::planner::semantic::finalize_stage_batch(
                graph,
                _stage.tasks,
                batch.into_inner(),
                report,
                stage_record,
            )?;
        }
    }
    stage_record.semantic_finalize_duration_nanos = semantic_finalize_start.elapsed().as_nanos();
    Ok(())
}

fn run_lowered_apply_pass<R>(
    graph: &mut SignalGraph,
    summary: &PlanSummary,
    lowered: lowering::LoweredStageExecutionForm,
    comparator_resolver: &mut R,
    executor: StageExecutor,
    stage_identities: &[StageSemanticIdentity],
    stage_record: &mut StageExecutionRecord,
) -> Result<StageScratch, SignalError>
where
    R: ComparatorPolicyResolver,
{
    match lowered {
        lowering::LoweredStageExecutionForm::Serial(lowered) => {
            stage_record.authority_policy = Some(lowered.authority_policy());
            run_serial_lowered_apply_pass(
                graph,
                summary,
                lowered,
                comparator_resolver,
                executor,
                stage_record,
            )
        }
        lowering::LoweredStageExecutionForm::Generic(lowered) => {
            lowering::validate_lowered_stage_plan(&lowered);
            stage_record.authority_policy = Some(lowered.authority_policy());
            let (stage_index, tasks, lowered_apply_plan, ..) = lowered.into_parts();
            let crate::logic::planner::types::LoweredApplyPlan::GroupedConcurrent(plan) =
                lowered_apply_plan
            else {
                return Err(SignalError::internal(
                    "generic stage dispatch received a serial apply plan after serial lowering",
                ));
            };
            concurrent::run_grouped_concurrent_apply_pass(
                graph,
                summary,
                stage_index,
                tasks,
                plan,
                comparator_resolver,
                stage_identities,
                stage_record,
            )
        }
    }
}

fn run_serial_lowered_apply_pass<R>(
    graph: &mut SignalGraph,
    summary: &PlanSummary,
    lowered: LoweredSerialStage,
    comparator_resolver: &mut R,
    executor: StageExecutor,
    stage_record: &mut StageExecutionRecord,
) -> Result<StageScratch, SignalError>
where
    R: ComparatorPolicyResolver,
{
    let prepared = PreparedSerialStageBatch::prepare(graph, lowered, stage_record)?;
    let applied = prepared.apply(graph, summary, comparator_resolver, executor)?;
    let (applied, pending_snapshots) = applied.split_pending_snapshots();
    Ok(StageScratch::new(
        StageFinalizeWork::Serial(applied),
        pending_snapshots,
    ))
}
