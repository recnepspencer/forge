use std::sync::Arc;

use worth_runtime_bridge::facade::{BridgeYieldedExecutionBasisPreflight, RuntimeBridge};

use super::super::retained_graph_execution::WorthQueryRetainedManagedGraphExecution;
use super::super::step_contract_admission::{
    admit_managed_step_contract, WorthQueryAdmittedManagedStepContract,
};
use super::super::WorthQueryYieldedDirectRun;
use super::direct_outcome::WorthQueryDirectReadmissionDenialKind;
use super::direct_state::{WorthQueryDirectYieldedParts, WorthQueryDirectYieldedState};
use crate::domain_computation::provider_session::graph_provider::WorthQueryGraphProviderCallReadmissionPlan;
use crate::domain_computation::{
    WorthQueryDirectExecutionResourceAttempt, WorthQueryExecutionRuntime,
};

pub(super) struct WorthQueryDirectResumePreflightValidated {
    state: WorthQueryDirectYieldedState,
    resource_attempt: WorthQueryDirectExecutionResourceAttempt,
    bridge: BridgeYieldedExecutionBasisPreflight,
    execution: WorthQueryRetainedManagedGraphExecution,
    contract: WorthQueryAdmittedManagedStepContract,
    call: WorthQueryGraphProviderCallReadmissionPlan,
    binding_identity: String,
}

pub(super) struct WorthQueryDirectResumePreflightValidatedParts {
    pub(super) state: WorthQueryDirectYieldedState,
    pub(super) resource_attempt: WorthQueryDirectExecutionResourceAttempt,
    pub(super) bridge: BridgeYieldedExecutionBasisPreflight,
    pub(super) execution: WorthQueryRetainedManagedGraphExecution,
    pub(super) contract: WorthQueryAdmittedManagedStepContract,
    pub(super) call: WorthQueryGraphProviderCallReadmissionPlan,
    pub(super) binding_identity: String,
}

pub(super) struct WorthQueryDirectResumePreflightDenied {
    kind: WorthQueryDirectReadmissionDenialKind,
    detail: Arc<str>,
    yielded: WorthQueryYieldedDirectRun,
}

pub(super) fn validate_direct_resume_preflight(
    yielded: WorthQueryYieldedDirectRun,
    query_runtime: &WorthQueryExecutionRuntime,
    bridge_runtime: &RuntimeBridge,
) -> Result<WorthQueryDirectResumePreflightValidated, WorthQueryDirectResumePreflightDenied> {
    if let Some((kind, detail)) = query_preflight_denial(&yielded, query_runtime) {
        return Err(WorthQueryDirectResumePreflightDenied::new(
            kind, detail, yielded,
        ));
    }
    let call = match yielded.execution.call.preflight_readmission(
        yielded.resource_attempt.binding_authority(),
        yielded.resource_attempt.evidence(),
    ) {
        Ok(call) => call,
        Err(denial) => {
            return Err(WorthQueryDirectResumePreflightDenied::new(
                WorthQueryDirectReadmissionDenialKind::ProviderCallBindingDenied,
                format!("provider call readmission denied: {denial:?}"),
                yielded,
            ));
        }
    };
    let parts = WorthQueryDirectYieldedParts::from_yielded(yielded);
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
                return Err(WorthQueryDirectResumePreflightDenied::new(
                    WorthQueryDirectReadmissionDenialKind::BridgeReadmissionDenied,
                    detail,
                    WorthQueryDirectYieldedParts {
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
            return Err(WorthQueryDirectResumePreflightDenied::new(
                WorthQueryDirectReadmissionDenialKind::ProviderStepContractDenied(denial.kind()),
                denial.detail(),
                WorthQueryDirectYieldedParts {
                    state: parts.state,
                    resource_attempt: parts.resource_attempt,
                    bridge: bridge.into_yielded(),
                    execution: parts.execution,
                }
                .into_yielded(),
            ));
        }
    };
    Ok(WorthQueryDirectResumePreflightValidated {
        state: parts.state,
        resource_attempt: parts.resource_attempt,
        bridge,
        execution: parts.execution,
        contract,
        call,
        binding_identity,
    })
}

impl WorthQueryDirectResumePreflightValidated {
    pub(super) fn into_parts(self) -> WorthQueryDirectResumePreflightValidatedParts {
        WorthQueryDirectResumePreflightValidatedParts {
            state: self.state,
            resource_attempt: self.resource_attempt,
            bridge: self.bridge,
            execution: self.execution,
            contract: self.contract,
            call: self.call,
            binding_identity: self.binding_identity,
        }
    }
}

impl WorthQueryDirectResumePreflightDenied {
    fn new(
        kind: WorthQueryDirectReadmissionDenialKind,
        detail: impl Into<Arc<str>>,
        yielded: WorthQueryYieldedDirectRun,
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
        WorthQueryDirectReadmissionDenialKind,
        Arc<str>,
        WorthQueryYieldedDirectRun,
    ) {
        (self.kind, self.detail, self.yielded)
    }
}

fn query_preflight_denial(
    yielded: &WorthQueryYieldedDirectRun,
    runtime: &WorthQueryExecutionRuntime,
) -> Option<(WorthQueryDirectReadmissionDenialKind, &'static str)> {
    let operation = yielded.resource_attempt.binding_authority();
    if !operation.belongs_to(runtime) {
        return Some((
            WorthQueryDirectReadmissionDenialKind::ForeignQueryRuntime,
            "yielded run belongs to a different Query execution runtime",
        ));
    }
    if !operation.belongs_to_current_installation(runtime) {
        return Some((
            WorthQueryDirectReadmissionDenialKind::StaleInstallationGeneration,
            "yielded run belongs to a stale installed-operation generation",
        ));
    }
    if yielded
        .resource_attempt
        .retained_capacity_reservation_count()
        == 0
    {
        return Some((
            WorthQueryDirectReadmissionDenialKind::RetainedCapacityMismatch,
            "yielded run no longer owns its nonempty capacity-reservation package",
        ));
    }
    if !yielded.relational_basis.is_live() {
        return Some((
            WorthQueryDirectReadmissionDenialKind::RelationalLeaseNotLive,
            "yielded Relational execution-basis lease is no longer live",
        ));
    }
    if !yielded.execution.provider_generation_matches_anchor() {
        return Some((
            WorthQueryDirectReadmissionDenialKind::ProviderCheckpointMismatch,
            "provider checkpoint generation no longer matches its retained provider anchor",
        ));
    }
    None
}
