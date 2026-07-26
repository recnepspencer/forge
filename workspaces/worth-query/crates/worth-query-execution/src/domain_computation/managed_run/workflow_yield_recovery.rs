use std::sync::Arc;

use worth_relational::facade::runtime::{
    RelationalExecutionBasisLease, RelationalExecutionBasisReleaseReceipt,
};
use worth_runtime_bridge::facade::{
    BridgeExecutionBasisFinalizationReceipt, BridgeYieldedExecutionBasis,
};

use super::{
    WorthQueryManagedProviderWorkEvidence, WorthQueryManagedRunCounters,
    WorthQueryPausedWorkflowGraphExecution, WorthQueryWorkflowYieldOutcome,
    WorthQueryYieldRecoveryKind, WorthQueryYieldRecoveryResourceEvidence,
    WorthQueryYieldTransitionCounters,
};
use crate::domain_computation::artifact_owner::{
    WorthQueryFrozenWorkflowArtifactAuthority, WorthQueryWorkflowArtifactRegistryEvidence,
};
use crate::domain_computation::{
    WorthQueryWorkflowExecutionAttemptReleaseReceipt, WorthQueryWorkflowExecutionResourceAttempt,
};

pub(super) enum WorthQueryWorkflowYieldRecoveryState {
    Running(WorthQueryPausedWorkflowGraphExecution),
    Terminalized(WorthQueryTerminalizedWorkflowYieldRecovery),
}

pub(super) struct WorthQueryTerminalizedWorkflowYieldRecovery {
    pub(super) logical_run_identity: Arc<str>,
    pub(super) attempt_identity: Arc<str>,
    pub(super) resource_attempt: WorthQueryWorkflowExecutionResourceAttempt,
    pub(super) relational_basis: RelationalExecutionBasisLease,
    pub(super) artifacts: WorthQueryFrozenWorkflowArtifactAuthority,
    pub(super) bridge: BridgeYieldedExecutionBasis,
    pub(super) run_counters: WorthQueryManagedRunCounters,
    pub(super) provider_work: WorthQueryManagedProviderWorkEvidence,
}

pub struct WorthQueryWorkflowYieldRecoveryRequired {
    kind: WorthQueryYieldRecoveryKind,
    detail: Arc<str>,
    counters: WorthQueryYieldTransitionCounters,
    resource_evidence: WorthQueryYieldRecoveryResourceEvidence,
    state: WorthQueryWorkflowYieldRecoveryState,
}

impl WorthQueryWorkflowYieldRecoveryRequired {
    pub const fn kind(&self) -> WorthQueryYieldRecoveryKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }

    pub const fn counters(&self) -> WorthQueryYieldTransitionCounters {
        self.counters
    }

    pub fn resource_evidence(&self) -> &WorthQueryYieldRecoveryResourceEvidence {
        &self.resource_evidence
    }

    pub fn run_counters(&self) -> &WorthQueryManagedRunCounters {
        match &self.state {
            WorthQueryWorkflowYieldRecoveryState::Running(paused) => {
                &paused.active.running.counters
            }
            WorthQueryWorkflowYieldRecoveryState::Terminalized(state) => &state.run_counters,
        }
    }

    pub fn running_attempt_recoverable(&self) -> bool {
        matches!(self.state, WorthQueryWorkflowYieldRecoveryState::Running(_))
    }

    pub fn artifact_evidence(&self) -> Option<WorthQueryWorkflowArtifactRegistryEvidence> {
        match &self.state {
            WorthQueryWorkflowYieldRecoveryState::Running(_) => None,
            WorthQueryWorkflowYieldRecoveryState::Terminalized(state) => {
                Some(state.artifacts.registry().evidence())
            }
        }
    }

    pub fn into_paused(self) -> Result<WorthQueryPausedWorkflowGraphExecution, Self> {
        match self.state {
            WorthQueryWorkflowYieldRecoveryState::Running(paused) => Ok(paused),
            state => Err(Self {
                kind: self.kind,
                detail: self.detail,
                counters: self.counters,
                resource_evidence: self.resource_evidence,
                state,
            }),
        }
    }

    pub fn release_terminalized(
        self,
    ) -> Result<WorthQueryWorkflowYieldRecoveryReleaseOutcome, Self> {
        let Self {
            kind,
            detail,
            counters,
            resource_evidence,
            state,
        } = self;
        let WorthQueryWorkflowYieldRecoveryState::Terminalized(state) = state else {
            return Err(Self {
                kind,
                detail,
                counters,
                resource_evidence,
                state,
            });
        };
        let registry = state.artifacts.registry();
        registry.close_cancelled();
        let artifact_evidence = registry.evidence();
        if artifact_evidence.retained_artifact_count() != 0
            || artifact_evidence.provider_release_pending_count() != 0
        {
            return Ok(WorthQueryWorkflowYieldRecoveryReleaseOutcome::Pending(
                WorthQueryWorkflowYieldRecoveryReleasePending {
                    artifact_evidence,
                    recovery: Self {
                        kind,
                        detail,
                        counters,
                        resource_evidence,
                        state: WorthQueryWorkflowYieldRecoveryState::Terminalized(state),
                    },
                },
            ));
        }
        drop(state.artifacts);
        let release = WorthQueryWorkflowYieldRecoveryRelease {
            logical_run_identity: state.logical_run_identity,
            attempt_identity: state.attempt_identity,
            bridge: state.bridge.release(),
            relational: state.relational_basis.release(),
            attempt: state.resource_attempt.release(),
            artifact_evidence,
            run_counters: state.run_counters,
            provider_work: state.provider_work,
            yield_counters: counters,
            recovery_evidence: resource_evidence,
        };
        Ok(classify_terminalized_release(release))
    }
}

fn classify_terminalized_release(
    release: WorthQueryWorkflowYieldRecoveryRelease,
) -> WorthQueryWorkflowYieldRecoveryReleaseOutcome {
    if release
        .artifact_evidence
        .provider_release_recovery_required_count()
        == 0
    {
        WorthQueryWorkflowYieldRecoveryReleaseOutcome::Complete(release)
    } else {
        WorthQueryWorkflowYieldRecoveryReleaseOutcome::RecoveryRequired(release)
    }
}

pub enum WorthQueryWorkflowYieldRecoveryReleaseOutcome {
    Complete(WorthQueryWorkflowYieldRecoveryRelease),
    Pending(WorthQueryWorkflowYieldRecoveryReleasePending),
    RecoveryRequired(WorthQueryWorkflowYieldRecoveryRelease),
}

pub struct WorthQueryWorkflowYieldRecoveryReleasePending {
    artifact_evidence: WorthQueryWorkflowArtifactRegistryEvidence,
    recovery: WorthQueryWorkflowYieldRecoveryRequired,
}

impl WorthQueryWorkflowYieldRecoveryReleasePending {
    pub const fn artifact_evidence(&self) -> WorthQueryWorkflowArtifactRegistryEvidence {
        self.artifact_evidence
    }

    pub const fn pending_artifact_owner_count(&self) -> usize {
        self.artifact_evidence.retained_artifact_count()
    }

    pub fn recovery(&self) -> &WorthQueryWorkflowYieldRecoveryRequired {
        &self.recovery
    }

    pub fn retry(
        self,
    ) -> Result<
        WorthQueryWorkflowYieldRecoveryReleaseOutcome,
        WorthQueryWorkflowYieldRecoveryRequired,
    > {
        self.recovery.release_terminalized()
    }
}

pub struct WorthQueryWorkflowYieldRecoveryRelease {
    logical_run_identity: Arc<str>,
    attempt_identity: Arc<str>,
    bridge: BridgeExecutionBasisFinalizationReceipt,
    relational: RelationalExecutionBasisReleaseReceipt,
    attempt: WorthQueryWorkflowExecutionAttemptReleaseReceipt,
    artifact_evidence: WorthQueryWorkflowArtifactRegistryEvidence,
    run_counters: WorthQueryManagedRunCounters,
    provider_work: WorthQueryManagedProviderWorkEvidence,
    yield_counters: WorthQueryYieldTransitionCounters,
    recovery_evidence: WorthQueryYieldRecoveryResourceEvidence,
}

impl WorthQueryWorkflowYieldRecoveryRelease {
    pub fn logical_run_identity(&self) -> &str {
        &self.logical_run_identity
    }

    pub fn yielded_attempt_identity(&self) -> &str {
        &self.attempt_identity
    }

    pub fn bridge(&self) -> &BridgeExecutionBasisFinalizationReceipt {
        &self.bridge
    }

    pub fn relational(&self) -> &RelationalExecutionBasisReleaseReceipt {
        &self.relational
    }

    pub fn attempt(&self) -> &WorthQueryWorkflowExecutionAttemptReleaseReceipt {
        &self.attempt
    }

    pub const fn artifact_evidence(&self) -> WorthQueryWorkflowArtifactRegistryEvidence {
        self.artifact_evidence
    }

    pub fn provider_work(&self) -> &WorthQueryManagedProviderWorkEvidence {
        &self.provider_work
    }

    pub fn run_counters(&self) -> &WorthQueryManagedRunCounters {
        &self.run_counters
    }

    pub const fn yield_counters(&self) -> WorthQueryYieldTransitionCounters {
        self.yield_counters
    }

    pub fn recovery_evidence(&self) -> &WorthQueryYieldRecoveryResourceEvidence {
        &self.recovery_evidence
    }
}

pub(super) fn running_recovery(
    kind: WorthQueryYieldRecoveryKind,
    detail: Arc<str>,
    counters: WorthQueryYieldTransitionCounters,
    paused: WorthQueryPausedWorkflowGraphExecution,
) -> WorthQueryWorkflowYieldOutcome {
    WorthQueryWorkflowYieldOutcome::RecoveryRequired(WorthQueryWorkflowYieldRecoveryRequired {
        kind,
        detail,
        counters,
        resource_evidence: WorthQueryYieldRecoveryResourceEvidence::default(),
        state: WorthQueryWorkflowYieldRecoveryState::Running(paused),
    })
}

pub(super) fn terminalized_recovery(
    kind: WorthQueryYieldRecoveryKind,
    detail: Arc<str>,
    counters: WorthQueryYieldTransitionCounters,
    state: WorthQueryTerminalizedWorkflowYieldRecovery,
    resource_evidence: WorthQueryYieldRecoveryResourceEvidence,
) -> WorthQueryWorkflowYieldOutcome {
    WorthQueryWorkflowYieldOutcome::RecoveryRequired(WorthQueryWorkflowYieldRecoveryRequired {
        kind,
        detail,
        counters,
        resource_evidence,
        state: WorthQueryWorkflowYieldRecoveryState::Terminalized(state),
    })
}
