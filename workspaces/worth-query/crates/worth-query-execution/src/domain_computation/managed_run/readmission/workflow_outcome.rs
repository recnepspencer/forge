use std::sync::Arc;

use worth_runtime_bridge::facade::{
    BridgeExecutionBasisReadmissionCleanupOutcome, BridgeExecutionBasisReadmissionPending,
    BridgeExecutionBasisReadmissionRecoveryRequired,
};

use super::super::provider_restore::{
    WorthQueryManagedGraphRestorePending, WorthQueryManagedGraphRestoreRecoveryRequired,
};
use super::super::{WorthQueryActiveWorkflowGraphExecution, WorthQueryYieldedWorkflowRun};
use super::counters::WorthQueryReadmissionCounters;
use super::workflow_state::{WorthQueryWorkflowYieldedParts, WorthQueryWorkflowYieldedState};
use crate::domain_computation::provider_session::graph_provider::bounded_step::WorthQueryProviderExecutionReleaseEvidence;
use crate::domain_computation::provider_session::readmission::WorthQueryWorkflowResourceReadmissionPending;
use crate::domain_computation::{
    WorthQueryProviderCheckpointEvidence, WorthQueryProviderCheckpointReleaseEvidence,
    WorthQueryWorkflowExecutionResourceAttempt,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryWorkflowReadmissionDenialKind {
    ForeignQueryRuntime,
    StaleInstallationGeneration,
    RetainedCapacityMismatch,
    RelationalLeaseNotLive,
    ProviderCheckpointMismatch,
    ArtifactGenerationMismatch,
    BridgeReadmissionDenied,
    WorkflowStageResourcesUnavailable,
    ProviderCallBindingDenied,
    ProviderStepContractDenied(super::super::WorthQueryManagedStepContractDenialKind),
    ProviderRestoreDenied,
    ArtifactGenerationDenied,
    ArtifactAuthorityDenied,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryWorkflowReadmissionRecoveryKind {
    BridgeCleanupFailed,
    ProviderRestorePanicked,
    ProviderRestoreRejectedAfterExecutionAdmission,
    RestoredExecutionReleaseRecoveryRequired,
    CheckpointReleasePanicked,
    ArtifactGenerationRollbackFailed,
}

pub enum WorthQueryWorkflowReadmissionOutcome {
    Readmitted(WorthQueryActiveWorkflowGraphExecution),
    Denied(WorthQueryWorkflowReadmissionDenied),
    RecoveryRequired(WorthQueryWorkflowReadmissionRecoveryRequired),
}

pub struct WorthQueryWorkflowReadmissionDenied {
    kind: WorthQueryWorkflowReadmissionDenialKind,
    detail: Arc<str>,
    yielded: WorthQueryYieldedWorkflowRun,
    counters: WorthQueryReadmissionCounters,
}

pub struct WorthQueryWorkflowReadmissionRecoveryRequired {
    kind: WorthQueryWorkflowReadmissionRecoveryKind,
    detail: Arc<str>,
    counters: WorthQueryReadmissionCounters,
    resource: WorthQueryWorkflowReadmissionRecoveryResource,
}

pub enum WorthQueryWorkflowReadmissionRecoveryRetryOutcome {
    Yielded(WorthQueryYieldedWorkflowRun),
    RecoveryRequired(WorthQueryWorkflowReadmissionRecoveryRequired),
}

pub(super) enum WorthQueryWorkflowReadmissionRecoveryResource {
    BridgeCleanup {
        state: WorthQueryWorkflowYieldedState,
        resource_attempt: WorthQueryWorkflowExecutionResourceAttempt,
        bridge: BridgeExecutionBasisReadmissionRecoveryRequired,
        execution: super::super::retained_graph_execution::WorthQueryRetainedManagedGraphExecution,
    },
    Provider {
        state: WorthQueryWorkflowYieldedState,
        resource_attempt: WorthQueryWorkflowResourceReadmissionPending,
        bridge: BridgeExecutionBasisReadmissionPending,
        provider: WorthQueryManagedGraphRestoreRecoveryRequired,
    },
    ProviderPending {
        state: WorthQueryWorkflowYieldedState,
        resource_attempt: WorthQueryWorkflowResourceReadmissionPending,
        bridge: BridgeExecutionBasisReadmissionPending,
        provider: WorthQueryManagedGraphRestorePending,
    },
}

impl WorthQueryWorkflowReadmissionDenied {
    pub(super) fn new(
        kind: WorthQueryWorkflowReadmissionDenialKind,
        detail: impl Into<Arc<str>>,
        yielded: WorthQueryYieldedWorkflowRun,
        counters: WorthQueryReadmissionCounters,
    ) -> Self {
        Self {
            kind,
            detail: detail.into(),
            yielded,
            counters,
        }
    }

    pub const fn kind(&self) -> WorthQueryWorkflowReadmissionDenialKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }

    pub const fn counters(&self) -> WorthQueryReadmissionCounters {
        self.counters
    }

    pub fn into_yielded(self) -> WorthQueryYieldedWorkflowRun {
        self.yielded
    }
}

impl WorthQueryWorkflowReadmissionRecoveryRequired {
    pub(super) fn bridge_cleanup(
        detail: impl Into<Arc<str>>,
        counters: WorthQueryReadmissionCounters,
        state: WorthQueryWorkflowYieldedState,
        resource_attempt: WorthQueryWorkflowExecutionResourceAttempt,
        execution: super::super::retained_graph_execution::WorthQueryRetainedManagedGraphExecution,
        bridge: BridgeExecutionBasisReadmissionRecoveryRequired,
    ) -> Self {
        Self {
            kind: WorthQueryWorkflowReadmissionRecoveryKind::BridgeCleanupFailed,
            detail: detail.into(),
            counters,
            resource: WorthQueryWorkflowReadmissionRecoveryResource::BridgeCleanup {
                state,
                resource_attempt,
                bridge,
                execution,
            },
        }
    }

    pub(super) fn provider(
        kind: WorthQueryWorkflowReadmissionRecoveryKind,
        detail: impl Into<Arc<str>>,
        counters: WorthQueryReadmissionCounters,
        state: WorthQueryWorkflowYieldedState,
        resource_attempt: WorthQueryWorkflowResourceReadmissionPending,
        bridge: BridgeExecutionBasisReadmissionPending,
        provider: WorthQueryManagedGraphRestoreRecoveryRequired,
    ) -> Self {
        Self {
            kind,
            detail: detail.into(),
            counters,
            resource: WorthQueryWorkflowReadmissionRecoveryResource::Provider {
                state,
                resource_attempt,
                bridge,
                provider,
            },
        }
    }

    pub(super) fn provider_pending(
        detail: impl Into<Arc<str>>,
        counters: WorthQueryReadmissionCounters,
        state: WorthQueryWorkflowYieldedState,
        resource_attempt: WorthQueryWorkflowResourceReadmissionPending,
        bridge: BridgeExecutionBasisReadmissionPending,
        provider: WorthQueryManagedGraphRestorePending,
    ) -> Self {
        Self {
            kind: WorthQueryWorkflowReadmissionRecoveryKind::ArtifactGenerationRollbackFailed,
            detail: detail.into(),
            counters,
            resource: WorthQueryWorkflowReadmissionRecoveryResource::ProviderPending {
                state,
                resource_attempt,
                bridge,
                provider,
            },
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
            WorthQueryWorkflowReadmissionRecoveryResource::BridgeCleanup { execution, .. } => {
                execution.checkpoint_evidence()
            }
            WorthQueryWorkflowReadmissionRecoveryResource::Provider { provider, .. } => {
                provider.checkpoint_evidence()
            }
            WorthQueryWorkflowReadmissionRecoveryResource::ProviderPending { provider, .. } => {
                provider.checkpoint_evidence()
            }
        }
    }

    pub fn checkpoint_release(&self) -> Option<&WorthQueryProviderCheckpointReleaseEvidence> {
        match &self.resource {
            WorthQueryWorkflowReadmissionRecoveryResource::Provider { provider, .. } => {
                provider.checkpoint_release()
            }
            WorthQueryWorkflowReadmissionRecoveryResource::BridgeCleanup { .. }
            | WorthQueryWorkflowReadmissionRecoveryResource::ProviderPending { .. } => None,
        }
    }

    pub fn checkpoint_authority_retained(&self) -> bool {
        match &self.resource {
            WorthQueryWorkflowReadmissionRecoveryResource::Provider { provider, .. } => {
                provider.checkpoint_retained()
            }
            WorthQueryWorkflowReadmissionRecoveryResource::BridgeCleanup { .. }
            | WorthQueryWorkflowReadmissionRecoveryResource::ProviderPending { .. } => true,
        }
    }

    pub fn restored_execution_release_evidence(
        &self,
    ) -> Option<&WorthQueryProviderExecutionReleaseEvidence> {
        match &self.resource {
            WorthQueryWorkflowReadmissionRecoveryResource::Provider { provider, .. } => {
                provider.restored_execution_release_evidence()
            }
            WorthQueryWorkflowReadmissionRecoveryResource::BridgeCleanup { .. }
            | WorthQueryWorkflowReadmissionRecoveryResource::ProviderPending { .. } => None,
        }
    }

    pub fn replacement_execution_active(&self) -> bool {
        matches!(
            self.resource,
            WorthQueryWorkflowReadmissionRecoveryResource::ProviderPending { .. }
        )
    }

    pub const fn bridge_cleanup_pending(&self) -> bool {
        true
    }

    pub fn fresh_resource_attempt_pending(&self) -> bool {
        !matches!(
            self.resource,
            WorthQueryWorkflowReadmissionRecoveryResource::BridgeCleanup { .. }
        )
    }

    pub fn retained_authority_count(&self) -> usize {
        match &self.resource {
            WorthQueryWorkflowReadmissionRecoveryResource::BridgeCleanup {
                state,
                resource_attempt,
                bridge,
                execution,
            } => {
                let _ = (
                    state.artifacts.run_identity(),
                    resource_attempt.attempt_identity(),
                    bridge.yielded_receipt(),
                    execution.checkpoint_evidence(),
                );
                5
            }
            WorthQueryWorkflowReadmissionRecoveryResource::Provider {
                state,
                resource_attempt,
                bridge,
                provider,
            } => {
                let _ = (
                    state.artifacts.run_identity(),
                    resource_attempt.attempt_identity(),
                    bridge.fresh_request_identity(),
                    provider.kind(),
                );
                5
            }
            WorthQueryWorkflowReadmissionRecoveryResource::ProviderPending {
                state,
                resource_attempt,
                bridge,
                provider,
            } => {
                let _ = (
                    state.artifacts.run_identity(),
                    resource_attempt.attempt_identity(),
                    bridge.fresh_request_identity(),
                    provider,
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
            WorthQueryWorkflowReadmissionRecoveryResource::BridgeCleanup {
                state,
                resource_attempt,
                bridge,
                execution,
            } => Ok(retry_workflow_bridge_cleanup(
                state,
                resource_attempt,
                execution,
                bridge.retry_cleanup(),
                counters,
            )),
            WorthQueryWorkflowReadmissionRecoveryResource::Provider {
                mut state,
                resource_attempt,
                bridge,
                provider,
            } => {
                let retryable = match provider.into_retryable() {
                    Ok(retryable) => retryable,
                    Err(_) => {
                        unreachable!("retained checkpoint posture was checked before recovery")
                    }
                };
                if let Some(release) = &retryable.restored_execution_release {
                    state
                        .provider_work
                        .record_provider_execution_release(release);
                }
                Ok(retry_workflow_bridge_cleanup(
                    state,
                    resource_attempt.abort(),
                    retryable.retained,
                    bridge.abort(),
                    counters,
                ))
            }
            WorthQueryWorkflowReadmissionRecoveryResource::ProviderPending { .. } => {
                unreachable!("active replacement execution was rejected before recovery")
            }
        }
    }
}

fn retry_workflow_bridge_cleanup(
    state: WorthQueryWorkflowYieldedState,
    resource_attempt: WorthQueryWorkflowExecutionResourceAttempt,
    execution: super::super::retained_graph_execution::WorthQueryRetainedManagedGraphExecution,
    bridge: BridgeExecutionBasisReadmissionCleanupOutcome,
    counters: WorthQueryReadmissionCounters,
) -> WorthQueryWorkflowReadmissionRecoveryRetryOutcome {
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
                    state,
                    resource_attempt,
                    execution,
                    bridge,
                ),
            )
        }
    }
}
