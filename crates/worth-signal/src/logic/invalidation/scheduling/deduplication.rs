use crate::data::proof::invalidation::progression::{
    InvalidationProgressionOwner, InvalidationWorkBindingAxes, ReadyInvalidationBatch,
};
use crate::data::proof::invalidation::revalidation::NodeInvalidationInput;

pub(crate) fn merge_repeated_current_admission(
    graph: &mut crate::data::graph::SignalGraph,
    target: crate::data::handle::NodeId,
) -> Result<(), crate::data::error::SignalError> {
    let NodeInvalidationInput::Resolved(input) = graph.node_invalidation_input(target)? else {
        return Err(crate::data::error::SignalError::invalid_input(
            "repeated admission requires current resolved invalidation work",
        ));
    };
    if input.dependency_causes().is_none() {
        return Err(crate::data::error::SignalError::invalid_input(
            "repeated admission requires canonical dependency causes",
        ));
    }
    graph.record_repeated_invalidation_admission(target);
    Ok(())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(super) struct ReadyWorkKey {
    graph_instance: u64,
    target: crate::data::handle::NodeId,
    dependency_revision: crate::data::proof::invalidation::binding::DependencyRevision,
    readiness_epoch: crate::data::proof::invalidation::progression::InvalidationReadinessEpoch,
    stage: u32,
}

impl ReadyWorkKey {
    pub(super) fn from_ready(ready: &ReadyInvalidationBatch) -> Self {
        let axes = InvalidationProgressionOwner::ready_binding(ready);
        Self {
            graph_instance: axes.graph_instance,
            target: axes.target,
            dependency_revision: axes.dependency_revision,
            readiness_epoch: axes.readiness_epoch,
            stage: axes.stage_order.stage,
        }
    }
}

pub(super) fn same_canonical_work(
    left: &ReadyInvalidationBatch,
    right: &ReadyInvalidationBatch,
) -> bool {
    let left = binding(left);
    let right = binding(right);
    left.graph_instance == right.graph_instance
        && left.target == right.target
        && left.dependency_revision == right.dependency_revision
        && left.origin == right.origin
        && left.readiness_epoch == right.readiness_epoch
        && left.stage_order.stage == right.stage_order.stage
}

fn binding(ready: &ReadyInvalidationBatch) -> &InvalidationWorkBindingAxes {
    InvalidationProgressionOwner::ready_binding(ready)
}
