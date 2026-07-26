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
    WorthQueryManagedRunCounters, WorthQueryYieldTransitionCounters, WorthQueryYieldedDirectRun,
};
use crate::domain_computation::provider_session::readmission::WorthQueryDirectResourceReadmissionPending;
use crate::domain_computation::{
    WorthQueryDirectExecutionResourceAttempt, WorthQueryGraphProviderCall,
};

pub(super) struct WorthQueryDirectYieldedState {
    pub(super) logical_run_identity: Arc<str>,
    pub(super) yielded_attempt_identity: Arc<str>,
    pub(super) relational_basis: RelationalExecutionBasisLease,
    pub(super) run_counters: WorthQueryManagedRunCounters,
    pub(super) provider_work: WorthQueryManagedProviderWorkLedger,
    pub(super) yield_counters: WorthQueryYieldTransitionCounters,
}

pub(super) struct WorthQueryDirectYieldedParts {
    pub(super) state: WorthQueryDirectYieldedState,
    pub(super) resource_attempt: WorthQueryDirectExecutionResourceAttempt,
    pub(super) bridge: BridgeYieldedExecutionBasis,
    pub(super) execution: WorthQueryRetainedManagedGraphExecution,
}

pub(super) struct WorthQueryDirectProvisionalResourceAttempt {
    pub(super) state: WorthQueryDirectYieldedState,
    pub(super) execution: WorthQueryRetainedManagedGraphExecution,
    pub(super) resource: WorthQueryDirectResourceReadmissionPending,
    pub(super) bridge: BridgeYieldedExecutionBasisPreflight,
    pub(super) fresh_call: WorthQueryGraphProviderCall,
    pub(super) contract: WorthQueryAdmittedManagedStepContract,
    pub(super) binding_identity: String,
}

pub(super) struct WorthQueryDirectBridgeReadmissionPending {
    pub(super) state: WorthQueryDirectYieldedState,
    pub(super) execution: WorthQueryRetainedManagedGraphExecution,
    pub(super) resource: WorthQueryDirectResourceReadmissionPending,
    pub(super) bridge: BridgeExecutionBasisReadmissionPending,
    pub(super) fresh_call: WorthQueryGraphProviderCall,
    pub(super) contract: WorthQueryAdmittedManagedStepContract,
}

pub(super) struct WorthQueryDirectProviderRestorePending {
    pub(super) state: WorthQueryDirectYieldedState,
    pub(super) provider: WorthQueryManagedGraphRestorePending,
    pub(super) resource: WorthQueryDirectResourceReadmissionPending,
    pub(super) bridge: BridgeExecutionBasisReadmissionPending,
}

pub(super) struct WorthQueryDirectCommitReady {
    pub(super) state: WorthQueryDirectYieldedState,
    pub(super) execution: WorthQueryManagedGraphExecution,
    pub(super) resource: WorthQueryDirectResourceReadmissionPending,
    pub(super) bridge: BridgeExecutionBasisReadmissionPending,
}

pub(super) struct WorthQueryDirectRollbackPending {
    pub(super) state: WorthQueryDirectYieldedState,
    pub(super) execution: WorthQueryRetainedManagedGraphExecution,
    pub(super) resource: WorthQueryDirectResourceReadmissionPending,
    pub(super) bridge: BridgeExecutionBasisReadmissionPending,
}

pub(super) struct WorthQueryDirectBridgeCleanupRecoveryState {
    pub(super) state: WorthQueryDirectYieldedState,
    pub(super) resource_attempt: WorthQueryDirectExecutionResourceAttempt,
    pub(super) execution: WorthQueryRetainedManagedGraphExecution,
    pub(super) bridge: BridgeExecutionBasisReadmissionRecoveryRequired,
}

pub(super) struct WorthQueryDirectYieldedReassembly {
    pub(super) state: WorthQueryDirectYieldedState,
    pub(super) resource_attempt: WorthQueryDirectExecutionResourceAttempt,
    pub(super) execution: WorthQueryRetainedManagedGraphExecution,
}

pub(super) struct WorthQueryDirectProviderRecoveryState {
    pub(super) state: WorthQueryDirectYieldedState,
    pub(super) resource: WorthQueryDirectResourceReadmissionPending,
    pub(super) bridge: BridgeExecutionBasisReadmissionPending,
    pub(super) provider: WorthQueryManagedGraphRestoreRecoveryRequired,
}

impl WorthQueryDirectYieldedParts {
    pub(super) fn from_yielded(yielded: WorthQueryYieldedDirectRun) -> Self {
        let WorthQueryYieldedDirectRun {
            logical_run_identity,
            attempt_identity,
            resource_attempt,
            relational_basis,
            bridge,
            execution,
            run_counters,
            provider_work,
            yield_counters,
        } = yielded;
        Self {
            state: WorthQueryDirectYieldedState {
                logical_run_identity,
                yielded_attempt_identity: attempt_identity,
                relational_basis,
                run_counters,
                provider_work,
                yield_counters,
            },
            resource_attempt,
            bridge,
            execution,
        }
    }

    pub(super) fn into_yielded(self) -> WorthQueryYieldedDirectRun {
        WorthQueryYieldedDirectRun {
            logical_run_identity: self.state.logical_run_identity,
            attempt_identity: self.state.yielded_attempt_identity,
            resource_attempt: self.resource_attempt,
            relational_basis: self.state.relational_basis,
            bridge: self.bridge,
            execution: self.execution,
            run_counters: self.state.run_counters,
            provider_work: self.state.provider_work,
            yield_counters: self.state.yield_counters,
        }
    }
}
