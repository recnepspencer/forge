use std::collections::BTreeMap;

use crate::data::aspect::Aspect;
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::proof::{
    DedupedNodeBatch, DirtyBatch, FrontierEntryClassification, FrontierInclusionBasis,
    FrontierPlan, FrontierPredictedCounters, FrontierWavePlan, InvalidationSeedBatch,
    PartitionScopeSet, SortedSourceBatch, TouchedScopeSummary,
};

use super::super::subscription::subscriber_invalidation_evidence;
use super::super::subscription::SubscriptionInvalidationEvidence;
use super::seeds::{collect_live_subscribers_into, prepare_invalidation_seed_batch};

pub(super) fn plan_invalidation_frontier(
    graph: &mut SignalGraph,
    dirty: &DirtyBatch,
) -> Result<FrontierPlan, SignalError> {
    let (seed_batch, scoped_ids) = prepare_invalidation_seed_batch(graph, dirty);
    let (groups, partition_scoped_checks) = collect_direct_groups(graph, &seed_batch, &scoped_ids)?;
    let mut evidence = FrontierEvidence::from_seeds(&seed_batch);
    for group in groups.into_values() {
        evidence.record_group(group);
    }
    Ok(evidence.into_plan(seed_batch, partition_scoped_checks))
}

fn collect_direct_groups(
    graph: &mut SignalGraph,
    seed_batch: &InvalidationSeedBatch,
    scoped_ids: &[Vec<crate::data::output::InternedPartitionSubscription>],
) -> Result<(BTreeMap<u8, AspectFrontierPlanBuilder>, u64), SignalError> {
    let mut groups = BTreeMap::<u8, AspectFrontierPlanBuilder>::new();
    let mut partition_scoped_checks = 0_u64;
    for (seed_index, seed) in seed_batch.as_slice().iter().enumerate() {
        let changed_mask = crate::data::aspect::AspectMask::from_aspect(seed.aspect);
        let mut direct_subscribers = Vec::new();
        collect_live_subscribers_into(graph, seed.source_node, &mut direct_subscribers);
        for subscriber in direct_subscribers {
            graph
                .telemetry_mut()
                .invalidation
                .direct_subscriber_candidates_examined += 1;
            if !graph
                .get_contract(subscriber)?
                .cares_about_change(changed_mask, seed.changed_scopes.as_slice())
            {
                graph
                    .telemetry_mut()
                    .invalidation
                    .direct_contract_rejections += 1;
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
                graph
                    .telemetry_mut()
                    .invalidation
                    .direct_causality_rejections += 1;
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
    Ok((groups, partition_scoped_checks))
}

struct FrontierEvidence {
    direct_waves: Vec<FrontierWavePlan>,
    seed_scopes: Vec<crate::data::output::PartitionSubscription>,
    inclusion_scopes: Vec<crate::data::output::PartitionSubscription>,
    direct_dirty_scopes: Vec<crate::data::output::PartitionSubscription>,
    maybe_stale_scopes: Vec<crate::data::output::PartitionSubscription>,
    touched_nodes: Vec<NodeId>,
    touched_sources: SortedSourceBatch,
    predicted: FrontierPredictedCounters,
}

impl FrontierEvidence {
    fn from_seeds(seed_batch: &InvalidationSeedBatch) -> Self {
        let mut seed_scopes = Vec::new();
        for seed in seed_batch.as_slice() {
            seed_scopes.extend_from_slice(seed.changed_scopes.as_slice());
        }
        Self {
            direct_waves: Vec::new(),
            seed_scopes,
            inclusion_scopes: Vec::new(),
            direct_dirty_scopes: Vec::new(),
            maybe_stale_scopes: Vec::new(),
            touched_nodes: seed_batch
                .as_slice()
                .iter()
                .map(|seed| seed.source_node)
                .collect(),
            touched_sources: SortedSourceBatch::new(
                seed_batch.as_slice().iter().map(|seed| seed.source_node),
            ),
            predicted: FrontierPredictedCounters {
                seed_count: seed_batch.as_slice().len() as u64,
                ..FrontierPredictedCounters::default()
            },
        }
    }

    fn record_group(&mut self, group: AspectFrontierPlanBuilder) {
        let wave_index = self.direct_waves.len() as u32;
        let wave = group.into_wave_plan(wave_index);
        if wave.entries.is_empty() {
            return;
        }
        self.predicted.group_count += 1;
        self.predicted.direct_wave_count += 1;
        self.predicted.transitive_wave_count += 1;
        for entry in &wave.entries {
            self.touched_nodes.push(entry.node);
            self.inclusion_scopes
                .extend_from_slice(entry.narrowed_scopes.as_slice());
            match entry.classification {
                FrontierEntryClassification::DirectDirty => {
                    self.predicted.direct_dirty_count += 1;
                    self.direct_dirty_scopes
                        .extend_from_slice(entry.narrowed_scopes.as_slice());
                }
                FrontierEntryClassification::MaybeStale => {
                    self.predicted.maybe_stale_count += 1;
                    self.maybe_stale_scopes
                        .extend_from_slice(entry.narrowed_scopes.as_slice());
                }
            }
            match entry.inclusion_basis {
                FrontierInclusionBasis::PartitionScopeOverlap => {
                    self.predicted.partition_match_count += 1;
                }
                FrontierInclusionBasis::DetailScopeOverlap => {
                    self.predicted.detail_match_count += 1;
                }
                FrontierInclusionBasis::DirectSubscriptionMatch
                | FrontierInclusionBasis::TransitiveReachability => {}
            }
        }
        self.direct_waves.push(wave);
    }

    fn into_plan(
        mut self,
        seed_batch: InvalidationSeedBatch,
        partition_scoped_checks: u64,
    ) -> FrontierPlan {
        self.predicted.partition_scoped_checks = partition_scoped_checks;
        self.predicted.cycle_check_candidate_count = self
            .direct_waves
            .iter()
            .map(|wave| wave.entries.len() as u64)
            .sum();
        let touched_scope_summary = TouchedScopeSummary::new_invalidation(
            PartitionScopeSet::new(self.seed_scopes),
            PartitionScopeSet::new(self.inclusion_scopes),
            PartitionScopeSet::new(self.direct_dirty_scopes),
            PartitionScopeSet::new(self.maybe_stale_scopes),
            DedupedNodeBatch::new(self.touched_nodes),
            self.touched_sources,
        );
        FrontierPlan::new(
            seed_batch,
            self.direct_waves,
            touched_scope_summary,
            self.predicted,
        )
    }
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
                crate::data::proof::FrontierWaveEntryPlan::new(
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
