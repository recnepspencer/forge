use std::collections::HashMap;

use crate::data::bitset::DenseBitset;
use crate::data::comparator::ComparatorPolicyResolver;
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::node::NodeState;
use crate::data::proof::DedupedNodeBatch;
use crate::logic::evaluation::EvaluationRequestMode;

use super::super::types::{CandidateTask, MaybeStaleAdmission, TaskReason};
use super::admission::verify_required_context;

#[derive(Debug, Clone, Copy, Default)]
pub(super) struct PlannedNode {
    pub(super) direct_request: bool,
    pub(super) maybe_stale_admission: Option<MaybeStaleAdmission>,
}

#[derive(Debug, Clone, Copy, Default)]
pub(super) struct PlanningStats {
    pub(super) contract_pruned_count: u32,
}

pub(super) struct PlanTopology {
    pub(super) targets: Vec<NodeId>,
    pub(super) planned: Vec<Option<PlannedNode>>,
    pub(super) planned_nodes: Vec<NodeId>,
    pub(super) stats: PlanningStats,
}

pub(super) fn discover_plan_topology(
    graph: &mut SignalGraph,
    targets: &[NodeId],
    request_mode: EvaluationRequestMode,
    resolver: &mut impl ComparatorPolicyResolver,
) -> Result<PlanTopology, SignalError> {
    let (arena, _, _, _) = graph.as_parts_mut();
    let arena_capacity = arena.len();
    let mut planned = vec![None::<PlannedNode>; arena_capacity];
    let mut planned_nodes = Vec::<NodeId>::new();
    let mut visiting = DenseBitset::new();
    let mut stats = PlanningStats::default();
    visiting.ensure_len(arena_capacity);
    let targets = DedupedNodeBatch::canonicalize_unordered(targets.iter().copied()).into_vec();

    for &target in &targets {
        graph.get_state(target)?;
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
            &mut stats,
        )?;
    }

    Ok(PlanTopology {
        targets,
        planned,
        planned_nodes,
        stats,
    })
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
    #[derive(Debug, Clone, Copy)]
    enum VisitFrame {
        Enter(CandidateTask),
        Exit(NodeId),
    }

    let mut stack = vec![VisitFrame::Enter(candidate)];
    while let Some(frame) = stack.pop() {
        match frame {
            VisitFrame::Enter(candidate) => {
                let node = candidate.node;
                let node_index = node.index() as usize;
                if visiting.contains(node_index) {
                    return Err(SignalError::invalid_input(format!(
                        "cycle detected while building evaluation plan at {node}"
                    )));
                }
                if let Some(existing) = &mut planned[node_index] {
                    existing.direct_request |= candidate.direct_request;
                    continue;
                }

                verify_required_context(
                    node,
                    graph.get_contract(node)?.semantics.required_context,
                )?;
                let state = graph.get_state(node)?;
                let should_include = matches!(state, NodeState::Dirty | NodeState::MaybeStale)
                    || (candidate.direct_request
                        && matches!(candidate.request_mode, EvaluationRequestMode::ForceOnDemand));
                if !should_include {
                    stats.contract_pruned_count += 1;
                    continue;
                }

                planned[node_index] = Some(PlannedNode {
                    direct_request: candidate.direct_request,
                    maybe_stale_admission: None,
                });
                planned_nodes.push(node);
                visiting.mark(node_index);
                stack.push(VisitFrame::Exit(node));

                match state {
                    NodeState::Dirty => {
                        let dependencies = graph.runtime_dependencies_of(node)?.to_vec();
                        for dependency in dependencies.into_iter().rev() {
                            stack.push(VisitFrame::Enter(CandidateTask {
                                node: dependency.source(),
                                request_mode: candidate.request_mode,
                                direct_request: false,
                                trigger_reason: TaskReason::DependencyRequired,
                            }));
                        }
                    }
                    NodeState::MaybeStale => {
                        let preview =
                            super::validation::preview_maybe_stale(graph, node, resolver)?;
                        if let Some(existing) = &mut planned[node_index] {
                            existing.maybe_stale_admission = Some(MaybeStaleAdmission {
                                unchanged_at_admission: preview.unchanged,
                            });
                        }
                        let upstream_reason =
                            if matches!(candidate.trigger_reason, TaskReason::MaybeStaleValidation)
                            {
                                TaskReason::MaybeStaleValidation
                            } else {
                                TaskReason::DependencyRequired
                            };
                        for source in preview.requires_upstream_evaluation.into_iter().rev() {
                            stack.push(VisitFrame::Enter(CandidateTask {
                                node: source,
                                request_mode: candidate.request_mode,
                                direct_request: false,
                                trigger_reason: upstream_reason,
                            }));
                        }
                    }
                    NodeState::Clean
                        if candidate.direct_request
                            && matches!(
                                candidate.request_mode,
                                EvaluationRequestMode::ForceOnDemand
                            ) =>
                    {
                        let dependencies = graph.runtime_dependencies_of(node)?.to_vec();
                        for dependency in dependencies.into_iter().rev() {
                            if !matches!(graph.get_state(dependency.source())?, NodeState::Clean) {
                                stack.push(VisitFrame::Enter(CandidateTask {
                                    node: dependency.source(),
                                    request_mode: candidate.request_mode,
                                    direct_request: false,
                                    trigger_reason: TaskReason::DependencyRequired,
                                }));
                            }
                        }
                    }
                    NodeState::Clean => {}
                }
            }
            VisitFrame::Exit(node) => {
                visiting.clear(node.index() as usize);
            }
        }
    }

    Ok(())
}

pub(super) struct DepthCache {
    index_by_node: HashMap<NodeId, usize>,
    depths: Vec<u32>,
}

impl DepthCache {
    pub(super) fn depth_for(&self, node: NodeId) -> Option<u32> {
        self.index_by_node
            .get(&node)
            .and_then(|index| self.depths.get(*index).copied())
    }

    pub(super) fn max_depth(&self) -> usize {
        self.depths.iter().copied().max().unwrap_or(0) as usize
    }
}

pub(super) fn compute_depths(
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
