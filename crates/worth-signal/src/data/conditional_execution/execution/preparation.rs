use crate::data::comparator::ComparatorPolicyResolver;
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::node::EvaluationCondition;
use crate::logic::prepared::PreparedDependencyCapture;

use super::super::dependency_versions::{
    dependency_change_is_meaningful, observed_dependency_versions,
    SignalConditionalDependencyVersion,
};
use super::super::execution_proof::{prepare_execution_proof, SignalConditionalReadyRecipe};
use super::super::SignalConditionalDecisionCounters;
use super::SignalConditionalExecutionRequest;

pub(super) struct PreparedConditionalAttempt {
    pub(super) dependency_versions: Vec<SignalConditionalDependencyVersion>,
    pub(super) ready: SignalConditionalReadyRecipe,
    pub(super) node: crate::data::handle::NodeId,
    pub(super) output_aspect: crate::data::aspect::Aspect,
    pub(super) output_version_before: u64,
    pub(super) dependencies: PreparedDependencyCapture,
    pub(super) dependency_changed: bool,
    pub(super) passive_dependency_hit: bool,
}

pub(super) fn prepare_conditional_attempt(
    graph: &mut SignalGraph,
    request: &SignalConditionalExecutionRequest<'_>,
    comparator: &mut impl ComparatorPolicyResolver,
    counters: &mut SignalConditionalDecisionCounters,
) -> Result<PreparedConditionalAttempt, SignalError> {
    validate_request(graph, request, counters)?;
    let dependency_versions = observed_dependency_versions(graph, request.contract)?;
    counters.dependency_observation_reads += dependency_versions.len();
    let ready = prepare_execution_proof(graph, request, &dependency_versions)?;
    let node = request.contract.node();
    let output_aspect = crate::data::aspect::Aspect::new(0);
    counters.output_version_reads += 1;
    let output_version_before = graph.node_version_for_scope(node, output_aspect, None)?;
    let dependencies = capture_dependencies(graph, node, counters)?;
    let dependency_changed =
        dependency_change_is_meaningful(graph, request.contract, comparator, counters)?;
    let has_dependencies =
        !request.contract.dependency_aspects().is_empty() || !dependencies.as_slice().is_empty();
    let external_trigger_requested = request.force_on_demand
        || matches!(
            request.contract.condition(),
            EvaluationCondition::Installed(identity)
                if matches!(
                    identity.role(),
                    crate::data::node::InstalledSignalConditionRole::TemporalWake
                )
        );
    Ok(PreparedConditionalAttempt {
        dependency_versions,
        ready,
        node,
        output_aspect,
        output_version_before,
        dependencies,
        dependency_changed,
        passive_dependency_hit: !dependency_changed
            && has_dependencies
            && !external_trigger_requested,
    })
}

fn validate_request(
    graph: &SignalGraph,
    request: &SignalConditionalExecutionRequest<'_>,
    counters: &mut SignalConditionalDecisionCounters,
) -> Result<(), SignalError> {
    counters.request_admission_checks += 1;
    if request.contract.graph_instance_id() != graph.runtime_instance_id()
        || request.attempt == 0
        || request.snapshot_identity.is_empty()
        || request.execution_identity.is_empty()
    {
        return Err(SignalError::invalid_input(
            "conditional request carried a foreign graph, empty snapshot, or zero attempt",
        ));
    }
    counters.contract_lookups += 1;
    graph.get_contract(request.contract.node()).map(|_| ())
}

fn capture_dependencies(
    graph: &mut SignalGraph,
    node: crate::data::handle::NodeId,
    counters: &mut SignalConditionalDecisionCounters,
) -> Result<PreparedDependencyCapture, SignalError> {
    graph.refresh_runtime_dependencies_of(node)?;
    let mut capture = PreparedDependencyCapture::new();
    for edge in graph.current_runtime_dependencies_of(node)? {
        counters.runtime_dependency_edges_captured += 1;
        capture.record(edge.source(), edge.aspect(), edge.scope_ref().cloned());
    }
    Ok(capture)
}
