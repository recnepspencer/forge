use crate::data::aspect::{Aspect, AspectMask};
use crate::data::bitset::BitsetFrontier;
use crate::data::error::SignalError;
use crate::data::graph::{ScratchLeaseKind, SignalGraph, TraversalScratch};
use crate::data::handle::NodeId;
use crate::data::node::NodeState;
use crate::data::output::{
    scopes_overlap, ChangedRegion, InternedPartitionSubscription, PartitionMatchMode,
    PartitionSubscription,
};
use crate::diagnostics::failure::{ExecutionFailureContext, ExecutionFailurePhase};
use crate::diagnostics::recorder::{record_invalidation_lineage, DiagnosticsRecorder};
use std::ops::DerefMut;

/// Propagate invalidation downstream from a changed source node.
///
/// **Push Phase:**
/// 1. Mark the source node `Dirty`.
/// 2. Direct subscribers that read the changed aspect -> `Dirty`.
///    Direct subscribers reading a different aspect -> `MaybeStale`.
/// 3. All transitive downstream subscribers -> `MaybeStale`.
/// 4. Cycle detection via visited set -> structured error on cycle.
pub fn mark_dirty(
    mut graph: impl DerefMut<Target = SignalGraph>,
    source: NodeId,
    changed_aspect: Aspect,
) -> Result<(), SignalError> {
    mark_dirty_with_regions(graph.deref_mut(), source, changed_aspect, &[])
}

/// Propagate invalidation downstream from a changed source node with changed regions.
pub fn mark_dirty_with_regions(
    mut graph: impl DerefMut<Target = SignalGraph>,
    source: NodeId,
    changed_aspect: Aspect,
    changed_regions: &[ChangedRegion],
) -> Result<(), SignalError> {
    let graph = graph.deref_mut();
    graph.note_change_input(source, changed_aspect, changed_regions);
    let result = graph.with_scratch(ScratchLeaseKind::Invalidation, |graph, scratch| {
        let len = graph.arena_capacity();
        scratch.visited.next_pass(len);
        scratch.cycle_visiting.next_pass(len);
        scratch.cycle_finished.next_pass(len);
        scratch.node_buffer_a.clear();
        scratch.node_buffer_b.clear();
        mark_dirty_with_scratch(graph, scratch, source, changed_aspect, changed_regions)
    });
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
    let mut traversal =
        InvalidationTraversal::new(graph, scratch, source, changed_aspect, changed_regions);
    traversal.mark_source()?;
    traversal.collect_direct_subscribers();
    traversal.ensure_acyclic_reachability()?;
    traversal.mark_direct_subscribers()?;
    traversal.seed_transitive_frontier();
    traversal.propagate_transitive_maybe_stale()
}

struct InvalidationTraversal<'graph, 'scratch> {
    graph: &'graph mut SignalGraph,
    scratch: &'scratch mut TraversalScratch,
    source: NodeId,
    changed_aspect: Aspect,
    changed_scopes: Vec<PartitionSubscription>,
    changed_scope_ids: Vec<InternedPartitionSubscription>,
}

impl<'graph, 'scratch> InvalidationTraversal<'graph, 'scratch> {
    fn new(
        graph: &'graph mut SignalGraph,
        scratch: &'scratch mut TraversalScratch,
        source: NodeId,
        changed_aspect: Aspect,
        changed_regions: &[ChangedRegion],
    ) -> Self {
        let changed_scopes = changed_regions_to_dirty_scopes(changed_regions);
        let changed_scope_ids = intern_changed_regions(graph, changed_regions);
        Self {
            graph,
            scratch,
            source,
            changed_aspect,
            changed_scopes,
            changed_scope_ids,
        }
    }

    fn mark_source(&mut self) -> Result<(), SignalError> {
        let source_was_clean = {
            let source_entry = self.graph.get_entry_mut(self.source)?;
            let source_was_clean = matches!(*source_entry.get_state(), NodeState::Clean);
            source_entry.transition_dirty(self.changed_aspect, &self.changed_scopes);
            source_was_clean
        };
        if source_was_clean {
            record_invalidation_lineage(
                self.graph,
                self.source,
                format!(
                    "source invalidated on aspect {}",
                    self.changed_aspect.index()
                ),
            );
        }
        self.scratch.visited.mark(self.source.index() as usize);
        Ok(())
    }

    fn collect_direct_subscribers(&mut self) {
        collect_live_subscribers_into(self.graph, self.source, &mut self.scratch.node_buffer_a);
    }

    fn ensure_acyclic_reachability(&mut self) -> Result<(), SignalError> {
        let cycle_roots = self.scratch.node_buffer_a.clone();
        detect_reachable_cycles(self.graph, self.scratch, &cycle_roots)
    }

    fn mark_direct_subscribers(&mut self) -> Result<(), SignalError> {
        let invalidation_stats = mark_direct_subscribers(
            self.graph,
            self.source,
            self.changed_aspect,
            &self.changed_scopes,
            &self.changed_scope_ids,
            &self.scratch.node_buffer_a,
        )?;
        self.graph.record_invalidation_diagnostics(
            invalidation_stats.invalidated_direct_subscribers,
            invalidation_stats.maybe_stale_direct_subscribers,
            invalidation_stats.partition_scoped_checks,
        );
        for &node in &self.scratch.node_buffer_a {
            self.scratch.visited.mark(node.index() as usize);
        }
        Ok(())
    }

    fn seed_transitive_frontier(&mut self) {
        self.scratch.node_buffer_b.clear();
        for &subscriber in &self.scratch.node_buffer_a {
            append_live_subscribers(self.graph, subscriber, &mut self.scratch.node_buffer_b);
        }
    }

    fn propagate_transitive_maybe_stale(&mut self) -> Result<(), SignalError> {
        propagate_maybe_stale(
            self.graph,
            self.scratch,
            self.changed_aspect,
            &self.changed_scopes,
        )
    }
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
        let previous_state = graph.get_state(sub)?;
        {
            let entry = graph.get_entry_mut(sub)?;
            match new_state {
                NodeState::Dirty => entry.transition_dirty(changed_aspect, changed_scopes),
                NodeState::MaybeStale => entry.transition_maybe_stale(changed_aspect),
                NodeState::Clean => entry.transition_clean(),
            }
        }
        if matches!(previous_state, NodeState::Clean) {
            record_invalidation_lineage(
                graph,
                sub,
                format!(
                    "direct subscriber invalidated from {} on aspect {}",
                    source.index(),
                    changed_aspect.index()
                ),
            );
        }
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
    let (partition_checks, outcome) = {
        let dependencies = graph.runtime_dependencies_of(downstream)?;
        let mut outcome = SubscriptionDirtyMatch::Unmatched;
        let mut partition_checks = 0_u64;
        let source_key = (source.index(), source.generation());
        let start = dependencies
            .partition_point(|dep| (dep.source().index(), dep.source().generation()) < source_key);
        let end = dependencies
            .partition_point(|dep| (dep.source().index(), dep.source().generation()) <= source_key);

        for dep in &dependencies[start..end] {
            if !dep.aspect_mask().intersects(changed_mask) {
                continue;
            }
            let Some(scope) = dep.scope_ref() else {
                outcome = SubscriptionDirtyMatch::WholeAspect;
                break;
            };
            partition_checks += 1;
            // Diagnostics store records aggregate counts at the invalidation boundary.
            if let Some(interned_scope) = dep.interned_scope() {
                for changed_scope_id in changed_scope_ids {
                    if scopes_overlap(&interned_scope, changed_scope_id) {
                        outcome = match scope.match_mode {
                            PartitionMatchMode::WholePartition => {
                                SubscriptionDirtyMatch::WholePartition
                            }
                            PartitionMatchMode::PartitionAndDetail => {
                                SubscriptionDirtyMatch::PartitionAndDetail
                            }
                        };
                        break;
                    }
                }
            } else {
                for changed_scope in changed_scopes {
                    if scopes_overlap(scope, changed_scope) {
                        outcome = match scope.match_mode {
                            PartitionMatchMode::WholePartition => {
                                SubscriptionDirtyMatch::WholePartition
                            }
                            PartitionMatchMode::PartitionAndDetail => {
                                SubscriptionDirtyMatch::PartitionAndDetail
                            }
                        };
                        break;
                    }
                }
            }
            if !matches!(outcome, SubscriptionDirtyMatch::Unmatched) {
                break;
            }
        }
        (partition_checks, outcome)
    };

    graph.telemetry_mut().partition_scoped_invalidation_checks += partition_checks;
    Ok(outcome)
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
    let token_count_before = graph.partition_interner_mut().token_count();
    let interned = changed_regions
        .iter()
        .map(|region| graph.partition_interner_mut().intern_changed_region(region))
        .collect::<Vec<_>>();
    let token_count_after = graph.partition_interner_mut().token_count();
    graph.telemetry_mut().partition_interner_growth_delta +=
        token_count_after.saturating_sub(token_count_before) as u64;
    interned
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

fn collect_live_subscribers_into(graph: &mut SignalGraph, node: NodeId, out: &mut Vec<NodeId>) {
    out.clear();
    append_live_subscribers(graph, node, out);
}

fn append_live_subscribers(graph: &mut SignalGraph, node: NodeId, out: &mut Vec<NodeId>) {
    let Ok(subscribers) = graph.runtime_subscribers_of(node) else {
        return;
    };
    for &subscriber in subscribers {
        out.push(subscriber);
    }
}

fn detect_reachable_cycles(
    graph: &mut SignalGraph,
    scratch: &mut TraversalScratch,
    candidates: &[NodeId],
) -> Result<(), SignalError> {
    for &candidate in candidates {
        detect_cycle_from(graph, scratch, candidate)?;
    }
    Ok(())
}

fn detect_cycle_from(
    graph: &mut SignalGraph,
    scratch: &mut TraversalScratch,
    node: NodeId,
) -> Result<(), SignalError> {
    scratch.cycle_stack.clear();
    scratch.cycle_stack.push((node, false));
    while let Some((current, expanded)) = scratch.cycle_stack.pop() {
        let index = current.index() as usize;
        if expanded {
            scratch.cycle_visiting.clear_mark(index);
            scratch.cycle_finished.mark(index);
            continue;
        }
        if scratch.cycle_finished.is_marked(index) {
            continue;
        }
        if scratch.cycle_visiting.is_marked(index) {
            return Err(circular_reference_error(current));
        }

        scratch.cycle_visiting.mark(index);
        scratch.cycle_stack.push((current, true));
        if let Ok(subscribers) = graph.runtime_subscribers_of(current) {
            for &subscriber in subscribers.iter().rev() {
                scratch.cycle_stack.push((subscriber, false));
            }
        }
    }
    Ok(())
}

/// Walk the transitive frontier, marking all reachable nodes `MaybeStale`.
fn propagate_maybe_stale(
    graph: &mut SignalGraph,
    scratch: &mut TraversalScratch,
    changed_aspect: Aspect,
    _changed_scopes: &[PartitionSubscription],
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
                continue;
            }

            scratch.visited.mark(node.index() as usize);

            let already_dirty = matches!(
                graph.get_entry(node).map(|entry| *entry.get_state()),
                Ok(NodeState::Dirty)
            );

            if !already_dirty {
                let previous_state = graph.get_state(node)?;
                {
                    let entry = graph.get_entry_mut(node)?;
                    entry.transition_maybe_stale(changed_aspect);
                }
                if matches!(previous_state, NodeState::Clean) {
                    record_invalidation_lineage(
                        graph,
                        node,
                        format!(
                            "transitive subscriber invalidated by aspect {}",
                            changed_aspect.index()
                        ),
                    );
                }
            }

            let Ok(subscribers) = graph.runtime_subscribers_of(node) else {
                continue;
            };
            for &subscriber in subscribers {
                if !scratch.visited.is_marked(subscriber.index() as usize) {
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

/// Produce a structured error for a circular reference.
fn circular_reference_error(node: NodeId) -> SignalError {
    SignalError::InvalidInput {
        message: format!("Circular reference detected at signal node: {}", node),
        context: None,
    }
}
