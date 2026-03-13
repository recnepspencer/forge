use crate::data::aspect::Aspect;
use crate::data::bitset::BitsetFrontier;
use crate::data::error::SignalError;
use crate::data::graph::{ScratchLeaseKind, SignalGraph, TraversalScratch};
use crate::data::handle::NodeId;
use crate::data::node::NodeState;
use crate::data::output::{ChangedRegion, InternedPartitionSubscription, PartitionSubscription};
use crate::data::proof::{
    DedupedNodeBatch, DirtyBatch, FrontierWave, InvalidationFrontier, NarrowedPropagationSet,
    PartitionScopeSet, SemanticBatchCommit, SortedSourceBatch,
};
use crate::diagnostics::failure::{ExecutionFailureContext, ExecutionFailurePhase};
use crate::diagnostics::lineage::InvalidationCause;
use crate::diagnostics::recorder::{record_invalidation_lineage, DiagnosticsRecorder};
use std::ops::DerefMut;

use super::cycles::detect_reachable_cycles;
use super::subscription::{subscribes_to_aspect, SubscriptionDirtyMatch};

#[cfg(any(test, doctest))]
pub fn mark_dirty(
    mut graph: impl DerefMut<Target = SignalGraph>,
    source: NodeId,
    changed_aspect: Aspect,
) -> Result<(), SignalError> {
    let _ = mark_dirty_batch(
        graph.deref_mut(),
        &DirtyBatch::singleton(source, changed_aspect, Vec::<ChangedRegion>::new()),
    )?;
    Ok(())
}

#[cfg(any(test, doctest))]
pub fn mark_dirty_with_regions(
    mut graph: impl DerefMut<Target = SignalGraph>,
    source: NodeId,
    changed_aspect: Aspect,
    changed_regions: &[ChangedRegion],
) -> Result<(), SignalError> {
    let _ = mark_dirty_batch(
        graph.deref_mut(),
        &DirtyBatch::singleton(source, changed_aspect, changed_regions.to_vec()),
    )?;
    Ok(())
}

pub fn mark_dirty_batch(
    mut graph: impl DerefMut<Target = SignalGraph>,
    dirty: &DirtyBatch,
) -> Result<SemanticBatchCommit, SignalError> {
    let graph = graph.deref_mut();
    for entry in dirty.as_slice() {
        graph.note_change_input(
            entry.source,
            entry.changed_aspect,
            entry.changed_regions.as_slice(),
        );
    }
    let result = graph.with_scratch(ScratchLeaseKind::Invalidation, |graph, scratch| {
        let scratch = scratch.traversal_mut();
        let (arena, _, _, _) = graph.as_parts_mut();
        let len = arena.len();
        let entries = dirty.as_slice();
        let mut start = 0usize;
        while start < entries.len() {
            let changed_aspect = entries[start].changed_aspect;
            scratch.visited.next_pass(len);
            scratch.node_buffer_a.clear();
            scratch.node_buffer_b.clear();
            scratch.node_buffer_c.clear();
            let mut end = start;
            while end < entries.len() && entries[end].changed_aspect == changed_aspect {
                scratch.cycle_visiting.next_pass(len);
                scratch.cycle_finished.next_pass(len);
                let frontier = mark_dirty_with_scratch(
                    graph,
                    scratch,
                    entries[end].source,
                    entries[end].changed_aspect,
                    entries[end].changed_regions.as_slice(),
                )?;
                scratch
                    .node_buffer_c
                    .extend_from_slice(frontier.wave.direct_subscribers.as_slice());
                end += 1;
            }
            let narrowed = NarrowedPropagationSet::new(
                changed_aspect,
                SortedSourceBatch::canonicalize_unordered(
                    entries[start..end].iter().map(|entry| entry.source),
                ),
                PartitionScopeSet::from_changed_regions(
                    &crate::data::output::CanonicalChangedRegions::canonicalize_unordered(
                        entries[start..end]
                            .iter()
                            .flat_map(|entry| entry.changed_regions.as_slice().iter().cloned()),
                    ),
                ),
            );
            let frontier = InvalidationFrontier::new(
                narrowed,
                FrontierWave::new(
                    DedupedNodeBatch::canonicalize_unordered(scratch.node_buffer_c.iter().copied()),
                    DedupedNodeBatch::canonicalize_unordered(scratch.node_buffer_b.iter().copied()),
                ),
            );
            graph.telemetry_mut().invalidation.narrowed_frontier_width +=
                frontier.wave.direct_subscribers.len() as u64;
            graph.telemetry_mut().invalidation.transitive_frontier_width +=
                frontier.wave.transitive_frontier.len() as u64;
            graph.record_invalidation_diagnostics(
                0,
                0,
                0,
                frontier.wave.direct_subscribers.len() as u32,
                frontier.wave.transitive_frontier.len() as u32,
            );
            propagate_maybe_stale(graph, scratch, &frontier)?;
            start = end;
        }
        Ok(())
    });
    if let Err(err) = result {
        graph.clear_pending_diagnostics_input();
        DiagnosticsRecorder::new(graph).record_failure(ExecutionFailureContext::from_error(
            ExecutionFailurePhase::Invalidation,
            &err,
            None,
        ));
        return Err(err);
    }
    Ok(SemanticBatchCommit::new(dirty.clone()))
}

fn mark_dirty_with_scratch(
    graph: &mut SignalGraph,
    scratch: &mut TraversalScratch,
    source: NodeId,
    changed_aspect: Aspect,
    changed_regions: &[ChangedRegion],
) -> Result<InvalidationFrontier, SignalError> {
    let mut traversal =
        InvalidationTraversal::new(graph, scratch, source, changed_aspect, changed_regions);
    traversal.mark_source()?;
    traversal.collect_direct_subscribers();
    traversal.ensure_acyclic_reachability()?;
    traversal.mark_direct_subscribers()?;
    traversal.append_transitive_frontier();
    Ok(traversal.frontier())
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
                InvalidationCause::SourceAspectChanged {
                    aspect_index: self.changed_aspect.index(),
                },
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
            0,
            0,
        );
        for &node in &self.scratch.node_buffer_a {
            self.scratch.visited.mark(node.index() as usize);
        }
        Ok(())
    }

    fn append_transitive_frontier(&mut self) {
        for &subscriber in &self.scratch.node_buffer_a {
            append_live_subscribers(self.graph, subscriber, &mut self.scratch.node_buffer_b);
        }
    }

    fn frontier(&self) -> InvalidationFrontier {
        InvalidationFrontier::new(
            NarrowedPropagationSet::new(
                self.changed_aspect,
                SortedSourceBatch::canonicalize_unordered(std::iter::once(self.source)),
                self.changed_scopes.clone(),
            ),
            FrontierWave::new(
                DedupedNodeBatch::canonicalize_unordered(
                    self.scratch.node_buffer_a.iter().copied(),
                ),
                DedupedNodeBatch::canonicalize_unordered(
                    self.scratch.node_buffer_b.iter().copied(),
                ),
            ),
        )
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
    let changed_mask = crate::data::aspect::AspectMask::from_aspect(changed_aspect);
    for &sub in direct_subs {
        let contract_cares = graph
            .get_contract(sub)?
            .cares_about_change(changed_mask, changed_scopes);
        if !contract_cares {
            continue;
        }
        let checks_before = graph
            .telemetry()
            .invalidation
            .partition_scoped_invalidation_checks;
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
            .invalidation
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
                InvalidationCause::DirectDependencyChanged {
                    dependency: source,
                    aspect_index: changed_aspect.index(),
                },
            );
        }
        graph
            .telemetry_mut()
            .invalidation
            .invalidation_nodes_visited += 1;
        match dirty_match {
            SubscriptionDirtyMatch::WholePartition => {
                graph
                    .telemetry_mut()
                    .invalidation
                    .partition_match_dirty_count += 1;
            }
            SubscriptionDirtyMatch::PartitionAndDetail => {
                graph.telemetry_mut().invalidation.detail_match_dirty_count += 1;
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
    let (_, _, _, observation) = graph.as_parts_mut();
    let token_count_before = observation.partition_interner_mut().token_count();
    let interned = changed_regions
        .iter()
        .map(|region| {
            observation
                .partition_interner_mut()
                .intern_changed_region(region)
        })
        .collect::<Vec<_>>();
    let token_count_after = observation.partition_interner_mut().token_count();
    observation
        .telemetry_mut()
        .invalidation
        .partition_interner_growth_delta +=
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
    frontier: &InvalidationFrontier,
) -> Result<(), SignalError> {
    let mut wave = BitsetFrontier::new();
    for &node in frontier.wave.transitive_frontier.as_slice() {
        wave.seed(node.index() as usize);
    }

    while wave.has_current() {
        scratch.node_buffer_a.clear();
        scratch.node_buffer_a.extend(
            wave.current_iter()
                .filter_map(|idx| graph.live_node_id_at(idx)),
        );
        for &node in &scratch.node_buffer_a {
            graph
                .telemetry_mut()
                .invalidation
                .invalidation_nodes_visited += 1;
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
                    entry.transition_maybe_stale(frontier.narrowed.changed_aspect);
                }
                if matches!(previous_state, NodeState::Clean) {
                    record_invalidation_lineage(
                        graph,
                        node,
                        InvalidationCause::TransitiveDependencyChanged {
                            aspect_index: frontier.narrowed.changed_aspect.index(),
                        },
                    );
                }
            }

            let Ok(subscribers) = graph.runtime_subscribers_of(node) else {
                continue;
            };
            for &subscriber in subscribers {
                if !scratch.visited.is_marked(subscriber.index() as usize) {
                    wave.mark_next(subscriber.index() as usize);
                }
            }
        }
        wave.advance();
    }

    Ok(())
}
