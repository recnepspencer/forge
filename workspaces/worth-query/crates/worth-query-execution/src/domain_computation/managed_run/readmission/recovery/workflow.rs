use std::sync::Arc;

use worth_runtime_bridge::facade::BridgeExecutionBasisReadmissionCleanupOutcome;

use super::workflow_cleanup::WorthQueryWorkflowReadmissionCleanupRequired;
use crate::domain_computation::managed_run::provider_restore::WorthQueryManagedGraphRestoreRecoveryRetryOutcome;
use crate::domain_computation::managed_run::readmission::counters::WorthQueryReadmissionCounters;
use crate::domain_computation::managed_run::readmission::workflow_state::{
    WorthQueryWorkflowBridgeCleanupRecoveryState,
    WorthQueryWorkflowProviderGenerationRecoveryState,
    WorthQueryWorkflowProviderPendingRecoveryState, WorthQueryWorkflowProviderRecoveryState,
    WorthQueryWorkflowYieldedParts, WorthQueryWorkflowYieldedReassembly,
};
use crate::domain_computation::managed_run::WorthQueryYieldedWorkflowRun;
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryWorkflowReadmissionRecoveryPosture {
    YieldReassemblyPending,
    TerminalCleanupRequired,
    ArtifactGenerationCleanupRequired,
}

#[must_use = "workflow readmission recovery must be explicitly resolved"]
pub struct WorthQueryWorkflowReadmissionRecoveryRequired {
    kind: WorthQueryWorkflowReadmissionRecoveryKind,
    detail: Arc<str>,
    counters: WorthQueryReadmissionCounters,
    resource: WorthQueryWorkflowReadmissionRecoveryResource,
}

#[must_use = "workflow readmission recovery retains yielded, cleanup, or recovery authority"]
pub enum WorthQueryWorkflowReadmissionRecoveryRetryOutcome {
    Yielded(WorthQueryYieldedWorkflowRun),
    RecoveryRequired(WorthQueryWorkflowReadmissionRecoveryRequired),
    CleanupRequired(WorthQueryWorkflowReadmissionCleanupRequired),
}

enum WorthQueryWorkflowReadmissionRecoveryResource {
    BridgeCleanup(WorthQueryWorkflowBridgeCleanupRecoveryState),
    Provider(WorthQueryWorkflowProviderRecoveryState),
    ProviderGeneration(WorthQueryWorkflowProviderGenerationRecoveryState),
    ProviderPending(WorthQueryWorkflowProviderPendingRecoveryState),
}

impl WorthQueryWorkflowReadmissionRecoveryRequired {
    pub(in crate::domain_computation::managed_run::readmission) fn bridge_cleanup(
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

    pub(in crate::domain_computation::managed_run::readmission) fn provider(
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

    pub(in crate::domain_computation::managed_run::readmission) fn provider_generation(
        detail: impl Into<Arc<str>>,
        counters: WorthQueryReadmissionCounters,
        recovery: WorthQueryWorkflowProviderGenerationRecoveryState,
    ) -> Self {
        Self {
            kind: WorthQueryWorkflowReadmissionRecoveryKind::ArtifactGenerationRollbackFailed,
            detail: detail.into(),
            counters,
            resource: WorthQueryWorkflowReadmissionRecoveryResource::ProviderGeneration(recovery),
        }
    }

    pub(in crate::domain_computation::managed_run::readmission) fn provider_pending(
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

    pub fn posture(&self) -> WorthQueryWorkflowReadmissionRecoveryPosture {
        match &self.resource {
            WorthQueryWorkflowReadmissionRecoveryResource::BridgeCleanup(_) => {
                WorthQueryWorkflowReadmissionRecoveryPosture::YieldReassemblyPending
            }
            WorthQueryWorkflowReadmissionRecoveryResource::Provider(recovery)
                if recovery.provider.checkpoint_release().is_none() =>
            {
                WorthQueryWorkflowReadmissionRecoveryPosture::YieldReassemblyPending
            }
            WorthQueryWorkflowReadmissionRecoveryResource::Provider(_) => {
                WorthQueryWorkflowReadmissionRecoveryPosture::TerminalCleanupRequired
            }
            WorthQueryWorkflowReadmissionRecoveryResource::ProviderGeneration(_)
            | WorthQueryWorkflowReadmissionRecoveryResource::ProviderPending(_) => {
                WorthQueryWorkflowReadmissionRecoveryPosture::ArtifactGenerationCleanupRequired
            }
        }
    }

    pub fn checkpoint(&self) -> &WorthQueryProviderCheckpointEvidence {
        match &self.resource {
            WorthQueryWorkflowReadmissionRecoveryResource::BridgeCleanup(recovery) => {
                recovery.execution.checkpoint_evidence()
            }
            WorthQueryWorkflowReadmissionRecoveryResource::Provider(recovery) => {
                recovery.provider.checkpoint_evidence()
            }
            WorthQueryWorkflowReadmissionRecoveryResource::ProviderGeneration(recovery) => {
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
            WorthQueryWorkflowReadmissionRecoveryResource::ProviderGeneration(recovery) => {
                recovery.provider.checkpoint_release()
            }
            WorthQueryWorkflowReadmissionRecoveryResource::BridgeCleanup(_)
            | WorthQueryWorkflowReadmissionRecoveryResource::ProviderPending(_) => None,
        }
    }

    pub fn restored_execution_release_evidence(
        &self,
    ) -> Option<&WorthQueryProviderExecutionReleaseEvidence> {
        match &self.resource {
            WorthQueryWorkflowReadmissionRecoveryResource::Provider(recovery) => {
                recovery.provider.restored_execution_release_evidence()
            }
            WorthQueryWorkflowReadmissionRecoveryResource::ProviderGeneration(recovery) => {
                recovery.provider.restored_execution_release_evidence()
            }
            WorthQueryWorkflowReadmissionRecoveryResource::BridgeCleanup(_)
            | WorthQueryWorkflowReadmissionRecoveryResource::ProviderPending(_) => None,
        }
    }

    pub fn retry_to_yielded(self) -> WorthQueryWorkflowReadmissionRecoveryRetryOutcome {
        let counters = self.counters;
        match self.resource {
            WorthQueryWorkflowReadmissionRecoveryResource::BridgeCleanup(recovery) => {
                let WorthQueryWorkflowBridgeCleanupRecoveryState {
                    state,
                    resource_attempt,
                    bridge,
                    execution,
                } = recovery;
                retry_workflow_bridge_cleanup(
                    WorthQueryWorkflowYieldedReassembly {
                        state,
                        resource_attempt,
                        execution,
                    },
                    bridge.retry_cleanup(),
                    counters,
                )
            }
            WorthQueryWorkflowReadmissionRecoveryResource::Provider(mut recovery) => {
                match recovery.provider.retry_or_cleanup() {
                    WorthQueryManagedGraphRestoreRecoveryRetryOutcome::Retryable(retryable) => {
                        if let Some(release) = &retryable.restored_execution_release {
                            recovery
                                .state
                                .provider_work
                                .record_provider_execution_release(release);
                        }
                        retry_workflow_bridge_cleanup(
                            WorthQueryWorkflowYieldedReassembly {
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
                    ) => WorthQueryWorkflowReadmissionRecoveryRetryOutcome::CleanupRequired(
                        WorthQueryWorkflowReadmissionCleanupRequired::provider(
                            recovery.state,
                            recovery.resource.abort(),
                            recovery.bridge,
                            provider,
                            None,
                            counters,
                        ),
                    ),
                }
            }
            WorthQueryWorkflowReadmissionRecoveryResource::ProviderGeneration(recovery) => {
                WorthQueryWorkflowReadmissionRecoveryRetryOutcome::CleanupRequired(
                    WorthQueryWorkflowReadmissionCleanupRequired::provider(
                        recovery.state,
                        recovery.resource.abort(),
                        recovery.bridge,
                        recovery.provider.into_cleanup(),
                        Some(recovery.generation_rollback),
                        counters,
                    ),
                )
            }
            WorthQueryWorkflowReadmissionRecoveryResource::ProviderPending(recovery) => {
                WorthQueryWorkflowReadmissionRecoveryRetryOutcome::CleanupRequired(
                    WorthQueryWorkflowReadmissionCleanupRequired::provider(
                        recovery.state,
                        recovery.resource.abort(),
                        recovery.bridge,
                        recovery.provider.into_cleanup(),
                        Some(recovery.generation_rollback),
                        counters,
                    ),
                )
            }
        }
    }

    pub fn into_cleanup(self) -> WorthQueryWorkflowReadmissionCleanupRequired {
        match self.resource {
            WorthQueryWorkflowReadmissionRecoveryResource::BridgeCleanup(recovery) => {
                WorthQueryWorkflowReadmissionCleanupRequired::bridge_recovery(
                    recovery,
                    self.counters,
                )
            }
            WorthQueryWorkflowReadmissionRecoveryResource::Provider(recovery) => {
                WorthQueryWorkflowReadmissionCleanupRequired::provider(
                    recovery.state,
                    recovery.resource.abort(),
                    recovery.bridge,
                    recovery.provider.into_cleanup(),
                    None,
                    self.counters,
                )
            }
            WorthQueryWorkflowReadmissionRecoveryResource::ProviderGeneration(recovery) => {
                WorthQueryWorkflowReadmissionCleanupRequired::provider(
                    recovery.state,
                    recovery.resource.abort(),
                    recovery.bridge,
                    recovery.provider.into_cleanup(),
                    Some(recovery.generation_rollback),
                    self.counters,
                )
            }
            WorthQueryWorkflowReadmissionRecoveryResource::ProviderPending(recovery) => {
                WorthQueryWorkflowReadmissionCleanupRequired::provider(
                    recovery.state,
                    recovery.resource.abort(),
                    recovery.bridge,
                    recovery.provider.into_cleanup(),
                    Some(recovery.generation_rollback),
                    self.counters,
                )
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
