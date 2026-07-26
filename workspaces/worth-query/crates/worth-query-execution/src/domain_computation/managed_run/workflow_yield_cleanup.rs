use std::sync::Arc;

use worth_relational::facade::runtime::{
    RelationalExecutionBasisLease, RelationalExecutionBasisReleaseReceipt,
};
use worth_runtime_bridge::facade::BridgeExecutionBasisFinalizationReceipt;

use super::{
    WorthQueryManagedProviderWorkEvidence, WorthQueryManagedRunCounters,
    WorthQueryYieldTransitionCounters, WorthQueryYieldedWorkflowRun,
};
use crate::domain_computation::artifact_owner::{
    WorthQueryFrozenWorkflowArtifactAuthority, WorthQueryWorkflowArtifactRegistryEvidence,
};
use crate::domain_computation::{
    WorthQueryProviderCheckpointEvidence, WorthQueryProviderCheckpointReleaseEvidence,
    WorthQueryWorkflowExecutionAttemptReleaseReceipt, WorthQueryWorkflowExecutionResourceAttempt,
};

pub enum WorthQueryWorkflowYieldCleanupOutcome {
    Complete(WorthQueryWorkflowYieldCleanupReceipt),
    Pending(WorthQueryWorkflowYieldCleanupPending),
    RecoveryRequired(WorthQueryWorkflowYieldCleanupReceipt),
}

pub struct WorthQueryWorkflowYieldCleanupPending {
    logical_run_identity: Arc<str>,
    attempt_identity: Arc<str>,
    resource_attempt: WorthQueryWorkflowExecutionResourceAttempt,
    relational_basis: RelationalExecutionBasisLease,
    bridge: BridgeExecutionBasisFinalizationReceipt,
    artifacts: WorthQueryFrozenWorkflowArtifactAuthority,
    checkpoint_release: WorthQueryProviderCheckpointReleaseEvidence,
    artifact_evidence: WorthQueryWorkflowArtifactRegistryEvidence,
    run_counters: WorthQueryManagedRunCounters,
    provider_work: WorthQueryManagedProviderWorkEvidence,
    yield_counters: WorthQueryYieldTransitionCounters,
}

impl WorthQueryWorkflowYieldCleanupPending {
    pub const fn artifact_evidence(&self) -> WorthQueryWorkflowArtifactRegistryEvidence {
        self.artifact_evidence
    }

    pub fn checkpoint(&self) -> &WorthQueryProviderCheckpointEvidence {
        self.checkpoint_release.checkpoint()
    }

    pub fn checkpoint_release(&self) -> &WorthQueryProviderCheckpointReleaseEvidence {
        &self.checkpoint_release
    }

    pub fn retry(self) -> WorthQueryWorkflowYieldCleanupOutcome {
        cleanup_without_checkpoint_owner(self)
    }

    pub const fn yield_counters(&self) -> WorthQueryYieldTransitionCounters {
        self.yield_counters
    }

    pub fn run_counters(&self) -> &WorthQueryManagedRunCounters {
        &self.run_counters
    }
}

pub struct WorthQueryWorkflowYieldCleanupReceipt {
    logical_run_identity: Arc<str>,
    attempt_identity: Arc<str>,
    checkpoint_release: WorthQueryProviderCheckpointReleaseEvidence,
    bridge: BridgeExecutionBasisFinalizationReceipt,
    relational: RelationalExecutionBasisReleaseReceipt,
    attempt: WorthQueryWorkflowExecutionAttemptReleaseReceipt,
    artifact_evidence: WorthQueryWorkflowArtifactRegistryEvidence,
    provider_work: WorthQueryManagedProviderWorkEvidence,
    run_counters: WorthQueryManagedRunCounters,
    yield_counters: WorthQueryYieldTransitionCounters,
}

impl WorthQueryWorkflowYieldCleanupReceipt {
    pub fn logical_run_identity(&self) -> &str {
        &self.logical_run_identity
    }

    pub fn yielded_attempt_identity(&self) -> &str {
        &self.attempt_identity
    }

    pub fn checkpoint(&self) -> &WorthQueryProviderCheckpointEvidence {
        self.checkpoint_release.checkpoint()
    }

    pub fn checkpoint_release(&self) -> &WorthQueryProviderCheckpointReleaseEvidence {
        &self.checkpoint_release
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
}

pub(super) fn cleanup_yielded_workflow(
    yielded: WorthQueryYieldedWorkflowRun,
) -> WorthQueryWorkflowYieldCleanupOutcome {
    let checkpoint_release = yielded.execution.release();
    cleanup_without_checkpoint_owner(WorthQueryWorkflowYieldCleanupPending {
        logical_run_identity: yielded.logical_run_identity,
        attempt_identity: yielded.attempt_identity,
        resource_attempt: yielded.resource_attempt,
        relational_basis: yielded.relational_basis,
        bridge: yielded.bridge.release(),
        artifacts: yielded.artifacts,
        checkpoint_release,
        artifact_evidence: yielded.artifact_evidence,
        run_counters: yielded.run_counters,
        provider_work: yielded.provider_work.into_evidence(),
        yield_counters: yielded.yield_counters,
    })
}

fn cleanup_without_checkpoint_owner(
    mut pending: WorthQueryWorkflowYieldCleanupPending,
) -> WorthQueryWorkflowYieldCleanupOutcome {
    let registry = pending.artifacts.registry();
    registry.close_cancelled();
    pending.artifact_evidence = registry.evidence();
    if pending.artifact_evidence.retained_artifact_count() != 0
        || pending.artifact_evidence.provider_release_pending_count() != 0
    {
        return WorthQueryWorkflowYieldCleanupOutcome::Pending(pending);
    }
    drop(pending.artifacts);
    let receipt = WorthQueryWorkflowYieldCleanupReceipt {
        logical_run_identity: pending.logical_run_identity,
        attempt_identity: pending.attempt_identity,
        checkpoint_release: pending.checkpoint_release,
        bridge: pending.bridge,
        relational: pending.relational_basis.release(),
        attempt: pending.resource_attempt.release(),
        artifact_evidence: pending.artifact_evidence,
        run_counters: pending.run_counters,
        provider_work: pending.provider_work,
        yield_counters: pending.yield_counters,
    };
    if receipt.checkpoint_release.disposition().recovery_required()
        || receipt
            .artifact_evidence
            .provider_release_recovery_required_count()
            != 0
    {
        WorthQueryWorkflowYieldCleanupOutcome::RecoveryRequired(receipt)
    } else {
        WorthQueryWorkflowYieldCleanupOutcome::Complete(receipt)
    }
}
