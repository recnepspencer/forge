use crate::data::aspect::AspectMask;
use crate::data::dependency::CanonicalDependencies;
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::host_computed::{admit_or_error, HostComputedApiFamily};
use crate::data::node::AuthorityPolicy;
use crate::data::output::{ChangedRegion, PartitionSubscription};
use crate::logic::planner::semantic::StageSemanticIdentity;
use crate::logic::planner::types::ExecutionRecordId;
use crate::logic::prepared::{
    PreparedEvaluation, PreparedEvaluationOrigin, PreparedEvaluationOutcome,
};

use super::preparation::SerialFinalizeSeed;

#[derive(Debug, Clone)]
pub(super) struct LoweredSerialTask {
    pub(super) node: NodeId,
    pub(super) record_id: ExecutionRecordId,
    pub(super) desired_dependencies: CanonicalDependencies,
    pub(super) prepared: PreparedEvaluation,
    pub(super) dependency_updates: u32,
}

#[derive(Debug, Clone)]
pub(super) struct SerialStageLoweringMaterial {
    pub(super) task: LoweredSerialTask,
    pub(super) finalize_seed: SerialFinalizeSeed,
    pub(super) produced_aspects: AspectMask,
    pub(super) changed_regions: Vec<ChangedRegion>,
    pub(super) touched_sources: Vec<NodeId>,
    pub(super) touched_scopes: Vec<PartitionSubscription>,
    pub(super) authority_policy: AuthorityPolicy,
}

pub(super) fn lower_serial_task_patch(
    graph: &mut SignalGraph,
    patch: crate::logic::planner::precompute::PreparedTaskPatch,
    stage_identities: &[StageSemanticIdentity],
) -> Result<SerialStageLoweringMaterial, SignalError> {
    let task_index = patch.task_index;
    let node = patch.node;
    let prepared = patch.prepared;
    graph.refresh_runtime_dependencies_of(node)?;
    let current_dependencies =
        CanonicalDependencies::from_slice(graph.current_runtime_dependencies_of(node)?);
    let admitted = {
        let mut telemetry_guard = graph.telemetry_mut();
        let telemetry = telemetry_guard.as_deref_mut();
        admit_or_error(
            HostComputedApiFamily::CorePreparedEvaluation,
            node,
            current_dependencies.as_slice(),
            prepared,
            telemetry,
        )?
    };
    let (prepared, _admitted_reads, dependency_patch) = admitted.into_parts();
    let next_dependencies = CanonicalDependencies::from_slice(dependency_patch.next_dependencies());
    let before_state = graph.get_state(node)?;
    let before_artifact_state = graph.node_runtime_artifact_finalize_image(node)?;
    let contract = graph.get_contract(node)?;
    let recomputed = matches!(prepared.outcome, PreparedEvaluationOutcome::Evaluate)
        && !matches!(prepared.origin, PreparedEvaluationOrigin::MemoizedReuse);
    let partition_aware = !prepared.result.changed_regions.is_empty();
    let rewiring = super::super::lowering_support::rewiring_summary_from_lowered_edges(
        current_dependencies.as_slice(),
        next_dependencies.as_slice(),
    );
    let dependency_updates = super::super::lowering_support::count_dependency_updates(
        current_dependencies.as_slice(),
        next_dependencies.as_slice(),
    );
    let touched_sources = current_dependencies
        .as_slice()
        .iter()
        .chain(next_dependencies.as_slice().iter())
        .map(|edge| edge.source())
        .collect::<Vec<_>>();
    let touched_scopes = next_dependencies
        .as_slice()
        .iter()
        .filter_map(|edge| edge.scope_ref().cloned())
        .collect::<Vec<_>>();
    let changed_regions = prepared.result.changed_regions.clone();
    let identity = stage_identities[task_index];
    let finalize_seed = SerialFinalizeSeed::from_execution_parts(
        task_index,
        node,
        identity,
        before_state,
        before_artifact_state,
        dependency_updates,
        recomputed,
        partition_aware,
        rewiring,
    );

    Ok(SerialStageLoweringMaterial {
        task: LoweredSerialTask {
            node,
            record_id: identity.record_id,
            desired_dependencies: next_dependencies,
            prepared,
            dependency_updates,
        },
        finalize_seed,
        produced_aspects: contract.semantics.produces,
        changed_regions,
        touched_sources,
        touched_scopes,
        authority_policy: contract.authority.policy,
    })
}
