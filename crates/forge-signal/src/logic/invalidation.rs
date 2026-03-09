use crate::data::aspect::{Aspect, AspectMask};
use crate::data::bitset::BitsetFrontier;
use crate::data::error::SignalError;
use crate::data::graph::{ScratchLeaseKind, SignalGraph, TraversalScratch};
use crate::data::handle::NodeId;
use crate::data::node::NodeState;
use crate::data::output::{
    ChangedRegion, InternedPartitionSubscription, PartitionMatchMode, PartitionSubscription,
};
use crate::diagnostics::failure::{ExecutionFailureContext, ExecutionFailurePhase};
use crate::diagnostics::recorder::DiagnosticsRecorder;

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
    mark_dirty_with_regions(graph, source, changed_aspect, &[])
}

/// Propagate invalidation downstream from a changed source node with changed regions.
pub fn mark_dirty_with_regions(
    graph: &mut SignalGraph,
    source: NodeId,
    changed_aspect: Aspect,
    changed_regions: &[ChangedRegion],
) -> Result<(), SignalError> {
    graph.note_change_input(source, changed_aspect, changed_regions);
    let mut scratch = graph.acquire_scratch(ScratchLeaseKind::Invalidation)?;
    let len = graph.arena_capacity();
    scratch.visited.next_pass(len);
    scratch.node_buffer_a.clear();
    scratch.node_buffer_b.clear();

    let result =
        mark_dirty_with_scratch(graph, &mut scratch, source, changed_aspect, changed_regions);
    graph.restore_scratch(ScratchLeaseKind::Invalidation, scratch)?;
    if let Err(err) = &result {
        graph.clear_pending_diagnostics_input();
        DiagnosticsRecorder::new(graph).record_failure(ExecutionFailureContext::from_error(
            ExecutionFailurePhase::Invalidation,
            err,
            None,
        ));
    }
    result
}

fn mark_dirty_with_scratch(
    graph: &mut SignalGraph,
    scratch: &mut TraversalScratch,
    source: NodeId,
    changed_aspect: Aspect,
    changed_regions: &[ChangedRegion],
) -> Result<(), SignalError> {
    let changed_scopes = changed_regions_to_dirty_scopes(changed_regions);
    let changed_scope_ids = intern_changed_regions(graph, changed_regions);
    {
        let source_entry = graph.get_entry_mut(source)?;
        source_entry.set_state(NodeState::Dirty);
        source_entry.add_dirty_aspect(changed_aspect);
        source_entry.set_dirty_partition_scopes(changed_scopes.iter().cloned());
    }

    scratch.visited.mark(source.index() as usize);

    collect_live_subscribers_into(graph, source, &mut scratch.node_buffer_a);
    detect_cycles_in_set(scratch, &scratch.node_buffer_a)?;
    let invalidation_stats = mark_direct_subscribers(
        graph,
        source,
        changed_aspect,
        &changed_scopes,
        &changed_scope_ids,
        &scratch.node_buffer_a,
    )?;
    graph.record_invalidation_diagnostics(
        invalidation_stats.invalidated_direct_subscribers,
        invalidation_stats.maybe_stale_direct_subscribers,
        invalidation_stats.partition_scoped_checks,
    );
    let direct_sub_count = scratch.node_buffer_a.len();
    for index in 0..direct_sub_count {
        let node = scratch.node_buffer_a[index];
        scratch.visited.mark(node.index() as usize);
    }

    scratch.node_buffer_b.clear();
    for &sub in &scratch.node_buffer_a {
        append_live_subscribers(graph, sub, &mut scratch.node_buffer_b);
    }

    propagate_maybe_stale(graph, scratch, changed_aspect, &changed_scopes)
}

/// Mark each direct subscriber as `Dirty` (matching aspect) or `MaybeStale`.
fn mark_direct_subscribers(
    graph: &mut SignalGraph,
    source: NodeId,
    changed_aspect: Aspect,
    changed_scopes: &[PartitionSubscription],
    changed_scope_ids: &[InternedPartitionSubscription],
    direct_subs: &[NodeId],
) -> Result<InvalidationStats, SignalError> {
    let mut stats = InvalidationStats::default();
    for &sub in direct_subs {
        let checks_before = graph.telemetry().partition_scoped_invalidation_checks;
        let dirty_match = subscribes_to_aspect(
            graph,
            sub,
            source,
            changed_aspect,
            changed_scopes,
            changed_scope_ids,
        )?;
        let new_state = match dirty_match {
            SubscriptionDirtyMatch::WholeAspect
            | SubscriptionDirtyMatch::WholePartition
            | SubscriptionDirtyMatch::PartitionAndDetail => NodeState::Dirty,
            SubscriptionDirtyMatch::Unmatched => NodeState::MaybeStale,
        };
        stats.partition_scoped_checks += graph
            .telemetry()
            .partition_scoped_invalidation_checks
            .saturating_sub(checks_before) as u32;
        match new_state {
            NodeState::Dirty => stats.invalidated_direct_subscribers += 1,
            NodeState::MaybeStale => stats.maybe_stale_direct_subscribers += 1,
            NodeState::Clean => {}
        }
        let entry = graph.get_entry_mut(sub)?;
        entry.set_state(new_state);
        entry.add_dirty_aspect(changed_aspect);
        entry.set_dirty_partition_scopes(changed_scopes.iter().cloned());
        graph.telemetry_mut().invalidation_nodes_visited += 1;
        match dirty_match {
            SubscriptionDirtyMatch::WholePartition => {
                graph.telemetry_mut().partition_match_dirty_count += 1;
            }
            SubscriptionDirtyMatch::PartitionAndDetail => {
                graph.telemetry_mut().detail_match_dirty_count += 1;
            }
            _ => {}
        }
    }
    Ok(stats)
}

/// Check whether `downstream` subscribes to `changed_aspect` of `source`.
fn subscribes_to_aspect(
    graph: &mut SignalGraph,
    downstream: NodeId,
    source: NodeId,
    changed_aspect: Aspect,
    changed_scopes: &[PartitionSubscription],
    changed_scope_ids: &[InternedPartitionSubscription],
) -> Result<SubscriptionDirtyMatch, SignalError> {
    let changed_mask = AspectMask::from_aspect(changed_aspect);
    let deps = graph.get_entry(downstream)?.get_dependencies().to_vec();
    for dep in &deps {
        if dep.source() != source || !dep.aspect_mask().intersects(changed_mask) {
            continue;
        }
        let Some(scope) = dep.scope_ref() else {
            return Ok(SubscriptionDirtyMatch::WholeAspect);
        };
        graph.telemetry_mut().partition_scoped_invalidation_checks += 1;
        // Diagnostics store records aggregate counts at the invalidation boundary.
        if let Some(interned_scope) = dep.interned_scope() {
            for changed_scope_id in changed_scope_ids {
                if interned_partition_scope_matches(interned_scope, *changed_scope_id) {
                    return Ok(match scope.match_mode {
                        PartitionMatchMode::WholePartition => {
                            SubscriptionDirtyMatch::WholePartition
                        }
                        PartitionMatchMode::PartitionAndDetail => {
                            SubscriptionDirtyMatch::PartitionAndDetail
                        }
                    });
                }
            }
        } else {
            for changed_scope in changed_scopes {
                if partition_scope_matches(scope, changed_scope) {
                    return Ok(match scope.match_mode {
                        PartitionMatchMode::WholePartition => {
                            SubscriptionDirtyMatch::WholePartition
                        }
                        PartitionMatchMode::PartitionAndDetail => {
                            SubscriptionDirtyMatch::PartitionAndDetail
                        }
                    });
                }
            }
        }
        return Ok(SubscriptionDirtyMatch::Unmatched);
    }
    Ok(SubscriptionDirtyMatch::Unmatched)
}

#[derive(Debug, Clone, Copy, Default)]
struct InvalidationStats {
    invalidated_direct_subscribers: u32,
    maybe_stale_direct_subscribers: u32,
    partition_scoped_checks: u32,
}

fn intern_changed_regions(
    graph: &mut SignalGraph,
    changed_regions: &[ChangedRegion],
) -> Vec<InternedPartitionSubscription> {
    changed_regions
        .iter()
        .map(|region| graph.partition_interner_mut().intern_changed_region(region))
        .collect()
}

fn interned_partition_scope_matches(
    subscription: InternedPartitionSubscription,
    changed: InternedPartitionSubscription,
) -> bool {
    if subscription.partition != changed.partition {
        return false;
    }
    match subscription.match_mode {
        PartitionMatchMode::WholePartition => true,
        PartitionMatchMode::PartitionAndDetail => subscription.detail == changed.detail,
    }
}

fn changed_regions_to_dirty_scopes(
    changed_regions: &[ChangedRegion],
) -> Vec<PartitionSubscription> {
    changed_regions
        .iter()
        .map(|region| {
            if let Some(detail) = &region.detail {
                PartitionSubscription::partition_and_detail(
                    region.partition.clone(),
                    detail.clone(),
                )
            } else {
                PartitionSubscription::whole_partition(region.partition.clone())
            }
        })
        .collect()
}

fn partition_scope_matches(
    subscription: &PartitionSubscription,
    changed: &PartitionSubscription,
) -> bool {
    if subscription.partition != changed.partition {
        return false;
    }
    match subscription.match_mode {
        PartitionMatchMode::WholePartition => true,
        PartitionMatchMode::PartitionAndDetail => subscription.detail == changed.detail,
    }
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
    changed_scopes: &[PartitionSubscription],
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
                entry.set_dirty_partition_scopes(changed_scopes.iter().cloned());
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

enum SubscriptionDirtyMatch {
    WholeAspect,
    WholePartition,
    PartitionAndDetail,
    Unmatched,
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
