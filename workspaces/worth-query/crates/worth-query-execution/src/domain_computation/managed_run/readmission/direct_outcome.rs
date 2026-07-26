use std::sync::Arc;

use worth_runtime_bridge::facade::{
    BridgeExecutionBasisReadmissionCleanupOutcome, BridgeExecutionBasisReadmissionPending,
    BridgeExecutionBasisReadmissionRecoveryRequired,
};

use super::super::provider_restore::WorthQueryManagedGraphRestoreRecoveryRequired;
use super::super::{WorthQueryActiveDirectGraphExecution, WorthQueryYieldedDirectRun};
use super::counters::WorthQueryReadmissionCounters;
use super::direct_state::{WorthQueryDirectYieldedParts, WorthQueryDirectYieldedState};
use crate::domain_computation::provider_session::graph_provider::bounded_step::WorthQueryProviderExecutionReleaseEvidence;
use crate::domain_computation::provider_session::readmission::WorthQueryDirectResourceReadmissionPending;
use crate::domain_computation::{
    WorthQueryDirectExecutionResourceAttempt, WorthQueryProviderCheckpointEvidence,
    WorthQueryProviderCheckpointReleaseEvidence,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryDirectReadmissionDenialKind {
    ForeignQueryRuntime,
    StaleInstallationGeneration,
    RetainedCapacityMismatch,
    RelationalLeaseNotLive,
    ProviderCheckpointMismatch,
    BridgeReadmissionDenied,
    ProviderCallBindingDenied,
    ProviderStepContractDenied(super::super::WorthQueryManagedStepContractDenialKind),
    ProviderRestoreDenied,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryDirectReadmissionRecoveryKind {
    BridgeCleanupFailed,
    ProviderRestorePanicked,
    RestoredExecutionReleaseRecoveryRequired,
    CheckpointReleasePanicked,
}

pub enum WorthQueryDirectReadmissionOutcome {
    Readmitted(WorthQueryActiveDirectGraphExecution),
    Denied(WorthQueryDirectReadmissionDenied),
    RecoveryRequired(WorthQueryDirectReadmissionRecoveryRequired),
}

pub struct WorthQueryDirectReadmissionDenied {
    kind: WorthQueryDirectReadmissionDenialKind,
    detail: Arc<str>,
    yielded: WorthQueryYieldedDirectRun,
    counters: WorthQueryReadmissionCounters,
}

pub struct WorthQueryDirectReadmissionRecoveryRequired {
    kind: WorthQueryDirectReadmissionRecoveryKind,
    detail: Arc<str>,
    counters: WorthQueryReadmissionCounters,
    resource: WorthQueryDirectReadmissionRecoveryResource,
}

pub enum WorthQueryDirectReadmissionRecoveryRetryOutcome {
    Yielded(WorthQueryYieldedDirectRun),
    RecoveryRequired(WorthQueryDirectReadmissionRecoveryRequired),
}

pub(super) enum WorthQueryDirectReadmissionRecoveryResource {
    BridgeCleanup {
        state: WorthQueryDirectYieldedState,
        resource_attempt: WorthQueryDirectExecutionResourceAttempt,
        bridge: BridgeExecutionBasisReadmissionRecoveryRequired,
        execution: super::super::retained_graph_execution::WorthQueryRetainedManagedGraphExecution,
    },
    Provider {
        state: WorthQueryDirectYieldedState,
        resource_attempt: WorthQueryDirectResourceReadmissionPending,
        bridge: BridgeExecutionBasisReadmissionPending,
        provider: WorthQueryManagedGraphRestoreRecoveryRequired,
    },
}

impl WorthQueryDirectReadmissionDenied {
    pub(super) fn new(
        kind: WorthQueryDirectReadmissionDenialKind,
        detail: impl Into<Arc<str>>,
        yielded: WorthQueryYieldedDirectRun,
        counters: WorthQueryReadmissionCounters,
    ) -> Self {
        Self {
            kind,
            detail: detail.into(),
            yielded,
            counters,
        }
    }

    pub const fn kind(&self) -> WorthQueryDirectReadmissionDenialKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }

    pub const fn counters(&self) -> WorthQueryReadmissionCounters {
        self.counters
    }

    pub fn into_yielded(self) -> WorthQueryYieldedDirectRun {
        self.yielded
    }
}

impl WorthQueryDirectReadmissionRecoveryRequired {
    pub(super) fn bridge_cleanup(
        detail: impl Into<Arc<str>>,
        counters: WorthQueryReadmissionCounters,
        state: WorthQueryDirectYieldedState,
        resource_attempt: WorthQueryDirectExecutionResourceAttempt,
        execution: super::super::retained_graph_execution::WorthQueryRetainedManagedGraphExecution,
        bridge: BridgeExecutionBasisReadmissionRecoveryRequired,
    ) -> Self {
        Self {
            kind: WorthQueryDirectReadmissionRecoveryKind::BridgeCleanupFailed,
            detail: detail.into(),
            counters,
            resource: WorthQueryDirectReadmissionRecoveryResource::BridgeCleanup {
                state,
                resource_attempt,
                bridge,
                execution,
            },
        }
    }

    pub(super) fn provider(
        kind: WorthQueryDirectReadmissionRecoveryKind,
        detail: impl Into<Arc<str>>,
        counters: WorthQueryReadmissionCounters,
        state: WorthQueryDirectYieldedState,
        resource_attempt: WorthQueryDirectResourceReadmissionPending,
        bridge: BridgeExecutionBasisReadmissionPending,
        provider: WorthQueryManagedGraphRestoreRecoveryRequired,
    ) -> Self {
        Self {
            kind,
            detail: detail.into(),
            counters,
            resource: WorthQueryDirectReadmissionRecoveryResource::Provider {
                state,
                resource_attempt,
                bridge,
                provider,
            },
        }
    }

    pub const fn kind(&self) -> WorthQueryDirectReadmissionRecoveryKind {
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
            WorthQueryDirectReadmissionRecoveryResource::BridgeCleanup { execution, .. } => {
                execution.checkpoint_evidence()
            }
            WorthQueryDirectReadmissionRecoveryResource::Provider { provider, .. } => {
                provider.checkpoint_evidence()
            }
        }
    }

    pub fn checkpoint_release(&self) -> Option<&WorthQueryProviderCheckpointReleaseEvidence> {
        match &self.resource {
            WorthQueryDirectReadmissionRecoveryResource::BridgeCleanup { .. } => None,
            WorthQueryDirectReadmissionRecoveryResource::Provider { provider, .. } => {
                provider.checkpoint_release()
            }
        }
    }

    pub fn checkpoint_authority_retained(&self) -> bool {
        match &self.resource {
            WorthQueryDirectReadmissionRecoveryResource::BridgeCleanup { .. } => true,
            WorthQueryDirectReadmissionRecoveryResource::Provider { provider, .. } => {
                provider.checkpoint_retained()
            }
        }
    }

    pub fn restored_execution_release_evidence(
        &self,
    ) -> Option<&WorthQueryProviderExecutionReleaseEvidence> {
        match &self.resource {
            WorthQueryDirectReadmissionRecoveryResource::BridgeCleanup { .. } => None,
            WorthQueryDirectReadmissionRecoveryResource::Provider { provider, .. } => {
                provider.restored_execution_release_evidence()
            }
        }
    }

    pub const fn bridge_cleanup_pending(&self) -> bool {
        true
    }

    pub fn fresh_resource_attempt_pending(&self) -> bool {
        matches!(
            self.resource,
            WorthQueryDirectReadmissionRecoveryResource::Provider { .. }
        )
    }

    pub fn retained_authority_count(&self) -> usize {
        match &self.resource {
            WorthQueryDirectReadmissionRecoveryResource::BridgeCleanup {
                state,
                resource_attempt,
                bridge,
                execution,
            } => {
                let _ = (
                    &state.logical_run_identity,
                    resource_attempt.attempt_identity(),
                    bridge.yielded_receipt(),
                    execution.checkpoint_evidence(),
                );
                4
            }
            WorthQueryDirectReadmissionRecoveryResource::Provider {
                state,
                resource_attempt,
                bridge,
                provider,
            } => {
                let _ = (
                    &state.logical_run_identity,
                    resource_attempt.attempt_identity(),
                    bridge.fresh_request_identity(),
                    provider.kind(),
                );
                4
            }
        }
    }

    pub fn retry_to_yielded(self) -> Result<WorthQueryDirectReadmissionRecoveryRetryOutcome, Self> {
        if !self.checkpoint_authority_retained() {
            return Err(self);
        }
        let counters = self.counters;
        match self.resource {
            WorthQueryDirectReadmissionRecoveryResource::BridgeCleanup {
                state,
                resource_attempt,
                bridge,
                execution,
            } => Ok(retry_direct_bridge_cleanup(
                state,
                resource_attempt,
                execution,
                bridge.retry_cleanup(),
                counters,
            )),
            WorthQueryDirectReadmissionRecoveryResource::Provider {
                state,
                resource_attempt,
                bridge,
                provider,
            } => {
                let execution = match provider.into_retained() {
                    Ok(execution) => execution,
                    Err(_) => {
                        unreachable!("retained checkpoint posture was checked before recovery")
                    }
                };
                Ok(retry_direct_bridge_cleanup(
                    state,
                    resource_attempt.abort(),
                    execution,
                    bridge.abort(),
                    counters,
                ))
            }
        }
    }
}

fn retry_direct_bridge_cleanup(
    state: WorthQueryDirectYieldedState,
    resource_attempt: WorthQueryDirectExecutionResourceAttempt,
    execution: super::super::retained_graph_execution::WorthQueryRetainedManagedGraphExecution,
    bridge: BridgeExecutionBasisReadmissionCleanupOutcome,
    counters: WorthQueryReadmissionCounters,
) -> WorthQueryDirectReadmissionRecoveryRetryOutcome {
    match bridge {
        BridgeExecutionBasisReadmissionCleanupOutcome::Complete(bridge) => {
            WorthQueryDirectReadmissionRecoveryRetryOutcome::Yielded(
                WorthQueryDirectYieldedParts {
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
            WorthQueryDirectReadmissionRecoveryRetryOutcome::RecoveryRequired(
                WorthQueryDirectReadmissionRecoveryRequired::bridge_cleanup(
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
