pub(crate) mod validation;

use std::collections::HashMap;

use crate::data::bitset::DenseBitset;
use crate::data::comparator::{
    ComparatorPolicyResolver, DefaultComparatorPolicyResolver, DefaultComparatorResolver,
    VersionComparatorPolicy,
};
use crate::data::error::SignalError;
use crate::data::graph::TraversalScratch;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::node::NodeState;
use crate::logic::evaluation::EvaluationRequestMode;

use super::types::{
    EvaluationCursor, EvaluationPlan, EvaluationSession, EvaluationTask, ExecutionStage,
    PlanSummary, StageBarrier, StageCursor, TaskReason,
};
use self::validation::{preview_maybe_stale, runtime_sorted_dependencies};

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
    let cursor = build_evaluation_cursor_with_policy_resolver(
        graph,
        targets,
        request_mode,
        resolver,
    )?;
    Ok(materialize_plan_from_cursor(cursor))
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
    let summary = populate_plan_buffers(
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
) -> Result<EvaluationSession<'a>, SignalError> {
    let summary = populate_plan_buffers(
        graph,
        targets,
        request_mode,
        resolver,
        &mut scratch.planner_targets,
        &mut scratch.planner_tasks,
        &mut scratch.planner_stages,
    )?;

    Ok(EvaluationSession {
        targets: &scratch.planner_targets,
        tasks: &scratch.planner_tasks,
        stages: &scratch.planner_stages,
        summary,
    })
}

pub(crate) fn materialize_plan_from_cursor(cursor: EvaluationCursor) -> EvaluationPlan {
    let mut stages = Vec::with_capacity(cursor.stages.len());
    for stage in &cursor.stages {
        stages.push(ExecutionStage {
            index: stage.index,
            tasks: cursor.tasks[stage.start..stage.end].to_vec(),
            barrier: stage.barrier,
        });
    }
    EvaluationPlan {
        request_mode: cursor.request_mode,
        targets: cursor.targets,
        stages,
        summary: cursor.summary,
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct PlannedNode {
    direct_request: bool,
}

fn visit_node(
    graph: &mut SignalGraph,
    node: NodeId,
    request_mode: EvaluationRequestMode,
    direct_request: bool,
    reason: TaskReason,
    resolver: &mut impl ComparatorPolicyResolver,
    visiting: &mut DenseBitset,
    planned: &mut [Option<PlannedNode>],
    planned_nodes: &mut Vec<NodeId>,
) -> Result<(), SignalError> {
    let node_index = node.index() as usize;
    if visiting.contains(node_index) {
        return Err(SignalError::invalid_input(format!(
            "cycle detected while building evaluation plan at {node}"
        )));
    }
    if let Some(existing) = &mut planned[node_index] {
        existing.direct_request |= direct_request;
        return Ok(());
    }
    visiting.mark(node_index);

    let state = graph.get_state(node)?;
    let should_include = matches!(state, NodeState::Dirty | NodeState::MaybeStale)
        || (direct_request && matches!(request_mode, EvaluationRequestMode::ForceOnDemand));
    if should_include {
        planned[node_index] = Some(PlannedNode { direct_request });
        planned_nodes.push(node);
    }

    match state {
        NodeState::Dirty => {
            for dependency in runtime_sorted_dependencies(graph, node)? {
                visit_node(
                    graph,
                    dependency.source(),
                    request_mode,
                    false,
                    TaskReason::DependencyRequired,
                    resolver,
                    visiting,
                    planned,
                    planned_nodes,
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
                    planned_nodes,
                )?;
            }
        }
        NodeState::Clean
            if direct_request && matches!(request_mode, EvaluationRequestMode::ForceOnDemand) =>
        {
            for dependency in runtime_sorted_dependencies(graph, node)? {
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
                        planned_nodes,
                    )?;
                }
            }
        }
        NodeState::Clean => {}
    }

    visiting.clear(node_index);
    Ok(())
}

struct DepthCache {
    index_by_node: HashMap<NodeId, usize>,
    depths: Vec<u32>,
}

impl DepthCache {
    fn depth_for(&self, node: NodeId) -> Option<u32> {
        self.index_by_node
            .get(&node)
            .and_then(|index| self.depths.get(*index).copied())
    }
}

fn populate_plan_buffers(
    graph: &mut SignalGraph,
    targets: &[NodeId],
    request_mode: EvaluationRequestMode,
    resolver: &mut impl ComparatorPolicyResolver,
    out_targets: &mut Vec<NodeId>,
    out_tasks: &mut Vec<EvaluationTask>,
    out_stages: &mut Vec<StageCursor>,
) -> Result<PlanSummary, SignalError> {
    let arena_capacity = graph.arena_capacity();
    let mut planned = vec![None::<PlannedNode>; arena_capacity];
    let mut planned_nodes = Vec::<NodeId>::new();
    let mut visiting = DenseBitset::new();
    visiting.ensure_len(arena_capacity);

    out_targets.clear();
    out_targets.extend_from_slice(targets);
    out_targets.sort_by_key(|node| node_sort_key(*node));
    out_targets.dedup();

    for &target in out_targets.iter() {
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
            &mut planned_nodes,
        )?;
    }

    let depth_cache = compute_depths(graph, &planned_nodes)?;
    let max_depth = depth_cache.depths.iter().copied().max().unwrap_or(0) as usize;
    let mut stages_by_depth = vec![Vec::<EvaluationTask>::new(); max_depth + 1];
    planned_nodes.sort_by_key(|node| node_sort_key(*node));
    for node in planned_nodes {
        let planned_node = planned[node.index() as usize]
            .expect("planned node list should only contain planned nodes");
        let reason = classify_reason(graph, node, planned_node.direct_request, request_mode)?;
        let task = EvaluationTask {
            node,
            request_mode,
            direct_request: planned_node.direct_request,
            reason,
        };
        let depth = depth_cache.depth_for(node).ok_or_else(|| {
            SignalError::internal("planned node missing depth cache entry during stage assembly")
        })? as usize;
        stages_by_depth[depth].push(task);
    }

    out_tasks.clear();
    out_stages.clear();
    for mut stage_tasks in stages_by_depth {
        if stage_tasks.is_empty() {
            continue;
        }
        stage_tasks.sort_by_key(|task| node_sort_key(task.node));
        let start = out_tasks.len();
        let end = start + stage_tasks.len();
        let stage_index = out_stages.len() as u32;
        out_tasks.extend(stage_tasks);
        out_stages.push(StageCursor {
            index: stage_index,
            start,
            end,
            barrier: Some(StageBarrier::StageBoundary),
        });
    }

    Ok(PlanSummary {
        requested_target_count: out_targets.len() as u32,
        stage_count: out_stages.len() as u32,
        task_count: out_tasks.len() as u32,
        max_stage_width: out_stages
            .iter()
            .map(|stage| (stage.end - stage.start) as u32)
            .max()
            .unwrap_or(0),
    })
}

fn compute_depths(
    graph: &mut SignalGraph,
    planned_nodes: &[NodeId],
) -> Result<DepthCache, SignalError> {
    let mut index_by_node = HashMap::with_capacity(planned_nodes.len());
    for (index, node) in planned_nodes.iter().copied().enumerate() {
        index_by_node.insert(node, index);
    }

    let mut indegree = vec![0_u32; planned_nodes.len()];
    let mut outgoing = vec![Vec::<usize>::new(); planned_nodes.len()];

    for (node_index, &node) in planned_nodes.iter().enumerate() {
        for dependency in graph.runtime_dependencies_of(node)? {
            let source = dependency.source();
            let Some(&source_index) = index_by_node.get(&source) else {
                continue;
            };
            indegree[node_index] += 1;
            outgoing[source_index].push(node_index);
        }
    }

    let mut frontier = planned_nodes
        .iter()
        .enumerate()
        .filter_map(|(index, node)| (indegree[index] == 0).then_some(*node))
        .collect::<Vec<_>>();
    frontier.sort_by_key(|node| node_sort_key(*node));

    let mut depths = vec![0_u32; planned_nodes.len()];
    let mut visited = 0usize;

    while let Some(node) = frontier.pop() {
        let node_index = *index_by_node
            .get(&node)
            .ok_or_else(|| SignalError::internal("planned node missing compact depth index"))?;
        visited += 1;
        let depth = graph
            .runtime_dependencies_of(node)?
            .iter()
            .filter_map(|dependency| {
                index_by_node
                    .get(&dependency.source())
                    .and_then(|source_index| depths.get(*source_index).copied())
            })
            .max()
            .map_or(0, |parent| parent + 1);
        depths[node_index] = depth;

        {
            let mut newly_ready = Vec::new();
            for &child_index in &outgoing[node_index] {
                let degree = &mut indegree[child_index];
                *degree = degree.saturating_sub(1);
                if *degree == 0 {
                    newly_ready.push(planned_nodes[child_index]);
                }
            }
            newly_ready.sort_by_key(|child| node_sort_key(*child));
            frontier.extend(newly_ready.into_iter().rev());
        }
    }

    if visited != planned_nodes.len() {
        return Err(SignalError::internal(
            "planner depth computation encountered a cycle in the planned graph",
        ));
    }

    Ok(DepthCache {
        index_by_node,
        depths,
    })
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
    if !entry.get_dirty_partition_scopes().is_empty() {
        return Ok(TaskReason::PartitionScopedDependency);
    }

    let trace = entry.get_trace_summary();
    if trace.is_some_and(|summary| {
        summary.output_change == crate::data::output::OutputChange::Unchanged
    }) {
        return Ok(TaskReason::OutputDiffDependent);
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
    !crate::data::output::scope_touched_by_trace(trace_summary, scope)
}

pub(crate) fn node_sort_key(node: NodeId) -> (u32, u32) {
    (node.index(), node.generation())
}
