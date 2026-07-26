use std::sync::Arc;

use worth_runtime_bridge::facade::BridgeExecutionBasisReadmissionCleanupOutcome;

use super::direct_cleanup::WorthQueryDirectReadmissionCleanupRequired;
use crate::domain_computation::managed_run::provider_restore::WorthQueryManagedGraphRestoreRecoveryRetryOutcome;
use crate::domain_computation::managed_run::readmission::counters::WorthQueryReadmissionCounters;
use crate::domain_computation::managed_run::readmission::direct_state::{
    WorthQueryDirectBridgeCleanupRecoveryState, WorthQueryDirectProviderRecoveryState,
    WorthQueryDirectYieldedParts, WorthQueryDirectYieldedReassembly,
};
use crate::domain_computation::managed_run::WorthQueryYieldedDirectRun;
use crate::domain_computation::provider_session::graph_provider::bounded_step::WorthQueryProviderExecutionReleaseEvidence;
use crate::domain_computation::{
    WorthQueryProviderCheckpointEvidence, WorthQueryProviderCheckpointReleaseEvidence,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryDirectReadmissionRecoveryKind {
    BridgeCleanupFailed,
    ProviderRestorePanicked,
    ProviderRestoreRejectedAfterExecutionAdmission,
    RestoredExecutionReleaseRecoveryRequired,
    CheckpointReleasePanicked,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryDirectReadmissionRecoveryPosture {
    YieldReassemblyPending,
    TerminalCleanupRequired,
}

#[must_use = "direct readmission recovery must be explicitly resolved"]
pub struct WorthQueryDirectReadmissionRecoveryRequired {
    kind: WorthQueryDirectReadmissionRecoveryKind,
    detail: Arc<str>,
    counters: WorthQueryReadmissionCounters,
    resource: WorthQueryDirectReadmissionRecoveryResource,
}

#[must_use = "direct readmission recovery retains yielded, cleanup, or recovery authority"]
pub enum WorthQueryDirectReadmissionRecoveryRetryOutcome {
    Yielded(WorthQueryYieldedDirectRun),
    RecoveryRequired(WorthQueryDirectReadmissionRecoveryRequired),
    CleanupRequired(WorthQueryDirectReadmissionCleanupRequired),
}

enum WorthQueryDirectReadmissionRecoveryResource {
    BridgeCleanup(WorthQueryDirectBridgeCleanupRecoveryState),
    Provider(WorthQueryDirectProviderRecoveryState),
}

impl WorthQueryDirectReadmissionRecoveryRequired {
    pub(in crate::domain_computation::managed_run::readmission) fn bridge_cleanup(
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

    pub(in crate::domain_computation::managed_run::readmission) fn provider(
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

    pub fn posture(&self) -> WorthQueryDirectReadmissionRecoveryPosture {
        match &self.resource {
            WorthQueryDirectReadmissionRecoveryResource::BridgeCleanup(_) => {
                WorthQueryDirectReadmissionRecoveryPosture::YieldReassemblyPending
            }
            WorthQueryDirectReadmissionRecoveryResource::Provider(recovery)
                if recovery.provider.checkpoint_release().is_none() =>
            {
                WorthQueryDirectReadmissionRecoveryPosture::YieldReassemblyPending
            }
            WorthQueryDirectReadmissionRecoveryResource::Provider(_) => {
                WorthQueryDirectReadmissionRecoveryPosture::TerminalCleanupRequired
            }
        }
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

    pub fn retry_to_yielded(self) -> WorthQueryDirectReadmissionRecoveryRetryOutcome {
        let counters = self.counters;
        match self.resource {
            WorthQueryDirectReadmissionRecoveryResource::BridgeCleanup(recovery) => {
                let WorthQueryDirectBridgeCleanupRecoveryState {
                    state,
                    resource_attempt,
                    bridge,
                    execution,
                } = recovery;
                retry_direct_bridge_cleanup(
                    WorthQueryDirectYieldedReassembly {
                        state,
                        resource_attempt,
                        execution,
                    },
                    bridge.retry_cleanup(),
                    counters,
                )
            }
            WorthQueryDirectReadmissionRecoveryResource::Provider(mut recovery) => {
                match recovery.provider.retry_or_cleanup() {
                    WorthQueryManagedGraphRestoreRecoveryRetryOutcome::Retryable(retryable) => {
                        if let Some(release) = &retryable.restored_execution_release {
                            recovery
                                .state
                                .provider_work
                                .record_provider_execution_release(release);
                        }
                        retry_direct_bridge_cleanup(
                            WorthQueryDirectYieldedReassembly {
                                state: recovery.state,
                                resource_attempt: recovery.resource.abort(),
                                execution: retryable.retained,
                            },
                            recovery.bridge.abort(),
                            counters,
                        )
                    }
                    WorthQueryManagedGraphRestoreRecoveryRetryOutcome::CleanupRequired(
                        provider,
                    ) => WorthQueryDirectReadmissionRecoveryRetryOutcome::CleanupRequired(
                        WorthQueryDirectReadmissionCleanupRequired::provider(
                            recovery.state,
                            recovery.resource.abort(),
                            recovery.bridge,
                            provider,
                            counters,
                        ),
                    ),
                }
            }
        }
    }

    pub fn into_cleanup(self) -> WorthQueryDirectReadmissionCleanupRequired {
        match self.resource {
            WorthQueryDirectReadmissionRecoveryResource::BridgeCleanup(recovery) => {
                WorthQueryDirectReadmissionCleanupRequired::bridge_recovery(recovery, self.counters)
            }
            WorthQueryDirectReadmissionRecoveryResource::Provider(recovery) => {
                WorthQueryDirectReadmissionCleanupRequired::provider(
                    recovery.state,
                    recovery.resource.abort(),
                    recovery.bridge,
                    recovery.provider.into_cleanup(),
                    self.counters,
                )
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
