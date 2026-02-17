//! Push/pull evaluation engine for the signal graph.
//!
//! DOMAIN: Two-phase reactive evaluation with version-gated skip.
//!
//! INVARIANTS:
//! - Push phase never recomputes — only marks states
//! - Pull phase is lazy — only triggered by explicit reads
//! - Cycle detection aborts with structured error (no stack overflow)
//! - All traversals use explicit stacks (no recursion)
//!
//! DEPENDENCIES: `graph` (SignalGraph), `schema` (NodeState, Aspect, AspectVersion)

use std::collections::HashSet;

use forge_core::KernelError;

use crate::graph::SignalGraph;
use crate::handles::NodeId;
use crate::schema::{Aspect, AspectVersion, DependencySnapshot, NodeState};

/// Propagate invalidation downstream from a changed source node.
///
/// **Push Phase:**
/// 1. Mark the source node `Dirty`.
/// 2. Direct subscribers that read the changed aspect → `Dirty`.
///    Direct subscribers reading a different aspect → `MaybeStale`.
/// 3. All transitive downstream subscribers → `MaybeStale`.
/// 4. Cycle detection via visited set — returns structured error on cycle.
pub fn mark_dirty(
    graph: &mut SignalGraph,
    source: NodeId,
    changed_aspect: Aspect,
) -> Result<(), KernelError> {
    graph.get_entry_mut(source)?.set_state(NodeState::Dirty);

    let mut visited = HashSet::<NodeId>::new();
    visited.insert(source);

    let direct_subs = collect_live_subscribers(graph, source);
    detect_cycles_in_set(&direct_subs, &visited, source)?;

    mark_direct_subscribers(graph, source, changed_aspect, &direct_subs)?;
    insert_all(&mut visited, &direct_subs);

    let transitive_seeds = collect_transitive_seeds(graph, &direct_subs);
    propagate_maybe_stale(graph, &mut visited, transitive_seeds)
}

/// Mark each direct subscriber as `Dirty` (matching aspect) or `MaybeStale`.
fn mark_direct_subscribers(
    graph: &mut SignalGraph,
    source: NodeId,
    changed_aspect: Aspect,
    direct_subs: &[NodeId],
) -> Result<(), KernelError> {
    for sub in direct_subs {
        let reads_changed = subscribes_to_aspect(graph, *sub, source, changed_aspect)?;
        let new_state = if reads_changed {
            NodeState::Dirty
        } else {
            NodeState::MaybeStale
        };
        graph.get_entry_mut(*sub)?.set_state(new_state);
    }
    Ok(())
}

/// Check whether `downstream` subscribes to `changed_aspect` of `source`.
fn subscribes_to_aspect(
    graph: &SignalGraph,
    downstream: NodeId,
    source: NodeId,
    changed_aspect: Aspect,
) -> Result<bool, KernelError> {
    let deps = graph.get_entry(downstream)?.get_dependencies();
    let reads_aspect = deps
        .iter()
        .any(|dep| dep.source() == source && dep.aspect() == changed_aspect);
    Ok(reads_aspect)
}

/// Collect all live subscriber handles for a given node.
fn collect_live_subscribers(graph: &SignalGraph, node: NodeId) -> Vec<NodeId> {
    let all_subs = match graph.get_entry(node) {
        Ok(entry) => entry.get_subscribers().to_vec(),
        Err(_) => Vec::new(),
    };
    all_subs
        .into_iter()
        .filter(|s| graph.is_alive(*s))
        .collect()
}

/// Return an error if any node in `candidates` already appears in `visited`.
fn detect_cycles_in_set(
    candidates: &[NodeId],
    visited: &HashSet<NodeId>,
    _source: NodeId,
) -> Result<(), KernelError> {
    for candidate in candidates {
        if visited.contains(candidate) {
            return Err(circular_reference_error(*candidate));
        }
    }
    Ok(())
}

/// Gather subscribers of all direct subscribers (the transitive frontier).
fn collect_transitive_seeds(graph: &SignalGraph, direct_subs: &[NodeId]) -> Vec<NodeId> {
    let mut seeds = Vec::new();
    for sub in direct_subs {
        let sub_subs = collect_live_subscribers(graph, *sub);
        seeds.extend(sub_subs);
    }
    seeds
}

/// Walk the transitive frontier, marking all reachable nodes `MaybeStale`.
///
/// Skips nodes already visited or already `Dirty` (from direct marking).
/// Detects cycles via the visited set.
fn propagate_maybe_stale(
    graph: &mut SignalGraph,
    visited: &mut HashSet<NodeId>,
    initial_queue: Vec<NodeId>,
) -> Result<(), KernelError> {
    let mut queue = initial_queue;

    while let Some(node) = queue.pop() {
        if !graph.is_alive(node) || visited.contains(&node) {
            let is_cyclic = visited.contains(&node) && has_back_edge(graph, node, visited);
            if is_cyclic {
                return Err(circular_reference_error(node));
            }
        } else {
            visited.insert(node);

            let already_dirty = matches!(
                graph.get_entry(node).map(|e| *e.get_state()),
                Ok(NodeState::Dirty)
            );

            if !already_dirty {
                graph.get_entry_mut(node)?.set_state(NodeState::MaybeStale);
            }

            let downstream = collect_live_subscribers(graph, node);
            queue.extend(downstream);
        }
    }

    Ok(())
}

/// Check whether `node` has a subscriber that is also in `visited` and
/// has a dependency back on `node`, forming a true cycle.
fn has_back_edge(
    graph: &SignalGraph,
    node: NodeId,
    visited: &HashSet<NodeId>,
) -> bool {
    let subs = match graph.get_entry(node) {
        Ok(entry) => entry.get_subscribers().to_vec(),
        Err(_) => return false,
    };
    subs.iter().any(|s| {
        visited.contains(s)
            && graph
                .get_entry(*s)
                .is_ok_and(|e| e.get_dependencies().iter().any(|d| d.source() == node))
    })
}

/// Insert all handles from a slice into the visited set.
fn insert_all(visited: &mut HashSet<NodeId>, nodes: &[NodeId]) {
    for node in nodes {
        visited.insert(*node);
    }
}

// =========================================================================
// Pull Phase
// =========================================================================

/// Evaluate a node, recomputing only if necessary.
///
/// **Pull Phase:**
/// 1. `Clean(v)` → return immediately (cache hit).
/// 2. `MaybeStale` → recursively evaluate upstream dependencies.
///    If all subscribed aspect versions match the snapshot → revert to `Clean`.
/// 3. `Dirty` → call `compute` closure, record new versions and snapshot.
///
/// Uses an explicit stack to avoid stack overflow on deep graphs.
/// Tracks visited nodes to detect evaluation cycles.
pub fn evaluate<F>(
    graph: &mut SignalGraph,
    node: NodeId,
    compute: &mut F,
) -> Result<(), KernelError>
where
    F: FnMut(NodeId, &SignalGraph) -> Result<AspectVersion, KernelError>,
{
    let mut eval_stack: Vec<EvalTask> = vec![EvalTask::Evaluate(node)];
    let mut eval_visited = HashSet::<NodeId>::new();

    while let Some(task) = eval_stack.pop() {
        match task {
            EvalTask::Evaluate(current) => {
                process_evaluate_task(graph, current, &mut eval_stack, &mut eval_visited)?;
            }
            EvalTask::Recompute(current) => {
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
    eval_visited: &mut HashSet<NodeId>,
) -> Result<(), KernelError> {
    if !graph.is_alive(current) {
        return Ok(());
    }

    if eval_visited.contains(&current) {
        return Err(circular_reference_error(current));
    }
    eval_visited.insert(current);

    let state = *graph.get_entry(current)?.get_state();

    match state {
        NodeState::Clean(_) => Ok(()),

        NodeState::MaybeStale => {
            let upstream_unchanged = check_upstream_unchanged(graph, current)?;
            if upstream_unchanged {
                return revert_to_clean(graph, current);
            }
            push_deps_then_recompute(graph, current, eval_stack)
        }

        NodeState::Dirty => push_deps_then_recompute(graph, current, eval_stack),
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

    if matches!(state, NodeState::Clean(_)) {
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
    let clean_version = ver.topology() + ver.geometry();
    graph
        .get_entry_mut(node)?
        .set_state(NodeState::Clean(clean_version));
    Ok(())
}

/// Compare a node's `DependencySnapshot` against current upstream versions.
///
/// Returns `true` if all subscribed aspect versions are unchanged,
/// meaning the node can safely revert to `Clean`.
fn check_upstream_unchanged(
    graph: &SignalGraph,
    node: NodeId,
) -> Result<bool, KernelError> {
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
    entry.set_state(NodeState::Clean(new_version.topology() + new_version.geometry()));

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
        message: format!(
            "Circular reference detected at signal node: {}",
            node
        ),
        context: None,
    }
}
