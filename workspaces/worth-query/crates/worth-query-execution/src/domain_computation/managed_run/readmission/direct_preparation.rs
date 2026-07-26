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
use super::evidence::WorthQueryReadmissionProgress;
use super::recovery::WorthQueryDirectReadmissionRecoveryRequired;
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
        WorthQueryReadmissionProgress,
    ),
    WorthQueryDirectReadmissionOutcome,
> {
    let mut progress = WorthQueryReadmissionProgress::default();
    progress.checked_preflight();
    let preflight = match validate_direct_resume_preflight(yielded, query_runtime, bridge_runtime) {
        Ok(preflight) => preflight,
        Err(denial) => {
            let (kind, detail, yielded, bridge_counters) = denial.into_parts();
            if let Some(bridge_counters) = bridge_counters {
                progress.observe_bridge(bridge_counters);
            }
            return Err(denied(kind, detail, yielded, progress));
        }
    };
    let provisional = begin_resource_attempt(preflight, &mut progress);
    let pending = match begin_bridge_readmission(provisional, bridge_runtime, progress) {
        Ok((pending, next_progress)) => {
            progress = next_progress;
            pending
        }
        Err(outcome) => return Err(outcome),
    };
    Ok((pending, progress))
}

fn begin_resource_attempt(
    preflight: WorthQueryDirectResumePreflightValidated,
    progress: &mut WorthQueryReadmissionProgress,
) -> WorthQueryDirectProvisionalResourceAttempt {
    let parts = preflight.into_parts();
    let (resource, fresh_call) =
        WorthQueryDirectResourceReadmissionPending::begin(parts.resource_attempt, parts.call);
    progress.minted_fresh_resource_attempt();
    WorthQueryDirectProvisionalResourceAttempt {
        state: parts.state,
        execution: parts.execution,
        resource,
        bridge: parts.bridge,
        fresh_call,
        contract: parts.contract,
        binding_identity: parts.binding_identity,
    }
}

fn begin_bridge_readmission(
    provisional: WorthQueryDirectProvisionalResourceAttempt,
    bridge_runtime: &RuntimeBridge,
    mut progress: WorthQueryReadmissionProgress,
) -> Result<
    (
        WorthQueryDirectBridgeReadmissionPending,
        WorthQueryReadmissionProgress,
    ),
    WorthQueryDirectReadmissionOutcome,
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
                WorthQueryDirectBridgeReadmissionPending {
                    state: provisional.state,
                    execution: provisional.execution,
                    resource: provisional.resource,
                    bridge,
                    fresh_call: provisional.fresh_call,
                    contract: provisional.contract,
                },
                progress,
            ))
        }
        BridgeExecutionBasisReadmissionOutcome::Denied(denial) => {
            let detail = Arc::from(denial.detail());
            let (bridge, bridge_counters) = denial.into_returned_yielded().into_parts();
            progress.observe_bridge(bridge_counters);
            Err(denied(
                WorthQueryDirectReadmissionDenialKind::BridgeReadmissionDenied,
                detail,
                WorthQueryDirectYieldedParts {
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
            Err(WorthQueryDirectReadmissionOutcome::RecoveryRequired(
                WorthQueryDirectReadmissionRecoveryRequired::bridge_cleanup(
                    detail,
                    progress,
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
    progress: WorthQueryReadmissionProgress,
) -> WorthQueryDirectReadmissionOutcome {
    WorthQueryDirectReadmissionOutcome::Denied(WorthQueryDirectReadmissionDenied::new(
        kind,
        detail,
        yielded,
        progress.evidence(),
    ))
}
