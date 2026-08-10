use crate::data::aspect::AspectMask;
use crate::data::handle::NodeId;
use crate::data::output::CanonicalChangedRegions;
use crate::data::proof::{
    DedupedNodeBatch, DirtyDelta, PartitionScopeSet, SortedSourceBatch, StructuralDelta,
    TouchedScopeSummary,
};

use crate::logic::planner::types::LoweredTask;

pub(super) fn build_apply_footprint(
    node: NodeId,
    current_dependencies: &crate::data::dependency::CanonicalDependencies,
    next_dependencies: &crate::data::dependency::CanonicalDependencies,
) -> crate::logic::planner::types::ApplyFootprint {
    let mut touched_nodes = vec![node];
    touched_nodes.extend(
        current_dependencies
            .as_slice()
            .iter()
            .map(|edge| edge.source()),
    );
    touched_nodes.extend(
        next_dependencies
            .as_slice()
            .iter()
            .map(|edge| edge.source()),
    );
    let mut touched_sources = current_dependencies
        .as_slice()
        .iter()
        .map(|edge| edge.source())
        .collect::<Vec<_>>();
    touched_sources.extend(
        next_dependencies
            .as_slice()
            .iter()
            .map(|edge| edge.source()),
    );
    let partitions = PartitionScopeSet::new(
        current_dependencies
            .as_slice()
            .iter()
            .chain(next_dependencies.as_slice().iter())
            .filter_map(|edge| edge.scope_ref().cloned()),
    );
    crate::logic::planner::types::ApplyFootprint {
        partitions,
        touched_nodes: DedupedNodeBatch::new(touched_nodes),
        touched_sources: SortedSourceBatch::new(touched_sources),
    }
}

pub(super) fn build_lowered_dirty_delta(tasks: &[LoweredTask]) -> DirtyDelta {
    let mut changed_aspects = AspectMask::EMPTY;
    let mut changed_regions = Vec::new();
    let mut touched_nodes = Vec::new();
    for task in tasks {
        changed_aspects = changed_aspects | task.produced_aspects();
        changed_regions.extend_from_slice(&task.execution().prepared().result.changed_regions);
        touched_nodes.push(task.node());
    }
    DirtyDelta::new(
        changed_aspects,
        CanonicalChangedRegions::new(changed_regions),
        DedupedNodeBatch::new(touched_nodes),
    )
}

pub(super) fn build_touched_scope_summary(tasks: &[LoweredTask]) -> TouchedScopeSummary {
    let mut scopes = Vec::new();
    let mut touched_nodes = Vec::new();
    let mut touched_sources = Vec::new();
    for task in tasks {
        touched_nodes.push(task.node());
        touched_sources.extend_from_slice(task.footprint().touched_sources.as_slice());
        scopes.extend(
            task.dependency_inputs()
                .as_slice()
                .iter()
                .filter_map(|edge| edge.scope_ref().cloned()),
        );
    }
    TouchedScopeSummary::new(scopes, touched_nodes, touched_sources)
}

pub(super) fn structural_delta(
    dirty_delta: DirtyDelta,
    touched_scope: TouchedScopeSummary,
) -> StructuralDelta {
    StructuralDelta::new(Some(dirty_delta), Some(touched_scope))
}
