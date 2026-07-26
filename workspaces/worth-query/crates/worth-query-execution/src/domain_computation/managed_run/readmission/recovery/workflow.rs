use std::sync::Arc;

use worth_runtime_bridge::facade::BridgeExecutionBasisReadmissionCleanupOutcome;

use super::workflow_cleanup::WorthQueryWorkflowReadmissionCleanupRequired;
use crate::domain_computation::managed_run::readmission::evidence::{
    WorthQueryReadmissionEvidence, WorthQueryReadmissionProgress,
};
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
    progress: WorthQueryReadmissionProgress,
    recovery: WorthQueryWorkflowBridgeCleanupRecoveryState,
}

#[must_use = "provider or artifact uncertainty can only enter terminal cleanup"]
pub struct WorthQueryWorkflowReadmissionTerminalRecovery {
    kind: WorthQueryWorkflowReadmissionRecoveryKind,
    detail: Arc<str>,
    progress: WorthQueryReadmissionProgress,
    resource: WorthQueryWorkflowReadmissionRecoveryResource,
}

#[must_use = "yield reassembly retains yielded or exact Bridge cleanup recovery authority"]
pub enum WorthQueryWorkflowReadmissionYieldReassemblyOutcome {
    Yielded(WorthQueryWorkflowReadmissionYieldReassembled),
    RecoveryRequired(WorthQueryWorkflowReadmissionYieldReassemblyRecovery),
}

pub struct WorthQueryWorkflowReadmissionYieldReassembled {
    yielded: WorthQueryYieldedWorkflowRun,
    evidence: WorthQueryReadmissionEvidence,
}

enum WorthQueryWorkflowReadmissionRecoveryResource {
    Provider(WorthQueryWorkflowProviderRecoveryState),
    ProviderGeneration(WorthQueryWorkflowProviderGenerationRecoveryState),
    ProviderPending(WorthQueryWorkflowProviderPendingRecoveryState),
}

impl WorthQueryWorkflowReadmissionRecoveryRequired {
    pub(in crate::domain_computation::managed_run::readmission) fn bridge_cleanup(
        detail: impl Into<Arc<str>>,
        progress: WorthQueryReadmissionProgress,
        recovery: WorthQueryWorkflowBridgeCleanupRecoveryState,
    ) -> Self {
        Self::YieldReassembly(WorthQueryWorkflowReadmissionYieldReassemblyRecovery {
            kind: WorthQueryWorkflowReadmissionRecoveryKind::BridgeCleanupFailed,
            detail: detail.into(),
            progress,
            recovery,
        })
    }

    pub(in crate::domain_computation::managed_run::readmission) fn provider(
        kind: WorthQueryWorkflowReadmissionRecoveryKind,
        detail: impl Into<Arc<str>>,
        progress: WorthQueryReadmissionProgress,
        recovery: WorthQueryWorkflowProviderRecoveryState,
    ) -> Self {
        Self::TerminalCleanup(WorthQueryWorkflowReadmissionTerminalRecovery {
            kind,
            detail: detail.into(),
            progress,
            resource: WorthQueryWorkflowReadmissionRecoveryResource::Provider(recovery),
        })
    }

    pub(in crate::domain_computation::managed_run::readmission) fn provider_generation(
        detail: impl Into<Arc<str>>,
        progress: WorthQueryReadmissionProgress,
        recovery: WorthQueryWorkflowProviderGenerationRecoveryState,
    ) -> Self {
        Self::TerminalCleanup(WorthQueryWorkflowReadmissionTerminalRecovery {
            kind: WorthQueryWorkflowReadmissionRecoveryKind::ArtifactGenerationRollbackFailed,
            detail: detail.into(),
            progress,
            resource: WorthQueryWorkflowReadmissionRecoveryResource::ProviderGeneration(recovery),
        })
    }

    pub(in crate::domain_computation::managed_run::readmission) fn provider_pending(
        detail: impl Into<Arc<str>>,
        progress: WorthQueryReadmissionProgress,
        recovery: WorthQueryWorkflowProviderPendingRecoveryState,
    ) -> Self {
        Self::TerminalCleanup(WorthQueryWorkflowReadmissionTerminalRecovery {
            kind: WorthQueryWorkflowReadmissionRecoveryKind::ArtifactGenerationRollbackFailed,
            detail: detail.into(),
            progress,
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

    pub const fn readmission_evidence(&self) -> WorthQueryReadmissionEvidence {
        match self {
            Self::YieldReassembly(recovery) => recovery.progress.evidence(),
            Self::TerminalCleanup(recovery) => recovery.progress.evidence(),
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
            Self::YieldReassembly(recovery) => recovery.recovery.execution.checkpoint_evidence(),
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

    pub const fn readmission_evidence(&self) -> WorthQueryReadmissionEvidence {
        self.progress.evidence()
    }

    pub fn retry_to_yielded(self) -> WorthQueryWorkflowReadmissionYieldReassemblyOutcome {
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
            self.progress,
        )
    }

    pub fn into_cleanup(self) -> WorthQueryWorkflowReadmissionCleanupRequired {
        WorthQueryWorkflowReadmissionCleanupRequired::bridge_recovery(self.recovery, self.progress)
    }
}

impl WorthQueryWorkflowReadmissionTerminalRecovery {
    pub const fn kind(&self) -> WorthQueryWorkflowReadmissionRecoveryKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }

    pub const fn readmission_evidence(&self) -> WorthQueryReadmissionEvidence {
        self.progress.evidence()
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
                    self.progress,
                )
            }
            WorthQueryWorkflowReadmissionRecoveryResource::ProviderGeneration(recovery) => {
                WorthQueryWorkflowReadmissionCleanupRequired::provider(
                    recovery.state,
                    recovery.resource.abort(),
                    recovery.bridge,
                    recovery.provider.into_cleanup(),
                    Some(recovery.generation_rollback),
                    self.progress,
                )
            }
            WorthQueryWorkflowReadmissionRecoveryResource::ProviderPending(recovery) => {
                WorthQueryWorkflowReadmissionCleanupRequired::provider(
                    recovery.state,
                    recovery.resource.abort(),
                    recovery.bridge,
                    recovery.provider.into_cleanup(),
                    Some(recovery.generation_rollback),
                    self.progress,
                )
            }
        }
    }
}

impl WorthQueryWorkflowReadmissionYieldReassembled {
    pub const fn readmission_evidence(&self) -> WorthQueryReadmissionEvidence {
        self.evidence
    }

    pub fn into_yielded(self) -> WorthQueryYieldedWorkflowRun {
        self.yielded
    }
}

fn retry_workflow_bridge_cleanup(
    pending: WorthQueryWorkflowYieldedReassembly,
    bridge: BridgeExecutionBasisReadmissionCleanupOutcome,
    mut progress: WorthQueryReadmissionProgress,
) -> WorthQueryWorkflowReadmissionYieldReassemblyOutcome {
    let WorthQueryWorkflowYieldedReassembly {
        state,
        resource_attempt,
        execution,
    } = pending;
    match bridge {
        BridgeExecutionBasisReadmissionCleanupOutcome::Complete(returned) => {
            let (bridge, bridge_counters) = returned.into_parts();
            progress.observe_bridge(bridge_counters);
            WorthQueryWorkflowReadmissionYieldReassemblyOutcome::Yielded(
                WorthQueryWorkflowReadmissionYieldReassembled {
                    yielded: WorthQueryWorkflowYieldedParts {
                        state,
                        resource_attempt,
                        bridge,
                        execution,
                    }
                    .into_yielded(),
                    evidence: progress.evidence(),
                },
            )
        }
        BridgeExecutionBasisReadmissionCleanupOutcome::RecoveryRequired(bridge) => {
            let detail = bridge.detail().to_owned();
            progress.observe_bridge(bridge.counters());
            WorthQueryWorkflowReadmissionYieldReassemblyOutcome::RecoveryRequired(
                WorthQueryWorkflowReadmissionYieldReassemblyRecovery {
                    kind: WorthQueryWorkflowReadmissionRecoveryKind::BridgeCleanupFailed,
                    detail: detail.into(),
                    progress,
                    recovery: WorthQueryWorkflowBridgeCleanupRecoveryState {
                        state,
                        resource_attempt,
                        execution,
                        bridge,
                    },
                },
            )
        }
    }
}
