use crate::data::aspect::Aspect;
use crate::data::bitset::BitsetFrontier;
use crate::data::error::SignalError;
use crate::data::graph::{ScratchLeaseKind, SignalGraph, TraversalScratch};
use crate::data::handle::NodeId;
use crate::data::node::NodeState;
use crate::data::output::{
    ChangedRegion, InternedPartitionSubscription, PartitionSubscription,
};
use crate::diagnostics::failure::{ExecutionFailureContext, ExecutionFailurePhase};
use crate::diagnostics::recorder::{record_invalidation_lineage, DiagnosticsRecorder};
use std::ops::DerefMut;

use super::cycles::detect_reachable_cycles;
use super::subscription::{subscribes_to_aspect, SubscriptionDirtyMatch};

pub fn mark_dirty(
    mut graph: impl DerefMut<Target = SignalGraph>,
    source: NodeId,
    changed_aspect: Aspect,
) -> Result<(), SignalError> {
    mark_dirty_with_regions(graph.deref_mut(), source, changed_aspect, &[])
}

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
        propagate_maybe_stale(self.graph, self.scratch, self.changed_aspect)
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct InvalidationStats {
    invalidated_direct_subscribers: u32,
    maybe_stale_direct_subscribers: u32,
    partition_scoped_checks: u32,
}

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
    out.extend(subscribers.iter().copied());
}

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
