use std::sync::Arc;

use worth_relational::facade::runtime::RelationalExecutionBasisLease;
use worth_runtime_bridge::facade::{
    BridgeExecutionBasisReadmissionPending, BridgeExecutionBasisReadmissionRecoveryRequired,
    BridgeYieldedExecutionBasis, BridgeYieldedExecutionBasisPreflight,
};

use super::super::managed_graph_execution::WorthQueryManagedGraphExecution;
use super::super::provider_restore::{
    WorthQueryManagedGraphRestorePending, WorthQueryManagedGraphRestoreRecoveryRequired,
};
use super::super::provider_work::WorthQueryManagedProviderWorkLedger;
use super::super::retained_graph_execution::WorthQueryRetainedManagedGraphExecution;
use super::super::step_contract_admission::WorthQueryAdmittedManagedStepContract;
use super::super::{
    WorthQueryManagedRunCounters, WorthQueryYieldTransitionCounters, WorthQueryYieldedWorkflowRun,
};
use crate::domain_computation::artifact_owner::{
    WorthQueryArtifactOccurrenceLedger, WorthQueryArtifactProductionGenerationAbortFailure,
    WorthQueryArtifactProductionGenerationCommitted, WorthQueryArtifactProductionGenerationPending,
    WorthQueryFrozenWorkflowArtifactAuthority, WorthQueryWorkflowArtifactAuthority,
    WorthQueryWorkflowArtifactRegistryEvidence,
};
use crate::domain_computation::provider_session::graph_provider::bounded_step::WorthQueryGraphProviderStepArtifactContext;
use crate::domain_computation::provider_session::readmission::WorthQueryWorkflowResourceReadmissionPending;
use crate::domain_computation::{
    WorthQueryGraphProviderCall, WorthQueryWorkflowExecutionResourceAttempt,
};

pub(super) struct WorthQueryWorkflowYieldedState {
    pub(super) logical_run_identity: Arc<str>,
    pub(super) yielded_attempt_identity: Arc<str>,
    pub(super) relational_basis: RelationalExecutionBasisLease,
    pub(super) artifacts: WorthQueryFrozenWorkflowArtifactAuthority,
    pub(super) artifact_evidence: WorthQueryWorkflowArtifactRegistryEvidence,
    pub(super) run_counters: WorthQueryManagedRunCounters,
    pub(super) provider_work: WorthQueryManagedProviderWorkLedger,
    pub(super) provider_artifact_occurrences: Arc<WorthQueryArtifactOccurrenceLedger>,
    pub(super) yield_counters: WorthQueryYieldTransitionCounters,
}

pub(super) struct WorthQueryWorkflowYieldedParts {
    pub(super) state: WorthQueryWorkflowYieldedState,
    pub(super) resource_attempt: WorthQueryWorkflowExecutionResourceAttempt,
    pub(super) bridge: BridgeYieldedExecutionBasis,
    pub(super) execution: WorthQueryRetainedManagedGraphExecution,
}

pub(super) struct WorthQueryWorkflowReadmissionCommitState {
    pub(super) logical_run_identity: Arc<str>,
    pub(super) relational_basis: RelationalExecutionBasisLease,
    pub(super) artifacts: WorthQueryWorkflowArtifactAuthority,
    pub(super) run_counters: WorthQueryManagedRunCounters,
    pub(super) provider_work: WorthQueryManagedProviderWorkLedger,
    pub(super) provider_artifact_occurrences: Arc<WorthQueryArtifactOccurrenceLedger>,
}

pub(super) struct WorthQueryWorkflowProvisionalResourceAttempt {
    pub(super) state: WorthQueryWorkflowYieldedState,
    pub(super) execution: WorthQueryRetainedManagedGraphExecution,
    pub(super) resource: WorthQueryWorkflowResourceReadmissionPending,
    pub(super) bridge: BridgeYieldedExecutionBasisPreflight,
    pub(super) fresh_call: WorthQueryGraphProviderCall,
    pub(super) contract: WorthQueryAdmittedManagedStepContract,
    pub(super) binding_identity: String,
    pub(super) stage_identity: String,
}

pub(super) struct WorthQueryWorkflowBridgeReadmissionPending {
    pub(super) state: WorthQueryWorkflowYieldedState,
    pub(super) execution: WorthQueryRetainedManagedGraphExecution,
    pub(super) resource: WorthQueryWorkflowResourceReadmissionPending,
    pub(super) bridge: BridgeExecutionBasisReadmissionPending,
    pub(super) fresh_call: WorthQueryGraphProviderCall,
    pub(super) contract: WorthQueryAdmittedManagedStepContract,
    pub(super) stage_identity: String,
}

pub(super) struct WorthQueryWorkflowProviderRestorePending {
    pub(super) state: WorthQueryWorkflowYieldedState,
    pub(super) stage_identity: String,
    pub(super) provider: WorthQueryManagedGraphRestorePending,
    pub(super) resource: WorthQueryWorkflowResourceReadmissionPending,
    pub(super) bridge: BridgeExecutionBasisReadmissionPending,
}

pub(super) struct WorthQueryWorkflowArtifactGenerationPending {
    pub(super) state: WorthQueryWorkflowYieldedState,
    pub(super) provider: WorthQueryManagedGraphRestorePending,
    pub(super) resource: WorthQueryWorkflowResourceReadmissionPending,
    pub(super) bridge: BridgeExecutionBasisReadmissionPending,
    pub(super) generation: WorthQueryArtifactProductionGenerationPending,
    pub(super) artifact_context: Option<WorthQueryGraphProviderStepArtifactContext>,
}

pub(super) struct WorthQueryWorkflowCommitReady {
    pub(super) state: WorthQueryWorkflowReadmissionCommitState,
    pub(super) execution: WorthQueryManagedGraphExecution,
    pub(super) resource: WorthQueryWorkflowResourceReadmissionPending,
    pub(super) bridge: BridgeExecutionBasisReadmissionPending,
}

pub(super) struct WorthQueryWorkflowRollbackPending {
    pub(super) state: WorthQueryWorkflowYieldedState,
    pub(super) execution: WorthQueryRetainedManagedGraphExecution,
    pub(super) resource: WorthQueryWorkflowResourceReadmissionPending,
    pub(super) bridge: BridgeExecutionBasisReadmissionPending,
}

pub(super) struct WorthQueryWorkflowProviderAbortPending {
    pub(super) state: WorthQueryWorkflowYieldedState,
    pub(super) provider: WorthQueryManagedGraphRestorePending,
    pub(super) resource: WorthQueryWorkflowResourceReadmissionPending,
    pub(super) bridge: BridgeExecutionBasisReadmissionPending,
}

pub(super) struct WorthQueryWorkflowBridgeCleanupRecoveryState {
    pub(super) state: WorthQueryWorkflowYieldedState,
    pub(super) resource_attempt: WorthQueryWorkflowExecutionResourceAttempt,
    pub(super) execution: WorthQueryRetainedManagedGraphExecution,
    pub(super) bridge: BridgeExecutionBasisReadmissionRecoveryRequired,
}

pub(super) struct WorthQueryWorkflowProviderRecoveryState {
    pub(super) state: WorthQueryWorkflowYieldedState,
    pub(super) resource: WorthQueryWorkflowResourceReadmissionPending,
    pub(super) bridge: BridgeExecutionBasisReadmissionPending,
    pub(super) provider: WorthQueryManagedGraphRestoreRecoveryRequired,
}

pub(super) struct WorthQueryWorkflowProviderGenerationRecoveryState {
    pub(super) state: WorthQueryWorkflowYieldedState,
    pub(super) resource: WorthQueryWorkflowResourceReadmissionPending,
    pub(super) bridge: BridgeExecutionBasisReadmissionPending,
    pub(super) provider: WorthQueryManagedGraphRestoreRecoveryRequired,
    pub(super) generation_rollback: WorthQueryArtifactProductionGenerationAbortFailure,
}

pub(super) struct WorthQueryWorkflowProviderPendingRecoveryState {
    pub(super) state: WorthQueryWorkflowYieldedState,
    pub(super) resource: WorthQueryWorkflowResourceReadmissionPending,
    pub(super) bridge: BridgeExecutionBasisReadmissionPending,
    pub(super) provider: WorthQueryManagedGraphRestorePending,
    pub(super) generation_rollback: WorthQueryArtifactProductionGenerationAbortFailure,
}

pub(super) struct WorthQueryWorkflowYieldedReassembly {
    pub(super) state: WorthQueryWorkflowYieldedState,
    pub(super) resource_attempt: WorthQueryWorkflowExecutionResourceAttempt,
    pub(super) execution: WorthQueryRetainedManagedGraphExecution,
}

impl WorthQueryWorkflowYieldedState {
    pub(super) fn commit_artifact_generation(
        self,
        committed: WorthQueryArtifactProductionGenerationCommitted,
    ) -> WorthQueryWorkflowReadmissionCommitState {
        WorthQueryWorkflowReadmissionCommitState {
            logical_run_identity: self.logical_run_identity,
            relational_basis: self.relational_basis,
            artifacts: self.artifacts.activate_after_readmission(committed),
            run_counters: self.run_counters,
            provider_work: self.provider_work,
            provider_artifact_occurrences: self.provider_artifact_occurrences,
        }
    }
}

impl WorthQueryWorkflowYieldedParts {
    pub(super) fn from_yielded(yielded: WorthQueryYieldedWorkflowRun) -> Self {
        let WorthQueryYieldedWorkflowRun {
            logical_run_identity,
            attempt_identity,
            resource_attempt,
            relational_basis,
            bridge,
            execution,
            artifacts,
            artifact_evidence,
            run_counters,
            provider_work,
            provider_artifact_occurrences,
            yield_counters,
        } = yielded;
        Self {
            state: WorthQueryWorkflowYieldedState {
                logical_run_identity,
                yielded_attempt_identity: attempt_identity,
                relational_basis,
                artifacts,
                artifact_evidence,
                run_counters,
                provider_work,
                provider_artifact_occurrences,
                yield_counters,
            },
            resource_attempt,
            bridge,
            execution,
        }
    }

    pub(super) fn into_yielded(self) -> WorthQueryYieldedWorkflowRun {
        WorthQueryYieldedWorkflowRun {
            logical_run_identity: self.state.logical_run_identity,
            attempt_identity: self.state.yielded_attempt_identity,
            resource_attempt: self.resource_attempt,
            relational_basis: self.state.relational_basis,
            bridge: self.bridge,
            execution: self.execution,
            artifacts: self.state.artifacts,
            artifact_evidence: self.state.artifact_evidence,
            run_counters: self.state.run_counters,
            provider_work: self.state.provider_work,
            provider_artifact_occurrences: self.state.provider_artifact_occurrences,
            yield_counters: self.state.yield_counters,
        }
    }
}
