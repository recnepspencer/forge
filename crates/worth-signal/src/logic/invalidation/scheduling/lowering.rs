use worth_proof::TransitionOutcome;

use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::proof::invalidation::progression::{
    InvalidationOriginBinding, InvalidationOriginBindingAxes, InvalidationProgressionDenial,
    InvalidationProgressionOwner, InvalidationReadinessEpoch, InvalidationStageOrder,
    InvalidationWorkBatch, InvalidationWorkItem, LoweredInvalidationBatch,
};
use crate::data::proof::invalidation::revalidation::{
    CanonicalDependencyCauseSet, CanonicalInvalidationOrigin,
};

pub(crate) fn lower_current_work(
    graph: &SignalGraph,
    target: NodeId,
    input: CanonicalDependencyCauseSet,
    readiness_epoch: InvalidationReadinessEpoch,
    stage_order: InvalidationStageOrder,
) -> Result<LoweredInvalidationBatch, SignalError> {
    let revision = graph.dependency_revision(target)?;
    let origin = current_origin_binding(graph, target, &input)?;
    let axes = InvalidationOriginBindingAxes {
        graph_instance: graph.runtime_instance_id(),
        target,
        dependency_revision: revision,
        origin,
    };
    let batch = InvalidationWorkBatch::single(InvalidationWorkItem::new(target, revision, input));
    match InvalidationProgressionOwner::resolve_origin(batch, axes) {
        TransitionOutcome::Success(resolved) => Ok(InvalidationProgressionOwner::lower(
            resolved,
            readiness_epoch,
            stage_order,
        )),
        TransitionOutcome::Denied(denial)
        | TransitionOutcome::Deferred(denial)
        | TransitionOutcome::Stale(denial)
        | TransitionOutcome::RebindRequired(denial) => Err(progression_denial(denial)),
        TransitionOutcome::Failed(error) => Err(error),
    }
}

fn current_origin_binding(
    graph: &SignalGraph,
    target: NodeId,
    input: &CanonicalDependencyCauseSet,
) -> Result<InvalidationOriginBinding, SignalError> {
    match input.origin() {
        CanonicalInvalidationOrigin::SourceRecompute => {
            let generation = input.origin_generation().ok_or_else(|| {
                SignalError::internal("source invalidation omitted its admission generation")
            })?;
            Ok(InvalidationOriginBinding::SourceAdmission { generation })
        }
        CanonicalInvalidationOrigin::DependencyCommit => {
            let mut producer_commit_ordinals = input
                .dependency_causes()
                .ok_or_else(|| SignalError::internal("dependency invalidation omitted causes"))?
                .iter()
                .map(|cause| cause.binding_axes.output_commit_ordinal)
                .collect::<Vec<_>>();
            producer_commit_ordinals.sort_unstable();
            producer_commit_ordinals.dedup();
            Ok(InvalidationOriginBinding::DependencyCommit {
                cause_set: graph.pending_cause_set_id(target)?,
                producer_commit_ordinals,
            })
        }
        CanonicalInvalidationOrigin::StructuralRecompute => {
            let ordinal = input.origin_generation().ok_or_else(|| {
                SignalError::internal("structural invalidation omitted its mutation ordinal")
            })?;
            Ok(InvalidationOriginBinding::StructuralMutation { ordinal })
        }
    }
}

fn progression_denial(denial: InvalidationProgressionDenial) -> SignalError {
    SignalError::invalid_input(format!(
        "invalidation work could not enter topology lowering: {denial:?}"
    ))
}
