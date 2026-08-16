use crate::data::graph::SignalGraph;
use crate::data::proof::{
    DirtyBatch, FrontierSeedCause, InvalidationSeed, InvalidationSeedBatch, PartitionScopeSet,
};

pub(super) fn prepare_invalidation_seed_batch(
    _graph: &mut SignalGraph,
    dirty: &DirtyBatch,
) -> InvalidationSeedBatch {
    let mut seeds = Vec::with_capacity(dirty.as_slice().len());
    for entry in dirty.as_slice() {
        let seed_scopes = PartitionScopeSet::from_changed_regions(&entry.changed_regions);
        seeds.push(InvalidationSeed::new(
            entry.source,
            entry.changed_aspect,
            seed_scopes,
            FrontierSeedCause::DirtySource,
        ));
    }
    InvalidationSeedBatch::new(seeds)
}
