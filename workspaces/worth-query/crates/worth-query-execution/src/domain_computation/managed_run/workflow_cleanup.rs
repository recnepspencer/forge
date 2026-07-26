use std::sync::Arc;

use worth_relational::facade::runtime::RelationalExecutionBasisReleaseReceipt;
use worth_runtime_bridge::facade::{
    BridgeExecutionBasisFinalizationFailureKind, BridgeExecutionBasisFinalizationReceipt,
};

use super::direct_terminal::bridge_terminal_disposition;
use super::{
    WorthQueryManagedProviderWorkEvidence, WorthQueryManagedRunCleanupDisposition,
    WorthQueryManagedRunCounters, WorthQueryManagedRunTerminalKind, WorthQueryWorkflowRunTerminal,
};
use crate::domain_computation::WorthQueryWorkflowArtifactRegistryEvidence;
use crate::domain_computation::WorthQueryWorkflowExecutionAttemptReleaseReceipt;

pub enum WorthQueryWorkflowRunCleanupOutcome {
    Complete(WorthQueryWorkflowRunCleanupReceipt),
    Pending(WorthQueryWorkflowRunCleanupPending),
    RecoveryRequired(WorthQueryWorkflowRunCleanupFailure),
}

impl WorthQueryWorkflowRunCleanupOutcome {
    pub fn disposition(&self) -> WorthQueryManagedRunCleanupDisposition {
        match self {
            Self::Complete(receipt) => receipt.disposition(),
            Self::Pending(_) => WorthQueryManagedRunCleanupDisposition::CleanupPending,
            Self::RecoveryRequired(_) => WorthQueryManagedRunCleanupDisposition::RecoveryRequired,
        }
    }
}

pub struct WorthQueryWorkflowRunCleanupPending {
    artifact_evidence: WorthQueryWorkflowArtifactRegistryEvidence,
    terminal: WorthQueryWorkflowRunTerminal,
}

impl WorthQueryWorkflowRunCleanupPending {
    pub fn pending_artifact_owner_count(&self) -> usize {
        self.artifact_evidence.retained_artifact_count()
    }

    pub fn artifact_evidence(&self) -> WorthQueryWorkflowArtifactRegistryEvidence {
        self.artifact_evidence
    }

    pub fn retry(self) -> WorthQueryWorkflowRunCleanupOutcome {
        cleanup_workflow_terminal(self.terminal)
    }
}

pub struct WorthQueryWorkflowRunCleanupFailure {
    failure_kind: BridgeExecutionBasisFinalizationFailureKind,
    failure_detail: Arc<str>,
    artifact_evidence: WorthQueryWorkflowArtifactRegistryEvidence,
    terminal: WorthQueryWorkflowRunTerminal,
}

impl WorthQueryWorkflowRunCleanupFailure {
    pub fn failure_kind(&self) -> BridgeExecutionBasisFinalizationFailureKind {
        self.failure_kind
    }

    pub fn failure_detail(&self) -> &str {
        &self.failure_detail
    }

    pub fn artifact_evidence(&self) -> WorthQueryWorkflowArtifactRegistryEvidence {
        self.artifact_evidence
    }

    pub fn retry(self) -> WorthQueryWorkflowRunCleanupOutcome {
        cleanup_workflow_terminal(self.terminal)
    }
}

impl std::fmt::Debug for WorthQueryWorkflowRunCleanupFailure {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("WorthQueryWorkflowRunCleanupFailure")
            .field("failure_kind", &self.failure_kind)
            .field("failure_detail", &self.failure_detail)
            .field("run_identity", &self.terminal.identity())
            .finish()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryWorkflowRunCleanupReceipt {
    logical_run_identity: Arc<str>,
    identity: Arc<str>,
    terminal: WorthQueryManagedRunTerminalKind,
    disposition: WorthQueryManagedRunCleanupDisposition,
    bridge: BridgeExecutionBasisFinalizationReceipt,
    relational: RelationalExecutionBasisReleaseReceipt,
    attempt: WorthQueryWorkflowExecutionAttemptReleaseReceipt,
    artifact_evidence: WorthQueryWorkflowArtifactRegistryEvidence,
    counters: WorthQueryManagedRunCounters,
    provider_work: WorthQueryManagedProviderWorkEvidence,
}

impl WorthQueryWorkflowRunCleanupReceipt {
    pub fn run_identity(&self) -> &str {
        &self.identity
    }

    pub fn logical_run_identity(&self) -> &str {
        &self.logical_run_identity
    }

    pub fn terminal(&self) -> WorthQueryManagedRunTerminalKind {
        self.terminal
    }

    pub fn disposition(&self) -> WorthQueryManagedRunCleanupDisposition {
        self.disposition
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

    pub fn artifact_evidence(&self) -> WorthQueryWorkflowArtifactRegistryEvidence {
        self.artifact_evidence
    }

    pub fn provider_work(&self) -> &WorthQueryManagedProviderWorkEvidence {
        &self.provider_work
    }

    pub fn counters(&self) -> &WorthQueryManagedRunCounters {
        &self.counters
    }
}

pub(super) fn cleanup_workflow_terminal(
    terminal: WorthQueryWorkflowRunTerminal,
) -> WorthQueryWorkflowRunCleanupOutcome {
    let registry = terminal.artifacts.registry();
    if terminal.kind == WorthQueryManagedRunTerminalKind::Completed {
        registry.close_released();
    } else {
        registry.close_cancelled();
    }
    let artifact_evidence = registry.evidence();
    if artifact_evidence.retained_artifact_count() != 0
        || artifact_evidence.provider_release_pending_count() != 0
    {
        return WorthQueryWorkflowRunCleanupOutcome::Pending(WorthQueryWorkflowRunCleanupPending {
            artifact_evidence,
            terminal,
        });
    }

    finalize_workflow_terminal(terminal, artifact_evidence)
}

fn finalize_workflow_terminal(
    terminal: WorthQueryWorkflowRunTerminal,
    artifact_evidence: WorthQueryWorkflowArtifactRegistryEvidence,
) -> WorthQueryWorkflowRunCleanupOutcome {
    let WorthQueryWorkflowRunTerminal {
        logical_run_identity,
        identity,
        kind,
        resource_attempt,
        bridge_basis,
        relational_basis,
        artifacts,
        artifact_evidence_at_terminal,
        counters,
        provider_work,
    } = terminal;
    let bridge = match bridge_basis.finalize(bridge_terminal_disposition(kind)) {
        Ok(receipt) => receipt,
        Err(failure) => {
            let failure_kind = failure.kind();
            let failure_detail = Arc::from(failure.detail());
            return WorthQueryWorkflowRunCleanupOutcome::RecoveryRequired(
                WorthQueryWorkflowRunCleanupFailure {
                    failure_kind,
                    failure_detail,
                    artifact_evidence,
                    terminal: WorthQueryWorkflowRunTerminal {
                        logical_run_identity,
                        identity,
                        kind,
                        resource_attempt,
                        bridge_basis: failure.into_basis(),
                        relational_basis,
                        artifacts,
                        artifact_evidence_at_terminal,
                        counters,
                        provider_work,
                    },
                },
            );
        }
    };
    drop(artifacts);
    let relational = relational_basis.release();
    let attempt = resource_attempt.release();
    let disposition = if provider_work.requires_cleanup_recovery()
        || artifact_evidence.provider_release_recovery_required_count() != 0
    {
        WorthQueryManagedRunCleanupDisposition::RecoveryRequired
    } else {
        WorthQueryManagedRunCleanupDisposition::CleanupComplete
    };
    WorthQueryWorkflowRunCleanupOutcome::Complete(WorthQueryWorkflowRunCleanupReceipt {
        logical_run_identity,
        identity,
        terminal: kind,
        disposition,
        bridge,
        relational,
        attempt,
        artifact_evidence,
        counters,
        provider_work,
    })
}
