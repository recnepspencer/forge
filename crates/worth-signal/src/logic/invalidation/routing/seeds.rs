use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::output::{ChangedRegion, InternedPartitionSubscription};
use crate::data::proof::{
    DirtyBatch, FrontierSeedCause, InvalidationSeed, InvalidationSeedBatch, PartitionScopeSet,
};

pub(super) fn prepare_invalidation_seed_batch(
    graph: &mut SignalGraph,
    dirty: &DirtyBatch,
) -> (
    InvalidationSeedBatch,
    Vec<Vec<InternedPartitionSubscription>>,
) {
    let mut seeds = Vec::with_capacity(dirty.as_slice().len());
    let mut scoped_ids = Vec::with_capacity(dirty.as_slice().len());
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
    (InvalidationSeedBatch::new(seeds), scoped_ids)
}

pub(super) fn intern_changed_regions(
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

pub(super) fn collect_live_subscribers_into(
    graph: &mut SignalGraph,
    node: NodeId,
    out: &mut Vec<NodeId>,
) {
    out.clear();
    let Ok(subscribers) = graph.runtime_subscribers_of(node) else {
        return;
    };
    out.extend(subscribers.iter().copied());
}
