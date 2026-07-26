use std::sync::Arc;

use worth_runtime_bridge::facade::{
    BridgeExecutionBasisReadmissionOutcome, BridgeManagedExecutionIntent, RuntimeBridge,
};

use super::super::provider_restore::{self, WorthQueryManagedGraphRestoreOutcome};
use super::super::WorthQueryYieldedWorkflowRun;
use super::counters::WorthQueryReadmissionCounters;
use super::workflow_abort::{abort_without_provider, map_recovery_kind};
use super::workflow_completion::advance_artifact_generation;
use super::workflow_outcome::{
    WorthQueryWorkflowReadmissionDenialKind, WorthQueryWorkflowReadmissionDenied,
    WorthQueryWorkflowReadmissionOutcome,
};
use super::workflow_preflight::{
    validate_workflow_resume_preflight, WorthQueryWorkflowResumePreflightValidated,
};
use super::workflow_recovery::WorthQueryWorkflowReadmissionRecoveryRequired;
use super::workflow_state::{
    WorthQueryWorkflowBridgeCleanupRecoveryState, WorthQueryWorkflowBridgeReadmissionPending,
    WorthQueryWorkflowProviderRecoveryState, WorthQueryWorkflowProviderRestorePending,
    WorthQueryWorkflowProvisionalResourceAttempt, WorthQueryWorkflowRollbackPending,
    WorthQueryWorkflowYieldedParts,
};
use crate::domain_computation::provider_session::readmission::WorthQueryWorkflowResourceReadmissionPending;
use crate::domain_computation::WorthQueryExecutionRuntime;

pub(in crate::domain_computation::managed_run) fn readmit_workflow(
    yielded: WorthQueryYieldedWorkflowRun,
    query_runtime: &WorthQueryExecutionRuntime,
    bridge_runtime: &RuntimeBridge,
) -> WorthQueryWorkflowReadmissionOutcome {
    let mut counters = WorthQueryReadmissionCounters::default();
    counters.checked_preflight();
    let preflight = match validate_workflow_resume_preflight(yielded, query_runtime, bridge_runtime)
    {
        Ok(preflight) => preflight,
        Err(denial) => {
            let (kind, detail, yielded) = denial.into_parts();
            return denied(kind, detail, yielded, counters);
        }
    };
    let provisional = match begin_resource_attempt(preflight, counters) {
        Ok((provisional, next_counters)) => {
            counters = next_counters;
            provisional
        }
        Err(outcome) => return outcome,
    };
    let pending = match begin_bridge_readmission(provisional, bridge_runtime, counters) {
        Ok((pending, next_counters)) => {
            counters = next_counters;
            pending
        }
        Err(outcome) => return outcome,
    };
    restore_workflow(pending, bridge_runtime, counters)
}

fn begin_resource_attempt(
    preflight: WorthQueryWorkflowResumePreflightValidated,
    mut counters: WorthQueryReadmissionCounters,
) -> Result<
    (
        WorthQueryWorkflowProvisionalResourceAttempt,
        WorthQueryReadmissionCounters,
    ),
    WorthQueryWorkflowReadmissionOutcome,
> {
    let parts = preflight.into_parts();
    let resource = WorthQueryWorkflowResourceReadmissionPending::begin(parts.resource_attempt);
    counters.minted_fresh_resource_attempt();
    let Some((_resources, stage_evidence)) =
        resource.stage_resources_and_evidence(&parts.stage_identity)
    else {
        return Err(denied(
            WorthQueryWorkflowReadmissionDenialKind::WorkflowStageResourcesUnavailable,
            "fresh workflow attempt has no resources for the retained stage",
            WorthQueryWorkflowYieldedParts {
                state: parts.state,
                resource_attempt: resource.abort(),
                bridge: parts.bridge.into_yielded(),
                execution: parts.execution,
            }
            .into_yielded(),
            counters,
        ));
    };
    let fresh_call = match parts
        .execution
        .call
        .remint_for_readmission(resource.provider_session(), &stage_evidence)
    {
        Ok(call) => call,
        Err(denial) => {
            return Err(denied(
                WorthQueryWorkflowReadmissionDenialKind::ProviderCallBindingDenied,
                format!("workflow provider call readmission denied: {denial:?}"),
                WorthQueryWorkflowYieldedParts {
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
        WorthQueryWorkflowProvisionalResourceAttempt {
            state: parts.state,
            execution: parts.execution,
            resource,
            bridge: parts.bridge,
            fresh_call,
            contract: parts.contract,
            binding_identity: parts.binding_identity,
            stage_identity: parts.stage_identity,
        },
        counters,
    ))
}

fn begin_bridge_readmission(
    provisional: WorthQueryWorkflowProvisionalResourceAttempt,
    bridge_runtime: &RuntimeBridge,
    mut counters: WorthQueryReadmissionCounters,
) -> Result<
    (
        WorthQueryWorkflowBridgeReadmissionPending,
        WorthQueryReadmissionCounters,
    ),
    WorthQueryWorkflowReadmissionOutcome,
> {
    let intent = BridgeManagedExecutionIntent::new(
        provisional.binding_identity,
        provisional.resource.attempt_identity().as_str(),
    );
    counters.attempted_bridge_readmission();
    match bridge_runtime.readmit_yielded_execution_basis(provisional.bridge, intent) {
        BridgeExecutionBasisReadmissionOutcome::Pending(bridge) => Ok((
            WorthQueryWorkflowBridgeReadmissionPending {
                state: provisional.state,
                execution: provisional.execution,
                resource: provisional.resource,
                bridge,
                fresh_call: provisional.fresh_call,
                contract: provisional.contract,
                stage_identity: provisional.stage_identity,
            },
            counters,
        )),
        BridgeExecutionBasisReadmissionOutcome::Denied(denial) => {
            let detail = Arc::from(denial.detail());
            Err(denied(
                WorthQueryWorkflowReadmissionDenialKind::BridgeReadmissionDenied,
                detail,
                WorthQueryWorkflowYieldedParts {
                    state: provisional.state,
                    resource_attempt: provisional.resource.abort(),
                    bridge: denial.into_yielded(),
                    execution: provisional.execution,
                }
                .into_yielded(),
                counters,
            ))
        }
        BridgeExecutionBasisReadmissionOutcome::RecoveryRequired(recovery) => {
            let detail = Arc::from(recovery.detail());
            Err(WorthQueryWorkflowReadmissionOutcome::RecoveryRequired(
                WorthQueryWorkflowReadmissionRecoveryRequired::bridge_cleanup(
                    detail,
                    counters,
                    WorthQueryWorkflowBridgeCleanupRecoveryState {
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

fn restore_workflow(
    pending: WorthQueryWorkflowBridgeReadmissionPending,
    bridge_runtime: &RuntimeBridge,
    mut counters: WorthQueryReadmissionCounters,
) -> WorthQueryWorkflowReadmissionOutcome {
    let WorthQueryWorkflowBridgeReadmissionPending {
        state,
        execution,
        resource: resource_pending,
        bridge: bridge_pending,
        fresh_call,
        contract,
        stage_identity,
    } = pending;
    counters.attempted_provider_restore();
    let provider = match provider_restore::restore(execution, fresh_call, contract) {
        WorthQueryManagedGraphRestoreOutcome::Pending(provider) => {
            WorthQueryWorkflowProviderRestorePending {
                state,
                stage_identity,
                provider,
                resource: resource_pending,
                bridge: bridge_pending,
            }
        }
        WorthQueryManagedGraphRestoreOutcome::Denied(denial) => {
            let detail = Arc::from(denial.detail());
            return abort_without_provider(
                WorthQueryWorkflowReadmissionDenialKind::ProviderRestoreDenied,
                detail,
                WorthQueryWorkflowRollbackPending {
                    state,
                    execution: denial.into_retained(),
                    resource: resource_pending,
                    bridge: bridge_pending,
                },
                counters,
            );
        }
        WorthQueryManagedGraphRestoreOutcome::RecoveryRequired(recovery) => {
            let kind = map_recovery_kind(recovery.kind());
            let detail = Arc::from(recovery.detail());
            return WorthQueryWorkflowReadmissionOutcome::RecoveryRequired(
                WorthQueryWorkflowReadmissionRecoveryRequired::provider(
                    kind,
                    detail,
                    counters,
                    WorthQueryWorkflowProviderRecoveryState {
                        state,
                        resource: resource_pending,
                        bridge: bridge_pending,
                        provider: recovery,
                    },
                ),
            );
        }
    };
    advance_artifact_generation(provider, bridge_runtime, counters)
}

fn denied(
    kind: WorthQueryWorkflowReadmissionDenialKind,
    detail: impl Into<Arc<str>>,
    yielded: WorthQueryYieldedWorkflowRun,
    counters: WorthQueryReadmissionCounters,
) -> WorthQueryWorkflowReadmissionOutcome {
    WorthQueryWorkflowReadmissionOutcome::Denied(WorthQueryWorkflowReadmissionDenied::new(
        kind, detail, yielded, counters,
    ))
}
