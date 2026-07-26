use std::sync::Arc;

use worth_runtime_bridge::facade::BridgeExecutionBasisReadmissionCleanupOutcome;

use super::super::WorthQueryYieldedWorkflowRun;
use super::counters::WorthQueryReadmissionCounters;
use super::workflow_state::{
    WorthQueryWorkflowBridgeCleanupRecoveryState, WorthQueryWorkflowProviderPendingRecoveryState,
    WorthQueryWorkflowProviderRecoveryState, WorthQueryWorkflowYieldedParts,
    WorthQueryWorkflowYieldedReassembly,
};
use crate::domain_computation::provider_session::graph_provider::bounded_step::WorthQueryProviderExecutionReleaseEvidence;
use crate::domain_computation::{
    WorthQueryProviderCheckpointEvidence, WorthQueryProviderCheckpointReleaseEvidence,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryWorkflowReadmissionRecoveryKind {
    BridgeCleanupFailed,
    ProviderRestorePanicked,
    ProviderRestoreRejectedAfterExecutionAdmission,
    RestoredExecutionReleaseRecoveryRequired,
    CheckpointReleasePanicked,
    ArtifactGenerationRollbackFailed,
}

#[must_use = "workflow readmission recovery must be explicitly resolved"]
pub struct WorthQueryWorkflowReadmissionRecoveryRequired {
    kind: WorthQueryWorkflowReadmissionRecoveryKind,
    detail: Arc<str>,
    counters: WorthQueryReadmissionCounters,
    resource: WorthQueryWorkflowReadmissionRecoveryResource,
}

#[must_use = "workflow readmission recovery retry retains yielded or recovery authority"]
pub enum WorthQueryWorkflowReadmissionRecoveryRetryOutcome {
    Yielded(WorthQueryYieldedWorkflowRun),
    RecoveryRequired(WorthQueryWorkflowReadmissionRecoveryRequired),
}

enum WorthQueryWorkflowReadmissionRecoveryResource {
    BridgeCleanup(WorthQueryWorkflowBridgeCleanupRecoveryState),
    Provider(WorthQueryWorkflowProviderRecoveryState),
    ProviderPending(WorthQueryWorkflowProviderPendingRecoveryState),
}

impl WorthQueryWorkflowReadmissionRecoveryRequired {
    pub(super) fn bridge_cleanup(
        detail: impl Into<Arc<str>>,
        counters: WorthQueryReadmissionCounters,
        recovery: WorthQueryWorkflowBridgeCleanupRecoveryState,
    ) -> Self {
        Self {
            kind: WorthQueryWorkflowReadmissionRecoveryKind::BridgeCleanupFailed,
            detail: detail.into(),
            counters,
            resource: WorthQueryWorkflowReadmissionRecoveryResource::BridgeCleanup(recovery),
        }
    }

    pub(super) fn provider(
        kind: WorthQueryWorkflowReadmissionRecoveryKind,
        detail: impl Into<Arc<str>>,
        counters: WorthQueryReadmissionCounters,
        recovery: WorthQueryWorkflowProviderRecoveryState,
    ) -> Self {
        Self {
            kind,
            detail: detail.into(),
            counters,
            resource: WorthQueryWorkflowReadmissionRecoveryResource::Provider(recovery),
        }
    }

    pub(super) fn provider_pending(
        detail: impl Into<Arc<str>>,
        counters: WorthQueryReadmissionCounters,
        recovery: WorthQueryWorkflowProviderPendingRecoveryState,
    ) -> Self {
        Self {
            kind: WorthQueryWorkflowReadmissionRecoveryKind::ArtifactGenerationRollbackFailed,
            detail: detail.into(),
            counters,
            resource: WorthQueryWorkflowReadmissionRecoveryResource::ProviderPending(recovery),
        }
    }

    pub const fn kind(&self) -> WorthQueryWorkflowReadmissionRecoveryKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }

    pub const fn counters(&self) -> WorthQueryReadmissionCounters {
        self.counters
    }

    pub fn checkpoint(&self) -> &WorthQueryProviderCheckpointEvidence {
        match &self.resource {
            WorthQueryWorkflowReadmissionRecoveryResource::BridgeCleanup(recovery) => {
                recovery.execution.checkpoint_evidence()
            }
            WorthQueryWorkflowReadmissionRecoveryResource::Provider(recovery) => {
                recovery.provider.checkpoint_evidence()
            }
            WorthQueryWorkflowReadmissionRecoveryResource::ProviderPending(recovery) => {
                recovery.provider.checkpoint_evidence()
            }
        }
    }

    pub fn checkpoint_release(&self) -> Option<&WorthQueryProviderCheckpointReleaseEvidence> {
        match &self.resource {
            WorthQueryWorkflowReadmissionRecoveryResource::Provider(recovery) => {
                recovery.provider.checkpoint_release()
            }
            WorthQueryWorkflowReadmissionRecoveryResource::BridgeCleanup(_)
            | WorthQueryWorkflowReadmissionRecoveryResource::ProviderPending(_) => None,
        }
    }

    pub fn checkpoint_authority_retained(&self) -> bool {
        match &self.resource {
            WorthQueryWorkflowReadmissionRecoveryResource::Provider(recovery) => {
                recovery.provider.checkpoint_retained()
            }
            WorthQueryWorkflowReadmissionRecoveryResource::BridgeCleanup(_)
            | WorthQueryWorkflowReadmissionRecoveryResource::ProviderPending(_) => true,
        }
    }

    pub fn restored_execution_release_evidence(
        &self,
    ) -> Option<&WorthQueryProviderExecutionReleaseEvidence> {
        match &self.resource {
            WorthQueryWorkflowReadmissionRecoveryResource::Provider(recovery) => {
                recovery.provider.restored_execution_release_evidence()
            }
            WorthQueryWorkflowReadmissionRecoveryResource::BridgeCleanup(_)
            | WorthQueryWorkflowReadmissionRecoveryResource::ProviderPending(_) => None,
        }
    }

    pub fn replacement_execution_active(&self) -> bool {
        matches!(
            self.resource,
            WorthQueryWorkflowReadmissionRecoveryResource::ProviderPending(_)
        )
    }

    pub const fn bridge_cleanup_pending(&self) -> bool {
        true
    }

    pub fn fresh_resource_attempt_pending(&self) -> bool {
        !matches!(
            self.resource,
            WorthQueryWorkflowReadmissionRecoveryResource::BridgeCleanup(_)
        )
    }

    pub fn retained_authority_count(&self) -> usize {
        match &self.resource {
            WorthQueryWorkflowReadmissionRecoveryResource::BridgeCleanup(recovery) => {
                let _ = (
                    recovery.state.artifacts.run_identity(),
                    recovery.resource_attempt.attempt_identity(),
                    recovery.bridge.yielded_receipt(),
                    recovery.execution.checkpoint_evidence(),
                );
                5
            }
            WorthQueryWorkflowReadmissionRecoveryResource::Provider(recovery) => {
                let _ = (
                    recovery.state.artifacts.run_identity(),
                    recovery.resource.attempt_identity(),
                    recovery.bridge.fresh_request_identity(),
                    recovery.provider.kind(),
                );
                5
            }
            WorthQueryWorkflowReadmissionRecoveryResource::ProviderPending(recovery) => {
                let _ = (
                    recovery.state.artifacts.run_identity(),
                    recovery.resource.attempt_identity(),
                    recovery.bridge.fresh_request_identity(),
                    &recovery.provider,
                );
                5
            }
        }
    }

    pub fn retry_to_yielded(
        self,
    ) -> Result<WorthQueryWorkflowReadmissionRecoveryRetryOutcome, Self> {
        if !self.checkpoint_authority_retained() || self.replacement_execution_active() {
            return Err(self);
        }
        let counters = self.counters;
        match self.resource {
            WorthQueryWorkflowReadmissionRecoveryResource::BridgeCleanup(recovery) => {
                let WorthQueryWorkflowBridgeCleanupRecoveryState {
                    state,
                    resource_attempt,
                    bridge,
                    execution,
                } = recovery;
                Ok(retry_workflow_bridge_cleanup(
                    WorthQueryWorkflowYieldedReassembly {
                        state,
                        resource_attempt,
                        execution,
                    },
                    bridge.retry_cleanup(),
                    counters,
                ))
            }
            WorthQueryWorkflowReadmissionRecoveryResource::Provider(mut recovery) => {
                let retryable = match recovery.provider.into_retryable() {
                    Ok(retryable) => retryable,
                    Err(_) => {
                        unreachable!("retained checkpoint posture was checked before recovery")
                    }
                };
                if let Some(release) = &retryable.restored_execution_release {
                    recovery
                        .state
                        .provider_work
                        .record_provider_execution_release(release);
                }
                Ok(retry_workflow_bridge_cleanup(
                    WorthQueryWorkflowYieldedReassembly {
                        state: recovery.state,
                        resource_attempt: recovery.resource.abort(),
                        execution: retryable.retained,
                    },
                    recovery.bridge.abort(),
                    counters,
                ))
            }
            WorthQueryWorkflowReadmissionRecoveryResource::ProviderPending(_) => {
                unreachable!("active replacement execution was rejected before recovery")
            }
        }
    }
}

fn retry_workflow_bridge_cleanup(
    pending: WorthQueryWorkflowYieldedReassembly,
    bridge: BridgeExecutionBasisReadmissionCleanupOutcome,
    counters: WorthQueryReadmissionCounters,
) -> WorthQueryWorkflowReadmissionRecoveryRetryOutcome {
    let WorthQueryWorkflowYieldedReassembly {
        state,
        resource_attempt,
        execution,
    } = pending;
    match bridge {
        BridgeExecutionBasisReadmissionCleanupOutcome::Complete(bridge) => {
            WorthQueryWorkflowReadmissionRecoveryRetryOutcome::Yielded(
                WorthQueryWorkflowYieldedParts {
                    state,
                    resource_attempt,
                    bridge,
                    execution,
                }
                .into_yielded(),
            )
        }
        BridgeExecutionBasisReadmissionCleanupOutcome::RecoveryRequired(bridge) => {
            let detail = bridge.detail().to_owned();
            WorthQueryWorkflowReadmissionRecoveryRetryOutcome::RecoveryRequired(
                WorthQueryWorkflowReadmissionRecoveryRequired::bridge_cleanup(
                    detail,
                    counters,
                    WorthQueryWorkflowBridgeCleanupRecoveryState {
                        state,
                        resource_attempt,
                        execution,
                        bridge,
                    },
                ),
            )
        }
    }
}
