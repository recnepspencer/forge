pub(crate) mod validation;

use std::collections::HashMap;

use crate::data::bitset::DenseBitset;
use crate::data::comparator::{
    ComparatorPolicyResolver, DefaultComparatorPolicyResolver, DefaultComparatorResolver,
    VersionComparatorPolicy,
};
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::graph::TraversalScratch;
use crate::data::handle::NodeId;
use crate::data::node::{ContextRequirement, NodeState};
use crate::data::proof::{DedupedNodeBatch, LocallyOrderedShard};
use crate::logic::evaluation::EvaluationRequestMode;

use self::validation::{preview_maybe_stale, runtime_sorted_dependencies};
use super::types::{
    CandidateTask, EligibleTask, EligibleTaskAdmission, EvaluationCursor, EvaluationPlan,
    ExecutionStage, MaybeStaleAdmission, PlanSummary, SessionScratch, StageBarrier, StageCursor,
    TaskReason,
};

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
) -> Result<SessionScratch<'a>, SignalError> {
    let summary = populate_plan_buffers(
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
    maybe_stale_admission: Option<MaybeStaleAdmission>,
}

#[derive(Debug, Clone, Copy, Default)]
struct PlanningStats {
    contract_pruned_count: u32,
}

fn visit_node(
    graph: &mut SignalGraph,
    candidate: CandidateTask,
    resolver: &mut impl ComparatorPolicyResolver,
    visiting: &mut DenseBitset,
    planned: &mut [Option<PlannedNode>],
    planned_nodes: &mut Vec<NodeId>,
    stats: &mut PlanningStats,
) -> Result<(), SignalError> {
    let node = candidate.node;
    let node_index = node.index() as usize;
    if visiting.contains(node_index) {
        return Err(SignalError::invalid_input(format!(
            "cycle detected while building evaluation plan at {node}"
        )));
    }
    if let Some(existing) = &mut planned[node_index] {
        existing.direct_request |= candidate.direct_request;
        return Ok(());
    }
    visiting.mark(node_index);

    let entry = graph.get_entry(node)?;
    verify_required_context(node, graph.get_contract(node)?.semantics.required_context)?;
    let state = *entry.get_state();
    let dirty_partition_scopes = entry.get_dirty_partition_scopes();
    let contract_reads_dirty = graph
        .get_contract(node)?
        .cares_about_change(entry.get_dirty_aspects(), &dirty_partition_scopes);
    let should_include = (matches!(state, NodeState::Dirty | NodeState::MaybeStale)
        && contract_reads_dirty)
        || (candidate.direct_request
            && matches!(
                candidate.request_mode,
                EvaluationRequestMode::ForceOnDemand
            ));
    if should_include {
        planned[node_index] = Some(PlannedNode {
            direct_request: candidate.direct_request,
            maybe_stale_admission: None,
        });
        planned_nodes.push(node);
    } else {
        stats.contract_pruned_count += 1;
        visiting.clear(node_index);
        return Ok(());
    }

    match state {
        NodeState::Dirty => {
            for dependency in runtime_sorted_dependencies(graph, node)? {
                visit_node(
                    graph,
                    CandidateTask {
                        node: dependency.source(),
                        request_mode: candidate.request_mode,
                        direct_request: false,
                        trigger_reason: TaskReason::DependencyRequired,
                    },
                    resolver,
                    visiting,
                    planned,
                    planned_nodes,
                    stats,
                )?;
            }
        }
        NodeState::MaybeStale => {
            let preview = preview_maybe_stale(graph, node, resolver)?;
            if let Some(existing) = &mut planned[node_index] {
                existing.maybe_stale_admission = Some(MaybeStaleAdmission {
                    unchanged_at_admission: preview.unchanged,
                });
            }
            let upstream_reason = if matches!(candidate.trigger_reason, TaskReason::MaybeStaleValidation) {
                TaskReason::MaybeStaleValidation
            } else {
                TaskReason::DependencyRequired
            };
            for source in preview.requires_upstream_evaluation {
                visit_node(
                    graph,
                    CandidateTask {
                        node: source,
                        request_mode: candidate.request_mode,
                        direct_request: false,
                        trigger_reason: upstream_reason,
                    },
                    resolver,
                    visiting,
                    planned,
                    planned_nodes,
                    stats,
                )?;
            }
        }
        NodeState::Clean
            if candidate.direct_request
                && matches!(candidate.request_mode, EvaluationRequestMode::ForceOnDemand) =>
        {
            for dependency in runtime_sorted_dependencies(graph, node)? {
                if !matches!(graph.get_state(dependency.source())?, NodeState::Clean) {
                    visit_node(
                        graph,
                        CandidateTask {
                            node: dependency.source(),
                            request_mode: candidate.request_mode,
                            direct_request: false,
                            trigger_reason: TaskReason::DependencyRequired,
                        },
                        resolver,
                        visiting,
                        planned,
                        planned_nodes,
                        stats,
                    )?;
                }
            }
        }
        NodeState::Clean => {}
    }

    visiting.clear(node_index);
    Ok(())
}

fn verify_required_context(
    node: NodeId,
    requirement: ContextRequirement,
) -> Result<(), SignalError> {
    match requirement {
        ContextRequirement::None | ContextRequirement::DomainContext => Ok(()),
        ContextRequirement::RelationalSnapshot => {
            Err(SignalError::contract_violation(node, requirement))
        }
    }
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

fn admit_planned_node(
    graph: &SignalGraph,
    node: NodeId,
    direct_request: bool,
    request_mode: EvaluationRequestMode,
    maybe_stale_admission: Option<MaybeStaleAdmission>,
) -> Result<EligibleTask, SignalError> {
    let entry = graph.get_entry(node)?;
    let dirty_partition_scopes_present = !entry.get_dirty_partition_scopes().is_empty();
    let node_state_at_admission = Some(*entry.get_state());
    let reason = classify_reason(graph, node, direct_request, request_mode)?;
    Ok(EligibleTask {
        node,
        request_mode,
        direct_request,
        reason,
        admission: EligibleTaskAdmission {
            node_state_at_admission,
            dirty_partition_scopes_present,
            maybe_stale: maybe_stale_admission,
        },
    })
}

pub(crate) fn admit_direct_task_with_policy_resolver(
    graph: &SignalGraph,
    node: NodeId,
    request_mode: EvaluationRequestMode,
    resolver: &mut impl ComparatorPolicyResolver,
) -> Result<EligibleTask, SignalError> {
    let entry = graph.get_entry(node)?;
    let state = *entry.get_state();
    let maybe_stale_admission = if matches!(state, NodeState::MaybeStale) {
        let preview = preview_maybe_stale(graph, node, resolver)?;
        Some(MaybeStaleAdmission {
            unchanged_at_admission: preview.unchanged,
        })
    } else {
        None
    };
    admit_planned_node(graph, node, true, request_mode, maybe_stale_admission)
}

fn populate_plan_buffers(
    graph: &mut SignalGraph,
    targets: &[NodeId],
    request_mode: EvaluationRequestMode,
    resolver: &mut impl ComparatorPolicyResolver,
    out_targets: &mut Vec<NodeId>,
    out_tasks: &mut Vec<EligibleTask>,
    out_stages: &mut Vec<StageCursor>,
) -> Result<PlanSummary, SignalError> {
    let (arena, _, _, _) = graph.as_parts_mut();
    let arena_capacity = arena.len();
    let mut planned = vec![None::<PlannedNode>; arena_capacity];
    let mut planned_nodes = Vec::<NodeId>::new();
    let mut visiting = DenseBitset::new();
    let mut planning_stats = PlanningStats::default();
    visiting.ensure_len(arena_capacity);

    out_targets.clear();
    out_targets
        .extend(DedupedNodeBatch::canonicalize_unordered(targets.iter().copied()).into_vec());

    for &target in out_targets.iter() {
        graph.get_entry(target)?;
        visit_node(
            graph,
            CandidateTask {
                node: target,
                request_mode,
                direct_request: true,
                trigger_reason: TaskReason::RequestedTarget,
            },
            resolver,
            &mut visiting,
            &mut planned,
            &mut planned_nodes,
            &mut planning_stats,
        )?;
    }

    let depth_cache = compute_depths(graph, &planned_nodes)?;
    let max_depth = depth_cache.depths.iter().copied().max().unwrap_or(0) as usize;
    let mut stages_by_depth = vec![Vec::<EligibleTask>::new(); max_depth + 1];
    planned_nodes = DedupedNodeBatch::canonicalize_unordered(planned_nodes).into_vec();
    for node in planned_nodes {
        let planned_node = planned[node.index() as usize]
            .expect("planned node list should only contain planned nodes");
        let task = admit_planned_node(
            graph,
            node,
            planned_node.direct_request,
            request_mode,
            planned_node.maybe_stale_admission,
        )?;
        let depth = depth_cache.depth_for(node).ok_or_else(|| {
            SignalError::internal("planned node missing depth cache entry during stage assembly")
        })? as usize;
        stages_by_depth[depth].push(task);
    }

    out_tasks.clear();
    out_stages.clear();
    for stage_tasks in stages_by_depth {
        if stage_tasks.is_empty() {
            continue;
        }
        let stage_tasks = LocallyOrderedShard::canonicalize_unordered(stage_tasks).into_vec();
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
        contract_pruned_count: planning_stats.contract_pruned_count,
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
    frontier = DedupedNodeBatch::canonicalize_unordered(frontier).into_vec();

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
            let newly_ready = DedupedNodeBatch::canonicalize_unordered(newly_ready).into_vec();
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

    let trace = entry.get_runtime_artifact_state();
    if trace.is_some_and(|summary| {
        summary.output_change == crate::data::output::OutputChange::Unchanged
    }) {
        return Ok(TaskReason::OutputDiffDependent);
    }

    if trace.is_some_and(|summary| {
        summary.reuse_basis.source == crate::data::reuse::ReuseSource::MemoizedArtifact
    }) {
        return Ok(TaskReason::MemoValidation);
    }

    Ok(TaskReason::Dirty)
}

#[cfg(test)]
pub(crate) fn partition_scope_untouched(
    trace_summary: Option<&crate::data::trace::RuntimeArtifactState>,
    scope: &crate::data::output::PartitionSubscription,
) -> bool {
    !crate::data::output::scope_touched_by_artifact_state(trace_summary, scope)
}
