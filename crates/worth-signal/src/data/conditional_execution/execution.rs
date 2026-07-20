use crate::data::comparator::ComparatorPolicyResolver;
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::node::EvaluationCondition;
use crate::data::output::NodeEvaluationResult;
use crate::logic::evaluation::EvaluationVerdict;
use crate::logic::prepared::{PreparedDependencyCapture, PreparedEvaluation};
use worth_proof::{ExecuteReadyRecipeTransition, Transition};

use super::artifact_reuse::resolve_artifact_reuse;
use super::condition_resolution::{resolve_condition, ConditionDisposition};
use super::dependency_versions::{
    dependency_change_is_meaningful, observed_dependency_versions, record_dependency_versions,
    SignalConditionalDependencyVersion,
};
use super::execution_proof::{prepare_execution_proof, SignalConditionalExecutedRecipe};
use super::{
    InstalledSignalConditionResolver, InstalledSignalConditionalContract,
    SignalConditionalDecisionClass, SignalConditionalDecisionCounters,
    SignalConditionalDecisionEvidence,
};

pub struct SignalConditionalExecutionRequest<'a> {
    pub(super) contract: &'a InstalledSignalConditionalContract,
    pub(super) snapshot_identity: &'a str,
    pub(super) execution_identity: &'a str,
    pub(super) attempt: u64,
    pub(super) force_on_demand: bool,
}

#[derive(Debug)]
pub struct SignalConditionalExecutionFailure {
    error: SignalError,
    counters: SignalConditionalDecisionCounters,
}

impl SignalConditionalExecutionFailure {
    pub const fn counters(&self) -> SignalConditionalDecisionCounters {
        self.counters
    }

    pub fn into_error(self) -> SignalError {
        self.error
    }
}

impl<'a> SignalConditionalExecutionRequest<'a> {
    pub fn new(
        contract: &'a InstalledSignalConditionalContract,
        snapshot_identity: &'a str,
        execution_identity: &'a str,
        attempt: u64,
    ) -> Self {
        Self {
            contract,
            snapshot_identity,
            execution_identity,
            attempt,
            force_on_demand: false,
        }
    }

    pub fn force_on_demand(mut self) -> Self {
        self.force_on_demand = true;
        self
    }
}

impl SignalGraph {
    pub fn execute_installed_conditional(
        &mut self,
        request: SignalConditionalExecutionRequest<'_>,
        condition_resolver: &mut impl InstalledSignalConditionResolver,
        comparator_resolver: &mut impl ComparatorPolicyResolver,
        compute: impl FnOnce() -> Result<NodeEvaluationResult, SignalError>,
    ) -> Result<SignalConditionalDecisionEvidence, SignalConditionalExecutionFailure> {
        let mut counters = SignalConditionalDecisionCounters::default();
        let result = execute_conditional_attempt(
            self,
            request,
            condition_resolver,
            comparator_resolver,
            compute,
            &mut counters,
        );
        result.map_err(|error| SignalConditionalExecutionFailure { error, counters })
    }
}

fn execute_conditional_attempt(
    graph: &mut SignalGraph,
    request: SignalConditionalExecutionRequest<'_>,
    condition_resolver: &mut impl InstalledSignalConditionResolver,
    comparator_resolver: &mut impl ComparatorPolicyResolver,
    compute: impl FnOnce() -> Result<NodeEvaluationResult, SignalError>,
    counters: &mut SignalConditionalDecisionCounters,
) -> Result<SignalConditionalDecisionEvidence, SignalError> {
    validate_request(graph, &request)?;
    let observed_dependency_versions = observed_dependency_versions(graph, request.contract)?;
    let ready = prepare_execution_proof(graph, &request, &observed_dependency_versions)?;
    let node = request.contract.node();
    let output_aspect = crate::data::aspect::Aspect::new(0);
    let output_version_before = graph.node_version_for_scope(node, output_aspect, None)?;
    let dependencies = capture_dependencies(graph, node)?;
    let dependency_changed =
        dependency_change_is_meaningful(graph, request.contract, comparator_resolver, counters)?;
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
    let class = if !dependency_changed && has_dependencies && !external_trigger_requested {
        apply_passive(graph, node, dependencies, comparator_resolver)?;
        SignalConditionalDecisionClass::DependencyUnchanged
    } else {
        counters.condition_checks += 1;
        resolve_and_apply_condition(
            graph,
            &request,
            condition_resolver,
            comparator_resolver,
            compute,
            dependencies,
            counters,
        )?
    };
    let output_version_after = graph.node_version_for_scope(node, output_aspect, None)?;
    let artifact_reuse_admitted = resolve_artifact_reuse(
        request.contract.artifact_reuse(),
        class,
        dependency_changed,
        output_aspect,
        output_version_before,
        output_version_after,
        comparator_resolver,
        counters,
    )?;
    if !matches!(
        class,
        SignalConditionalDecisionClass::DeferredByCondition
            | SignalConditionalDecisionClass::DeferredTemporal
            | SignalConditionalDecisionClass::DeferredOnDemand
    ) {
        record_dependency_versions(graph, request.contract)?;
    }
    let executed = ExecuteReadyRecipeTransition.transition(ready).into_value();
    Ok(mint_evidence(
        request,
        class,
        *counters,
        artifact_reuse_admitted,
        observed_dependency_versions,
        executed,
    ))
}

fn resolve_and_apply_condition(
    graph: &mut SignalGraph,
    request: &SignalConditionalExecutionRequest<'_>,
    condition_resolver: &mut impl InstalledSignalConditionResolver,
    comparator_resolver: &mut impl ComparatorPolicyResolver,
    compute: impl FnOnce() -> Result<NodeEvaluationResult, SignalError>,
    dependencies: PreparedDependencyCapture,
    counters: &mut SignalConditionalDecisionCounters,
) -> Result<SignalConditionalDecisionClass, SignalError> {
    let node = request.contract.node();
    Ok(
        match resolve_condition(graph, request, condition_resolver)? {
            ConditionDisposition::Eligible => {
                counters.compute_contacts += 1;
                let prepared =
                    PreparedEvaluation::from_result(compute()?).with_dependencies(dependencies);
                let applied = crate::logic::evaluation::apply_prepared_evaluation_with_policy(
                    graph,
                    node,
                    prepared,
                    comparator_resolver,
                    None,
                )?;
                match applied.report.verdict {
                    EvaluationVerdict::Recomputed => {
                        counters.semantic_changes += 1;
                        SignalConditionalDecisionClass::ComputedChanged
                    }
                    EvaluationVerdict::Suppressed { .. } => {
                        SignalConditionalDecisionClass::ComputedRevertedClean
                    }
                    EvaluationVerdict::Deferred { .. } => {
                        return Err(SignalError::internal(
                            "computed conditional output cannot become a deferred verdict",
                        ));
                    }
                }
            }
            ConditionDisposition::Suppressed => {
                apply_passive(graph, node, dependencies, comparator_resolver)?;
                SignalConditionalDecisionClass::SuppressedBeforeCompute
            }
            ConditionDisposition::Deferred => {
                apply_deferred(graph, node, dependencies, comparator_resolver)?;
                SignalConditionalDecisionClass::DeferredByCondition
            }
            ConditionDisposition::DeferredTemporal => {
                apply_deferred(graph, node, dependencies, comparator_resolver)?;
                SignalConditionalDecisionClass::DeferredTemporal
            }
            ConditionDisposition::DeferredOnDemand => {
                apply_deferred(graph, node, dependencies, comparator_resolver)?;
                SignalConditionalDecisionClass::DeferredOnDemand
            }
        },
    )
}

fn validate_request(
    graph: &SignalGraph,
    request: &SignalConditionalExecutionRequest<'_>,
) -> Result<(), SignalError> {
    if request.contract.graph_instance_id() != graph.runtime_instance_id()
        || request.attempt == 0
        || request.snapshot_identity.is_empty()
        || request.execution_identity.is_empty()
    {
        return Err(SignalError::invalid_input(
            "conditional request carried a foreign graph, empty snapshot, or zero attempt",
        ));
    }
    graph.get_contract(request.contract.node()).map(|_| ())
}

fn capture_dependencies(
    graph: &mut SignalGraph,
    node: crate::data::handle::NodeId,
) -> Result<PreparedDependencyCapture, SignalError> {
    graph.refresh_runtime_dependencies_of(node)?;
    let mut capture = PreparedDependencyCapture::new();
    for edge in graph.current_runtime_dependencies_of(node)? {
        capture.record(edge.source(), edge.aspect(), edge.scope_ref().cloned());
    }
    Ok(capture)
}

fn apply_passive(
    graph: &mut SignalGraph,
    node: crate::data::handle::NodeId,
    dependencies: PreparedDependencyCapture,
    resolver: &mut impl ComparatorPolicyResolver,
) -> Result<(), SignalError> {
    let prepared =
        PreparedEvaluation::reverted_clean_by_condition().with_dependencies(dependencies);
    crate::logic::evaluation::apply_prepared_evaluation_with_policy(
        graph, node, prepared, resolver, None,
    )?;
    Ok(())
}

fn apply_deferred(
    graph: &mut SignalGraph,
    node: crate::data::handle::NodeId,
    dependencies: PreparedDependencyCapture,
    resolver: &mut impl ComparatorPolicyResolver,
) -> Result<(), SignalError> {
    let prepared = PreparedEvaluation::deferred_by_condition().with_dependencies(dependencies);
    crate::logic::evaluation::apply_prepared_evaluation_with_policy(
        graph, node, prepared, resolver, None,
    )?;
    Ok(())
}

fn mint_evidence(
    request: SignalConditionalExecutionRequest<'_>,
    class: SignalConditionalDecisionClass,
    counters: SignalConditionalDecisionCounters,
    artifact_reuse_admitted: bool,
    dependency_versions: Vec<SignalConditionalDependencyVersion>,
    executed: SignalConditionalExecutedRecipe,
) -> SignalConditionalDecisionEvidence {
    let node = request.contract.node();
    let dependency_identity = dependency_versions
        .iter()
        .map(|dependency| {
            let scope = dependency.scope.as_ref().map_or_else(
                || "unscoped".to_string(),
                |scope| {
                    let mode = match scope.match_mode {
                        crate::data::output::PartitionMatchMode::WholePartition => "whole",
                        crate::data::output::PartitionMatchMode::PartitionAndDetail => "detail",
                    };
                    format!(
                        "{}:{}:{}",
                        scope.partition.0.as_str(),
                        scope.detail.as_deref().unwrap_or("none"),
                        mode
                    )
                },
            );
            format!(
                "{}:{}:{}:{}",
                dependency.node.index(),
                dependency.aspect.index(),
                scope,
                dependency.version
            )
        })
        .collect::<Vec<_>>()
        .join("|");
    let contract_identity = format!(
        "condition={:?}|semantic-condition={:?}|dependencies={}|triggers={}|dependency-comparator={:?}|output-comparator={:?}|reuse={:?}",
        request.contract.condition(), request.contract.semantic_condition(),
        request.contract.dependency_aspects().bits(),
        request.contract.trigger_aspects().bits(),
        request.contract.dependency_comparator(),
        request.contract.output_comparator(),
        request.contract.artifact_reuse(),
    );
    SignalConditionalDecisionEvidence {
        identity: format!(
            "signal-conditional:{}:{}:{}:{}:{}:{class:?}:{}:{}",
            request.contract.graph_instance_id(),
            node.index(),
            node.generation(),
            request.execution_identity,
            request.attempt,
            dependency_identity,
            contract_identity,
        ),
        graph_instance_id: request.contract.graph_instance_id(),
        node,
        snapshot_identity: request.snapshot_identity.to_string(),
        execution_identity: request.execution_identity.to_string(),
        attempt: request.attempt,
        class,
        counters,
        artifact_reuse_admitted,
        condition: request.contract.condition().clone(),
        semantic_condition: request.contract.semantic_condition().clone(),
        dependency_aspects: request.contract.dependency_aspects(),
        trigger_aspects: request.contract.trigger_aspects(),
        dependency_comparator: request.contract.dependency_comparator().clone(),
        output_comparator: request.contract.output_comparator().clone(),
        artifact_reuse: request.contract.artifact_reuse().clone(),
        _dependency_versions: dependency_versions,
        _execution: executed,
    }
}
