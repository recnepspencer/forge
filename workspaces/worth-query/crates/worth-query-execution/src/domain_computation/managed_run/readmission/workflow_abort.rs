use std::sync::Arc;

use worth_runtime_bridge::facade::BridgeExecutionBasisReadmissionCleanupOutcome;

use super::super::provider_restore::{
    WorthQueryManagedGraphRestoreAbortOutcome, WorthQueryManagedGraphRestorePending,
    WorthQueryManagedGraphRestoreRecoveryKind,
};
use super::counters::WorthQueryReadmissionCounters;
use super::workflow_outcome::{
    WorthQueryWorkflowReadmissionDenialKind, WorthQueryWorkflowReadmissionDenied,
    WorthQueryWorkflowReadmissionOutcome, WorthQueryWorkflowReadmissionRecoveryKind,
    WorthQueryWorkflowReadmissionRecoveryRequired,
};
use super::workflow_state::{WorthQueryWorkflowYieldedParts, WorthQueryWorkflowYieldedState};
use crate::domain_computation::provider_session::readmission::WorthQueryWorkflowResourceReadmissionPending;

pub(super) fn abort_without_provider(
    kind: WorthQueryWorkflowReadmissionDenialKind,
    detail: impl Into<Arc<str>>,
    state: WorthQueryWorkflowYieldedState,
    execution: super::super::retained_graph_execution::WorthQueryRetainedManagedGraphExecution,
    resource_pending: WorthQueryWorkflowResourceReadmissionPending,
    bridge_pending: worth_runtime_bridge::facade::BridgeExecutionBasisReadmissionPending,
    counters: WorthQueryReadmissionCounters,
) -> WorthQueryWorkflowReadmissionOutcome {
    let detail = detail.into();
    match bridge_pending.abort() {
        BridgeExecutionBasisReadmissionCleanupOutcome::Complete(bridge) => {
            WorthQueryWorkflowReadmissionOutcome::Denied(WorthQueryWorkflowReadmissionDenied::new(
                kind,
                detail,
                WorthQueryWorkflowYieldedParts {
                    state,
                    resource_attempt: resource_pending.abort(),
                    bridge,
                    execution,
                }
                .into_yielded(),
                counters,
            ))
        }
        BridgeExecutionBasisReadmissionCleanupOutcome::RecoveryRequired(recovery) => {
            WorthQueryWorkflowReadmissionOutcome::RecoveryRequired(
                WorthQueryWorkflowReadmissionRecoveryRequired::bridge_cleanup(
                    format!("{detail}; Bridge cleanup failed: {}", recovery.detail()),
                    counters,
                    state,
                    resource_pending.abort(),
                    execution,
                    recovery,
                ),
            )
        }
    }
}

pub(super) fn abort_provider_pending(
    kind: WorthQueryWorkflowReadmissionDenialKind,
    detail: impl Into<Arc<str>>,
    state: WorthQueryWorkflowYieldedState,
    provider: WorthQueryManagedGraphRestorePending,
    resource_pending: WorthQueryWorkflowResourceReadmissionPending,
    bridge_pending: worth_runtime_bridge::facade::BridgeExecutionBasisReadmissionPending,
    counters: WorthQueryReadmissionCounters,
) -> WorthQueryWorkflowReadmissionOutcome {
    let detail = detail.into();
    match provider.abort() {
        WorthQueryManagedGraphRestoreAbortOutcome::Aborted(execution) => abort_without_provider(
            kind,
            detail,
            state,
            execution,
            resource_pending,
            bridge_pending,
            counters,
        ),
        WorthQueryManagedGraphRestoreAbortOutcome::RecoveryRequired(recovery) => {
            let recovery_kind = map_recovery_kind(recovery.kind());
            WorthQueryWorkflowReadmissionOutcome::RecoveryRequired(
                WorthQueryWorkflowReadmissionRecoveryRequired::provider(
                    recovery_kind,
                    format!("{detail}; {}", recovery.detail()),
                    counters,
                    state,
                    resource_pending,
                    bridge_pending,
                    recovery,
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
