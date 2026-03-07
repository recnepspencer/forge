use crate::data::aspect::{Aspect, AspectMask};
use crate::data::bitset::BitsetFrontier;
use crate::data::error::SignalError;
use crate::data::graph::{ScratchLeaseKind, SignalGraph, TraversalScratch};
use crate::data::handle::NodeId;
use crate::data::node::NodeState;

/// Propagate invalidation downstream from a changed source node.
///
/// **Push Phase:**
/// 1. Mark the source node `Dirty`.
/// 2. Direct subscribers that read the changed aspect -> `Dirty`.
///    Direct subscribers reading a different aspect -> `MaybeStale`.
/// 3. All transitive downstream subscribers -> `MaybeStale`.
/// 4. Cycle detection via visited set -> structured error on cycle.
pub fn mark_dirty(
    graph: &mut SignalGraph,
    source: NodeId,
    changed_aspect: Aspect,
) -> Result<(), SignalError> {
    let mut scratch = graph.acquire_scratch(ScratchLeaseKind::Invalidation)?;
    let len = graph.arena_capacity();
    scratch.visited.next_pass(len);
    scratch.node_buffer_a.clear();
    scratch.node_buffer_b.clear();

    let result = mark_dirty_with_scratch(graph, &mut scratch, source, changed_aspect);
    graph.restore_scratch(ScratchLeaseKind::Invalidation, scratch)?;
    result
}

fn mark_dirty_with_scratch(
    graph: &mut SignalGraph,
    scratch: &mut TraversalScratch,
    source: NodeId,
    changed_aspect: Aspect,
) -> Result<(), SignalError> {
    {
        let source_entry = graph.get_entry_mut(source)?;
        source_entry.set_state(NodeState::Dirty);
        source_entry.add_dirty_aspect(changed_aspect);
    }

    scratch.visited.mark(source.index() as usize);

    collect_live_subscribers_into(graph, source, &mut scratch.node_buffer_a);
    detect_cycles_in_set(scratch, &scratch.node_buffer_a)?;
    mark_direct_subscribers(graph, source, changed_aspect, &scratch.node_buffer_a)?;
    let direct_sub_count = scratch.node_buffer_a.len();
    for index in 0..direct_sub_count {
        let node = scratch.node_buffer_a[index];
        scratch.visited.mark(node.index() as usize);
    }

    scratch.node_buffer_b.clear();
    for &sub in &scratch.node_buffer_a {
        append_live_subscribers(graph, sub, &mut scratch.node_buffer_b);
    }

    propagate_maybe_stale(graph, scratch, changed_aspect)
}

/// Mark each direct subscriber as `Dirty` (matching aspect) or `MaybeStale`.
fn mark_direct_subscribers(
    graph: &mut SignalGraph,
    source: NodeId,
    changed_aspect: Aspect,
    direct_subs: &[NodeId],
) -> Result<(), SignalError> {
    for &sub in direct_subs {
        let reads_changed = subscribes_to_aspect(graph, sub, source, changed_aspect)?;
        let new_state = if reads_changed {
            NodeState::Dirty
        } else {
            NodeState::MaybeStale
        };
        let entry = graph.get_entry_mut(sub)?;
        entry.set_state(new_state);
        entry.add_dirty_aspect(changed_aspect);
        graph.telemetry_mut().invalidation_nodes_visited += 1;
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

fn collect_live_subscribers_into(graph: &SignalGraph, node: NodeId, out: &mut Vec<NodeId>) {
    out.clear();
    append_live_subscribers(graph, node, out);
}

fn append_live_subscribers(graph: &SignalGraph, node: NodeId, out: &mut Vec<NodeId>) {
    let Ok(entry) = graph.get_entry(node) else {
        return;
    };
    for &subscriber in entry.get_subscribers() {
        if graph.is_alive(subscriber) {
            out.push(subscriber);
        }
    }
}

/// Return an error if any node in `candidates` already appears in `visited`.
fn detect_cycles_in_set(
    scratch: &TraversalScratch,
    candidates: &[NodeId],
) -> Result<(), SignalError> {
    for &candidate in candidates {
        if scratch.visited.is_marked(candidate.index() as usize) {
            return Err(circular_reference_error(candidate));
        }
    }
    Ok(())
}

/// Walk the transitive frontier, marking all reachable nodes `MaybeStale`.
fn propagate_maybe_stale(
    graph: &mut SignalGraph,
    scratch: &mut TraversalScratch,
    changed_aspect: Aspect,
) -> Result<(), SignalError> {
    let mut frontier = BitsetFrontier::new();
    for &node in &scratch.node_buffer_b {
        frontier.seed(node.index() as usize);
    }

    while frontier.has_current() {
        scratch.node_buffer_a.clear();
        scratch.node_buffer_a.extend(
            frontier
                .current_iter()
                .filter_map(|idx| graph.live_node_id_at(idx)),
        );
        for &node in &scratch.node_buffer_a {
            graph.telemetry_mut().invalidation_nodes_visited += 1;
            if scratch.visited.is_marked(node.index() as usize) {
                if has_back_edge(graph, scratch, node) {
                    return Err(circular_reference_error(node));
                }
                continue;
            }

            scratch.visited.mark(node.index() as usize);

            let already_dirty = matches!(
                graph.get_entry(node).map(|entry| *entry.get_state()),
                Ok(NodeState::Dirty)
            );

            if !already_dirty {
                let entry = graph.get_entry_mut(node)?;
                entry.set_state(NodeState::MaybeStale);
                entry.add_dirty_aspect(changed_aspect);
            }

            let Ok(entry) = graph.get_entry(node) else {
                continue;
            };
            for &subscriber in entry.get_subscribers() {
                if graph.is_alive(subscriber) {
                    frontier.mark_next(subscriber.index() as usize);
                }
            }
        }
        frontier.advance();
    }

    Ok(())
}

/// Check whether `node` has a subscriber that is also in `visited` and
/// has a dependency back on `node`, forming a true cycle.
fn has_back_edge(graph: &SignalGraph, scratch: &TraversalScratch, node: NodeId) -> bool {
    let Ok(entry) = graph.get_entry(node) else {
        return false;
    };
    entry.get_subscribers().iter().any(|subscriber| {
        scratch.visited.is_marked(subscriber.index() as usize)
            && graph
                .get_entry(*subscriber)
                .is_ok_and(|e| e.get_dependencies().iter().any(|d| d.source() == node))
    })
}

/// Produce a structured error for a circular reference.
fn circular_reference_error(node: NodeId) -> SignalError {
    SignalError::InvalidInput {
        message: format!("Circular reference detected at signal node: {}", node),
        context: None,
    }
}
