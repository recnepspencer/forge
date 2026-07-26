use std::sync::Arc;

use worth_relational::facade::runtime::RelationalExecutionBasisLease;
use worth_runtime_bridge::facade::{
    BridgeExecutionBasisFinalizationReceipt, BridgeYieldedExecutionBasis,
};

use super::provider_work::WorthQueryManagedProviderWorkLedger;
use super::retained_graph_execution::WorthQueryRetainedManagedGraphExecution;
use super::{
    WorthQueryManagedProviderWorkEvidence, WorthQueryManagedRunCounters,
    WorthQueryPausedWorkflowGraphExecution, WorthQueryYieldTransitionCounters,
};
use crate::domain_computation::artifact_owner::{
    WorthQueryArtifactOccurrenceLedger, WorthQueryFrozenWorkflowArtifactAuthority,
    WorthQueryWorkflowArtifactRegistryEvidence,
};
use crate::domain_computation::{
    WorthQueryExecutionResourceAttemptEvidence, WorthQueryProviderCheckpointEvidence,
    WorthQueryWorkflowExecutionResourceAttempt,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryWorkflowYieldDenialKind {
    InstallationGenerationStale,
    YieldNotInstalled,
    CheckpointUnavailable,
    RetainedBytesExceeded,
}

pub struct WorthQueryWorkflowYieldDenied {
    pub(super) kind: WorthQueryWorkflowYieldDenialKind,
    pub(super) detail: Arc<str>,
    pub(super) paused: WorthQueryPausedWorkflowGraphExecution,
    pub(super) counters: WorthQueryYieldTransitionCounters,
}

impl WorthQueryWorkflowYieldDenied {
    pub const fn kind(&self) -> WorthQueryWorkflowYieldDenialKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }

    pub const fn counters(&self) -> WorthQueryYieldTransitionCounters {
        self.counters
    }

    pub fn run_counters(&self) -> &WorthQueryManagedRunCounters {
        &self.paused.active.running.counters
    }

    pub fn into_paused(self) -> WorthQueryPausedWorkflowGraphExecution {
        self.paused
    }
}

#[must_use = "yield outcomes must be resolved into yielded, denied, or recovery authority"]
pub enum WorthQueryWorkflowYieldOutcome {
    Yielded(WorthQueryYieldedWorkflowRun),
    Denied(WorthQueryWorkflowYieldDenied),
    RecoveryRequired(super::WorthQueryWorkflowYieldRecoveryRequired),
}

pub struct WorthQueryYieldedWorkflowRun {
    pub(super) logical_run_identity: Arc<str>,
    pub(super) attempt_identity: Arc<str>,
    pub(super) resource_attempt: WorthQueryWorkflowExecutionResourceAttempt,
    pub(super) relational_basis: RelationalExecutionBasisLease,
    pub(super) bridge: BridgeYieldedExecutionBasis,
    pub(super) execution: WorthQueryRetainedManagedGraphExecution,
    pub(super) artifacts: WorthQueryFrozenWorkflowArtifactAuthority,
    pub(super) artifact_evidence: WorthQueryWorkflowArtifactRegistryEvidence,
    pub(super) run_counters: WorthQueryManagedRunCounters,
    pub(super) provider_work: WorthQueryManagedProviderWorkLedger,
    pub(super) provider_artifact_occurrences: Arc<WorthQueryArtifactOccurrenceLedger>,
    pub(super) yield_counters: WorthQueryYieldTransitionCounters,
}

impl WorthQueryYieldedWorkflowRun {
    pub fn logical_run_identity(&self) -> &str {
        &self.logical_run_identity
    }

    pub fn yielded_attempt_identity(&self) -> &str {
        &self.attempt_identity
    }

    pub fn operation_binding_identity(&self) -> &str {
        self.resource_attempt.binding_authority().binding_identity()
    }

    pub fn installed_operation_identity(&self) -> &str {
        self.resource_attempt
            .binding_authority()
            .operation_identity()
    }

    pub fn semantic_basis_identity(&self) -> &str {
        self.resource_attempt.binding_authority().basis_identity()
    }

    pub fn installation_generation(
        &self,
    ) -> worth_query_installation::facade::WorthQueryInstallationGeneration {
        self.resource_attempt
            .binding_authority()
            .installation_generation()
    }

    pub fn resource_attempt_evidence(&self) -> &WorthQueryExecutionResourceAttemptEvidence {
        self.resource_attempt.evidence()
    }

    pub fn resource_attempt_identity(&self) -> &str {
        self.resource_attempt.attempt_identity().as_str()
    }

    pub fn provider_session_identity(&self) -> &str {
        self.resource_attempt.provider_session().identity()
    }

    pub fn relational_basis_identity(
        &self,
    ) -> &worth_relational::facade::runtime::RelationalExecutionBasisIdentity {
        self.relational_basis.identity()
    }

    pub fn artifact_run_identity(&self) -> &str {
        self.artifacts.run_identity()
    }

    pub fn checkpoint(&self) -> &WorthQueryProviderCheckpointEvidence {
        self.execution.checkpoint_evidence()
    }

    pub fn provider_work(&self) -> WorthQueryManagedProviderWorkEvidence {
        self.provider_work.snapshot()
    }

    pub fn run_counters(&self) -> &WorthQueryManagedRunCounters {
        &self.run_counters
    }

    pub const fn yield_counters(&self) -> WorthQueryYieldTransitionCounters {
        self.yield_counters
    }

    pub const fn artifact_evidence(&self) -> WorthQueryWorkflowArtifactRegistryEvidence {
        self.artifact_evidence
    }

    pub fn retained_capacity_reservation_count(&self) -> usize {
        self.resource_attempt.retained_capacity_reservation_count()
    }

    pub fn bridge(&self) -> &BridgeExecutionBasisFinalizationReceipt {
        self.bridge.receipt()
    }

    pub fn bridge_request_identity(&self) -> &str {
        self.bridge.basis_request_identity()
    }

    pub fn cleanup(self) -> super::WorthQueryWorkflowYieldCleanupOutcome {
        super::workflow_yield_cleanup::cleanup_yielded_workflow(self)
    }

    pub fn readmit_same_runtime(
        self,
        query_runtime: &crate::domain_computation::WorthQueryExecutionRuntime,
        bridge_runtime: &worth_runtime_bridge::facade::RuntimeBridge,
    ) -> super::WorthQueryWorkflowReadmissionOutcome {
        super::readmission::readmit_workflow(self, query_runtime, bridge_runtime)
    }
}
