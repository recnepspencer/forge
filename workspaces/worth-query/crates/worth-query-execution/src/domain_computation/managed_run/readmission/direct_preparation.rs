use std::sync::Arc;

use worth_runtime_bridge::facade::{
    BridgeExecutionBasisReadmissionOutcome, BridgeManagedExecutionIntent, RuntimeBridge,
};

use super::direct_outcome::{
    WorthQueryDirectReadmissionDenialKind, WorthQueryDirectReadmissionDenied,
    WorthQueryDirectReadmissionOutcome,
};
use super::direct_preflight::{
    validate_direct_resume_preflight, WorthQueryDirectResumePreflightValidated,
};
use super::direct_state::{
    WorthQueryDirectBridgeCleanupRecoveryState, WorthQueryDirectBridgeReadmissionPending,
    WorthQueryDirectProvisionalResourceAttempt, WorthQueryDirectYieldedParts,
};
use super::recovery::WorthQueryDirectReadmissionRecoveryRequired;
use super::WorthQueryReadmissionCounters;
use crate::domain_computation::managed_run::WorthQueryYieldedDirectRun;
use crate::domain_computation::provider_session::readmission::WorthQueryDirectResourceReadmissionPending;
use crate::domain_computation::WorthQueryExecutionRuntime;

pub(in crate::domain_computation::managed_run) fn prepare_direct_provider_restore(
    yielded: WorthQueryYieldedDirectRun,
    query_runtime: &WorthQueryExecutionRuntime,
    bridge_runtime: &RuntimeBridge,
) -> Result<
    (
        WorthQueryDirectBridgeReadmissionPending,
        WorthQueryReadmissionCounters,
    ),
    WorthQueryDirectReadmissionOutcome,
> {
    let mut counters = WorthQueryReadmissionCounters::default();
    counters.checked_preflight();
    let preflight = match validate_direct_resume_preflight(yielded, query_runtime, bridge_runtime) {
        Ok(preflight) => preflight,
        Err(denial) => {
            let (kind, detail, yielded) = denial.into_parts();
            return Err(denied(kind, detail, yielded, counters));
        }
    };
    let provisional = match begin_resource_attempt(preflight, counters) {
        Ok((provisional, next_counters)) => {
            counters = next_counters;
            provisional
        }
        Err(outcome) => return Err(outcome),
    };
    let pending = match begin_bridge_readmission(provisional, bridge_runtime, counters) {
        Ok((pending, next_counters)) => {
            counters = next_counters;
            pending
        }
        Err(outcome) => return Err(outcome),
    };
    Ok((pending, counters))
}

fn begin_resource_attempt(
    preflight: WorthQueryDirectResumePreflightValidated,
    mut counters: WorthQueryReadmissionCounters,
) -> Result<
    (
        WorthQueryDirectProvisionalResourceAttempt,
        WorthQueryReadmissionCounters,
    ),
    WorthQueryDirectReadmissionOutcome,
> {
    let parts = preflight.into_parts();
    let resource = WorthQueryDirectResourceReadmissionPending::begin(parts.resource_attempt);
    counters.minted_fresh_resource_attempt();
    let fresh_call = match parts
        .execution
        .call
        .remint_for_readmission(resource.provider_session(), resource.evidence())
    {
        Ok(call) => call,
        Err(denial) => {
            return Err(denied(
                WorthQueryDirectReadmissionDenialKind::ProviderCallBindingDenied,
                format!("provider call readmission denied: {denial:?}"),
                WorthQueryDirectYieldedParts {
                    state: parts.state,
                    resource_attempt: resource.abort(),
                    bridge: parts.bridge.into_yielded(),
                    execution: parts.execution,
                }
                .into_yielded(),
                counters,
            ));
        }
    };
    Ok((
        WorthQueryDirectProvisionalResourceAttempt {
            state: parts.state,
            execution: parts.execution,
            resource,
            bridge: parts.bridge,
            fresh_call,
            contract: parts.contract,
            binding_identity: parts.binding_identity,
        },
        counters,
    ))
}

fn begin_bridge_readmission(
    provisional: WorthQueryDirectProvisionalResourceAttempt,
    bridge_runtime: &RuntimeBridge,
    mut counters: WorthQueryReadmissionCounters,
) -> Result<
    (
        WorthQueryDirectBridgeReadmissionPending,
        WorthQueryReadmissionCounters,
    ),
    WorthQueryDirectReadmissionOutcome,
> {
    let intent = BridgeManagedExecutionIntent::new(
        provisional.binding_identity,
        provisional.resource.attempt_identity().as_str(),
    );
    counters.attempted_bridge_readmission();
    match bridge_runtime.readmit_yielded_execution_basis(provisional.bridge, intent) {
        BridgeExecutionBasisReadmissionOutcome::Pending(bridge) => Ok((
            WorthQueryDirectBridgeReadmissionPending {
                state: provisional.state,
                execution: provisional.execution,
                resource: provisional.resource,
                bridge,
                fresh_call: provisional.fresh_call,
                contract: provisional.contract,
            },
            counters,
        )),
        BridgeExecutionBasisReadmissionOutcome::Denied(denial) => Err(denied(
            WorthQueryDirectReadmissionDenialKind::BridgeReadmissionDenied,
            Arc::from(denial.detail()),
            WorthQueryDirectYieldedParts {
                state: provisional.state,
                resource_attempt: provisional.resource.abort(),
                bridge: denial.into_yielded(),
                execution: provisional.execution,
            }
            .into_yielded(),
            counters,
        )),
        BridgeExecutionBasisReadmissionOutcome::RecoveryRequired(recovery) => {
            let detail = Arc::from(recovery.detail());
            Err(WorthQueryDirectReadmissionOutcome::RecoveryRequired(
                WorthQueryDirectReadmissionRecoveryRequired::bridge_cleanup(
                    detail,
                    counters,
                    WorthQueryDirectBridgeCleanupRecoveryState {
                        state: provisional.state,
                        resource_attempt: provisional.resource.abort(),
                        execution: provisional.execution,
                        bridge: recovery,
                    },
                ),
            ))
        }
    }
}

fn denied(
    kind: WorthQueryDirectReadmissionDenialKind,
    detail: impl Into<Arc<str>>,
    yielded: WorthQueryYieldedDirectRun,
    counters: WorthQueryReadmissionCounters,
) -> WorthQueryDirectReadmissionOutcome {
    WorthQueryDirectReadmissionOutcome::Denied(WorthQueryDirectReadmissionDenied::new(
        kind, detail, yielded, counters,
    ))
}
