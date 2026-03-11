use crate::data::comparator::ComparatorPolicyResolver;
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::logic::prepared::PreparedEvaluation;

use super::execution_context::ExecutionContext;
use super::execution_diagnostics::RecordedPlan;
use super::stage_execution::execute_stage;
use super::types::{
    EvaluationPlan, EvaluationSession, EvaluationTask, ExecutionReport, PlanSummary, StageExecutor,
};

#[derive(Clone, Copy)]
pub(crate) struct StageSlice<'a> {
    pub(crate) index: u32,
    pub(crate) tasks: &'a [EvaluationTask],
}

pub fn execute_prepared_plan<F>(
    graph: &mut SignalGraph,
    plan: &EvaluationPlan,
    precompute: &F,
) -> Result<ExecutionReport, SignalError>
where
    F: Fn(
            NodeId,
            &crate::logic::prepared::ExecutionReadView<'_>,
        ) -> Result<PreparedEvaluation, SignalError>
        + Sync,
{
    let mut comparator = crate::data::comparator::DefaultComparatorResolver;
    let mut resolver = crate::data::comparator::DefaultComparatorPolicyResolver {
        fallback: crate::data::comparator::VersionComparatorPolicy::Exact,
        custom: &mut comparator,
    };
    execute_prepared_plan_with_policy(
        graph,
        plan,
        precompute,
        &mut resolver,
        StageExecutor::Serial,
    )
}

pub fn execute_prepared_plan_with_policy<F>(
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
    let recorded_plan = RecordedPlan::from_plan(plan, profile);
    let maybe_stale_validation_tasks = plan
        .stages
        .iter()
        .flat_map(|stage| stage.tasks.iter())
        .filter(|task| matches!(task.reason, super::types::TaskReason::MaybeStaleValidation))
        .count() as u64;
    let report = execute_plan_stage_slices_with_policy(
        graph,
        &plan.summary,
        plan.stages.len(),
        maybe_stale_validation_tasks,
        plan.stages
            .iter()
            .map(|stage| StageSlice {
                index: stage.index,
                tasks: &stage.tasks,
        }),
        precompute,
        recorded_plan,
        comparator_resolver,
        executor,
    )?;
    Ok(report)
}

pub(crate) fn execute_evaluation_session_with_policy<F>(
    graph: &mut SignalGraph,
    session: &EvaluationSession<'_>,
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
    let recorded_plan = RecordedPlan::from_session(session, profile);
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
        recorded_plan,
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
    recorded_plan: RecordedPlan,
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
        recorded_plan,
        precompute,
        comparator_resolver,
        executor,
    );
    for stage in stages {
        execute_stage(&mut context, &stage)?;
    }
    Ok(context.finish())
}
