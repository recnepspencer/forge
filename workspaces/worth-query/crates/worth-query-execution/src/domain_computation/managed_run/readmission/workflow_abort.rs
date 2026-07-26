use std::sync::Arc;

use worth_runtime_bridge::facade::BridgeExecutionBasisReadmissionCleanupOutcome;

use super::super::provider_restore::{
    WorthQueryManagedGraphRestoreAbortOutcome, WorthQueryManagedGraphRestoreRecoveryKind,
};
use super::evidence::WorthQueryReadmissionProgress;
use super::recovery::{
    WorthQueryWorkflowReadmissionRecoveryKind, WorthQueryWorkflowReadmissionRecoveryRequired,
};
use super::workflow_outcome::{
    WorthQueryWorkflowReadmissionDenialKind, WorthQueryWorkflowReadmissionDenied,
    WorthQueryWorkflowReadmissionOutcome,
};
use super::workflow_state::{
    WorthQueryWorkflowBridgeCleanupRecoveryState, WorthQueryWorkflowProviderAbortPending,
    WorthQueryWorkflowProviderRecoveryState, WorthQueryWorkflowRollbackPending,
    WorthQueryWorkflowYieldedParts,
};

pub(super) fn abort_without_provider(
    kind: WorthQueryWorkflowReadmissionDenialKind,
    detail: impl Into<Arc<str>>,
    pending: WorthQueryWorkflowRollbackPending,
    mut progress: WorthQueryReadmissionProgress,
) -> WorthQueryWorkflowReadmissionOutcome {
    let detail = detail.into();
    let WorthQueryWorkflowRollbackPending {
        state,
        execution,
        resource,
        bridge,
    } = pending;
    match bridge.abort() {
        BridgeExecutionBasisReadmissionCleanupOutcome::Complete(returned) => {
            let (bridge, bridge_counters) = returned.into_parts();
            progress.observe_bridge(bridge_counters);
            WorthQueryWorkflowReadmissionOutcome::Denied(WorthQueryWorkflowReadmissionDenied::new(
                kind,
                detail,
                WorthQueryWorkflowYieldedParts {
                    state,
                    resource_attempt: resource.abort(),
                    bridge,
                    execution,
                }
                .into_yielded(),
                progress.evidence(),
            ))
        }
        BridgeExecutionBasisReadmissionCleanupOutcome::RecoveryRequired(recovery) => {
            progress.observe_bridge(recovery.counters());
            WorthQueryWorkflowReadmissionOutcome::RecoveryRequired(
                WorthQueryWorkflowReadmissionRecoveryRequired::bridge_cleanup(
                    format!("{detail}; Bridge cleanup failed: {}", recovery.detail()),
                    progress,
                    WorthQueryWorkflowBridgeCleanupRecoveryState {
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

pub(super) fn abort_provider_pending(
    kind: WorthQueryWorkflowReadmissionDenialKind,
    detail: impl Into<Arc<str>>,
    pending: WorthQueryWorkflowProviderAbortPending,
    progress: WorthQueryReadmissionProgress,
) -> WorthQueryWorkflowReadmissionOutcome {
    let detail = detail.into();
    let WorthQueryWorkflowProviderAbortPending {
        state,
        provider,
        resource,
        bridge,
    } = pending;
    match provider.abort() {
        WorthQueryManagedGraphRestoreAbortOutcome::Aborted(execution) => abort_without_provider(
            kind,
            detail,
            WorthQueryWorkflowRollbackPending {
                state,
                execution,
                resource,
                bridge,
            },
            progress,
        ),
        WorthQueryManagedGraphRestoreAbortOutcome::RecoveryRequired(recovery) => {
            let recovery_kind = map_recovery_kind(recovery.kind());
            WorthQueryWorkflowReadmissionOutcome::RecoveryRequired(
                WorthQueryWorkflowReadmissionRecoveryRequired::provider(
                    recovery_kind,
                    format!("{detail}; {}", recovery.detail()),
                    progress,
                    WorthQueryWorkflowProviderRecoveryState {
                        state,
                        resource,
                        bridge,
                        provider: recovery,
                    },
                ),
            )
        }
    }
}

pub(super) fn map_recovery_kind(
    kind: WorthQueryManagedGraphRestoreRecoveryKind,
) -> WorthQueryWorkflowReadmissionRecoveryKind {
    match kind {
        WorthQueryManagedGraphRestoreRecoveryKind::ProviderRestorePanicked => {
            WorthQueryWorkflowReadmissionRecoveryKind::ProviderRestorePanicked
        }
        WorthQueryManagedGraphRestoreRecoveryKind::
            ProviderRestoreRejectedAfterExecutionAdmission => {
                WorthQueryWorkflowReadmissionRecoveryKind::
                    ProviderRestoreRejectedAfterExecutionAdmission
            }
        WorthQueryManagedGraphRestoreRecoveryKind::RestoredExecutionReleaseRecoveryRequired => {
            WorthQueryWorkflowReadmissionRecoveryKind::RestoredExecutionReleaseRecoveryRequired
        }
        WorthQueryManagedGraphRestoreRecoveryKind::CheckpointReleasePanicked => {
            WorthQueryWorkflowReadmissionRecoveryKind::CheckpointReleasePanicked
        }
    }
}
