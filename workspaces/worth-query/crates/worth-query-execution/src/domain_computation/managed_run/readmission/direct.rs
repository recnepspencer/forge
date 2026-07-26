use std::sync::Arc;

use worth_runtime_bridge::facade::{
    BridgeExecutionBasisReadmissionCleanupOutcome, BridgeExecutionBasisReadmissionOutcome,
    BridgeManagedExecutionIntent, RuntimeBridge,
};

use super::super::provider_restore::{
    self, WorthQueryManagedGraphRestoreCommitOutcome, WorthQueryManagedGraphRestoreOutcome,
    WorthQueryManagedGraphRestoreRecoveryKind,
};
use super::super::run_identity::WorthQueryManagedRunIdentity;
use super::super::{
    WorthQueryActiveDirectGraphExecution, WorthQueryRunningDirectRun, WorthQueryYieldedDirectRun,
};
use super::counters::WorthQueryReadmissionCounters;
use super::direct_outcome::{
    WorthQueryDirectReadmissionDenialKind, WorthQueryDirectReadmissionDenied,
    WorthQueryDirectReadmissionOutcome, WorthQueryDirectReadmissionRecoveryKind,
    WorthQueryDirectReadmissionRecoveryRequired,
};
use super::direct_preflight::{
    validate_direct_resume_preflight, WorthQueryDirectResumePreflightValidated,
};
use super::direct_state::{
    WorthQueryDirectBridgeCleanupRecoveryState, WorthQueryDirectBridgeReadmissionPending,
    WorthQueryDirectCommitReady, WorthQueryDirectProviderRecoveryState,
    WorthQueryDirectProviderRestorePending, WorthQueryDirectProvisionalResourceAttempt,
    WorthQueryDirectRollbackPending, WorthQueryDirectYieldedParts,
};
use crate::domain_computation::provider_session::readmission::WorthQueryDirectResourceReadmissionPending;
use crate::domain_computation::WorthQueryExecutionRuntime;

pub(in crate::domain_computation::managed_run) fn readmit_direct(
    yielded: WorthQueryYieldedDirectRun,
    query_runtime: &WorthQueryExecutionRuntime,
    bridge_runtime: &RuntimeBridge,
) -> WorthQueryDirectReadmissionOutcome {
    let mut counters = WorthQueryReadmissionCounters::default();
    counters.checked_preflight();
    let preflight = match validate_direct_resume_preflight(yielded, query_runtime, bridge_runtime) {
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
    restore_direct(pending, bridge_runtime, counters)
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
        BridgeExecutionBasisReadmissionOutcome::Denied(denial) => {
            let detail = Arc::from(denial.detail());
            Err(denied(
                WorthQueryDirectReadmissionDenialKind::BridgeReadmissionDenied,
                detail,
                WorthQueryDirectYieldedParts {
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

fn restore_direct(
    pending: WorthQueryDirectBridgeReadmissionPending,
    bridge_runtime: &RuntimeBridge,
    mut counters: WorthQueryReadmissionCounters,
) -> WorthQueryDirectReadmissionOutcome {
    let WorthQueryDirectBridgeReadmissionPending {
        state,
        execution,
        resource: resource_pending,
        bridge: bridge_pending,
        fresh_call,
        contract,
    } = pending;
    counters.attempted_provider_restore();
    let provider = match provider_restore::restore(execution, fresh_call, contract) {
        WorthQueryManagedGraphRestoreOutcome::Pending(provider) => {
            WorthQueryDirectProviderRestorePending {
                state,
                provider,
                resource: resource_pending,
                bridge: bridge_pending,
            }
        }
        WorthQueryManagedGraphRestoreOutcome::Denied(denial) => {
            let detail = Arc::from(denial.detail());
            return abort_to_denial(
                WorthQueryDirectReadmissionDenialKind::ProviderRestoreDenied,
                detail,
                WorthQueryDirectRollbackPending {
                    state,
                    execution: denial.into_retained(),
                    resource: resource_pending,
                    bridge: bridge_pending,
                },
                counters,
            );
        }
        WorthQueryManagedGraphRestoreOutcome::RecoveryRequired(recovery) => {
            let kind = recovery_kind(recovery.kind());
            let detail = Arc::from(recovery.detail());
            return WorthQueryDirectReadmissionOutcome::RecoveryRequired(
                WorthQueryDirectReadmissionRecoveryRequired::provider(
                    kind,
                    detail,
                    counters,
                    WorthQueryDirectProviderRecoveryState {
                        state,
                        resource: resource_pending,
                        bridge: bridge_pending,
                        provider: recovery,
                    },
                ),
            );
        }
    };
    commit_provider_restore(provider, bridge_runtime, counters)
}

fn commit_provider_restore(
    pending: WorthQueryDirectProviderRestorePending,
    bridge_runtime: &RuntimeBridge,
    counters: WorthQueryReadmissionCounters,
) -> WorthQueryDirectReadmissionOutcome {
    let WorthQueryDirectProviderRestorePending {
        state,
        provider,
        resource,
        bridge,
    } = pending;
    let execution = match provider.commit(None) {
        WorthQueryManagedGraphRestoreCommitOutcome::Restored(execution) => execution,
        WorthQueryManagedGraphRestoreCommitOutcome::RecoveryRequired(recovery) => {
            let kind = recovery_kind(recovery.kind());
            let detail = Arc::from(recovery.detail());
            return WorthQueryDirectReadmissionOutcome::RecoveryRequired(
                WorthQueryDirectReadmissionRecoveryRequired::provider(
                    kind,
                    detail,
                    counters,
                    WorthQueryDirectProviderRecoveryState {
                        state,
                        resource,
                        bridge,
                        provider: recovery,
                    },
                ),
            );
        }
    };
    commit_direct(
        WorthQueryDirectCommitReady {
            state,
            execution,
            resource,
            bridge,
        },
        bridge_runtime,
        counters,
    )
}

fn commit_direct(
    pending: WorthQueryDirectCommitReady,
    bridge_runtime: &RuntimeBridge,
    mut counters: WorthQueryReadmissionCounters,
) -> WorthQueryDirectReadmissionOutcome {
    let WorthQueryDirectCommitReady {
        state,
        execution,
        resource,
        bridge,
    } = pending;
    let bridge_basis = bridge_runtime.commit_yielded_execution_basis_readmission(bridge);
    let resource_attempt = resource.commit();
    let identity = WorthQueryManagedRunIdentity::resumed(
        "direct",
        Arc::clone(&state.logical_run_identity),
        resource_attempt.attempt_identity().as_str(),
        &bridge_basis,
        &state.relational_basis,
    );
    let (logical_run_identity, identity) = identity.into_parts();
    let provider_work = state
        .provider_work
        .rebind_provider_session(resource_attempt.provider_session().identity());
    counters.committed_attempt();
    WorthQueryDirectReadmissionOutcome::Readmitted(WorthQueryActiveDirectGraphExecution::new(
        WorthQueryRunningDirectRun {
            logical_run_identity,
            identity,
            resource_attempt,
            bridge_basis,
            relational_basis: state.relational_basis,
            counters: state.run_counters,
            provider_work,
        },
        execution,
    ))
}

fn abort_to_denial(
    kind: WorthQueryDirectReadmissionDenialKind,
    detail: impl Into<Arc<str>>,
    pending: WorthQueryDirectRollbackPending,
    counters: WorthQueryReadmissionCounters,
) -> WorthQueryDirectReadmissionOutcome {
    let detail = detail.into();
    let WorthQueryDirectRollbackPending {
        state,
        execution,
        resource,
        bridge,
    } = pending;
    match bridge.abort() {
        BridgeExecutionBasisReadmissionCleanupOutcome::Complete(bridge) => denied(
            kind,
            detail,
            WorthQueryDirectYieldedParts {
                state,
                resource_attempt: resource.abort(),
                bridge,
                execution,
            }
            .into_yielded(),
            counters,
        ),
        BridgeExecutionBasisReadmissionCleanupOutcome::RecoveryRequired(recovery) => {
            WorthQueryDirectReadmissionOutcome::RecoveryRequired(
                WorthQueryDirectReadmissionRecoveryRequired::bridge_cleanup(
                    format!("{detail}; Bridge cleanup failed: {}", recovery.detail()),
                    counters,
                    WorthQueryDirectBridgeCleanupRecoveryState {
                        state,
                        resource_attempt: resource.abort(),
                        execution,
                        bridge: recovery,
                    },
                ),
            )
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

fn recovery_kind(
    kind: WorthQueryManagedGraphRestoreRecoveryKind,
) -> WorthQueryDirectReadmissionRecoveryKind {
    match kind {
        WorthQueryManagedGraphRestoreRecoveryKind::ProviderRestorePanicked => {
            WorthQueryDirectReadmissionRecoveryKind::ProviderRestorePanicked
        }
        WorthQueryManagedGraphRestoreRecoveryKind::
            ProviderRestoreRejectedAfterExecutionAdmission => {
                WorthQueryDirectReadmissionRecoveryKind::
                    ProviderRestoreRejectedAfterExecutionAdmission
            }
        WorthQueryManagedGraphRestoreRecoveryKind::RestoredExecutionReleaseRecoveryRequired => {
            WorthQueryDirectReadmissionRecoveryKind::RestoredExecutionReleaseRecoveryRequired
        }
        WorthQueryManagedGraphRestoreRecoveryKind::CheckpointReleasePanicked => {
            WorthQueryDirectReadmissionRecoveryKind::CheckpointReleasePanicked
        }
    }
}
