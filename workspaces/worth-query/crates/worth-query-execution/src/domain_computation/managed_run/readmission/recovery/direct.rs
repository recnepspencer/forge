use std::sync::Arc;

use worth_runtime_bridge::facade::BridgeExecutionBasisReadmissionCleanupOutcome;

use super::direct_cleanup::WorthQueryDirectReadmissionCleanupRequired;
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
pub enum WorthQueryDirectReadmissionRecoveryRequired {
    YieldReassembly(WorthQueryDirectReadmissionYieldReassemblyRecovery),
    TerminalCleanup(WorthQueryDirectReadmissionTerminalRecovery),
}

#[must_use = "yield reassembly recovery must retry Bridge cleanup or become terminal cleanup"]
pub struct WorthQueryDirectReadmissionYieldReassemblyRecovery {
    kind: WorthQueryDirectReadmissionRecoveryKind,
    detail: Arc<str>,
    counters: WorthQueryReadmissionCounters,
    recovery: WorthQueryDirectBridgeCleanupRecoveryState,
}

#[must_use = "provider recovery uncertainty can only enter terminal cleanup"]
pub struct WorthQueryDirectReadmissionTerminalRecovery {
    kind: WorthQueryDirectReadmissionRecoveryKind,
    detail: Arc<str>,
    counters: WorthQueryReadmissionCounters,
    recovery: WorthQueryDirectProviderRecoveryState,
}

#[must_use = "direct readmission recovery retains yielded, cleanup, or recovery authority"]
pub enum WorthQueryDirectReadmissionRecoveryRetryOutcome {
    Yielded(WorthQueryYieldedDirectRun),
    RecoveryRequired(WorthQueryDirectReadmissionRecoveryRequired),
    CleanupRequired(WorthQueryDirectReadmissionCleanupRequired),
}

impl WorthQueryDirectReadmissionRecoveryRequired {
    pub(in crate::domain_computation::managed_run::readmission) fn bridge_cleanup(
        detail: impl Into<Arc<str>>,
        counters: WorthQueryReadmissionCounters,
        recovery: WorthQueryDirectBridgeCleanupRecoveryState,
    ) -> Self {
        Self::YieldReassembly(WorthQueryDirectReadmissionYieldReassemblyRecovery {
            kind: WorthQueryDirectReadmissionRecoveryKind::BridgeCleanupFailed,
            detail: detail.into(),
            counters,
            recovery,
        })
    }

    pub(in crate::domain_computation::managed_run::readmission) fn provider(
        kind: WorthQueryDirectReadmissionRecoveryKind,
        detail: impl Into<Arc<str>>,
        counters: WorthQueryReadmissionCounters,
        recovery: WorthQueryDirectProviderRecoveryState,
    ) -> Self {
        Self::TerminalCleanup(WorthQueryDirectReadmissionTerminalRecovery {
            kind,
            detail: detail.into(),
            counters,
            recovery,
        })
    }

    pub const fn kind(&self) -> WorthQueryDirectReadmissionRecoveryKind {
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

    pub const fn posture(&self) -> WorthQueryDirectReadmissionRecoveryPosture {
        match self {
            Self::YieldReassembly(_) => {
                WorthQueryDirectReadmissionRecoveryPosture::YieldReassemblyPending
            }
            Self::TerminalCleanup(_) => {
                WorthQueryDirectReadmissionRecoveryPosture::TerminalCleanupRequired
            }
        }
    }

    pub fn checkpoint(&self) -> &WorthQueryProviderCheckpointEvidence {
        match self {
            Self::YieldReassembly(recovery) => recovery.execution.checkpoint_evidence(),
            Self::TerminalCleanup(recovery) => recovery.provider.checkpoint_evidence(),
        }
    }

    pub fn checkpoint_release(&self) -> Option<&WorthQueryProviderCheckpointReleaseEvidence> {
        match self {
            Self::YieldReassembly(_) => None,
            Self::TerminalCleanup(recovery) => recovery.provider.checkpoint_release(),
        }
    }

    pub fn restored_execution_release_evidence(
        &self,
    ) -> Option<&WorthQueryProviderExecutionReleaseEvidence> {
        match self {
            Self::YieldReassembly(_) => None,
            Self::TerminalCleanup(recovery) => {
                recovery.provider.restored_execution_release_evidence()
            }
        }
    }
}

impl WorthQueryDirectReadmissionYieldReassemblyRecovery {
    pub const fn kind(&self) -> WorthQueryDirectReadmissionRecoveryKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }

    pub const fn counters(&self) -> WorthQueryReadmissionCounters {
        self.counters
    }

    pub fn retry_to_yielded(self) -> WorthQueryDirectReadmissionRecoveryRetryOutcome {
        let WorthQueryDirectBridgeCleanupRecoveryState {
            state,
            resource_attempt,
            bridge,
            execution,
        } = self.recovery;
        retry_direct_bridge_cleanup(
            WorthQueryDirectYieldedReassembly {
                state,
                resource_attempt,
                execution,
            },
            bridge.retry_cleanup(),
            self.counters,
        )
    }

    pub fn into_cleanup(self) -> WorthQueryDirectReadmissionCleanupRequired {
        WorthQueryDirectReadmissionCleanupRequired::bridge_recovery(self.recovery, self.counters)
    }
}

impl WorthQueryDirectReadmissionTerminalRecovery {
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
        self.recovery.provider.checkpoint_evidence()
    }

    pub fn checkpoint_release(&self) -> Option<&WorthQueryProviderCheckpointReleaseEvidence> {
        self.recovery.provider.checkpoint_release()
    }

    pub fn restored_execution_release_evidence(
        &self,
    ) -> Option<&WorthQueryProviderExecutionReleaseEvidence> {
        self.recovery.provider.restored_execution_release_evidence()
    }

    pub fn into_cleanup(self) -> WorthQueryDirectReadmissionCleanupRequired {
        WorthQueryDirectReadmissionCleanupRequired::provider(
            self.recovery.state,
            self.recovery.resource.abort(),
            self.recovery.bridge,
            self.recovery.provider.into_cleanup(),
            self.counters,
        )
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
