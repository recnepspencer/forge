use std::sync::Arc;

use super::provider_work::WorthQueryManagedProviderWorkLedger;
use super::retained_graph_execution::WorthQueryRetainedManagedGraphExecution;
use super::{
    WorthQueryManagedProviderWorkEvidence, WorthQueryManagedRunCounters,
    WorthQueryPausedDirectGraphExecution, WorthQueryYieldTransitionCounters,
};
use crate::domain_computation::{
    WorthQueryDirectExecutionResourceAttempt, WorthQueryExecutionResourceAttemptEvidence,
    WorthQueryProviderCheckpointEvidence,
};
use worth_relational::facade::runtime::RelationalExecutionBasisLease;
use worth_runtime_bridge::facade::{
    BridgeExecutionBasisFinalizationReceipt, BridgeYieldedExecutionBasis,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryDirectYieldDenialKind {
    InstallationGenerationStale,
    YieldNotInstalled,
    CheckpointUnavailable,
    SignalAttemptNotActive,
    QueueNotDrained,
    PartialEffectPostureMismatch,
}

pub struct WorthQueryDirectYieldDenied {
    pub(super) kind: WorthQueryDirectYieldDenialKind,
    pub(super) detail: Arc<str>,
    pub(super) paused: WorthQueryPausedDirectGraphExecution,
    pub(super) counters: WorthQueryYieldTransitionCounters,
}

impl WorthQueryDirectYieldDenied {
    pub const fn kind(&self) -> WorthQueryDirectYieldDenialKind {
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

    pub fn into_paused(self) -> WorthQueryPausedDirectGraphExecution {
        self.paused
    }
}

pub enum WorthQueryDirectYieldOutcome {
    Yielded(WorthQueryYieldedDirectRun),
    Denied(WorthQueryDirectYieldDenied),
    RecoveryRequired(super::WorthQueryDirectYieldRecoveryRequired),
}

pub struct WorthQueryYieldedDirectRun {
    pub(super) logical_run_identity: Arc<str>,
    pub(super) attempt_identity: Arc<str>,
    pub(super) resource_attempt: WorthQueryDirectExecutionResourceAttempt,
    pub(super) relational_basis: RelationalExecutionBasisLease,
    pub(super) bridge: BridgeYieldedExecutionBasis,
    pub(super) execution: WorthQueryRetainedManagedGraphExecution,
    pub(super) run_counters: WorthQueryManagedRunCounters,
    pub(super) provider_work: WorthQueryManagedProviderWorkLedger,
    pub(super) yield_counters: WorthQueryYieldTransitionCounters,
}

impl WorthQueryYieldedDirectRun {
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

    pub fn retained_capacity_reservation_count(&self) -> usize {
        self.resource_attempt.retained_capacity_reservation_count()
    }

    pub fn bridge(&self) -> &BridgeExecutionBasisFinalizationReceipt {
        self.bridge.receipt()
    }

    pub fn bridge_request_identity(&self) -> &str {
        self.bridge.basis_request_identity()
    }

    pub fn cleanup(self) -> super::WorthQueryDirectYieldCleanupOutcome {
        super::direct_yield_cleanup::cleanup_yielded_direct(self)
    }

    pub fn readmit_same_runtime(
        self,
        query_runtime: &crate::domain_computation::WorthQueryExecutionRuntime,
        bridge_runtime: &worth_runtime_bridge::facade::RuntimeBridge,
    ) -> super::WorthQueryDirectReadmissionOutcome {
        super::readmission::readmit_direct(self, query_runtime, bridge_runtime)
    }

    pub fn export_checkpoint(self) -> super::WorthQueryDirectCheckpointExportOutcome {
        super::checkpoint_export::export_direct_checkpoint(self)
    }
}
