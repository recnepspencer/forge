use std::collections::{BTreeMap, BTreeSet};

use crate::data::comparator::{
    ComparatorPolicyResolver, DefaultComparatorPolicyResolver, DefaultComparatorResolver,
    VersionComparatorPolicy,
};
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::node::NodeState;
use crate::logic::evaluation::EvaluationRequestMode;

use super::types::{
    EvaluationPlan, EvaluationTask, ExecutionStage, PlanSummary, StageBarrier, TaskReason,
};
use super::validation::{preview_maybe_stale, sorted_dependencies};

pub fn build_evaluation_plan(
    graph: &SignalGraph,
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
    graph: &SignalGraph,
    targets: &[NodeId],
    request_mode: EvaluationRequestMode,
    resolver: &mut impl ComparatorPolicyResolver,
) -> Result<EvaluationPlan, SignalError> {
    let mut planned = BTreeMap::<NodeId, PlannedNode>::new();
    let mut visiting = BTreeSet::<NodeId>::new();

    let mut deduped_targets = targets.to_vec();
    deduped_targets.sort_by_key(|node| node_sort_key(*node));
    deduped_targets.dedup();

    for &target in &deduped_targets {
        graph.get_entry(target)?;
        visit_node(
            graph,
            target,
            request_mode,
            true,
            TaskReason::RequestedTarget,
            resolver,
            &mut visiting,
            &mut planned,
        )?;
    }

    let depth_cache = compute_depths(graph, &planned)?;
    let max_depth = depth_cache.values().copied().max().unwrap_or(0) as usize;
    let mut stages_by_depth = vec![Vec::<EvaluationTask>::new(); max_depth + 1];
    for (&node, planned_node) in &planned {
        let reason = classify_reason(graph, node, planned_node.direct_request, request_mode)?;
        let task = EvaluationTask {
            node,
            request_mode,
            direct_request: planned_node.direct_request,
            reason,
        };
        let depth = *depth_cache.get(&node).unwrap_or(&0) as usize;
        stages_by_depth[depth].push(task);
    }

    let mut stages = Vec::new();
    for mut tasks in stages_by_depth {
        if tasks.is_empty() {
            continue;
        }
        tasks.sort_by_key(|task| node_sort_key(task.node));
        stages.push(ExecutionStage {
            index: stages.len() as u32,
            tasks,
            barrier: Some(StageBarrier::StageBoundary),
        });
    }

    let summary = PlanSummary {
        requested_target_count: deduped_targets.len() as u32,
        stage_count: stages.len() as u32,
        task_count: stages.iter().map(|stage| stage.tasks.len() as u32).sum(),
        max_stage_width: stages
            .iter()
            .map(|stage| stage.tasks.len() as u32)
            .max()
            .unwrap_or(0),
    };

    Ok(EvaluationPlan {
        request_mode,
        targets: deduped_targets,
        stages,
        summary,
    })
}

#[derive(Debug, Clone, Copy, Default)]
struct PlannedNode {
    direct_request: bool,
}

fn visit_node(
    graph: &SignalGraph,
    node: NodeId,
    request_mode: EvaluationRequestMode,
    direct_request: bool,
    reason: TaskReason,
    resolver: &mut impl ComparatorPolicyResolver,
    visiting: &mut BTreeSet<NodeId>,
    planned: &mut BTreeMap<NodeId, PlannedNode>,
) -> Result<(), SignalError> {
    if !visiting.insert(node) {
        return Err(SignalError::invalid_input(format!(
            "cycle detected while building evaluation plan at {node}"
        )));
    }

    let state = graph.get_state(node)?;
    let should_include = matches!(state, NodeState::Dirty | NodeState::MaybeStale)
        || (direct_request && matches!(request_mode, EvaluationRequestMode::ForceOnDemand));
    if should_include {
        planned
            .entry(node)
            .and_modify(|existing| existing.direct_request |= direct_request)
            .or_insert(PlannedNode { direct_request });
    }

    match state {
        NodeState::Dirty => {
            for dependency in sorted_dependencies(graph, node)? {
                visit_node(
                    graph,
                    dependency.source(),
                    request_mode,
                    false,
                    TaskReason::DependencyRequired,
                    resolver,
                    visiting,
                    planned,
                )?;
            }
        }
        NodeState::MaybeStale => {
            let preview = preview_maybe_stale(graph, node, resolver)?;
            let upstream_reason = if matches!(reason, TaskReason::MaybeStaleValidation) {
                TaskReason::MaybeStaleValidation
            } else {
                TaskReason::DependencyRequired
            };
            for source in preview.requires_upstream_evaluation {
                visit_node(
                    graph,
                    source,
                    request_mode,
                    false,
                    upstream_reason,
                    resolver,
                    visiting,
                    planned,
                )?;
            }
        }
        NodeState::Clean
            if direct_request && matches!(request_mode, EvaluationRequestMode::ForceOnDemand) =>
        {
            for dependency in sorted_dependencies(graph, node)? {
                if !matches!(graph.get_state(dependency.source())?, NodeState::Clean) {
                    visit_node(
                        graph,
                        dependency.source(),
                        request_mode,
                        false,
                        TaskReason::DependencyRequired,
                        resolver,
                        visiting,
                        planned,
                    )?;
                }
            }
        }
        NodeState::Clean => {}
    }

    visiting.remove(&node);
    Ok(())
}

fn compute_depths(
    graph: &SignalGraph,
    planned: &BTreeMap<NodeId, PlannedNode>,
) -> Result<BTreeMap<NodeId, u32>, SignalError> {
    let planned_ids: BTreeSet<NodeId> = planned.keys().copied().collect();
    let mut indegree = BTreeMap::<NodeId, u32>::new();
    let mut outgoing = BTreeMap::<NodeId, Vec<NodeId>>::new();

    for &node in &planned_ids {
        indegree.entry(node).or_insert(0);
        outgoing.entry(node).or_default();
    }

    for &node in &planned_ids {
        for dependency in graph.dependencies_of(node)? {
            let source = dependency.source();
            if !planned_ids.contains(&source) {
                continue;
            }
            *indegree.entry(node).or_insert(0) += 1;
            outgoing.entry(source).or_default().push(node);
        }
    }

    let mut frontier = indegree
        .iter()
        .filter_map(|(&node, &degree)| (degree == 0).then_some(node))
        .collect::<Vec<_>>();
    frontier.sort_by_key(|node| node_sort_key(*node));

    let mut depths = BTreeMap::<NodeId, u32>::new();
    let mut visited = 0usize;

    while let Some(node) = frontier.pop() {
        visited += 1;
        let depth = graph
            .dependencies_of(node)?
            .iter()
            .filter(|dependency| planned_ids.contains(&dependency.source()))
            .filter_map(|dependency| depths.get(&dependency.source()).copied())
            .max()
            .map_or(0, |parent| parent + 1);
        depths.insert(node, depth);

        if let Some(children) = outgoing.get(&node) {
            let mut newly_ready = Vec::new();
            for &child in children {
                let degree = indegree
                    .get_mut(&child)
                    .ok_or_else(|| SignalError::internal("planned child missing indegree entry"))?;
                *degree = degree.saturating_sub(1);
                if *degree == 0 {
                    newly_ready.push(child);
                }
            }
            newly_ready.sort_by_key(|child| node_sort_key(*child));
            frontier.extend(newly_ready.into_iter().rev());
        }
    }

    if visited != planned_ids.len() {
        return Err(SignalError::internal(
            "planner depth computation encountered a cycle in the planned graph",
        ));
    }

    Ok(depths)
}

fn classify_reason(
    graph: &SignalGraph,
    node: NodeId,
    direct_request: bool,
    request_mode: EvaluationRequestMode,
) -> Result<TaskReason, SignalError> {
    if direct_request {
        return Ok(match request_mode {
            EvaluationRequestMode::Default => TaskReason::RequestedTarget,
            EvaluationRequestMode::ForceOnDemand => TaskReason::ConditionForced,
        });
    }

    let state = graph.get_state(node)?;
    if matches!(state, NodeState::MaybeStale) {
        return Ok(TaskReason::MaybeStaleValidation);
    }

    let entry = graph.get_entry(node)?;
    let trace = entry.get_trace_summary();
    if trace.is_some_and(|summary| {
        summary.output_change == crate::data::output::OutputChange::Unchanged
    }) {
        return Ok(TaskReason::OutputDiffDependent);
    }

    if !entry.get_dirty_partition_scopes().is_empty() {
        return Ok(TaskReason::PartitionScopedDependency);
    }

    if trace.is_some_and(|summary| {
        summary.memoized_origin == crate::data::output::MemoizedResultOrigin::MemoizedFromCache
    }) {
        return Ok(TaskReason::MemoValidation);
    }

    Ok(TaskReason::Dirty)
}

#[cfg(test)]
pub(crate) fn partition_scope_untouched(
    trace_summary: Option<&crate::data::trace::TraceSummary>,
    scope: &crate::data::output::PartitionSubscription,
) -> bool {
    trace_summary.is_none_or(|summary| {
        !summary.changed_regions.iter().any(|region| {
            if scope.partition != region.partition {
                return false;
            }
            match scope.match_mode {
                crate::data::output::PartitionMatchMode::WholePartition => true,
                crate::data::output::PartitionMatchMode::PartitionAndDetail => {
                    scope.detail == region.detail
                }
            }
        })
    })
}

pub(crate) fn node_sort_key(node: NodeId) -> (u32, u32) {
    (node.index(), node.generation())
}
