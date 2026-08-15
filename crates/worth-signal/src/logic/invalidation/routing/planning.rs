use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::proof::{
    DedupedNodeBatch, DirtyBatch, FrontierPlan, InvalidationPlanningEstimate, PartitionScopeSet,
    SortedSourceBatch, TouchedScopeSummary,
};

use super::seeds::prepare_invalidation_seed_batch;

pub(super) fn plan_invalidation_frontier(
    graph: &mut SignalGraph,
    dirty: &DirtyBatch,
) -> Result<FrontierPlan, SignalError> {
    let seed_batch = prepare_invalidation_seed_batch(graph, dirty);
    let seed_scopes = PartitionScopeSet::new(
        seed_batch
            .as_slice()
            .iter()
            .flat_map(|seed| seed.changed_scopes.as_slice().iter().cloned()),
    );
    let touched_nodes =
        DedupedNodeBatch::new(seed_batch.as_slice().iter().map(|seed| seed.source_node));
    let touched_sources =
        SortedSourceBatch::new(seed_batch.as_slice().iter().map(|seed| seed.source_node));
    let predicted = InvalidationPlanningEstimate {
        seed_count: seed_batch.as_slice().len() as u64,
        ..InvalidationPlanningEstimate::default()
    };
    let touched_scope_summary = TouchedScopeSummary::new_invalidation(
        seed_scopes,
        PartitionScopeSet::default(),
        PartitionScopeSet::default(),
        PartitionScopeSet::default(),
        touched_nodes,
        touched_sources,
    );
    Ok(FrontierPlan::new(
        seed_batch,
        Vec::new(),
        touched_scope_summary,
        predicted,
    ))
}
