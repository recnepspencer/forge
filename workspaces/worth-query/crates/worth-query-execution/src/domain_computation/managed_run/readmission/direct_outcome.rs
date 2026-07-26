use std::sync::Arc;

use worth_runtime_bridge::facade::BridgeExecutionBasisReadmissionCleanupOutcome;

use super::super::{WorthQueryActiveDirectGraphExecution, WorthQueryYieldedDirectRun};
use super::counters::WorthQueryReadmissionCounters;
use super::direct_state::{
    WorthQueryDirectBridgeCleanupRecoveryState, WorthQueryDirectProviderRecoveryState,
    WorthQueryDirectYieldedParts, WorthQueryDirectYieldedReassembly,
};
use crate::domain_computation::provider_session::graph_provider::bounded_step::WorthQueryProviderExecutionReleaseEvidence;
use crate::domain_computation::{
    WorthQueryProviderCheckpointEvidence, WorthQueryProviderCheckpointReleaseEvidence,
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
    ProviderRestoreRejectedAfterExecutionAdmission,
    RestoredExecutionReleaseRecoveryRequired,
    CheckpointReleasePanicked,
}

#[must_use = "direct readmission outcomes retain running, yielded, or recovery authority"]
pub enum WorthQueryDirectReadmissionOutcome {
    Readmitted(WorthQueryActiveDirectGraphExecution),
    Denied(WorthQueryDirectReadmissionDenied),
    RecoveryRequired(WorthQueryDirectReadmissionRecoveryRequired),
}

#[must_use = "direct readmission denial retains the yielded run capability"]
pub struct WorthQueryDirectReadmissionDenied {
    kind: WorthQueryDirectReadmissionDenialKind,
    detail: Arc<str>,
    yielded: WorthQueryYieldedDirectRun,
    counters: WorthQueryReadmissionCounters,
}

#[must_use = "direct readmission recovery must be explicitly resolved"]
pub struct WorthQueryDirectReadmissionRecoveryRequired {
    kind: WorthQueryDirectReadmissionRecoveryKind,
    detail: Arc<str>,
    counters: WorthQueryReadmissionCounters,
    resource: WorthQueryDirectReadmissionRecoveryResource,
}

#[must_use = "direct readmission recovery retry retains yielded or recovery authority"]
pub enum WorthQueryDirectReadmissionRecoveryRetryOutcome {
    Yielded(WorthQueryYieldedDirectRun),
    RecoveryRequired(WorthQueryDirectReadmissionRecoveryRequired),
}

enum WorthQueryDirectReadmissionRecoveryResource {
    BridgeCleanup(WorthQueryDirectBridgeCleanupRecoveryState),
    Provider(WorthQueryDirectProviderRecoveryState),
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
        recovery: WorthQueryDirectBridgeCleanupRecoveryState,
    ) -> Self {
        Self {
            kind: WorthQueryDirectReadmissionRecoveryKind::BridgeCleanupFailed,
            detail: detail.into(),
            counters,
            resource: WorthQueryDirectReadmissionRecoveryResource::BridgeCleanup(recovery),
        }
    }

    pub(super) fn provider(
        kind: WorthQueryDirectReadmissionRecoveryKind,
        detail: impl Into<Arc<str>>,
        counters: WorthQueryReadmissionCounters,
        recovery: WorthQueryDirectProviderRecoveryState,
    ) -> Self {
        Self {
            kind,
            detail: detail.into(),
            counters,
            resource: WorthQueryDirectReadmissionRecoveryResource::Provider(recovery),
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
            WorthQueryDirectReadmissionRecoveryResource::BridgeCleanup(recovery) => {
                recovery.execution.checkpoint_evidence()
            }
            WorthQueryDirectReadmissionRecoveryResource::Provider(recovery) => {
                recovery.provider.checkpoint_evidence()
            }
        }
    }

    pub fn checkpoint_release(&self) -> Option<&WorthQueryProviderCheckpointReleaseEvidence> {
        match &self.resource {
            WorthQueryDirectReadmissionRecoveryResource::BridgeCleanup(_) => None,
            WorthQueryDirectReadmissionRecoveryResource::Provider(recovery) => {
                recovery.provider.checkpoint_release()
            }
        }
    }

    pub fn checkpoint_authority_retained(&self) -> bool {
        match &self.resource {
            WorthQueryDirectReadmissionRecoveryResource::BridgeCleanup(_) => true,
            WorthQueryDirectReadmissionRecoveryResource::Provider(recovery) => {
                recovery.provider.checkpoint_retained()
            }
        }
    }

    pub fn restored_execution_release_evidence(
        &self,
    ) -> Option<&WorthQueryProviderExecutionReleaseEvidence> {
        match &self.resource {
            WorthQueryDirectReadmissionRecoveryResource::BridgeCleanup(_) => None,
            WorthQueryDirectReadmissionRecoveryResource::Provider(recovery) => {
                recovery.provider.restored_execution_release_evidence()
            }
        }
    }

    pub const fn bridge_cleanup_pending(&self) -> bool {
        true
    }

    pub fn fresh_resource_attempt_pending(&self) -> bool {
        matches!(
            self.resource,
            WorthQueryDirectReadmissionRecoveryResource::Provider(_)
        )
    }

    pub fn retained_authority_count(&self) -> usize {
        match &self.resource {
            WorthQueryDirectReadmissionRecoveryResource::BridgeCleanup(recovery) => {
                let _ = (
                    &recovery.state.logical_run_identity,
                    recovery.resource_attempt.attempt_identity(),
                    recovery.bridge.yielded_receipt(),
                    recovery.execution.checkpoint_evidence(),
                );
                4
            }
            WorthQueryDirectReadmissionRecoveryResource::Provider(recovery) => {
                let _ = (
                    &recovery.state.logical_run_identity,
                    recovery.resource.attempt_identity(),
                    recovery.bridge.fresh_request_identity(),
                    recovery.provider.kind(),
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
            WorthQueryDirectReadmissionRecoveryResource::BridgeCleanup(recovery) => {
                let WorthQueryDirectBridgeCleanupRecoveryState {
                    state,
                    resource_attempt,
                    bridge,
                    execution,
                } = recovery;
                Ok(retry_direct_bridge_cleanup(
                    WorthQueryDirectYieldedReassembly {
                        state,
                        resource_attempt,
                        execution,
                    },
                    bridge.retry_cleanup(),
                    counters,
                ))
            }
            WorthQueryDirectReadmissionRecoveryResource::Provider(mut recovery) => {
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
                Ok(retry_direct_bridge_cleanup(
                    WorthQueryDirectYieldedReassembly {
                        state: recovery.state,
                        resource_attempt: recovery.resource.abort(),
                        execution: retryable.retained,
                    },
                    recovery.bridge.abort(),
                    counters,
                ))
            }
        }
    }
}

fn retry_direct_bridge_cleanup(
    pending: WorthQueryDirectYieldedReassembly,
    bridge: BridgeExecutionBasisReadmissionCleanupOutcome,
    counters: WorthQueryReadmissionCounters,
) -> WorthQueryDirectReadmissionRecoveryRetryOutcome {
    let WorthQueryDirectYieldedReassembly {
        state,
        resource_attempt,
        execution,
    } = pending;
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
                    WorthQueryDirectBridgeCleanupRecoveryState {
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
