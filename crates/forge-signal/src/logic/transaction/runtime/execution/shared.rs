use crate::data::comparator::TierPolicyResolver;
use crate::data::error::SignalError;
use crate::data::graph::{EvaluationStrategy, GcPressure, ScratchLeaseKind, SignalGraph};
use crate::data::handle::NodeId;
use crate::data::telemetry::RuntimeTelemetry;
use crate::logic::context::EvaluationContext;
use crate::logic::evaluation::IntoEvaluationOutput;
use crate::logic::evaluation::EvaluationRequestMode;
use crate::logic::planner::{
    build_evaluation_session_with_policy_resolver, execute_evaluation_session_with_policy,
    execute_prepared_plan_with_policy, EvaluationPlan, ExecutionReport, PlanSummary,
    StageExecutor,
};
use crate::logic::prepared::{ExecutionReadView, PreparedEvaluation};

use super::super::config::SignalRuntimeConfig;

pub(super) struct SessionExecutionError {
    pub error: SignalError,
    pub plan_summary: PlanSummary,
}

pub(super) fn absorb_execution_report_telemetry(
    telemetry: &mut RuntimeTelemetry,
    report: &ExecutionReport,
) {
    let mut maybe_stale_validation_tasks = 0_u64;
    let mut stage_execution_nanos = 0_u128;
    let mut max_tasks_in_stage = 0_u64;
    #[cfg(feature = "parallel")]
    let mut parallel_stages = 0_u64;

    for stage in &report.stages {
        stage_execution_nanos += stage.duration_nanos;
        max_tasks_in_stage = max_tasks_in_stage.max(stage.task_records.len() as u64);
        #[cfg(feature = "parallel")]
        if matches!(
            stage.outcome,
            crate::logic::planner::StageExecutionOutcome::CompletedParallel
        ) {
            parallel_stages += 1;
        }
        for record in &stage.task_records {
            if matches!(
                record.scheduled_reason,
                crate::logic::planner::TaskReason::MaybeStaleValidation
            ) {
                maybe_stale_validation_tasks += 1;
            }
        }
    }

    telemetry.planner.plans_built += 1;
    telemetry.planner.stages_built += report.stage_count as u64;
    telemetry.planner.tasks_scheduled += report.task_count as u64;
    telemetry.planner.tasks_pruned_before_execution += report.tasks_pruned as u64;
    telemetry.planner.maybe_stale_validation_tasks += maybe_stale_validation_tasks;
    telemetry.execution.stage_execution_count += report.stage_count as u64;
    telemetry.execution.stage_execution_nanos += stage_execution_nanos;
    telemetry.execution.execution_snapshots_built += report.execution_snapshots_built as u64;
    telemetry.execution.prepared_evaluations_produced += report.prepared_evaluations_produced as u64;
    telemetry.execution.prepared_evaluations_applied += report.prepared_evaluations_applied as u64;
    telemetry.execution.dependency_capture_updates += report.dependency_capture_updates as u64;
    telemetry.execution.execution_snapshot_nanos += report.execution_snapshot_nanos;
    telemetry.execution.stage_precompute_nanos += report.stage_precompute_nanos;
    telemetry.execution.stage_apply_nanos += report.stage_apply_nanos;
    #[cfg(not(feature = "parallel"))]
    let parallel_stages = 0_u64;
    if parallel_stages > 0 {
        telemetry.execution.parallel_executor_usage_count += 1;
        telemetry.execution.parallel_stage_dispatch_count += parallel_stages;
        telemetry.execution.parallel_precompute_task_count += report.task_count as u64;
    } else {
        telemetry.execution.serial_executor_usage_count += 1;
        telemetry.execution.serial_precompute_task_count += report.task_count as u64;
    }
    telemetry.execution.max_tasks_in_stage =
        telemetry.execution.max_tasks_in_stage.max(max_tasks_in_stage);
}

pub(super) fn executor_for_strategy(strategy: EvaluationStrategy) -> StageExecutor {
    strategy.parallelism.stage_executor()
}

pub(super) fn apply_strategy_maintenance(graph: &mut SignalGraph, strategy: EvaluationStrategy) {
    if matches!(strategy.gc_pressure, GcPressure::CompactAfterEvaluation) {
        graph.run_gc_epoch();
    }
}

fn prepare_with_context<Ctx, F, O>(
    graph: &SignalGraph,
    domain_ctx: &Ctx,
    node: NodeId,
    evaluator: &F,
) -> Result<PreparedEvaluation, SignalError>
where
    F: for<'ctx> Fn(&mut EvaluationContext<'ctx, Ctx>) -> Result<O, SignalError> + Sync,
    O: IntoEvaluationOutput,
{
    let mut eval_ctx = EvaluationContext::new(graph, node, domain_ctx);
    let output = evaluator(&mut eval_ctx)?;
    Ok(eval_ctx.into_prepared(output))
}

pub(super) fn execute_targets_with_runtime_config<T, Ctx, F, O>(
    graph: &mut SignalGraph,
    config: &SignalRuntimeConfig<T>,
    domain_ctx: &Ctx,
    targets: &[NodeId],
    request_mode: EvaluationRequestMode,
    evaluator: &F,
    executor: StageExecutor,
) -> Result<ExecutionReport, SignalError>
where
    T: Copy + Ord,
    Ctx: Sync,
    F: for<'ctx> Fn(&mut EvaluationContext<'ctx, Ctx>) -> Result<O, SignalError> + Sync,
    O: IntoEvaluationOutput,
{
    execute_targets_with_runtime_config_detailed(
        graph,
        config,
        domain_ctx,
        targets,
        request_mode,
        evaluator,
        executor,
    )
    .map_err(|failure| failure.error)
}

pub(super) fn execute_targets_with_prepared_runtime_config_detailed<T, F>(
    graph: &mut SignalGraph,
    config: &SignalRuntimeConfig<T>,
    targets: &[NodeId],
    request_mode: EvaluationRequestMode,
    precompute: &F,
    executor: StageExecutor,
) -> Result<ExecutionReport, SessionExecutionError>
where
    T: Copy + Ord,
    F: Fn(NodeId, &ExecutionReadView<'_>) -> Result<PreparedEvaluation, SignalError> + Sync,
{
    let mut resolver = TierPolicyResolver::new(
        config.node_meta(),
        config.tier_policies(),
        config.fallback_comparator(),
    );
    let mut session_summary = PlanSummary::default();
    Ok(graph.with_scratch(ScratchLeaseKind::Evaluation, |graph, scratch| {
        let session = build_evaluation_session_with_policy_resolver(
            graph,
            scratch,
            targets,
            request_mode,
            &mut resolver,
        )?;
        session_summary = session.summary.clone();
        execute_evaluation_session_with_policy(graph, &session, precompute, &mut resolver, executor)
    })
    .map_err(|error| SessionExecutionError {
        error,
        plan_summary: session_summary,
    })?)
}

pub(super) fn execute_targets_with_runtime_config_detailed<T, Ctx, F, O>(
    graph: &mut SignalGraph,
    config: &SignalRuntimeConfig<T>,
    domain_ctx: &Ctx,
    targets: &[NodeId],
    request_mode: EvaluationRequestMode,
    evaluator: &F,
    executor: StageExecutor,
) -> Result<ExecutionReport, SessionExecutionError>
where
    T: Copy + Ord,
    Ctx: Sync,
    F: for<'ctx> Fn(&mut EvaluationContext<'ctx, Ctx>) -> Result<O, SignalError> + Sync,
    O: IntoEvaluationOutput,
{
    let mut resolver = TierPolicyResolver::new(
        config.node_meta(),
        config.tier_policies(),
        config.fallback_comparator(),
    );
    let mut session_summary = PlanSummary::default();
    Ok(graph.with_scratch(ScratchLeaseKind::Evaluation, |graph, scratch| {
        let session = build_evaluation_session_with_policy_resolver(
            graph,
            scratch,
            targets,
            request_mode,
            &mut resolver,
        )?;
        session_summary = session.summary.clone();
        execute_evaluation_session_with_policy(
            graph,
            &session,
            &|node, view: &ExecutionReadView<'_>| {
                prepare_with_context(view.graph(), domain_ctx, node, evaluator)
            },
            &mut resolver,
            executor,
        )
    })
    .map_err(|error| SessionExecutionError {
        error,
        plan_summary: session_summary,
    })?)
}

pub(super) fn execute_plan_with_runtime_config<T, Ctx, F, O>(
    graph: &mut SignalGraph,
    config: &SignalRuntimeConfig<T>,
    domain_ctx: &Ctx,
    plan: &EvaluationPlan,
    evaluator: &F,
    executor: StageExecutor,
) -> Result<ExecutionReport, SignalError>
where
    T: Copy + Ord,
    Ctx: Sync,
    F: for<'ctx> Fn(&mut EvaluationContext<'ctx, Ctx>) -> Result<O, SignalError> + Sync,
    O: IntoEvaluationOutput,
{
    let mut resolver = TierPolicyResolver::new(
        config.node_meta(),
        config.tier_policies(),
        config.fallback_comparator(),
    );
    execute_prepared_plan_with_policy(
        graph,
        plan,
        domain_ctx,
        evaluator,
        &mut resolver,
        executor,
    )
}
