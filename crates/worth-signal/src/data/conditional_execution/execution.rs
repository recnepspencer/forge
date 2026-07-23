use crate::data::comparator::ComparatorPolicyResolver;
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::output::NodeEvaluationResult;
use worth_proof::{ExecuteReadyRecipeTransition, Transition};

mod application;
mod preparation;

use super::artifact_reuse::{resolve_artifact_reuse, SignalConditionalArtifactReuseObservation};
use super::dependency_versions::{record_dependency_versions, SignalConditionalDependencyVersion};
use super::execution_proof::SignalConditionalExecutedRecipe;
use super::{
    InstalledSignalConditionResolver, InstalledSignalConditionalContract,
    SignalConditionalDecisionClass, SignalConditionalDecisionCounters,
    SignalConditionalDecisionEvidence,
};
use application::{apply_passive, ConditionalResolutionAttempt};
use preparation::{prepare_conditional_attempt, PreparedConditionalAttempt};

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
        let providers = ConditionalExecutionProviders {
            condition: condition_resolver,
            comparator: comparator_resolver,
            compute: Some(compute),
        };
        let result = execute_conditional_attempt(self, request, providers, &mut counters);
        result.map_err(|error| SignalConditionalExecutionFailure { error, counters })
    }
}

struct ConditionalExecutionProviders<'a, Condition, Comparator, Compute> {
    condition: &'a mut Condition,
    comparator: &'a mut Comparator,
    compute: Option<Compute>,
}

struct ConditionalExecutionCompletion {
    dependency_versions: Vec<SignalConditionalDependencyVersion>,
    ready: super::execution_proof::SignalConditionalReadyRecipe,
    node: crate::data::handle::NodeId,
    output_aspect: crate::data::aspect::Aspect,
    output_version_before: u64,
    dependency_changed: bool,
}

struct ConditionalFinalization<'a> {
    request: SignalConditionalExecutionRequest<'a>,
    completion: ConditionalExecutionCompletion,
    class: SignalConditionalDecisionClass,
}

fn execute_conditional_attempt<Condition, Comparator, Compute>(
    graph: &mut SignalGraph,
    request: SignalConditionalExecutionRequest<'_>,
    mut providers: ConditionalExecutionProviders<'_, Condition, Comparator, Compute>,
    counters: &mut SignalConditionalDecisionCounters,
) -> Result<SignalConditionalDecisionEvidence, SignalError>
where
    Condition: InstalledSignalConditionResolver,
    Comparator: ComparatorPolicyResolver,
    Compute: FnOnce() -> Result<NodeEvaluationResult, SignalError>,
{
    let prepared = prepare_conditional_attempt(graph, &request, providers.comparator, counters)?;
    let PreparedConditionalAttempt {
        dependency_versions,
        ready,
        node,
        output_aspect,
        output_version_before,
        dependencies,
        dependency_changed,
        passive_dependency_hit,
    } = prepared;
    let class = if passive_dependency_hit {
        counters.application_contacts += 1;
        apply_passive(graph, node, dependencies, providers.comparator)?;
        SignalConditionalDecisionClass::DependencyUnchanged
    } else {
        counters.condition_checks += 1;
        ConditionalResolutionAttempt {
            graph,
            request: &request,
            providers: &mut providers,
            dependencies,
            counters,
        }
        .resolve()?
    };
    finalize_conditional_attempt(
        graph,
        ConditionalFinalization {
            request,
            completion: ConditionalExecutionCompletion {
                dependency_versions,
                ready,
                node,
                output_aspect,
                output_version_before,
                dependency_changed,
            },
            class,
        },
        providers.comparator,
        counters,
    )
}

fn finalize_conditional_attempt(
    graph: &mut SignalGraph,
    finalization: ConditionalFinalization<'_>,
    comparator: &mut impl ComparatorPolicyResolver,
    counters: &mut SignalConditionalDecisionCounters,
) -> Result<SignalConditionalDecisionEvidence, SignalError> {
    let ConditionalFinalization {
        request,
        completion,
        class,
    } = finalization;
    retain_outcome_counter(class, counters);
    counters.output_version_reads += 1;
    let output_version_after =
        graph.node_version_for_scope(completion.node, completion.output_aspect, None)?;
    let artifact_reuse_admitted = resolve_artifact_reuse(
        SignalConditionalArtifactReuseObservation {
            policy: request.contract.artifact_reuse(),
            class,
            dependency_changed: completion.dependency_changed,
            aspect: completion.output_aspect,
            before: completion.output_version_before,
            after: output_version_after,
        },
        comparator,
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
    let executed = ExecuteReadyRecipeTransition
        .transition(completion.ready)
        .into_value();
    counters.decisions_delivered += 1;
    Ok(mint_evidence(
        request,
        ConditionalDecisionOutcome {
            class,
            counters: *counters,
            artifact_reuse_admitted,
            dependency_versions: completion.dependency_versions,
            executed,
        },
    ))
}

fn retain_outcome_counter(
    class: SignalConditionalDecisionClass,
    counters: &mut SignalConditionalDecisionCounters,
) {
    match class {
        SignalConditionalDecisionClass::ComputedRevertedClean => {
            counters.reverted_clean_outcomes += 1;
        }
        SignalConditionalDecisionClass::DeferredByCondition => {
            counters.condition_deferrals += 1;
        }
        SignalConditionalDecisionClass::DeferredTemporal => {
            counters.temporal_deferrals += 1;
        }
        SignalConditionalDecisionClass::DeferredOnDemand => {
            counters.on_demand_deferrals += 1;
        }
        SignalConditionalDecisionClass::ComputedChanged
        | SignalConditionalDecisionClass::DependencyUnchanged
        | SignalConditionalDecisionClass::SuppressedBeforeCompute => {}
    }
}

struct ConditionalDecisionOutcome {
    class: SignalConditionalDecisionClass,
    counters: SignalConditionalDecisionCounters,
    artifact_reuse_admitted: bool,
    dependency_versions: Vec<SignalConditionalDependencyVersion>,
    executed: SignalConditionalExecutedRecipe,
}

fn mint_evidence(
    request: SignalConditionalExecutionRequest<'_>,
    outcome: ConditionalDecisionOutcome,
) -> SignalConditionalDecisionEvidence {
    let projection_basis = super::identity::decision_projection_basis(
        request.contract,
        request.snapshot_identity,
        request.execution_identity,
        request.attempt,
        outcome.class,
        &outcome.dependency_versions,
    );
    let (authority, projection) =
        super::identity::mint_signal_conditional_decision_identity(projection_basis);
    SignalConditionalDecisionEvidence {
        _authority: authority,
        projection,
        contract_authority: std::sync::Arc::clone(&request.contract.authority),
        attempt: request.attempt,
        class: outcome.class,
        counters: outcome.counters,
        artifact_reuse_admitted: outcome.artifact_reuse_admitted,
        _dependency_versions: outcome.dependency_versions,
        _execution: outcome.executed,
    }
}
