use std::sync::Arc;

use worth_runtime_bridge::facade::{BridgeYieldedExecutionBasisPreflight, RuntimeBridge};

use super::super::retained_graph_execution::WorthQueryRetainedManagedGraphExecution;
use super::super::step_contract_admission::{
    admit_managed_step_contract, WorthQueryAdmittedManagedStepContract,
};
use super::super::WorthQueryYieldedWorkflowRun;
use super::workflow_outcome::WorthQueryWorkflowReadmissionDenialKind;
use super::workflow_state::{WorthQueryWorkflowYieldedParts, WorthQueryWorkflowYieldedState};
use crate::domain_computation::{
    WorthQueryExecutionRuntime, WorthQueryWorkflowExecutionResourceAttempt,
};

pub(super) struct WorthQueryWorkflowResumePreflightValidated {
    state: WorthQueryWorkflowYieldedState,
    resource_attempt: WorthQueryWorkflowExecutionResourceAttempt,
    bridge: BridgeYieldedExecutionBasisPreflight,
    execution: WorthQueryRetainedManagedGraphExecution,
    contract: WorthQueryAdmittedManagedStepContract,
    binding_identity: String,
    stage_identity: String,
}

pub(super) struct WorthQueryWorkflowResumePreflightValidatedParts {
    pub(super) state: WorthQueryWorkflowYieldedState,
    pub(super) resource_attempt: WorthQueryWorkflowExecutionResourceAttempt,
    pub(super) bridge: BridgeYieldedExecutionBasisPreflight,
    pub(super) execution: WorthQueryRetainedManagedGraphExecution,
    pub(super) contract: WorthQueryAdmittedManagedStepContract,
    pub(super) binding_identity: String,
    pub(super) stage_identity: String,
}

pub(super) struct WorthQueryWorkflowResumePreflightDenied {
    kind: WorthQueryWorkflowReadmissionDenialKind,
    detail: Arc<str>,
    yielded: WorthQueryYieldedWorkflowRun,
}

pub(super) fn validate_workflow_resume_preflight(
    yielded: WorthQueryYieldedWorkflowRun,
    query_runtime: &WorthQueryExecutionRuntime,
    bridge_runtime: &RuntimeBridge,
) -> Result<WorthQueryWorkflowResumePreflightValidated, WorthQueryWorkflowResumePreflightDenied> {
    if let Some((kind, detail)) = query_preflight_denial(&yielded, query_runtime) {
        return Err(WorthQueryWorkflowResumePreflightDenied::new(
            kind, detail, yielded,
        ));
    }
    let parts = WorthQueryWorkflowYieldedParts::from_yielded(yielded);
    let Some(stage_identity) = parts.execution.call.stage_identity().map(str::to_owned) else {
        return Err(WorthQueryWorkflowResumePreflightDenied::new(
            WorthQueryWorkflowReadmissionDenialKind::WorkflowStageResourcesUnavailable,
            "retained workflow provider call has no stage identity",
            parts.into_yielded(),
        ));
    };
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
                return Err(WorthQueryWorkflowResumePreflightDenied::new(
                    WorthQueryWorkflowReadmissionDenialKind::BridgeReadmissionDenied,
                    detail,
                    WorthQueryWorkflowYieldedParts {
                        state: parts.state,
                        resource_attempt: parts.resource_attempt,
                        bridge: denial.into_yielded(),
                        execution: parts.execution,
                    }
                    .into_yielded(),
                ));
            }
        };
    let contract = match admit_managed_step_contract(
        parts.execution.contract().clone(),
        bridge.step_contract(),
    ) {
        Ok(contract) => contract,
        Err(denial) => {
            return Err(WorthQueryWorkflowResumePreflightDenied::new(
                WorthQueryWorkflowReadmissionDenialKind::ProviderStepContractDenied(denial.kind()),
                denial.detail(),
                WorthQueryWorkflowYieldedParts {
                    state: parts.state,
                    resource_attempt: parts.resource_attempt,
                    bridge: bridge.into_yielded(),
                    execution: parts.execution,
                }
                .into_yielded(),
            ));
        }
    };
    Ok(WorthQueryWorkflowResumePreflightValidated {
        state: parts.state,
        resource_attempt: parts.resource_attempt,
        bridge,
        execution: parts.execution,
        contract,
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
    ) -> Self {
        Self {
            kind,
            detail: detail.into(),
            yielded,
        }
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        WorthQueryWorkflowReadmissionDenialKind,
        Arc<str>,
        WorthQueryYieldedWorkflowRun,
    ) {
        (self.kind, self.detail, self.yielded)
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
