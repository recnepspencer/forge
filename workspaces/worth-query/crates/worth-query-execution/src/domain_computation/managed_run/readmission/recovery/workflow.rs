use std::sync::Arc;

use worth_runtime_bridge::facade::BridgeExecutionBasisReadmissionCleanupOutcome;

use super::workflow_cleanup::WorthQueryWorkflowReadmissionCleanupRequired;
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
pub enum WorthQueryWorkflowReadmissionRecoveryRequired {
    YieldReassembly(WorthQueryWorkflowReadmissionYieldReassemblyRecovery),
    TerminalCleanup(WorthQueryWorkflowReadmissionTerminalRecovery),
}

#[must_use = "yield reassembly recovery must retry Bridge cleanup or become terminal cleanup"]
pub struct WorthQueryWorkflowReadmissionYieldReassemblyRecovery {
    kind: WorthQueryWorkflowReadmissionRecoveryKind,
    detail: Arc<str>,
    counters: WorthQueryReadmissionCounters,
    recovery: WorthQueryWorkflowBridgeCleanupRecoveryState,
}

#[must_use = "provider or artifact uncertainty can only enter terminal cleanup"]
pub struct WorthQueryWorkflowReadmissionTerminalRecovery {
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
        Self::YieldReassembly(WorthQueryWorkflowReadmissionYieldReassemblyRecovery {
            kind: WorthQueryWorkflowReadmissionRecoveryKind::BridgeCleanupFailed,
            detail: detail.into(),
            counters,
            recovery,
        })
    }

    pub(in crate::domain_computation::managed_run::readmission) fn provider(
        kind: WorthQueryWorkflowReadmissionRecoveryKind,
        detail: impl Into<Arc<str>>,
        counters: WorthQueryReadmissionCounters,
        recovery: WorthQueryWorkflowProviderRecoveryState,
    ) -> Self {
        Self::TerminalCleanup(WorthQueryWorkflowReadmissionTerminalRecovery {
            kind,
            detail: detail.into(),
            counters,
            resource: WorthQueryWorkflowReadmissionRecoveryResource::Provider(recovery),
        })
    }

    pub(in crate::domain_computation::managed_run::readmission) fn provider_generation(
        detail: impl Into<Arc<str>>,
        counters: WorthQueryReadmissionCounters,
        recovery: WorthQueryWorkflowProviderGenerationRecoveryState,
    ) -> Self {
        Self::TerminalCleanup(WorthQueryWorkflowReadmissionTerminalRecovery {
            kind: WorthQueryWorkflowReadmissionRecoveryKind::ArtifactGenerationRollbackFailed,
            detail: detail.into(),
            counters,
            resource: WorthQueryWorkflowReadmissionRecoveryResource::ProviderGeneration(recovery),
        })
    }

    pub(in crate::domain_computation::managed_run::readmission) fn provider_pending(
        detail: impl Into<Arc<str>>,
        counters: WorthQueryReadmissionCounters,
        recovery: WorthQueryWorkflowProviderPendingRecoveryState,
    ) -> Self {
        Self::TerminalCleanup(WorthQueryWorkflowReadmissionTerminalRecovery {
            kind: WorthQueryWorkflowReadmissionRecoveryKind::ArtifactGenerationRollbackFailed,
            detail: detail.into(),
            counters,
            resource: WorthQueryWorkflowReadmissionRecoveryResource::ProviderPending(recovery),
        })
    }

    pub const fn kind(&self) -> WorthQueryWorkflowReadmissionRecoveryKind {
        match self {
            Self::YieldReassembly(recovery) => recovery.kind,
            Self::TerminalCleanup(recovery) => recovery.kind,
        }
    }

    pub fn detail(&self) -> &str {
        match self {
            Self::YieldReassembly(recovery) => &recovery.detail,
            Self::TerminalCleanup(recovery) => &recovery.detail,
        }
    }

    pub const fn counters(&self) -> WorthQueryReadmissionCounters {
        match self {
            Self::YieldReassembly(recovery) => recovery.counters,
            Self::TerminalCleanup(recovery) => recovery.counters,
        }
    }

    pub fn posture(&self) -> WorthQueryWorkflowReadmissionRecoveryPosture {
        match self {
            Self::YieldReassembly(_) => {
                WorthQueryWorkflowReadmissionRecoveryPosture::YieldReassemblyPending
            }
            Self::TerminalCleanup(recovery) => recovery.posture(),
        }
    }

    pub fn checkpoint(&self) -> &WorthQueryProviderCheckpointEvidence {
        match self {
            Self::YieldReassembly(recovery) => recovery.execution.checkpoint_evidence(),
            Self::TerminalCleanup(recovery) => recovery.checkpoint(),
        }
    }

    pub fn checkpoint_release(&self) -> Option<&WorthQueryProviderCheckpointReleaseEvidence> {
        match self {
            Self::YieldReassembly(_) => None,
            Self::TerminalCleanup(recovery) => recovery.checkpoint_release(),
        }
    }

    pub fn restored_execution_release_evidence(
        &self,
    ) -> Option<&WorthQueryProviderExecutionReleaseEvidence> {
        match self {
            Self::YieldReassembly(_) => None,
            Self::TerminalCleanup(recovery) => recovery.restored_execution_release_evidence(),
        }
    }
}

impl WorthQueryWorkflowReadmissionYieldReassemblyRecovery {
    pub const fn kind(&self) -> WorthQueryWorkflowReadmissionRecoveryKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }

    pub const fn counters(&self) -> WorthQueryReadmissionCounters {
        self.counters
    }

    pub fn retry_to_yielded(self) -> WorthQueryWorkflowReadmissionRecoveryRetryOutcome {
        let WorthQueryWorkflowBridgeCleanupRecoveryState {
            state,
            resource_attempt,
            bridge,
            execution,
        } = self.recovery;
        retry_workflow_bridge_cleanup(
            WorthQueryWorkflowYieldedReassembly {
                state,
                resource_attempt,
                execution,
            },
            bridge.retry_cleanup(),
            self.counters,
        )
    }

    pub fn into_cleanup(self) -> WorthQueryWorkflowReadmissionCleanupRequired {
        WorthQueryWorkflowReadmissionCleanupRequired::bridge_recovery(self.recovery, self.counters)
    }
}

impl WorthQueryWorkflowReadmissionTerminalRecovery {
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
            WorthQueryWorkflowReadmissionRecoveryResource::Provider(recovery) => {
                let _ = recovery;
                WorthQueryWorkflowReadmissionRecoveryPosture::TerminalCleanupRequired
            }
            WorthQueryWorkflowReadmissionRecoveryResource::ProviderGeneration(recovery) => {
                let _ = recovery;
                WorthQueryWorkflowReadmissionRecoveryPosture::ArtifactGenerationCleanupRequired
            }
            WorthQueryWorkflowReadmissionRecoveryResource::ProviderPending(recovery) => {
                let _ = recovery;
                WorthQueryWorkflowReadmissionRecoveryPosture::ArtifactGenerationCleanupRequired
            }
        }
    }

    pub fn checkpoint(&self) -> &WorthQueryProviderCheckpointEvidence {
        match &self.resource {
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
            WorthQueryWorkflowReadmissionRecoveryResource::ProviderPending(_) => None,
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
            WorthQueryWorkflowReadmissionRecoveryResource::ProviderPending(_) => None,
        }
    }

    pub fn into_cleanup(self) -> WorthQueryWorkflowReadmissionCleanupRequired {
        match self.resource {
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
