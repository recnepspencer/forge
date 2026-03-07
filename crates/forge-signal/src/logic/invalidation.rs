use crate::data::aspect::{Aspect, AspectMask};
use crate::data::bitset::BitsetFrontier;
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::node::NodeState;

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
) -> Result<(), SignalError> {
    graph.begin_visit_pass();
    graph.get_entry_mut(source)?.set_state(NodeState::Dirty);

    graph.visited_mark(source);

    let direct_subs = collect_live_subscribers(graph, source);
    detect_cycles_in_set(graph, &direct_subs, source)?;

    mark_direct_subscribers(graph, source, changed_aspect, &direct_subs)?;
    insert_all(graph, &direct_subs);

    let transitive_seeds = collect_transitive_seeds(graph, &direct_subs);
    propagate_maybe_stale(graph, transitive_seeds)
}

/// Mark each direct subscriber as `Dirty` (matching aspect) or `MaybeStale`.
fn mark_direct_subscribers(
    graph: &mut SignalGraph,
    source: NodeId,
    changed_aspect: Aspect,
    direct_subs: &[NodeId],
) -> Result<(), SignalError> {
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
) -> Result<bool, SignalError> {
    let changed_mask = AspectMask::from_aspect(changed_aspect);
    let deps = graph.get_entry(downstream)?.get_dependencies();
    let reads_aspect = deps
        .iter()
        .any(|dep| dep.source() == source && dep.aspect_mask().intersects(changed_mask));
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
    graph: &SignalGraph,
    candidates: &[NodeId],
    _source: NodeId,
) -> Result<(), SignalError> {
    for candidate in candidates {
        if graph.visited_contains(*candidate) {
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
    initial_queue: Vec<NodeId>,
) -> Result<(), SignalError> {
    let mut frontier = BitsetFrontier::new();
    for node in initial_queue {
        frontier.seed(node.index() as usize);
    }

    while frontier.has_current() {
        let current = frontier.current_indices();
        for idx in current {
            let Some(node) = graph.live_node_id_at(idx) else {
                continue;
            };
            if graph.visited_contains(node) {
                if has_back_edge(graph, node) {
                    return Err(circular_reference_error(node));
                }
                continue;
            }

            graph.visited_mark(node);

            let already_dirty = matches!(
                graph.get_entry(node).map(|e| *e.get_state()),
                Ok(NodeState::Dirty)
            );

            if !already_dirty {
                graph.get_entry_mut(node)?.set_state(NodeState::MaybeStale);
            }

            for sub in collect_live_subscribers(graph, node) {
                frontier.mark_next(sub.index() as usize);
            }
        }
        frontier.advance();
    }

    Ok(())
}

/// Check whether `node` has a subscriber that is also in `visited` and
/// has a dependency back on `node`, forming a true cycle.
fn has_back_edge(graph: &SignalGraph, node: NodeId) -> bool {
    let subs = match graph.get_entry(node) {
        Ok(entry) => entry.get_subscribers().to_vec(),
        Err(_) => return false,
    };
    subs.iter().any(|s| {
        graph.visited_contains(*s)
            && graph
                .get_entry(*s)
                .is_ok_and(|e| e.get_dependencies().iter().any(|d| d.source() == node))
    })
}

/// Insert all handles from a slice into the visited set.
fn insert_all(graph: &mut SignalGraph, nodes: &[NodeId]) {
    for node in nodes {
        graph.visited_mark(*node);
    }
}

/// Produce a structured error for a circular reference.
fn circular_reference_error(node: NodeId) -> SignalError {
    SignalError::InvalidInput {
        message: format!("Circular reference detected at signal node: {}", node),
        context: None,
    }
}
