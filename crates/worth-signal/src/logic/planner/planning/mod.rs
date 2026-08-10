pub(crate) mod validation;

mod admission;
mod evidence;
mod stage_formation;
mod topology;

use crate::data::comparator::{
    ComparatorPolicyResolver, DefaultComparatorPolicyResolver, DefaultComparatorResolver,
    VersionComparatorPolicy,
};
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::graph::TraversalScratch;
use crate::data::handle::NodeId;
use crate::logic::evaluation::EvaluationRequestMode;

use super::types::{EvaluationCursor, EvaluationPlan, SessionScratch};

pub fn build_evaluation_plan(
    graph: &mut SignalGraph,
    targets: &[NodeId],
    request_mode: EvaluationRequestMode,
) -> Result<EvaluationPlan, SignalError> {
    let mut comparator = DefaultComparatorResolver;
    let mut resolver = DefaultComparatorPolicyResolver {
        fallback: VersionComparatorPolicy::Exact,
        custom: &mut comparator,
    };
    build_evaluation_plan_with_policy_resolver(graph, targets, request_mode, &mut resolver)
}

pub fn build_evaluation_plan_with_policy_resolver(
    graph: &mut SignalGraph,
    targets: &[NodeId],
    request_mode: EvaluationRequestMode,
    resolver: &mut impl ComparatorPolicyResolver,
) -> Result<EvaluationPlan, SignalError> {
    let cursor =
        build_evaluation_cursor_with_policy_resolver(graph, targets, request_mode, resolver)?;
    Ok(evidence::materialize_plan_from_cursor(cursor))
}

pub(crate) fn build_evaluation_cursor_with_policy_resolver(
    graph: &mut SignalGraph,
    targets: &[NodeId],
    request_mode: EvaluationRequestMode,
    resolver: &mut impl ComparatorPolicyResolver,
) -> Result<EvaluationCursor, SignalError> {
    let mut deduped_targets = Vec::new();
    let mut flat_tasks = Vec::new();
    let mut stages = Vec::new();
    let summary = stage_formation::populate_plan_buffers(
        graph,
        targets,
        request_mode,
        resolver,
        &mut deduped_targets,
        &mut flat_tasks,
        &mut stages,
    )?;

    Ok(EvaluationCursor {
        request_mode,
        targets: deduped_targets,
        tasks: flat_tasks,
        stages,
        summary,
    })
}

pub(crate) fn build_evaluation_session_with_policy_resolver<'a>(
    graph: &mut SignalGraph,
    scratch: &'a mut TraversalScratch,
    targets: &[NodeId],
    request_mode: EvaluationRequestMode,
    resolver: &mut impl ComparatorPolicyResolver,
) -> Result<SessionScratch<'a>, SignalError> {
    let summary = stage_formation::populate_plan_buffers(
        graph,
        targets,
        request_mode,
        resolver,
        &mut scratch.planner_targets,
        &mut scratch.planner_tasks,
        &mut scratch.planner_stages,
    )?;

    Ok(SessionScratch {
        targets: &scratch.planner_targets,
        tasks: &scratch.planner_tasks,
        stages: &scratch.planner_stages,
        summary,
    })
}

pub(crate) use admission::admit_direct_task_with_policy_resolver;
#[cfg(test)]
pub(crate) use validation::partition_scope_untouched;
