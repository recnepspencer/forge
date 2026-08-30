use std::sync::{Arc, Mutex, MutexGuard, TryLockError};

use crate::state::SignalBranchId;

use super::branch_registry::SignalBranchCellConstruction;
use super::counters::SignalOwnerServiceCounters;
use super::lifecycle_state::{
    SignalOwnerAdmissionMismatch, SignalOwnerLifecycleIdentity, SignalOwnerOperationAdmission,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SignalBranchCellAdmissionDenial {
    ForeignOwner,
    ExpiredLifecycle,
}

#[derive(Debug)]
pub(crate) struct SignalBranchExecutionCell<S> {
    state: Mutex<S>,
    owner_runtime_instance_id: u64,
    owner_lifecycle_identity: SignalOwnerLifecycleIdentity,
    branch_id: SignalBranchId,
    counters: Arc<SignalOwnerServiceCounters>,
}

impl<S> SignalBranchExecutionCell<S> {
    pub(super) fn new(
        _construction: SignalBranchCellConstruction,
        state: S,
        owner_runtime_instance_id: u64,
        owner_lifecycle_identity: SignalOwnerLifecycleIdentity,
        branch_id: SignalBranchId,
        counters: Arc<SignalOwnerServiceCounters>,
    ) -> Self {
        Self {
            state: Mutex::new(state),
            owner_runtime_instance_id,
            owner_lifecycle_identity,
            branch_id,
            counters,
        }
    }

    pub(crate) fn branch_id(&self) -> SignalBranchId {
        self.branch_id
    }

    pub(crate) fn with_state<R>(
        &self,
        admission: &SignalOwnerOperationAdmission,
        operation: impl FnOnce(&mut S, &SignalBranchCellWork<'_>) -> R,
    ) -> Result<R, SignalBranchCellAdmissionDenial> {
        self.validate_admission(admission)?;
        self.counters.record_target_cell_contact();
        let mut state = self.lock_state_after_contention_observation();
        let work = SignalBranchCellWork {
            counters: &self.counters,
        };
        Ok(operation(&mut state, &work))
    }

    fn validate_admission(
        &self,
        admission: &SignalOwnerOperationAdmission,
    ) -> Result<(), SignalBranchCellAdmissionDenial> {
        admission
            .authorize(
                self.owner_runtime_instance_id,
                self.owner_lifecycle_identity,
            )
            .map_err(SignalBranchCellAdmissionDenial::from)
    }

    fn lock_state_after_contention_observation(&self) -> MutexGuard<'_, S> {
        match self.state.try_lock() {
            Ok(state) => state,
            Err(TryLockError::WouldBlock) => {
                self.counters.record_target_cell_wait();
                self.state
                    .lock()
                    .unwrap_or_else(std::sync::PoisonError::into_inner)
            }
            Err(TryLockError::Poisoned(poisoned)) => poisoned.into_inner(),
        }
    }
}

impl From<SignalOwnerAdmissionMismatch> for SignalBranchCellAdmissionDenial {
    fn from(mismatch: SignalOwnerAdmissionMismatch) -> Self {
        match mismatch {
            SignalOwnerAdmissionMismatch::ForeignOwner => Self::ForeignOwner,
            SignalOwnerAdmissionMismatch::ExpiredLifecycle => Self::ExpiredLifecycle,
        }
    }
}

pub(crate) struct SignalBranchCellWork<'a> {
    counters: &'a SignalOwnerServiceCounters,
}

impl SignalBranchCellWork<'_> {
    pub(crate) fn record_canonical_movement(&self) {
        self.counters.record_canonical_movement();
    }

    pub(crate) fn record_retention_registry_contact(&self) {
        self.counters.record_retention_registry_contact();
    }

    pub(crate) fn record_fork_source_capture(&self) {
        self.counters.record_fork_source_capture();
    }

    pub(crate) fn record_forked_mutable_graph_node_copy(&self) {
        self.counters.record_forked_mutable_graph_node_copy();
    }

    pub(crate) fn record_diagnostic_event(&self) {
        self.counters.record_diagnostic_event();
    }

    pub(crate) fn record_dropped_diagnostic_event(&self) {
        self.counters.record_dropped_diagnostic_event();
    }
}
