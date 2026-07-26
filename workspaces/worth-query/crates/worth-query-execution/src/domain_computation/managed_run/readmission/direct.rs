use std::sync::Arc;

use worth_runtime_bridge::facade::{BridgeExecutionBasisReadmissionCleanupOutcome, RuntimeBridge};

use super::super::provider_restore::{
    self, WorthQueryManagedGraphRestoreCommitOutcome, WorthQueryManagedGraphRestoreOutcome,
    WorthQueryManagedGraphRestoreRecoveryKind,
};
use super::super::run_identity::WorthQueryManagedRunIdentity;
use super::super::{
    WorthQueryActiveDirectGraphExecution, WorthQueryRunningDirectRun, WorthQueryYieldedDirectRun,
};
use super::direct_outcome::{
    WorthQueryDirectReadmissionDenialKind, WorthQueryDirectReadmissionDenied,
    WorthQueryDirectReadmissionOutcome,
};
use super::direct_preparation::prepare_direct_provider_restore;
use super::direct_state::{
    WorthQueryDirectBridgeCleanupRecoveryState, WorthQueryDirectBridgeReadmissionPending,
    WorthQueryDirectCommitReady, WorthQueryDirectProviderRecoveryState,
    WorthQueryDirectProviderRestorePending, WorthQueryDirectRollbackPending,
    WorthQueryDirectYieldedParts,
};
use super::evidence::WorthQueryReadmissionProgress;
use super::recovery::{
    WorthQueryDirectReadmissionRecoveryKind, WorthQueryDirectReadmissionRecoveryRequired,
};
use crate::domain_computation::WorthQueryExecutionRuntime;

pub(in crate::domain_computation::managed_run) fn readmit_direct(
    yielded: WorthQueryYieldedDirectRun,
    query_runtime: &WorthQueryExecutionRuntime,
    bridge_runtime: &RuntimeBridge,
) -> WorthQueryDirectReadmissionOutcome {
    let (pending, counters) =
        match prepare_direct_provider_restore(yielded, query_runtime, bridge_runtime) {
            Ok(prepared) => prepared,
            Err(outcome) => return outcome,
        };
    restore_direct(pending, bridge_runtime, counters)
}

pub(in crate::domain_computation::managed_run) fn restore_direct(
    pending: WorthQueryDirectBridgeReadmissionPending,
    bridge_runtime: &RuntimeBridge,
    mut progress: WorthQueryReadmissionProgress,
) -> WorthQueryDirectReadmissionOutcome {
    let WorthQueryDirectBridgeReadmissionPending {
        state,
        execution,
        resource: resource_pending,
        bridge: bridge_pending,
        fresh_call,
        contract,
    } = pending;
    progress.attempted_provider_restore();
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
                progress,
            );
        }
        WorthQueryManagedGraphRestoreOutcome::RecoveryRequired(recovery) => {
            let kind = recovery_kind(recovery.kind());
            let detail = Arc::from(recovery.detail());
            return WorthQueryDirectReadmissionOutcome::RecoveryRequired(
                WorthQueryDirectReadmissionRecoveryRequired::provider(
                    kind,
                    detail,
                    progress,
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
    commit_provider_restore(provider, bridge_runtime, progress)
}

fn commit_provider_restore(
    pending: WorthQueryDirectProviderRestorePending,
    bridge_runtime: &RuntimeBridge,
    progress: WorthQueryReadmissionProgress,
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
                    progress,
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
        progress,
    )
}

fn commit_direct(
    pending: WorthQueryDirectCommitReady,
    bridge_runtime: &RuntimeBridge,
    mut progress: WorthQueryReadmissionProgress,
) -> WorthQueryDirectReadmissionOutcome {
    let WorthQueryDirectCommitReady {
        state,
        execution,
        resource,
        bridge,
    } = pending;
    let (bridge_basis, bridge_counters) = bridge_runtime
        .commit_yielded_execution_basis_readmission(bridge)
        .into_parts();
    progress.observe_bridge(bridge_counters);
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
    progress.committed_attempt();
    let active = WorthQueryActiveDirectGraphExecution::new(
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
    );
    WorthQueryDirectReadmissionOutcome::Readmitted(
        super::WorthQueryReadmittedDirectGraphExecution::new(active, progress.evidence()),
    )
}

fn abort_to_denial(
    kind: WorthQueryDirectReadmissionDenialKind,
    detail: impl Into<Arc<str>>,
    pending: WorthQueryDirectRollbackPending,
    mut progress: WorthQueryReadmissionProgress,
) -> WorthQueryDirectReadmissionOutcome {
    let detail = detail.into();
    let WorthQueryDirectRollbackPending {
        state,
        execution,
        resource,
        bridge,
    } = pending;
    match bridge.abort() {
        BridgeExecutionBasisReadmissionCleanupOutcome::Complete(returned) => {
            let (bridge, bridge_counters) = returned.into_parts();
            progress.observe_bridge(bridge_counters);
            denied(
                kind,
                detail,
                WorthQueryDirectYieldedParts {
                    state,
                    resource_attempt: resource.abort(),
                    bridge,
                    execution,
                }
                .into_yielded(),
                progress,
            )
        }
        BridgeExecutionBasisReadmissionCleanupOutcome::RecoveryRequired(recovery) => {
            progress.observe_bridge(recovery.counters());
            WorthQueryDirectReadmissionOutcome::RecoveryRequired(
                WorthQueryDirectReadmissionRecoveryRequired::bridge_cleanup(
                    format!("{detail}; Bridge cleanup failed: {}", recovery.detail()),
                    progress,
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
    progress: WorthQueryReadmissionProgress,
) -> WorthQueryDirectReadmissionOutcome {
    WorthQueryDirectReadmissionOutcome::Denied(WorthQueryDirectReadmissionDenied::new(
        kind,
        detail,
        yielded,
        progress.evidence(),
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
