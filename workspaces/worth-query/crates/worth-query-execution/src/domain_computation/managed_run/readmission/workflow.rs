use std::sync::Arc;

use worth_runtime_bridge::facade::{
    BridgeExecutionBasisReadmissionOutcome, BridgeManagedExecutionIntent, RuntimeBridge,
};

use super::super::provider_restore::{self, WorthQueryManagedGraphRestoreOutcome};
use super::super::WorthQueryYieldedWorkflowRun;
use super::evidence::WorthQueryReadmissionProgress;
use super::recovery::WorthQueryWorkflowReadmissionRecoveryRequired;
use super::workflow_abort::{abort_without_provider, map_recovery_kind};
use super::workflow_completion::advance_artifact_generation;
use super::workflow_outcome::{
    WorthQueryWorkflowReadmissionDenialKind, WorthQueryWorkflowReadmissionDenied,
    WorthQueryWorkflowReadmissionOutcome,
};
use super::workflow_preflight::{
    validate_workflow_resume_preflight, WorthQueryWorkflowResumePreflightValidated,
};
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
    let mut progress = WorthQueryReadmissionProgress::default();
    progress.checked_preflight();
    let preflight = match validate_workflow_resume_preflight(yielded, query_runtime, bridge_runtime)
    {
        Ok(preflight) => preflight,
        Err(denial) => {
            let (kind, detail, yielded, bridge_counters) = denial.into_parts();
            if let Some(bridge_counters) = bridge_counters {
                progress.observe_bridge(bridge_counters);
            }
            return denied(kind, detail, yielded, progress);
        }
    };
    let provisional = begin_resource_attempt(preflight, &mut progress);
    let pending = match begin_bridge_readmission(provisional, bridge_runtime, progress) {
        Ok((pending, next_progress)) => {
            progress = next_progress;
            pending
        }
        Err(outcome) => return outcome,
    };
    restore_workflow(pending, bridge_runtime, progress)
}

fn begin_resource_attempt(
    preflight: WorthQueryWorkflowResumePreflightValidated,
    progress: &mut WorthQueryReadmissionProgress,
) -> WorthQueryWorkflowProvisionalResourceAttempt {
    let parts = preflight.into_parts();
    let (resource, fresh_call) = WorthQueryWorkflowResourceReadmissionPending::begin(
        parts.resource_attempt,
        parts.stage_resources,
        parts.call,
    );
    progress.minted_fresh_resource_attempt();
    WorthQueryWorkflowProvisionalResourceAttempt {
        state: parts.state,
        execution: parts.execution,
        resource,
        bridge: parts.bridge,
        fresh_call,
        contract: parts.contract,
        binding_identity: parts.binding_identity,
        stage_identity: parts.stage_identity,
    }
}

fn begin_bridge_readmission(
    provisional: WorthQueryWorkflowProvisionalResourceAttempt,
    bridge_runtime: &RuntimeBridge,
    mut progress: WorthQueryReadmissionProgress,
) -> Result<
    (
        WorthQueryWorkflowBridgeReadmissionPending,
        WorthQueryReadmissionProgress,
    ),
    WorthQueryWorkflowReadmissionOutcome,
> {
    let intent = BridgeManagedExecutionIntent::new(
        provisional.binding_identity,
        provisional.resource.attempt_identity().as_str(),
    );
    progress.attempted_bridge_readmission();
    match bridge_runtime.readmit_yielded_execution_basis(provisional.bridge, intent) {
        BridgeExecutionBasisReadmissionOutcome::Pending(bridge) => {
            progress.observe_bridge(bridge.counters());
            Ok((
                WorthQueryWorkflowBridgeReadmissionPending {
                    state: provisional.state,
                    execution: provisional.execution,
                    resource: provisional.resource,
                    bridge,
                    fresh_call: provisional.fresh_call,
                    contract: provisional.contract,
                    stage_identity: provisional.stage_identity,
                },
                progress,
            ))
        }
        BridgeExecutionBasisReadmissionOutcome::Denied(denial) => {
            let detail = Arc::from(denial.detail());
            let (bridge, bridge_counters) = denial.into_returned_yielded().into_parts();
            progress.observe_bridge(bridge_counters);
            Err(denied(
                WorthQueryWorkflowReadmissionDenialKind::BridgeReadmissionDenied,
                detail,
                WorthQueryWorkflowYieldedParts {
                    state: provisional.state,
                    resource_attempt: provisional.resource.abort(),
                    bridge,
                    execution: provisional.execution,
                }
                .into_yielded(),
                progress,
            ))
        }
        BridgeExecutionBasisReadmissionOutcome::RecoveryRequired(recovery) => {
            let detail = Arc::from(recovery.detail());
            progress.observe_bridge(recovery.counters());
            Err(WorthQueryWorkflowReadmissionOutcome::RecoveryRequired(
                WorthQueryWorkflowReadmissionRecoveryRequired::bridge_cleanup(
                    detail,
                    progress,
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
    mut progress: WorthQueryReadmissionProgress,
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
    progress.attempted_provider_restore();
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
                progress,
            );
        }
        WorthQueryManagedGraphRestoreOutcome::RecoveryRequired(recovery) => {
            let kind = map_recovery_kind(recovery.kind());
            let detail = Arc::from(recovery.detail());
            return WorthQueryWorkflowReadmissionOutcome::RecoveryRequired(
                WorthQueryWorkflowReadmissionRecoveryRequired::provider(
                    kind,
                    detail,
                    progress,
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
    advance_artifact_generation(provider, bridge_runtime, progress)
}

fn denied(
    kind: WorthQueryWorkflowReadmissionDenialKind,
    detail: impl Into<Arc<str>>,
    yielded: WorthQueryYieldedWorkflowRun,
    progress: WorthQueryReadmissionProgress,
) -> WorthQueryWorkflowReadmissionOutcome {
    WorthQueryWorkflowReadmissionOutcome::Denied(WorthQueryWorkflowReadmissionDenied::new(
        kind,
        detail,
        yielded,
        progress.evidence(),
    ))
}
