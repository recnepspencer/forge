use std::sync::Arc;

use worth_query_admission::facade::resource_admission::WorthQueryAdmittedExecutionResourcePlan;
use worth_runtime_bridge::facade::{
    BridgeExecutionBasisReadmissionCounters, BridgeYieldedExecutionBasisPreflight, RuntimeBridge,
};

use super::super::retained_graph_execution::WorthQueryRetainedManagedGraphExecution;
use super::super::step_contract_admission::{
    admit_managed_step_contract, WorthQueryAdmittedManagedStepContract,
};
use super::super::WorthQueryYieldedWorkflowRun;
use super::workflow_outcome::WorthQueryWorkflowReadmissionDenialKind;
use super::workflow_state::{WorthQueryWorkflowYieldedParts, WorthQueryWorkflowYieldedState};
use crate::domain_computation::provider_session::graph_provider::WorthQueryGraphProviderCallReadmissionPlan;
use crate::domain_computation::{
    WorthQueryExecutionRuntime, WorthQueryWorkflowExecutionResourceAttempt,
};

pub(super) struct WorthQueryWorkflowResumePreflightValidated {
    state: WorthQueryWorkflowYieldedState,
    resource_attempt: WorthQueryWorkflowExecutionResourceAttempt,
    bridge: BridgeYieldedExecutionBasisPreflight,
    execution: WorthQueryRetainedManagedGraphExecution,
    contract: WorthQueryAdmittedManagedStepContract,
    call: WorthQueryGraphProviderCallReadmissionPlan,
    stage_resources: Arc<WorthQueryAdmittedExecutionResourcePlan>,
    binding_identity: String,
    stage_identity: String,
}

pub(super) struct WorthQueryWorkflowResumePreflightValidatedParts {
    pub(super) state: WorthQueryWorkflowYieldedState,
    pub(super) resource_attempt: WorthQueryWorkflowExecutionResourceAttempt,
    pub(super) bridge: BridgeYieldedExecutionBasisPreflight,
    pub(super) execution: WorthQueryRetainedManagedGraphExecution,
    pub(super) contract: WorthQueryAdmittedManagedStepContract,
    pub(super) call: WorthQueryGraphProviderCallReadmissionPlan,
    pub(super) stage_resources: Arc<WorthQueryAdmittedExecutionResourcePlan>,
    pub(super) binding_identity: String,
    pub(super) stage_identity: String,
}

pub(super) struct WorthQueryWorkflowResumePreflightDenied {
    kind: WorthQueryWorkflowReadmissionDenialKind,
    detail: Arc<str>,
    yielded: WorthQueryYieldedWorkflowRun,
    bridge_counters: Option<BridgeExecutionBasisReadmissionCounters>,
}

pub(super) fn validate_workflow_resume_preflight(
    yielded: WorthQueryYieldedWorkflowRun,
    query_runtime: &WorthQueryExecutionRuntime,
    bridge_runtime: &RuntimeBridge,
) -> Result<WorthQueryWorkflowResumePreflightValidated, WorthQueryWorkflowResumePreflightDenied> {
    if let Some((kind, detail)) = query_preflight_denial(&yielded, query_runtime) {
        return Err(WorthQueryWorkflowResumePreflightDenied::new(
            kind, detail, yielded, None,
        ));
    }
    let Some(stage_identity) = yielded.execution.call.stage_identity().map(str::to_owned) else {
        return Err(WorthQueryWorkflowResumePreflightDenied::new(
            WorthQueryWorkflowReadmissionDenialKind::WorkflowStageResourcesUnavailable,
            "retained workflow provider call has no stage identity",
            yielded,
            None,
        ));
    };
    let Some((stage_resources, stage_evidence)) = yielded
        .resource_attempt
        .stage_resources_and_evidence(&stage_identity)
    else {
        return Err(WorthQueryWorkflowResumePreflightDenied::new(
            WorthQueryWorkflowReadmissionDenialKind::WorkflowStageResourcesUnavailable,
            "yielded workflow attempt has no resources for the retained stage",
            yielded,
            None,
        ));
    };
    let call = match yielded.execution.call.preflight_readmission(
        yielded.resource_attempt.binding_authority(),
        &stage_evidence,
    ) {
        Ok(call) => call,
        Err(denial) => {
            return Err(WorthQueryWorkflowResumePreflightDenied::new(
                WorthQueryWorkflowReadmissionDenialKind::ProviderCallBindingDenied,
                format!("workflow provider call readmission denied: {denial:?}"),
                yielded,
                None,
            ));
        }
    };
    let parts = WorthQueryWorkflowYieldedParts::from_yielded(yielded);
    let binding_identity = parts
        .resource_attempt
        .binding_authority()
        .binding_identity()
        .to_owned();
    let bridge =
        match bridge_runtime.preflight_yielded_execution_basis(parts.bridge, &binding_identity) {
            Ok(preflight) => preflight,
            Err(denial) => {
                let detail = Arc::from(denial.detail());
                let (bridge, bridge_counters) = denial.into_returned_yielded().into_parts();
                return Err(WorthQueryWorkflowResumePreflightDenied::new(
                    WorthQueryWorkflowReadmissionDenialKind::BridgeReadmissionDenied,
                    detail,
                    WorthQueryWorkflowYieldedParts {
                        state: parts.state,
                        resource_attempt: parts.resource_attempt,
                        bridge,
                        execution: parts.execution,
                    }
                    .into_yielded(),
                    Some(bridge_counters),
                ));
            }
        };
    let contract = match admit_managed_step_contract(
        parts.execution.contract().clone(),
        bridge.step_contract(),
    ) {
        Ok(contract) => contract,
        Err(denial) => {
            let (bridge, bridge_counters) = bridge.into_returned_yielded().into_parts();
            return Err(WorthQueryWorkflowResumePreflightDenied::new(
                WorthQueryWorkflowReadmissionDenialKind::ProviderStepContractDenied(denial.kind()),
                denial.detail(),
                WorthQueryWorkflowYieldedParts {
                    state: parts.state,
                    resource_attempt: parts.resource_attempt,
                    bridge,
                    execution: parts.execution,
                }
                .into_yielded(),
                Some(bridge_counters),
            ));
        }
    };
    Ok(WorthQueryWorkflowResumePreflightValidated {
        state: parts.state,
        resource_attempt: parts.resource_attempt,
        bridge,
        execution: parts.execution,
        contract,
        call,
        stage_resources,
        binding_identity,
        stage_identity,
    })
}

impl WorthQueryWorkflowResumePreflightValidated {
    pub(super) fn into_parts(self) -> WorthQueryWorkflowResumePreflightValidatedParts {
        WorthQueryWorkflowResumePreflightValidatedParts {
            state: self.state,
            resource_attempt: self.resource_attempt,
            bridge: self.bridge,
            execution: self.execution,
            contract: self.contract,
            call: self.call,
            stage_resources: self.stage_resources,
            binding_identity: self.binding_identity,
            stage_identity: self.stage_identity,
        }
    }
}

impl WorthQueryWorkflowResumePreflightDenied {
    fn new(
        kind: WorthQueryWorkflowReadmissionDenialKind,
        detail: impl Into<Arc<str>>,
        yielded: WorthQueryYieldedWorkflowRun,
        bridge_counters: Option<BridgeExecutionBasisReadmissionCounters>,
    ) -> Self {
        Self {
            kind,
            detail: detail.into(),
            yielded,
            bridge_counters,
        }
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        WorthQueryWorkflowReadmissionDenialKind,
        Arc<str>,
        WorthQueryYieldedWorkflowRun,
        Option<BridgeExecutionBasisReadmissionCounters>,
    ) {
        (self.kind, self.detail, self.yielded, self.bridge_counters)
    }
}

fn query_preflight_denial(
    yielded: &WorthQueryYieldedWorkflowRun,
    runtime: &WorthQueryExecutionRuntime,
) -> Option<(WorthQueryWorkflowReadmissionDenialKind, &'static str)> {
    let operation = yielded.resource_attempt.binding_authority();
    if !operation.belongs_to(runtime) {
        return Some((
            WorthQueryWorkflowReadmissionDenialKind::ForeignQueryRuntime,
            "yielded workflow belongs to a different Query execution runtime",
        ));
    }
    if !operation.belongs_to_current_installation(runtime) {
        return Some((
            WorthQueryWorkflowReadmissionDenialKind::StaleInstallationGeneration,
            "yielded workflow belongs to a stale installed-operation generation",
        ));
    }
    if yielded
        .resource_attempt
        .retained_capacity_reservation_count()
        == 0
    {
        return Some((
            WorthQueryWorkflowReadmissionDenialKind::RetainedCapacityMismatch,
            "yielded workflow no longer owns its capacity-reservation package",
        ));
    }
    if !yielded.relational_basis.is_live() {
        return Some((
            WorthQueryWorkflowReadmissionDenialKind::RelationalLeaseNotLive,
            "yielded workflow Relational basis lease is no longer live",
        ));
    }
    if !yielded.execution.provider_generation_matches_anchor() {
        return Some((
            WorthQueryWorkflowReadmissionDenialKind::ProviderCheckpointMismatch,
            "workflow checkpoint generation no longer matches its provider anchor",
        ));
    }
    if !yielded.artifacts.registry_is_frozen_at_owned_generation()
        || yielded.artifacts.production_generation().ordinal()
            != yielded.artifact_evidence.production_generation()
    {
        return Some((
            WorthQueryWorkflowReadmissionDenialKind::ArtifactGenerationMismatch,
            "workflow artifact registry is not frozen at the yielded production generation",
        ));
    }
    None
}
