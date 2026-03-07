use crate::data::aspect::{Aspect, AspectVersion};
use crate::data::comparator::{
    ComparatorPolicyResolver, DefaultComparatorPolicyResolver, DefaultComparatorResolver,
    VersionComparatorResolver,
};
use crate::data::dependency::DependencySnapshot;
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::node::NodeState;
use std::time::Instant;

/// Evaluate a node, recomputing only if necessary.
///
/// Uses an explicit stack to avoid recursion on deep graphs.
pub fn evaluate<F>(
    graph: &mut SignalGraph,
    node: NodeId,
    compute: &mut F,
) -> Result<(), SignalError>
where
    F: FnMut(NodeId, &SignalGraph) -> Result<AspectVersion, SignalError>,
{
    let mut resolver = DefaultComparatorResolver;
    evaluate_with_resolver(graph, node, compute, &mut resolver)
}

/// Evaluate a node with a host-provided custom comparator resolver.
pub fn evaluate_with_resolver<F, R>(
    graph: &mut SignalGraph,
    node: NodeId,
    compute: &mut F,
    resolver: &mut R,
) -> Result<(), SignalError>
where
    F: FnMut(NodeId, &SignalGraph) -> Result<AspectVersion, SignalError>,
    R: VersionComparatorResolver,
{
    let mut policy = DefaultComparatorPolicyResolver {
        fallback: crate::data::comparator::VersionComparatorPolicy::Exact,
        custom: resolver,
    };
    evaluate_with_policy_resolver(graph, node, compute, &mut policy)
}

/// Evaluate a node with explicit comparator policy resolution.
pub fn evaluate_with_policy_resolver<F, R>(
    graph: &mut SignalGraph,
    node: NodeId,
    compute: &mut F,
    resolver: &mut R,
) -> Result<(), SignalError>
where
    F: FnMut(NodeId, &SignalGraph) -> Result<AspectVersion, SignalError>,
    R: ComparatorPolicyResolver,
{
    let eval_start = Instant::now();
    graph.telemetry_mut().evaluation_calls += 1;
    graph.begin_eval_pass();
    let mut eval_stack: Vec<EvalTask> = vec![EvalTask::Evaluate(node)];

    while let Some(task) = eval_stack.pop() {
        match task {
            EvalTask::Evaluate(current) => {
                process_evaluate_task(graph, current, &mut eval_stack, resolver)?;
            }
            EvalTask::Recompute(current) => {
                graph.active_clear(current);
                graph.visited_mark(current);
                process_recompute_task(graph, current, compute, resolver)?;
            }
        }
    }

    graph.telemetry_mut().evaluation_nanos += eval_start.elapsed().as_nanos();
    Ok(())
}

fn process_evaluate_task(
    graph: &mut SignalGraph,
    current: NodeId,
    eval_stack: &mut Vec<EvalTask>,
    resolver: &mut impl ComparatorPolicyResolver,
) -> Result<(), SignalError> {
    if !graph.is_alive(current) {
        return Ok(());
    }

    if graph.visited_contains(current) {
        return Ok(());
    }

    if graph.active_contains(current) {
        return Err(circular_reference_error(current));
    }

    let state = *graph.get_entry(current)?.get_state();

    match state {
        NodeState::Clean => {
            graph.visited_mark(current);
            Ok(())
        }

        NodeState::MaybeStale => {
            let upstream_unchanged = check_upstream_unchanged(graph, current, resolver)?;
            if upstream_unchanged {
                revert_to_clean(graph, current)?;
                graph.visited_mark(current);
                Ok(())
            } else {
                graph.active_mark(current);
                push_deps_then_recompute(graph, current, eval_stack)
            }
        }

        NodeState::Dirty => {
            graph.active_mark(current);
            push_deps_then_recompute(graph, current, eval_stack)
        }
    }
}

fn process_recompute_task<F>(
    graph: &mut SignalGraph,
    current: NodeId,
    compute: &mut F,
    resolver: &mut impl ComparatorPolicyResolver,
) -> Result<(), SignalError>
where
    F: FnMut(NodeId, &SignalGraph) -> Result<AspectVersion, SignalError>,
{
    graph.telemetry_mut().nodes_evaluated += 1;
    if !graph.is_alive(current) {
        return Ok(());
    }

    let state = *graph.get_entry(current)?.get_state();

    if matches!(state, NodeState::Clean) {
        return Ok(());
    }

    if matches!(state, NodeState::MaybeStale) {
        let upstream_unchanged = check_upstream_unchanged(graph, current, resolver)?;
        if upstream_unchanged {
            return revert_to_clean(graph, current);
        }
    }

    recompute_node(graph, current, compute)
}

fn push_deps_then_recompute(
    graph: &SignalGraph,
    current: NodeId,
    eval_stack: &mut Vec<EvalTask>,
) -> Result<(), SignalError> {
    let dep_sources: Vec<NodeId> = graph
        .get_entry(current)?
        .get_dependencies()
        .iter()
        .map(|d| d.source())
        .collect();

    eval_stack.push(EvalTask::Recompute(current));
    for dep in dep_sources {
        eval_stack.push(EvalTask::Evaluate(dep));
    }
    Ok(())
}

fn revert_to_clean(graph: &mut SignalGraph, node: NodeId) -> Result<(), SignalError> {
    graph.telemetry_mut().skipped_by_comparator += 1;
    graph.get_entry_mut(node)?.set_state(NodeState::Clean);
    Ok(())
}

fn check_upstream_unchanged(
    graph: &SignalGraph,
    node: NodeId,
    resolver: &mut impl ComparatorPolicyResolver,
) -> Result<bool, SignalError> {
    let snapshot = graph.get_entry(node)?.get_dep_snapshot().clone();
    let node_cfg = graph.get_entry(node)?.get_eval_config();
    let comparator = resolver.policy_for_node(node, node_cfg.comparator.as_ref());

    for (source, aspect, cached_version) in snapshot.entries() {
        if !graph.is_alive(*source) {
            return Ok(false);
        }
        let current_version = graph.get_entry(*source)?.get_aspect_version().get(*aspect);
        if comparator.has_meaningful_change(*aspect, *cached_version, current_version, resolver)? {
            return Ok(false);
        }
    }

    Ok(true)
}

fn recompute_node<F>(
    graph: &mut SignalGraph,
    node: NodeId,
    compute: &mut F,
) -> Result<(), SignalError>
where
    F: FnMut(NodeId, &SignalGraph) -> Result<AspectVersion, SignalError>,
{
    let new_version = compute(node, graph)?;
    let snapshot = build_dep_snapshot(graph, node)?;

    let entry = graph.get_entry_mut(node)?;
    entry.set_aspect_version(new_version);
    entry.set_dep_snapshot(snapshot);
    entry.set_state(NodeState::Clean);
    graph.telemetry_mut().nodes_recomputed += 1;

    Ok(())
}

fn build_dep_snapshot(
    graph: &SignalGraph,
    node: NodeId,
) -> Result<DependencySnapshot, SignalError> {
    let dep_edges: Vec<(NodeId, Aspect)> = graph
        .get_entry(node)?
        .get_dependencies()
        .iter()
        .map(|d| (d.source(), d.aspect()))
        .collect();

    let mut snapshot = DependencySnapshot::empty();
    for (source, aspect) in dep_edges {
        if graph.is_alive(source) {
            let ver = graph.get_entry(source)?.get_aspect_version().get(aspect);
            snapshot.record(source, aspect, ver);
        }
    }
    Ok(snapshot)
}

enum EvalTask {
    Evaluate(NodeId),
    Recompute(NodeId),
}

fn circular_reference_error(node: NodeId) -> SignalError {
    SignalError::InvalidInput {
        message: format!("Circular reference detected at signal node: {}", node),
        context: None,
    }
}
