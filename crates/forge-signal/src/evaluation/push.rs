use std::collections::HashSet;
use forge_core::KernelError;
use crate::graph::SignalGraph;
use crate::handles::NodeId;
use crate::schema::{Aspect, NodeState};

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
