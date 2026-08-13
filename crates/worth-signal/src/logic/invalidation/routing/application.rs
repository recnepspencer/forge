use crate::data::bitset::BitsetFrontier;
use crate::data::error::SignalError;
use crate::data::graph::{SignalGraph, TraversalScratch};
use crate::data::node::NodeState;
use crate::data::proof::{
    DedupedNodeBatch, FrontierExecutionCounters, FrontierExecutionSummary, FrontierPlan,
    FrontierWaveEntryPlan, FrontierWaveEntrySummary, FrontierWaveSummary, PartitionScopeSet,
    SortedSourceBatch, TouchedScopeSummary, TransitiveFrontierEntrySummary,
    TransitiveFrontierWaveSummary,
};
use crate::diagnostics::lineage::InvalidationCause;
use crate::diagnostics::recorder::record_invalidation_lineage;

use super::super::cycles::detect_reachable_cycles;
use super::counters::record_execution_counters;

pub(super) fn execute_invalidation_frontier(
    graph: &mut SignalGraph,
    scratch: &mut TraversalScratch,
    plan: &FrontierPlan,
) -> Result<FrontierExecutionSummary, SignalError> {
    let (arena, _, _, _) = graph.as_parts_mut();
    let len = arena.len();
    let mut direct_waves = Vec::new();
    let mut transitive_waves = Vec::new();
    let mut touched_nodes = plan
        .seed_batch
        .as_slice()
        .iter()
        .map(|seed| seed.source_node)
        .collect::<Vec<_>>();
    let touched_sources = SortedSourceBatch::new(
        plan.seed_batch
            .as_slice()
            .iter()
            .map(|seed| seed.source_node),
    );
    let maybe_stale_scopes = plan
        .touched_scope_summary
        .maybe_stale_scopes
        .as_slice()
        .to_vec();

    let mut counters = FrontierExecutionCounters {
        frontier_seed_count: plan.seed_batch.as_slice().len() as u64,
        frontier_group_count: plan.direct_waves.len() as u64,
        frontier_direct_wave_count: plan.direct_waves.len() as u64,
        frontier_transitive_wave_count: 0,
        frontier_partition_scoped_check_count: plan.predicted.partition_scoped_checks,
        frontier_direct_dirty_count: plan.predicted.direct_dirty_count,
        frontier_maybe_stale_count: plan.predicted.maybe_stale_count,
        frontier_partition_match_count: plan.predicted.partition_match_count,
        frontier_detail_match_count: plan.predicted.detail_match_count,
        frontier_cycle_check_candidate_count: plan.predicted.cycle_check_candidate_count,
        ..FrontierExecutionCounters::default()
    };

    let cycle_candidates = plan
        .direct_waves
        .iter()
        .flat_map(|wave| wave.entries.iter().map(|entry| entry.node))
        .collect::<Vec<_>>();
    scratch.cycle_visiting.next_pass(len);
    scratch.cycle_finished.next_pass(len);
    counters.frontier_cycle_check_visited_count =
        detect_reachable_cycles(graph, scratch, &cycle_candidates)?;

    for seed in plan.seed_batch.as_slice() {
        super::mark_source_seed(graph, seed)?;
    }

    for wave in &plan.direct_waves {
        scratch.visited.next_pass(len);
        let mut summary_entries = Vec::with_capacity(wave.entries.len());
        for entry in &wave.entries {
            super::apply_direct_entry(graph, entry, &plan.seed_batch)?;
            summary_entries.push(FrontierWaveEntrySummary::new(
                entry.node,
                entry.classification,
                entry.inclusion_basis,
                entry.narrowed_scopes.clone(),
            ));
            scratch.visited.mark(entry.node.index() as usize);
            touched_nodes.push(entry.node);
            graph
                .telemetry_mut()
                .invalidation
                .invalidation_nodes_visited += 1;
        }

        let mut transitive_summary_entries = Vec::new();
        execute_transitive_wave(
            graph,
            scratch,
            &wave.entries,
            &mut transitive_summary_entries,
        )?;
        counters.frontier_maybe_stale_count += transitive_summary_entries.len() as u64;
        touched_nodes.extend(transitive_summary_entries.iter().map(|entry| entry.node));

        graph.telemetry_mut().invalidation.narrowed_frontier_width += summary_entries.len() as u64;
        graph.telemetry_mut().invalidation.transitive_frontier_width +=
            transitive_summary_entries.len() as u64;
        if !transitive_summary_entries.is_empty() {
            counters.frontier_transitive_wave_count += 1;
        }

        direct_waves.push(FrontierWaveSummary::new(
            wave.wave_index,
            wave.aspect,
            summary_entries,
        ));
        transitive_waves.push(TransitiveFrontierWaveSummary::new(
            wave.wave_index,
            transitive_summary_entries,
        ));
    }

    let touched_scope_summary = TouchedScopeSummary::new_invalidation(
        plan.touched_scope_summary.seed_scopes.clone(),
        plan.touched_scope_summary.inclusion_scopes.clone(),
        plan.touched_scope_summary.direct_dirty_scopes.clone(),
        PartitionScopeSet::new(maybe_stale_scopes),
        DedupedNodeBatch::new(touched_nodes),
        touched_sources,
    );
    record_execution_counters(graph, &counters);

    Ok(FrontierExecutionSummary::new(
        plan.seed_batch.as_slice().len() as u64,
        direct_waves,
        transitive_waves,
        touched_scope_summary,
        counters,
    ))
}

fn execute_transitive_wave(
    graph: &mut SignalGraph,
    scratch: &mut TraversalScratch,
    roots: &[FrontierWaveEntryPlan],
    out: &mut Vec<TransitiveFrontierEntrySummary>,
) -> Result<(), SignalError> {
    let mut wave = BitsetFrontier::new();
    for root in roots {
        if let Ok(subscribers) = graph.runtime_subscribers_of(root.node) {
            for &subscriber in subscribers {
                wave.seed(subscriber.index() as usize);
            }
        }
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

            let previous_state = graph.get_state(node)?;
            graph.transition_node_pending_revalidation(node)?;
            if matches!(previous_state, NodeState::Clean) {
                record_invalidation_lineage(
                    graph,
                    node,
                    InvalidationCause::PendingDependencyRevalidation { upstream: None },
                );
            }

            out.push(TransitiveFrontierEntrySummary::new(node));

            if let Ok(subscribers) = graph.runtime_subscribers_of(node) {
                for &subscriber in subscribers {
                    if !scratch.visited.is_marked(subscriber.index() as usize) {
                        wave.mark_next(subscriber.index() as usize);
                    }
                }
            }
        }
        wave.advance();
    }
    Ok(())
}
