use std::collections::BTreeMap;
use std::ops::DerefMut;

use crate::data::aspect::Aspect;
use crate::data::bitset::BitsetFrontier;
use crate::data::error::SignalError;
use crate::data::graph::{ScratchLeaseKind, SignalGraph, TraversalScratch};
use crate::data::handle::NodeId;
use crate::data::node::NodeState;
use crate::data::output::{ChangedRegion, InternedPartitionSubscription};
use crate::data::proof::{
    DedupedNodeBatch, DirtyBatch, FrontierEntryClassification, FrontierExecutionCounters,
    FrontierExecutionSummary, FrontierInclusionBasis, FrontierPlan, FrontierPredictedCounters,
    FrontierSeedCause, FrontierWaveEntryPlan, FrontierWaveEntrySummary, FrontierWavePlan,
    FrontierWaveSummary, InvalidationSeed, InvalidationSeedBatch, InvalidationTraceRecord,
    PartitionScopeSet, SemanticBatchCommit, SortedSourceBatch, TouchedScopeSummary,
    TransitiveFrontierRoot,
};
use crate::diagnostics::failure::{ExecutionFailureContext, ExecutionFailurePhase};
use crate::diagnostics::lineage::InvalidationCause;
use crate::diagnostics::policy::FrontierTracingPolicy;
use crate::diagnostics::recorder::{record_invalidation_lineage, DiagnosticsRecorder};

use super::cycles::detect_reachable_cycles;
use super::subscription::{subscriber_invalidation_evidence, SubscriptionInvalidationEvidence};

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
    graph.telemetry_mut().invalidation.batch_width += dirty.as_slice().len() as u64;
    for entry in dirty.as_slice() {
        graph.note_change_input(
            entry.source,
            entry.changed_aspect,
            entry.changed_regions.as_slice(),
        );
    }

    let result = graph.with_scratch(ScratchLeaseKind::Invalidation, |graph, scratch| {
        let scratch = scratch.traversal_mut();
        let plan = plan_invalidation_frontier(graph, dirty)?;
        let mut summary = execute_invalidation_frontier(graph, scratch, &plan)?;
        let trace_records = retained_trace_records(graph, &plan)?;
        summary.counters.frontier_trace_retained_count = trace_records.len() as u64;
        graph
            .telemetry_mut()
            .invalidation
            .partition_scoped_invalidation_checks += plan.predicted.partition_scoped_checks;
        graph
            .telemetry_mut()
            .invalidation
            .frontier_trace_retained_count += summary.counters.frontier_trace_retained_count;
        graph.record_frontier_execution_diagnostics(summary, trace_records);
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

fn plan_invalidation_frontier(
    graph: &mut SignalGraph,
    dirty: &DirtyBatch,
) -> Result<FrontierPlan, SignalError> {
    let mut seeds = Vec::with_capacity(dirty.as_slice().len());
    let mut scoped_ids =
        Vec::<Vec<InternedPartitionSubscription>>::with_capacity(dirty.as_slice().len());
    for entry in dirty.as_slice() {
        let seed_scopes = PartitionScopeSet::from_changed_regions(&entry.changed_regions);
        let interned_scope_ids = intern_changed_regions(graph, entry.changed_regions.as_slice());
        scoped_ids.push(interned_scope_ids);
        seeds.push(InvalidationSeed::new(
            entry.source,
            entry.changed_aspect,
            seed_scopes,
            FrontierSeedCause::DirtySource,
        ));
    }
    let seed_batch = InvalidationSeedBatch::new(seeds);
    let mut groups = BTreeMap::<u8, AspectFrontierPlanBuilder>::new();
    let mut partition_scoped_checks = 0_u64;

    for (seed_index, seed) in seed_batch.as_slice().iter().enumerate() {
        let changed_mask = crate::data::aspect::AspectMask::from_aspect(seed.aspect);
        let mut direct_subscribers = Vec::new();
        collect_live_subscribers_into(graph, seed.source_node, &mut direct_subscribers);
        for subscriber in direct_subscribers {
            if !graph
                .get_contract(subscriber)?
                .cares_about_change(changed_mask, seed.changed_scopes.as_slice())
            {
                continue;
            }
            let Some(evidence) = subscriber_invalidation_evidence(
                graph,
                subscriber,
                seed.source_node,
                seed.aspect,
                seed.changed_scopes.as_slice(),
                scoped_ids[seed_index].as_slice(),
            )?
            else {
                continue;
            };
            partition_scoped_checks += evidence.partition_scoped_checks;
            groups
                .entry(seed.aspect.index() as u8)
                .or_insert_with(|| AspectFrontierPlanBuilder::new(seed.aspect))
                .record_direct_entry(
                    subscriber,
                    seed_index as u32,
                    seed.changed_scopes.clone(),
                    evidence,
                );
        }
    }

    let mut direct_waves = Vec::new();
    let mut transitive_roots = Vec::new();
    let mut seed_scopes = Vec::new();
    let mut inclusion_scopes = Vec::new();
    let mut direct_dirty_scopes = Vec::new();
    let mut maybe_stale_scopes = Vec::new();
    let mut touched_nodes = seed_batch
        .as_slice()
        .iter()
        .map(|seed| seed.source_node)
        .collect::<Vec<_>>();
    let touched_sources =
        SortedSourceBatch::new(seed_batch.as_slice().iter().map(|seed| seed.source_node));
    let mut predicted = FrontierPredictedCounters {
        seed_count: seed_batch.as_slice().len() as u64,
        ..FrontierPredictedCounters::default()
    };

    for seed in seed_batch.as_slice() {
        seed_scopes.extend_from_slice(seed.changed_scopes.as_slice());
    }

    for (_, group) in groups {
        let wave_index = direct_waves.len() as u32;
        let wave = group.into_wave_plan(wave_index);
        if wave.entries.is_empty() {
            continue;
        }
        predicted.group_count += 1;
        predicted.direct_wave_count += 1;
        predicted.transitive_wave_count += 1;
        for entry in &wave.entries {
            touched_nodes.push(entry.node);
            inclusion_scopes.extend_from_slice(entry.narrowed_scopes.as_slice());
            match entry.classification {
                FrontierEntryClassification::DirectDirty => {
                    predicted.direct_dirty_count += 1;
                    direct_dirty_scopes.extend_from_slice(entry.narrowed_scopes.as_slice());
                }
                FrontierEntryClassification::MaybeStale => {
                    predicted.maybe_stale_count += 1;
                    maybe_stale_scopes.extend_from_slice(entry.narrowed_scopes.as_slice());
                }
            }
            match entry.inclusion_basis {
                FrontierInclusionBasis::PartitionScopeOverlap => {
                    predicted.partition_match_count += 1;
                }
                FrontierInclusionBasis::DetailScopeOverlap => {
                    predicted.detail_match_count += 1;
                }
                FrontierInclusionBasis::DirectSubscriptionMatch
                | FrontierInclusionBasis::TransitiveReachability => {}
            }
            transitive_roots.push(TransitiveFrontierRoot::new(
                entry.node,
                wave.aspect,
                entry.classification,
                entry.narrowed_scopes.clone(),
                entry.source_seed_refs.iter().copied(),
            ));
        }
        direct_waves.push(wave);
    }

    transitive_roots.sort_unstable_by_key(|root| {
        (
            root.aspect.index(),
            root.node.index(),
            root.node.generation(),
        )
    });
    predicted.partition_scoped_checks = partition_scoped_checks;
    predicted.cycle_check_candidate_count = transitive_roots.len() as u64;
    let touched_scope_summary = TouchedScopeSummary::new_invalidation(
        PartitionScopeSet::new(seed_scopes),
        PartitionScopeSet::new(inclusion_scopes),
        PartitionScopeSet::default(),
        PartitionScopeSet::new(direct_dirty_scopes),
        PartitionScopeSet::new(maybe_stale_scopes),
        DedupedNodeBatch::new(touched_nodes),
        touched_sources,
    );
    Ok(FrontierPlan::new(
        seed_batch,
        direct_waves,
        transitive_roots,
        touched_scope_summary,
        predicted,
    ))
}

fn execute_invalidation_frontier(
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
    let mut transitive_reached_scopes = Vec::new();
    let mut maybe_stale_scopes = plan
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
        .transitive_roots
        .iter()
        .map(|root| root.node)
        .collect::<Vec<_>>();
    scratch.cycle_visiting.next_pass(len);
    scratch.cycle_finished.next_pass(len);
    counters.frontier_cycle_check_visited_count =
        detect_reachable_cycles(graph, scratch, &cycle_candidates)?;

    for seed in plan.seed_batch.as_slice() {
        mark_source_seed(graph, seed)?;
    }

    for wave in &plan.direct_waves {
        scratch.visited.next_pass(len);
        let mut summary_entries = Vec::with_capacity(wave.entries.len());
        for entry in &wave.entries {
            apply_direct_entry(graph, entry, wave.aspect, &plan.seed_batch)?;
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

        let root_scope_union = PartitionScopeSet::new(
            wave.entries
                .iter()
                .flat_map(|entry| entry.narrowed_scopes.as_slice().iter().cloned()),
        );
        let mut transitive_summary_entries = Vec::new();
        execute_transitive_wave(
            graph,
            scratch,
            wave.aspect,
            &wave.entries,
            &root_scope_union,
            &mut transitive_summary_entries,
        )?;
        transitive_reached_scopes.extend(
            transitive_summary_entries
                .iter()
                .flat_map(|entry| entry.narrowed_scopes.as_slice().iter().cloned()),
        );
        maybe_stale_scopes.extend(
            transitive_summary_entries
                .iter()
                .flat_map(|entry| entry.narrowed_scopes.as_slice().iter().cloned()),
        );
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
        transitive_waves.push(FrontierWaveSummary::new(
            wave.wave_index,
            wave.aspect,
            transitive_summary_entries,
        ));
    }

    let touched_scope_summary = TouchedScopeSummary::new_invalidation(
        plan.touched_scope_summary.seed_scopes.clone(),
        plan.touched_scope_summary.inclusion_scopes.clone(),
        PartitionScopeSet::new(transitive_reached_scopes),
        plan.touched_scope_summary.direct_dirty_scopes.clone(),
        PartitionScopeSet::new(maybe_stale_scopes),
        DedupedNodeBatch::new(touched_nodes),
        touched_sources,
    );

    graph.telemetry_mut().invalidation.frontier_seed_count += counters.frontier_seed_count;
    graph.telemetry_mut().invalidation.frontier_group_count += counters.frontier_group_count;
    graph
        .telemetry_mut()
        .invalidation
        .frontier_direct_wave_count += counters.frontier_direct_wave_count;
    graph
        .telemetry_mut()
        .invalidation
        .frontier_transitive_wave_count += counters.frontier_transitive_wave_count;
    graph
        .telemetry_mut()
        .invalidation
        .frontier_direct_dirty_count += counters.frontier_direct_dirty_count;
    graph
        .telemetry_mut()
        .invalidation
        .frontier_maybe_stale_count += counters.frontier_maybe_stale_count;
    graph
        .telemetry_mut()
        .invalidation
        .frontier_partition_match_count += counters.frontier_partition_match_count;
    graph
        .telemetry_mut()
        .invalidation
        .frontier_detail_match_count += counters.frontier_detail_match_count;
    graph
        .telemetry_mut()
        .invalidation
        .partition_match_dirty_count += counters.frontier_partition_match_count;
    graph.telemetry_mut().invalidation.detail_match_dirty_count +=
        counters.frontier_detail_match_count;
    graph
        .telemetry_mut()
        .invalidation
        .frontier_cycle_check_candidate_count += counters.frontier_cycle_check_candidate_count;
    graph
        .telemetry_mut()
        .invalidation
        .frontier_cycle_check_visited_count += counters.frontier_cycle_check_visited_count;

    Ok(FrontierExecutionSummary::new(
        plan.seed_batch.as_slice().len() as u64,
        direct_waves,
        transitive_waves,
        touched_scope_summary,
        counters,
    ))
}

fn retained_trace_records(
    graph: &SignalGraph,
    plan: &FrontierPlan,
) -> Result<Vec<InvalidationTraceRecord>, SignalError> {
    let tracing_policy = graph.runtime_policy().frontier_tracing_policy;
    if matches!(tracing_policy, FrontierTracingPolicy::SummaryOnly) {
        return Ok(Vec::new());
    }

    let mut records = Vec::new();
    for wave in &plan.direct_waves {
        for entry in &wave.entries {
            records.push(InvalidationTraceRecord::new(
                entry.node,
                wave.aspect,
                wave.wave_index,
                entry.classification,
                entry.inclusion_basis,
            ));
        }
    }
    Ok(records)
}

fn execute_transitive_wave(
    graph: &mut SignalGraph,
    scratch: &mut TraversalScratch,
    aspect: Aspect,
    roots: &[FrontierWaveEntryPlan],
    root_scope_union: &PartitionScopeSet,
    out: &mut Vec<FrontierWaveEntrySummary>,
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

            let already_dirty = matches!(
                graph.get_entry(node).map(|entry| *entry.get_state()),
                Ok(NodeState::Dirty)
            );
            if !already_dirty {
                let previous_state = graph.get_state(node)?;
                {
                    let entry = graph.get_entry_mut(node)?;
                    entry.transition_maybe_stale(aspect);
                }
                if matches!(previous_state, NodeState::Clean) {
                    record_invalidation_lineage(
                        graph,
                        node,
                        InvalidationCause::TransitiveDependencyChanged {
                            aspect_index: aspect.index(),
                        },
                    );
                }
            }

            out.push(FrontierWaveEntrySummary::new(
                node,
                FrontierEntryClassification::MaybeStale,
                FrontierInclusionBasis::TransitiveReachability,
                root_scope_union.clone(),
            ));

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

fn apply_direct_entry(
    graph: &mut SignalGraph,
    entry: &FrontierWaveEntryPlan,
    aspect: Aspect,
    seed_batch: &InvalidationSeedBatch,
) -> Result<(), SignalError> {
    let previous_state = graph.get_state(entry.node)?;
    {
        let target = graph.get_entry_mut(entry.node)?;
        match entry.classification {
            FrontierEntryClassification::DirectDirty => {
                target.transition_dirty(aspect, entry.narrowed_scopes.as_slice())
            }
            FrontierEntryClassification::MaybeStale => target.transition_maybe_stale(aspect),
        }
    }
    if matches!(previous_state, NodeState::Clean) {
        record_invalidation_lineage(
            graph,
            entry.node,
            InvalidationCause::DirectDependencyChanged {
                dependency: entry
                    .source_seed_refs
                    .first()
                    .copied()
                    .and_then(|idx| seed_batch.as_slice().get(idx as usize))
                    .map(|seed| seed.source_node)
                    .unwrap_or(entry.node),
                aspect_index: aspect.index(),
            },
        );
    }
    Ok(())
}

fn mark_source_seed(graph: &mut SignalGraph, seed: &InvalidationSeed) -> Result<(), SignalError> {
    let source_was_clean = {
        let source_entry = graph.get_entry_mut(seed.source_node)?;
        let source_was_clean = matches!(*source_entry.get_state(), NodeState::Clean);
        source_entry.transition_dirty(seed.aspect, seed.changed_scopes.as_slice());
        source_was_clean
    };
    if source_was_clean {
        record_invalidation_lineage(
            graph,
            seed.source_node,
            InvalidationCause::SourceAspectChanged {
                aspect_index: seed.aspect.index(),
            },
        );
    }
    Ok(())
}

#[derive(Debug)]
struct PlannedEntry {
    node: NodeId,
    classification: FrontierEntryClassification,
    inclusion_basis: FrontierInclusionBasis,
    narrowed_scopes: PartitionScopeSet,
    source_seed_refs: Vec<u32>,
}

struct AspectFrontierPlanBuilder {
    aspect: Aspect,
    entries: BTreeMap<NodeId, PlannedEntry>,
}

impl AspectFrontierPlanBuilder {
    fn new(aspect: Aspect) -> Self {
        Self {
            aspect,
            entries: BTreeMap::new(),
        }
    }

    fn record_direct_entry(
        &mut self,
        node: NodeId,
        source_seed_ref: u32,
        narrowed_scopes: PartitionScopeSet,
        evidence: SubscriptionInvalidationEvidence,
    ) {
        use std::collections::btree_map::Entry;
        match self.entries.entry(node) {
            Entry::Vacant(slot) => {
                slot.insert(PlannedEntry {
                    node,
                    classification: evidence.classification,
                    inclusion_basis: evidence.inclusion_basis,
                    narrowed_scopes,
                    source_seed_refs: vec![source_seed_ref],
                });
            }
            Entry::Occupied(mut slot) => {
                let current = slot.get_mut();
                current.classification =
                    preferred_classification(current.classification, evidence.classification);
                current.inclusion_basis =
                    preferred_basis(current.inclusion_basis, evidence.inclusion_basis);
                let mut scopes = current.narrowed_scopes.as_slice().to_vec();
                scopes.extend_from_slice(narrowed_scopes.as_slice());
                current.narrowed_scopes = PartitionScopeSet::new(scopes);
                current.source_seed_refs.push(source_seed_ref);
                current.source_seed_refs.sort_unstable();
                current.source_seed_refs.dedup();
            }
        }
    }

    fn into_wave_plan(self, wave_index: u32) -> FrontierWavePlan {
        FrontierWavePlan::new(
            wave_index,
            self.aspect,
            self.entries.into_values().map(|entry| {
                FrontierWaveEntryPlan::new(
                    entry.node,
                    entry.classification,
                    entry.inclusion_basis,
                    entry.narrowed_scopes,
                    entry.source_seed_refs,
                )
            }),
        )
    }
}

fn preferred_classification(
    left: FrontierEntryClassification,
    right: FrontierEntryClassification,
) -> FrontierEntryClassification {
    match (left, right) {
        (FrontierEntryClassification::DirectDirty, _)
        | (_, FrontierEntryClassification::DirectDirty) => FrontierEntryClassification::DirectDirty,
        _ => FrontierEntryClassification::MaybeStale,
    }
}

fn preferred_basis(
    left: FrontierInclusionBasis,
    right: FrontierInclusionBasis,
) -> FrontierInclusionBasis {
    use FrontierInclusionBasis::*;
    let rank = |basis| match basis {
        DirectSubscriptionMatch => 0_u8,
        PartitionScopeOverlap => 1,
        DetailScopeOverlap => 2,
        TransitiveReachability => 3,
    };
    if rank(left) <= rank(right) {
        left
    } else {
        right
    }
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

fn collect_live_subscribers_into(graph: &mut SignalGraph, node: NodeId, out: &mut Vec<NodeId>) {
    out.clear();
    let Ok(subscribers) = graph.runtime_subscribers_of(node) else {
        return;
    };
    out.extend(subscribers.iter().copied());
}
