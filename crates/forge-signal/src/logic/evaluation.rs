use crate::data::aspect::{Aspect, AspectVersion};
use crate::data::dependency::DependencySnapshot;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::node::NodeState;
use forge_core::KernelError;
use std::collections::HashSet;

/// Evaluate a node, recomputing only if necessary.
///
/// **Pull Phase:**
/// 1. `Clean(v)` → return immediately (cache hit).
/// 2. `MaybeStale` → recursively evaluate upstream dependencies.
///    If all subscribed aspect versions match the snapshot → revert to `Clean`.
/// 3. `Dirty` → call `compute` closure, record new versions and snapshot.
///
/// Uses an explicit stack to avoid stack overflow on deep graphs.
/// Tracks active nodes to detect evaluation cycles.
pub fn evaluate<F>(
    graph: &mut SignalGraph,
    node: NodeId,
    compute: &mut F,
) -> Result<(), KernelError>
where
    F: FnMut(NodeId, &SignalGraph) -> Result<AspectVersion, KernelError>,
{
    let mut eval_stack: Vec<EvalTask> = vec![EvalTask::Evaluate(node)];

    let mut active_path = HashSet::<NodeId>::new();

    let mut visited = HashSet::<NodeId>::new();

    while let Some(task) = eval_stack.pop() {
        match task {
            EvalTask::Evaluate(current) => {
                process_evaluate_task(
                    graph,
                    current,
                    &mut eval_stack,
                    &mut active_path,
                    &mut visited,
                )?;
            }
            EvalTask::Recompute(current) => {
                active_path.remove(&current);
                visited.insert(current);

                process_recompute_task(graph, current, compute)?;
            }
        }
    }

    Ok(())
}

/// Handle the `Evaluate` variant of the eval stack.
///
/// - Clean → skip.
/// - MaybeStale → check upstream snapshot; if unchanged, revert to Clean;
///   otherwise push deps then recompute.
/// - Dirty → push deps then recompute.
fn process_evaluate_task(
    graph: &mut SignalGraph,
    current: NodeId,
    eval_stack: &mut Vec<EvalTask>,
    active_path: &mut HashSet<NodeId>,
    visited: &mut HashSet<NodeId>,
) -> Result<(), KernelError> {
    if !graph.is_alive(current) {
        return Ok(());
    }

    if visited.contains(&current) {
        return Ok(());
    }

    if active_path.contains(&current) {
        return Err(circular_reference_error(current));
    }

    let state = *graph.get_entry(current)?.get_state();

    match state {
        NodeState::Clean => {
            visited.insert(current);
            Ok(())
        }

        NodeState::MaybeStale => {
            let upstream_unchanged = check_upstream_unchanged(graph, current)?;
            if upstream_unchanged {
                revert_to_clean(graph, current)?;
                visited.insert(current);
                Ok(())
            } else {
                active_path.insert(current);
                push_deps_then_recompute(graph, current, eval_stack)
            }
        }

        NodeState::Dirty => {
            active_path.insert(current);
            push_deps_then_recompute(graph, current, eval_stack)
        }
    }
}

/// Handle the `Recompute` variant of the eval stack.
///
/// Rechecks state (deps may have settled to Clean since scheduling).
fn process_recompute_task<F>(
    graph: &mut SignalGraph,
    current: NodeId,
    compute: &mut F,
) -> Result<(), KernelError>
where
    F: FnMut(NodeId, &SignalGraph) -> Result<AspectVersion, KernelError>,
{
    if !graph.is_alive(current) {
        return Ok(());
    }

    let state = *graph.get_entry(current)?.get_state();

    if matches!(state, NodeState::Clean) {
        return Ok(());
    }

    if matches!(state, NodeState::MaybeStale) {
        let upstream_unchanged = check_upstream_unchanged(graph, current)?;
        if upstream_unchanged {
            return revert_to_clean(graph, current);
        }
    }

    recompute_node(graph, current, compute)
}

/// Push upstream dependencies onto the eval stack, followed by a Recompute task.
fn push_deps_then_recompute(
    graph: &SignalGraph,
    current: NodeId,
    eval_stack: &mut Vec<EvalTask>,
) -> Result<(), KernelError> {
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

/// Revert a node to `Clean` using its current aspect version.
fn revert_to_clean(graph: &mut SignalGraph, node: NodeId) -> Result<(), KernelError> {
    let ver = graph.get_entry(node)?.get_aspect_version();
    let _clean_version = ver.topology() + ver.geometry();
    graph.get_entry_mut(node)?.set_state(NodeState::Clean);
    Ok(())
}

/// Compare a node's `DependencySnapshot` against current upstream versions.
///
/// Returns `true` if all subscribed aspect versions are unchanged,
/// meaning the node can safely revert to `Clean`.
fn check_upstream_unchanged(graph: &SignalGraph, node: NodeId) -> Result<bool, KernelError> {
    let snapshot = graph.get_entry(node)?.get_dep_snapshot().clone();

    for (source, aspect, cached_version) in snapshot.entries() {
        if !graph.is_alive(*source) {
            return Ok(false);
        }
        let current_version = graph.get_entry(*source)?.get_aspect_version().get(*aspect);
        if current_version != *cached_version {
            return Ok(false);
        }
    }

    Ok(true)
}

/// Execute the computation closure for a node and update its state.
fn recompute_node<F>(
    graph: &mut SignalGraph,
    node: NodeId,
    compute: &mut F,
) -> Result<(), KernelError>
where
    F: FnMut(NodeId, &SignalGraph) -> Result<AspectVersion, KernelError>,
{
    let new_version = compute(node, graph)?;

    let snapshot = build_dep_snapshot(graph, node)?;

    let entry = graph.get_entry_mut(node)?;
    entry.set_aspect_version(new_version);
    entry.set_dep_snapshot(snapshot);
    entry.set_state(NodeState::Clean);

    Ok(())
}

/// Capture the current upstream aspect versions for a node's dependencies.
fn build_dep_snapshot(
    graph: &SignalGraph,
    node: NodeId,
) -> Result<DependencySnapshot, KernelError> {
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

/// Internal task for the explicit evaluation stack.
enum EvalTask {
    /// Evaluate this node (may push dependencies first).
    Evaluate(NodeId),
    /// Recompute this node (dependencies already evaluated).
    Recompute(NodeId),
}

/// Produce a structured error for a circular reference.
fn circular_reference_error(node: NodeId) -> KernelError {
    KernelError::InvalidInput {
        message: format!("Circular reference detected at signal node: {}", node),
        context: None,
    }
}
