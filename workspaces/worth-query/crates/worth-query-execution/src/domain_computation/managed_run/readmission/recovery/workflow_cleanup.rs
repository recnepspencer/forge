use std::sync::Arc;

use worth_relational::facade::runtime::RelationalExecutionBasisReleaseReceipt;
use worth_runtime_bridge::facade::{
    BridgeExecutionBasisFinalizationReceipt, BridgeExecutionBasisReadmissionCleanupOutcome,
    BridgeExecutionBasisReadmissionPending, BridgeExecutionBasisReadmissionRecoveryRequired,
};

use crate::domain_computation::artifact_owner::{
    WorthQueryArtifactDenialKind, WorthQueryArtifactProductionGenerationAbortFailure,
    WorthQueryFrozenWorkflowArtifactAuthority, WorthQueryWorkflowArtifactRegistryEvidence,
};
use crate::domain_computation::managed_run::provider_restore::WorthQueryManagedGraphRestoreCleanupRequired;
use crate::domain_computation::managed_run::readmission::counters::WorthQueryReadmissionCounters;
use crate::domain_computation::managed_run::readmission::workflow_state::{
    WorthQueryWorkflowBridgeCleanupRecoveryState, WorthQueryWorkflowYieldedState,
};
use crate::domain_computation::managed_run::{
    WorthQueryManagedProviderWorkEvidence, WorthQueryManagedRunCounters,
    WorthQueryYieldTransitionCounters,
};
use crate::domain_computation::provider_session::graph_provider::bounded_step::WorthQueryProviderExecutionReleaseEvidence;
use crate::domain_computation::{
    WorthQueryProviderCheckpointReleaseEvidence, WorthQueryWorkflowExecutionAttemptReleaseReceipt,
    WorthQueryWorkflowExecutionResourceAttempt,
};

#[must_use = "workflow readmission cleanup retains its receipt or unfinished cleanup authority"]
pub enum WorthQueryWorkflowReadmissionCleanupOutcome {
    Complete(WorthQueryWorkflowReadmissionCleanupReceipt),
    Pending(WorthQueryWorkflowReadmissionCleanupPending),
    RecoveryRequired(WorthQueryWorkflowReadmissionCleanupReceipt),
}

#[must_use = "workflow readmission cleanup must be finished"]
pub struct WorthQueryWorkflowReadmissionCleanupRequired {
    state: WorthQueryWorkflowYieldedState,
    resource_attempt: WorthQueryWorkflowExecutionResourceAttempt,
    bridge: WorthQueryWorkflowReadmissionBridgeCleanup,
    provider: WorthQueryManagedGraphRestoreCleanupRequired,
    generation_rollback: Option<WorthQueryArtifactProductionGenerationAbortFailure>,
    counters: WorthQueryReadmissionCounters,
}

#[must_use = "pending workflow readmission cleanup retains unfinished owner authority"]
pub struct WorthQueryWorkflowReadmissionCleanupPending {
    partial: WorthQueryWorkflowReadmissionPartialCleanupReceipt,
    artifacts: Option<WorthQueryFrozenWorkflowArtifactAuthority>,
    bridge: Option<BridgeExecutionBasisReadmissionRecoveryRequired>,
}

pub struct WorthQueryWorkflowReadmissionCleanupReceipt {
    partial: WorthQueryWorkflowReadmissionPartialCleanupReceipt,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryArtifactGenerationRollbackEvidence {
    prior_generation: u64,
    pending_generation: u64,
    denial_kind: WorthQueryArtifactDenialKind,
    detail: &'static str,
}

struct WorthQueryWorkflowReadmissionPartialCleanupReceipt {
    logical_run_identity: Arc<str>,
    yielded_attempt_identity: Arc<str>,
    checkpoint_release: WorthQueryProviderCheckpointReleaseEvidence,
    restored_execution_release: Option<WorthQueryProviderExecutionReleaseEvidence>,
    bridge: Option<BridgeExecutionBasisFinalizationReceipt>,
    relational: RelationalExecutionBasisReleaseReceipt,
    attempt: WorthQueryWorkflowExecutionAttemptReleaseReceipt,
    artifact_evidence: WorthQueryWorkflowArtifactRegistryEvidence,
    generation_rollback: Option<WorthQueryArtifactGenerationRollbackEvidence>,
    run_counters: WorthQueryManagedRunCounters,
    provider_work: WorthQueryManagedProviderWorkEvidence,
    yield_counters: WorthQueryYieldTransitionCounters,
    readmission_counters: WorthQueryReadmissionCounters,
}

enum WorthQueryWorkflowReadmissionBridgeCleanup {
    Pending(BridgeExecutionBasisReadmissionPending),
    Recovery(BridgeExecutionBasisReadmissionRecoveryRequired),
}

impl WorthQueryWorkflowReadmissionCleanupRequired {
    pub(super) fn provider(
        state: WorthQueryWorkflowYieldedState,
        resource_attempt: WorthQueryWorkflowExecutionResourceAttempt,
        bridge: BridgeExecutionBasisReadmissionPending,
        provider: WorthQueryManagedGraphRestoreCleanupRequired,
        generation_rollback: Option<WorthQueryArtifactProductionGenerationAbortFailure>,
        counters: WorthQueryReadmissionCounters,
    ) -> Self {
        Self {
            state,
            resource_attempt,
            bridge: WorthQueryWorkflowReadmissionBridgeCleanup::Pending(bridge),
            provider,
            generation_rollback,
            counters,
        }
    }

    pub(super) fn bridge_recovery(
        recovery: WorthQueryWorkflowBridgeCleanupRecoveryState,
        counters: WorthQueryReadmissionCounters,
    ) -> Self {
        Self {
            state: recovery.state,
            resource_attempt: recovery.resource_attempt,
            bridge: WorthQueryWorkflowReadmissionBridgeCleanup::Recovery(recovery.bridge),
            provider: WorthQueryManagedGraphRestoreCleanupRequired::retained(
                recovery.execution,
                None,
            ),
            generation_rollback: None,
            counters,
        }
    }

    pub fn finish(mut self) -> WorthQueryWorkflowReadmissionCleanupOutcome {
        let provider = self.provider.finish();
        if let Some(release) = &provider.restored_execution_release {
            self.state
                .provider_work
                .record_provider_execution_release(release);
        }
        let artifacts = self.state.artifacts;
        let registry = artifacts.registry();
        registry.close_cancelled();
        let artifact_evidence = registry.evidence();
        let artifacts = (artifact_evidence.retained_artifact_count() != 0
            || artifact_evidence.provider_release_pending_count() != 0)
            .then_some(artifacts);
        let generation_rollback = self
            .generation_rollback
            .map(WorthQueryArtifactGenerationRollbackEvidence::from);
        let mut partial = WorthQueryWorkflowReadmissionPartialCleanupReceipt {
            logical_run_identity: self.state.logical_run_identity,
            yielded_attempt_identity: self.state.yielded_attempt_identity,
            checkpoint_release: provider.checkpoint_release,
            restored_execution_release: provider.restored_execution_release,
            bridge: None,
            relational: self.state.relational_basis.release(),
            attempt: self.resource_attempt.release(),
            artifact_evidence,
            generation_rollback,
            run_counters: self.state.run_counters,
            provider_work: self.state.provider_work.into_evidence(),
            yield_counters: self.state.yield_counters,
            readmission_counters: self.counters,
        };
        let bridge = match self.bridge {
            WorthQueryWorkflowReadmissionBridgeCleanup::Pending(bridge) => bridge.abort(),
            WorthQueryWorkflowReadmissionBridgeCleanup::Recovery(bridge) => bridge.retry_cleanup(),
        };
        let bridge = apply_bridge_cleanup(&mut partial, bridge);
        finish_or_retain_workflow_cleanup(partial, artifacts, bridge)
    }
}

impl WorthQueryWorkflowReadmissionCleanupPending {
    pub fn retry(mut self) -> WorthQueryWorkflowReadmissionCleanupOutcome {
        if let Some(artifacts) = self.artifacts.take() {
            let registry = artifacts.registry();
            registry.close_cancelled();
            self.partial.artifact_evidence = registry.evidence();
            if self.partial.artifact_evidence.retained_artifact_count() != 0
                || self
                    .partial
                    .artifact_evidence
                    .provider_release_pending_count()
                    != 0
            {
                self.artifacts = Some(artifacts);
            }
        }
        if let Some(bridge) = self.bridge.take() {
            self.bridge = apply_bridge_cleanup(&mut self.partial, bridge.retry_cleanup());
        }
        finish_or_retain_workflow_cleanup(self.partial, self.artifacts, self.bridge)
    }

    pub fn logical_run_identity(&self) -> &str {
        &self.partial.logical_run_identity
    }

    pub const fn artifact_evidence(&self) -> WorthQueryWorkflowArtifactRegistryEvidence {
        self.partial.artifact_evidence
    }

    pub fn bridge_cleanup_pending(&self) -> bool {
        self.bridge.is_some()
    }
}

impl WorthQueryWorkflowReadmissionCleanupReceipt {
    pub fn logical_run_identity(&self) -> &str {
        &self.partial.logical_run_identity
    }

    pub fn yielded_attempt_identity(&self) -> &str {
        &self.partial.yielded_attempt_identity
    }

    pub fn checkpoint_release(&self) -> &WorthQueryProviderCheckpointReleaseEvidence {
        &self.partial.checkpoint_release
    }

    pub fn restored_execution_release(
        &self,
    ) -> Option<&WorthQueryProviderExecutionReleaseEvidence> {
        self.partial.restored_execution_release.as_ref()
    }

    pub fn bridge(&self) -> &BridgeExecutionBasisFinalizationReceipt {
        self.partial
            .bridge
            .as_ref()
            .expect("complete workflow readmission cleanup has Bridge evidence")
    }

    pub fn relational(&self) -> &RelationalExecutionBasisReleaseReceipt {
        &self.partial.relational
    }

    pub fn attempt(&self) -> &WorthQueryWorkflowExecutionAttemptReleaseReceipt {
        &self.partial.attempt
    }

    pub const fn artifact_evidence(&self) -> WorthQueryWorkflowArtifactRegistryEvidence {
        self.partial.artifact_evidence
    }

    pub fn generation_rollback(&self) -> Option<&WorthQueryArtifactGenerationRollbackEvidence> {
        self.partial.generation_rollback.as_ref()
    }

    pub fn provider_work(&self) -> &WorthQueryManagedProviderWorkEvidence {
        &self.partial.provider_work
    }

    pub fn run_counters(&self) -> &WorthQueryManagedRunCounters {
        &self.partial.run_counters
    }

    pub const fn yield_counters(&self) -> WorthQueryYieldTransitionCounters {
        self.partial.yield_counters
    }

    pub const fn readmission_counters(&self) -> WorthQueryReadmissionCounters {
        self.partial.readmission_counters
    }
}

impl WorthQueryArtifactGenerationRollbackEvidence {
    pub const fn prior_generation(&self) -> u64 {
        self.prior_generation
    }

    pub const fn pending_generation(&self) -> u64 {
        self.pending_generation
    }

    pub const fn denial_kind(&self) -> WorthQueryArtifactDenialKind {
        self.denial_kind
    }

    pub const fn detail(&self) -> &'static str {
        self.detail
    }
}

impl From<WorthQueryArtifactProductionGenerationAbortFailure>
    for WorthQueryArtifactGenerationRollbackEvidence
{
    fn from(failure: WorthQueryArtifactProductionGenerationAbortFailure) -> Self {
        Self {
            prior_generation: failure.prior_generation().ordinal(),
            pending_generation: failure.pending_generation().ordinal(),
            denial_kind: failure.denial().kind(),
            detail: failure.detail(),
        }
    }
}

fn apply_bridge_cleanup(
    partial: &mut WorthQueryWorkflowReadmissionPartialCleanupReceipt,
    bridge: BridgeExecutionBasisReadmissionCleanupOutcome,
) -> Option<BridgeExecutionBasisReadmissionRecoveryRequired> {
    match bridge {
        BridgeExecutionBasisReadmissionCleanupOutcome::Complete(bridge) => {
            partial.bridge = Some(bridge.release());
            None
        }
        BridgeExecutionBasisReadmissionCleanupOutcome::RecoveryRequired(bridge) => Some(bridge),
    }
}

fn finish_or_retain_workflow_cleanup(
    partial: WorthQueryWorkflowReadmissionPartialCleanupReceipt,
    artifacts: Option<WorthQueryFrozenWorkflowArtifactAuthority>,
    bridge: Option<BridgeExecutionBasisReadmissionRecoveryRequired>,
) -> WorthQueryWorkflowReadmissionCleanupOutcome {
    if artifacts.is_some() || bridge.is_some() {
        return WorthQueryWorkflowReadmissionCleanupOutcome::Pending(
            WorthQueryWorkflowReadmissionCleanupPending {
                partial,
                artifacts,
                bridge,
            },
        );
    }
    let receipt = WorthQueryWorkflowReadmissionCleanupReceipt { partial };
    if workflow_cleanup_recovery_required(&receipt) {
        WorthQueryWorkflowReadmissionCleanupOutcome::RecoveryRequired(receipt)
    } else {
        WorthQueryWorkflowReadmissionCleanupOutcome::Complete(receipt)
    }
}

fn workflow_cleanup_recovery_required(
    receipt: &WorthQueryWorkflowReadmissionCleanupReceipt,
) -> bool {
    receipt
        .checkpoint_release()
        .disposition()
        .recovery_required()
        || receipt
            .restored_execution_release()
            .is_some_and(WorthQueryProviderExecutionReleaseEvidence::recovery_required)
        || receipt
            .artifact_evidence()
            .provider_release_recovery_required_count()
            != 0
        || receipt.generation_rollback().is_some()
}
