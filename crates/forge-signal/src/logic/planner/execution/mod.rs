use crate::data::comparator::ComparatorPolicyResolver;
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::logic::context::EvaluationContext;
use crate::logic::evaluation::IntoEvaluationOutput;
use crate::logic::prepared::PreparedEvaluation;

use self::context::ExecutionContext;
use self::diagnostics::{summarize_recorded_plan, summarize_recorded_session};
use self::stage::execute_stage;
use super::types::{
    EvaluationPlan, EvaluationTask, ExecutionReport, PlanSummary, SessionScratch, StageExecutor,
};

mod context;
pub(crate) mod diagnostics;
mod reporting;
mod stage;
pub(crate) mod task_reporting;

#[derive(Clone, Copy)]
pub(crate) struct StageSlice<'a> {
    pub(crate) index: u32,
    pub(crate) tasks: &'a [EvaluationTask],
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

pub fn execute_prepared_plan<Ctx, F, O>(
    graph: &mut SignalGraph,
    plan: &EvaluationPlan,
    domain_ctx: &Ctx,
    evaluator: &F,
) -> Result<ExecutionReport, SignalError>
where
    Ctx: Sync,
    F: for<'ctx> Fn(&mut EvaluationContext<'ctx, Ctx>) -> Result<O, SignalError> + Sync,
    O: IntoEvaluationOutput,
{
    let mut comparator = crate::data::comparator::DefaultComparatorResolver;
    let mut resolver = crate::data::comparator::DefaultComparatorPolicyResolver {
        fallback: crate::data::comparator::VersionComparatorPolicy::Exact,
        custom: &mut comparator,
    };
    execute_prepared_plan_with_policy(
        graph,
        plan,
        domain_ctx,
        evaluator,
        &mut resolver,
        StageExecutor::Serial,
    )
}

pub(crate) fn execute_prepared_plan_with_precompute<F>(
    graph: &mut SignalGraph,
    plan: &EvaluationPlan,
    precompute: &F,
    comparator_resolver: &mut impl ComparatorPolicyResolver,
    executor: StageExecutor,
) -> Result<ExecutionReport, SignalError>
where
    F: Fn(
            NodeId,
            &crate::logic::prepared::ExecutionReadView<'_>,
        ) -> Result<PreparedEvaluation, SignalError>
        + Sync,
{
    let profile = graph.diagnostics_profile();
    let (plan_summary, first_target) = summarize_recorded_plan(plan, profile);
    let maybe_stale_validation_tasks = plan
        .stages
        .iter()
        .flat_map(|stage| stage.tasks.iter())
        .filter(|task| matches!(task.reason, super::types::TaskReason::MaybeStaleValidation))
        .count() as u64;
    execute_plan_stage_slices_with_policy(
        graph,
        &plan.summary,
        plan.stages.len(),
        maybe_stale_validation_tasks,
        plan.stages.iter().map(|stage| StageSlice {
            index: stage.index,
            tasks: &stage.tasks,
        }),
        precompute,
        plan_summary,
        first_target,
        comparator_resolver,
        executor,
    )
}

pub fn execute_prepared_plan_with_policy<Ctx, F, O>(
    graph: &mut SignalGraph,
    plan: &EvaluationPlan,
    domain_ctx: &Ctx,
    evaluator: &F,
    comparator_resolver: &mut impl ComparatorPolicyResolver,
    executor: StageExecutor,
) -> Result<ExecutionReport, SignalError>
where
    Ctx: Sync,
    F: for<'ctx> Fn(&mut EvaluationContext<'ctx, Ctx>) -> Result<O, SignalError> + Sync,
    O: IntoEvaluationOutput,
{
    let profile = graph.diagnostics_profile();
    let (plan_summary, first_target) = summarize_recorded_plan(plan, profile);
    let maybe_stale_validation_tasks = plan
        .stages
        .iter()
        .flat_map(|stage| stage.tasks.iter())
        .filter(|task| matches!(task.reason, super::types::TaskReason::MaybeStaleValidation))
        .count() as u64;
    execute_plan_stage_slices_with_policy(
        graph,
        &plan.summary,
        plan.stages.len(),
        maybe_stale_validation_tasks,
        plan.stages.iter().map(|stage| StageSlice {
            index: stage.index,
            tasks: &stage.tasks,
        }),
        &|node, view: &crate::logic::prepared::ExecutionReadView<'_>| {
            prepare_with_context(view.graph(), domain_ctx, node, evaluator)
        },
        plan_summary,
        first_target,
        comparator_resolver,
        executor,
    )
}

pub(crate) fn execute_evaluation_session_with_policy<F>(
    graph: &mut SignalGraph,
    session: &SessionScratch<'_>,
    precompute: &F,
    comparator_resolver: &mut impl ComparatorPolicyResolver,
    executor: StageExecutor,
) -> Result<ExecutionReport, SignalError>
where
    F: Fn(
            NodeId,
            &crate::logic::prepared::ExecutionReadView<'_>,
        ) -> Result<PreparedEvaluation, SignalError>
        + Sync,
{
    let profile = graph.diagnostics_profile();
    let (plan_summary, first_target) = summarize_recorded_session(session, profile);
    let maybe_stale_validation_tasks = session
        .tasks
        .iter()
        .filter(|task| matches!(task.reason, super::types::TaskReason::MaybeStaleValidation))
        .count() as u64;
    let report = execute_plan_stage_slices_with_policy(
        graph,
        &session.summary,
        session.stages.len(),
        maybe_stale_validation_tasks,
        session.stages.iter().map(|stage| StageSlice {
            index: stage.index,
            tasks: &session.tasks[stage.start..stage.end],
        }),
        precompute,
        plan_summary,
        first_target,
        comparator_resolver,
        executor,
    )?;
    Ok(report)
}

fn execute_plan_stage_slices_with_policy<'a, F>(
    graph: &mut SignalGraph,
    summary: &PlanSummary,
    stage_count: usize,
    maybe_stale_validation_tasks: u64,
    stages: impl IntoIterator<Item = StageSlice<'a>>,
    precompute: &F,
    plan_summary: crate::diagnostics::summary::EvaluationPlanSummary,
    first_target: Option<NodeId>,
    comparator_resolver: &mut impl ComparatorPolicyResolver,
    executor: StageExecutor,
) -> Result<ExecutionReport, SignalError>
where
    F: Fn(
            NodeId,
            &crate::logic::prepared::ExecutionReadView<'_>,
        ) -> Result<PreparedEvaluation, SignalError>
        + Sync,
{
    let mut context = ExecutionContext::new(
        graph,
        summary,
        stage_count,
        maybe_stale_validation_tasks,
        plan_summary,
        first_target,
        precompute,
        comparator_resolver,
        executor,
    );
    for stage in stages {
        execute_stage(&mut context, &stage)?;
    }
    Ok(context.finish())
}
