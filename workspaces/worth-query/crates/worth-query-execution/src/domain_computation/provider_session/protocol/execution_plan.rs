use std::sync::Arc;

use worth_query_installation::facade::WorthQueryInstalledGraphParticipationAuthority;

use super::{
    WorthQueryProviderExecutionPlanContract, WorthQueryProviderPlanExecutionBinding,
    WorthQueryProviderSessionDenialKind, WorthQueryProviderSessionFailure,
    WorthQueryProviderSessionProtocolCounters, WorthQueryProviderSessionProtocolStage,
};
use crate::domain_computation::managed_run::{
    WorthQueryRunningDirectRun, WorthQueryRunningWorkflowRun,
};
use crate::domain_computation::provider_session::graph_provider::bounded_step::provider_anchor::WorthQueryGraphProviderAnchor;

struct WorthQueryProviderPlanAuthorityObservation<'a> {
    operation: &'a crate::domain_computation::WorthQueryExecutionBoundOperationAuthority,
    session_identity: &'a str,
    session_attempt_identity: &'a str,
    resource_authority_matches: bool,
    evidence_session_identity: &'a str,
    evidence_attempt_identity: &'a str,
    bridge: &'a worth_runtime_bridge::facade::BridgeBoundExecutionBasis,
    graph: &'a WorthQueryInstalledGraphParticipationAuthority,
    stage_identity: Option<&'a str>,
}

pub(super) enum WorthQueryProviderRunBorrow<'run> {
    Direct(&'run mut WorthQueryRunningDirectRun),
    Workflow(&'run mut WorthQueryRunningWorkflowRun),
}

impl WorthQueryProviderRunBorrow<'_> {
    pub(super) fn run_identity(&self) -> &str {
        match self {
            Self::Direct(run) => run.identity(),
            Self::Workflow(run) => run.identity(),
        }
    }
}

pub struct WorthQueryAdmittedProviderExecutionPlan<'run> {
    pub(super) run: WorthQueryProviderRunBorrow<'run>,
    pub(super) contract: WorthQueryProviderExecutionPlanContract,
    pub(super) provider: Arc<WorthQueryGraphProviderAnchor>,
    pub(super) counters: WorthQueryProviderSessionProtocolCounters,
}

impl std::fmt::Debug for WorthQueryAdmittedProviderExecutionPlan<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("WorthQueryAdmittedProviderExecutionPlan")
            .field("identity", &self.contract.identity())
            .field("run_identity", &self.run.run_identity())
            .finish_non_exhaustive()
    }
}

impl<'run> WorthQueryAdmittedProviderExecutionPlan<'run> {
    pub(crate) fn direct(
        run: &'run mut WorthQueryRunningDirectRun,
        graph: &WorthQueryInstalledGraphParticipationAuthority,
    ) -> Result<Self, WorthQueryProviderSessionFailure> {
        let mut counters = WorthQueryProviderSessionProtocolCounters::default();
        let operation = run.provider_plan_operation();
        let session = run.provider_plan_session();
        let (resources, evidence) = run.provider_plan_resources();
        counters.checked_authority();
        validate_common_authority(
            WorthQueryProviderPlanAuthorityObservation {
                operation,
                session_identity: session.identity(),
                session_attempt_identity: session.attempt_identity(),
                resource_authority_matches: operation
                    .admits_provider_plan_resources(None, resources),
                evidence_session_identity: evidence.provider_session_identity(),
                evidence_attempt_identity: evidence.provider_session_attempt_identity(),
                bridge: run.provider_plan_bridge_basis(),
                graph,
                stage_identity: None,
            },
            &counters,
        )?;
        let provider = retain_session_provider(graph, &counters)?;
        let snapshot_identity = run.execution_snapshot_reference();
        let contract = operation
            .provider_plan_contract(
                None,
                WorthQueryProviderPlanExecutionBinding {
                    managed_run_identity: run.identity(),
                    execution_basis_identity: run.provider_plan_bridge_basis().identity().as_str(),
                    admitted_session_identity: session.identity(),
                    resource_attempt_identity: session.attempt_identity(),
                    graph,
                    snapshot_identity: &snapshot_identity,
                    resource_envelope_identity: resources.envelope_identity(),
                    provider_identity: provider.provider_identity(),
                    provider_generation: provider.provider_generation(),
                },
            )
            .ok_or_else(|| undeclared_scope(&counters))?;
        counters.bound_closure_items(contract.closure_width());
        Ok(Self {
            run: WorthQueryProviderRunBorrow::Direct(run),
            contract,
            provider,
            counters,
        })
    }

    pub(in crate::domain_computation) fn workflow_stage(
        run: &'run mut WorthQueryRunningWorkflowRun,
        stage_identity: &str,
        graph: &WorthQueryInstalledGraphParticipationAuthority,
        owner: &crate::domain_computation::managed_run::WorthQueryWorkflowProviderPlanPermit,
    ) -> Result<Self, WorthQueryProviderSessionFailure> {
        let mut counters = WorthQueryProviderSessionProtocolCounters::default();
        let (resources, evidence) = run
            .provider_plan_stage_resources(stage_identity, owner)
            .ok_or_else(|| undeclared_scope(&counters))?;
        let operation = run.provider_plan_operation(owner);
        let session = run.provider_plan_session(owner);
        counters.checked_authority();
        validate_common_authority(
            WorthQueryProviderPlanAuthorityObservation {
                operation,
                session_identity: session.identity(),
                session_attempt_identity: session.attempt_identity(),
                resource_authority_matches: operation
                    .admits_provider_plan_resources(Some(stage_identity), &resources),
                evidence_session_identity: evidence.provider_session_identity(),
                evidence_attempt_identity: evidence.provider_session_attempt_identity(),
                bridge: run.provider_plan_bridge_basis(owner),
                graph,
                stage_identity: Some(stage_identity),
            },
            &counters,
        )?;
        let provider = retain_session_provider(graph, &counters)?;
        let snapshot_identity = run.execution_snapshot_reference();
        let contract = operation
            .provider_plan_contract(
                Some(stage_identity),
                WorthQueryProviderPlanExecutionBinding {
                    managed_run_identity: run.identity(),
                    execution_basis_identity: run
                        .provider_plan_bridge_basis(owner)
                        .identity()
                        .as_str(),
                    admitted_session_identity: session.identity(),
                    resource_attempt_identity: session.attempt_identity(),
                    graph,
                    snapshot_identity: &snapshot_identity,
                    resource_envelope_identity: resources.envelope_identity(),
                    provider_identity: provider.provider_identity(),
                    provider_generation: provider.provider_generation(),
                },
            )
            .ok_or_else(|| undeclared_scope(&counters))?;
        counters.bound_closure_items(contract.closure_width());
        Ok(Self {
            run: WorthQueryProviderRunBorrow::Workflow(run),
            contract,
            provider,
            counters,
        })
    }

    pub fn identity(&self) -> &str {
        self.contract.identity()
    }

    pub fn contract(&self) -> &WorthQueryProviderExecutionPlanContract {
        &self.contract
    }

    pub fn counters(&self) -> WorthQueryProviderSessionProtocolCounters {
        self.counters
    }
}

fn validate_common_authority(
    observation: WorthQueryProviderPlanAuthorityObservation<'_>,
    counters: &WorthQueryProviderSessionProtocolCounters,
) -> Result<(), WorthQueryProviderSessionFailure> {
    if !observation.operation.is_current_installation_generation() {
        return Err(failure(
            WorthQueryProviderSessionDenialKind::ForeignOperationAttempt,
            "provider plan operation belongs to a stale installation generation",
            counters,
        ));
    }
    if !observation.resource_authority_matches
        || observation.session_identity != observation.evidence_session_identity
        || observation.session_attempt_identity != observation.evidence_attempt_identity
    {
        return Err(failure(
            WorthQueryProviderSessionDenialKind::ForeignOperationAttempt,
            "provider plan inputs do not belong to the exact operation attempt",
            counters,
        ));
    }
    let intent = observation.bridge.managed_intent();
    if intent.operation_binding_identity() != observation.operation.binding_identity()
        || intent.resource_attempt_identity() != observation.session_attempt_identity
    {
        return Err(failure(
            WorthQueryProviderSessionDenialKind::ForeignExecutionBasis,
            "provider plan bridge basis belongs to a different managed intent",
            counters,
        ));
    }
    if !observation
        .operation
        .admits_provider_plan_graph(observation.stage_identity, observation.graph)
    {
        return Err(failure(
            WorthQueryProviderSessionDenialKind::ForeignGraphAuthority,
            "provider plan graph authority is not installed for this operation scope",
            counters,
        ));
    }
    Ok(())
}

fn retain_session_provider(
    graph: &WorthQueryInstalledGraphParticipationAuthority,
    counters: &WorthQueryProviderSessionProtocolCounters,
) -> Result<Arc<WorthQueryGraphProviderAnchor>, WorthQueryProviderSessionFailure> {
    let provider = graph
        .retain_provider_anchor::<WorthQueryGraphProviderAnchor>()
        .ok_or_else(|| {
            failure(
                WorthQueryProviderSessionDenialKind::ProviderIdentityMismatch,
                "installed graph authority does not retain the Query provider anchor",
                counters,
            )
        })?;
    if provider.provider_identity() != graph.provider_identity() {
        return Err(failure(
            WorthQueryProviderSessionDenialKind::ProviderIdentityMismatch,
            "installed graph provider identity differs from the retained provider",
            counters,
        ));
    }
    if !provider.supports_session_protocol() {
        return Err(failure(
            WorthQueryProviderSessionDenialKind::SessionProtocolUnsupported,
            "installed graph provider does not implement the sealed session protocol",
            counters,
        ));
    }
    Ok(provider)
}

fn undeclared_scope(
    counters: &WorthQueryProviderSessionProtocolCounters,
) -> WorthQueryProviderSessionFailure {
    failure(
        WorthQueryProviderSessionDenialKind::UndeclaredOperationScope,
        "provider plan scope is absent from the installed read/touch closure",
        counters,
    )
}

fn failure(
    kind: WorthQueryProviderSessionDenialKind,
    detail: &'static str,
    counters: &WorthQueryProviderSessionProtocolCounters,
) -> WorthQueryProviderSessionFailure {
    WorthQueryProviderSessionFailure::new(
        kind,
        WorthQueryProviderSessionProtocolStage::PlanAdmission,
        detail,
        *counters,
    )
}
