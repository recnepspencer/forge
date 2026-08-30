use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, Mutex, MutexGuard};

use crate::state::SignalBranchId;

use super::branch_execution_cell::SignalBranchExecutionCell;
use super::counters::SignalOwnerServiceCounters;
use super::lifecycle_state::{
    SignalOwnerAdmissionMismatch, SignalOwnerLifecycleIdentity, SignalOwnerLifecycleState,
    SignalOwnerOperationAdmission,
};

pub(super) struct SignalBranchCellConstruction(());

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SignalBranchRegistryDenial {
    ForeignOwner,
    ExpiredAdmission,
    DuplicateBranch(SignalBranchId),
    UnknownBranch(SignalBranchId),
    LiveCapacityExhausted { maximum_live_branches: usize },
    ReservationCapacityExhausted { maximum_reservations: usize },
    ExpiredReservation(SignalBranchId),
}

#[derive(Debug)]
struct SignalBranchRegistryState<S> {
    cells: BTreeMap<SignalBranchId, Arc<SignalBranchExecutionCell<S>>>,
    reservations: BTreeSet<SignalBranchId>,
}

#[derive(Debug)]
pub(crate) struct SignalBranchRegistry<S> {
    owner_runtime_instance_id: u64,
    owner_lifecycle_identity: SignalOwnerLifecycleIdentity,
    maximum_live_branches: usize,
    maximum_reservations: usize,
    state: Mutex<SignalBranchRegistryState<S>>,
    counters: Arc<SignalOwnerServiceCounters>,
}

impl<S> SignalBranchRegistry<S> {
    pub(crate) fn new(
        lifecycle: &SignalOwnerLifecycleState,
        maximum_live_branches: usize,
        maximum_reservations: usize,
    ) -> Self {
        Self {
            owner_runtime_instance_id: lifecycle.owner_runtime_instance_id(),
            owner_lifecycle_identity: lifecycle.lifecycle_identity(),
            maximum_live_branches,
            maximum_reservations,
            state: Mutex::new(SignalBranchRegistryState {
                cells: BTreeMap::new(),
                reservations: BTreeSet::new(),
            }),
            counters: lifecycle.counters(),
        }
    }

    pub(crate) fn lookup(
        &self,
        admission: &SignalOwnerOperationAdmission,
        branch_id: SignalBranchId,
    ) -> Result<Arc<SignalBranchExecutionCell<S>>, SignalBranchRegistryDenial> {
        self.validate_admission(admission)?;
        self.counters.record_branch_registry_lookup();
        self.lock_state()
            .cells
            .get(&branch_id)
            .cloned()
            .ok_or(SignalBranchRegistryDenial::UnknownBranch(branch_id))
    }

    pub(crate) fn reserve<'a>(
        &'a self,
        admission: &'a SignalOwnerOperationAdmission,
        branch_id: SignalBranchId,
    ) -> Result<SignalBranchReservation<'a, S>, SignalBranchRegistryDenial> {
        self.validate_admission(admission)?;
        self.counters.record_branch_registry_reservation();
        let mut state = self.lock_state();
        self.validate_available_identity(&state, branch_id)?;
        self.validate_capacity(&state)?;
        state.reservations.insert(branch_id);
        Ok(SignalBranchReservation {
            registry: self,
            admission,
            branch_id,
            consumed: false,
        })
    }

    pub(crate) fn live_count(&self) -> usize {
        self.lock_state().cells.len()
    }

    pub(crate) fn reservation_count(&self) -> usize {
        self.lock_state().reservations.len()
    }

    pub(crate) fn maximum_live_branches(&self) -> usize {
        self.maximum_live_branches
    }

    pub(crate) fn maximum_reservations(&self) -> usize {
        self.maximum_reservations
    }

    fn validate_admission(
        &self,
        admission: &SignalOwnerOperationAdmission,
    ) -> Result<(), SignalBranchRegistryDenial> {
        admission
            .authorize(
                self.owner_runtime_instance_id,
                self.owner_lifecycle_identity,
            )
            .map_err(SignalBranchRegistryDenial::from)
    }

    fn validate_available_identity(
        &self,
        state: &SignalBranchRegistryState<S>,
        branch_id: SignalBranchId,
    ) -> Result<(), SignalBranchRegistryDenial> {
        if state.cells.contains_key(&branch_id) || state.reservations.contains(&branch_id) {
            return Err(SignalBranchRegistryDenial::DuplicateBranch(branch_id));
        }
        Ok(())
    }

    fn validate_capacity(
        &self,
        state: &SignalBranchRegistryState<S>,
    ) -> Result<(), SignalBranchRegistryDenial> {
        if state.cells.len() + state.reservations.len() >= self.maximum_live_branches {
            return Err(SignalBranchRegistryDenial::LiveCapacityExhausted {
                maximum_live_branches: self.maximum_live_branches,
            });
        }
        if state.reservations.len() >= self.maximum_reservations {
            return Err(SignalBranchRegistryDenial::ReservationCapacityExhausted {
                maximum_reservations: self.maximum_reservations,
            });
        }
        Ok(())
    }

    fn lock_state(&self) -> MutexGuard<'_, SignalBranchRegistryState<S>> {
        self.state
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }
}

impl From<SignalOwnerAdmissionMismatch> for SignalBranchRegistryDenial {
    fn from(mismatch: SignalOwnerAdmissionMismatch) -> Self {
        match mismatch {
            SignalOwnerAdmissionMismatch::ForeignOwner => Self::ForeignOwner,
            SignalOwnerAdmissionMismatch::ExpiredLifecycle => Self::ExpiredAdmission,
        }
    }
}

#[derive(Debug)]
pub(crate) struct SignalBranchReservation<'a, S> {
    registry: &'a SignalBranchRegistry<S>,
    admission: &'a SignalOwnerOperationAdmission,
    branch_id: SignalBranchId,
    consumed: bool,
}

impl<S> SignalBranchReservation<'_, S> {
    pub(crate) fn install(
        self,
        state: S,
    ) -> Result<Arc<SignalBranchExecutionCell<S>>, SignalBranchRegistryDenial> {
        self.install_cell(state, false)
    }

    pub(crate) fn install_fork_destination(
        self,
        state: S,
    ) -> Result<Arc<SignalBranchExecutionCell<S>>, SignalBranchRegistryDenial> {
        self.install_cell(state, true)
    }

    fn install_cell(
        mut self,
        state: S,
        is_fork_destination: bool,
    ) -> Result<Arc<SignalBranchExecutionCell<S>>, SignalBranchRegistryDenial> {
        self.registry.validate_admission(self.admission)?;
        let cell = Arc::new(SignalBranchExecutionCell::new(
            SignalBranchCellConstruction(()),
            state,
            self.registry.owner_runtime_instance_id,
            self.registry.owner_lifecycle_identity,
            self.branch_id,
            Arc::clone(&self.registry.counters),
        ));
        let mut registry_state = self.registry.lock_state();
        if !registry_state.reservations.remove(&self.branch_id) {
            return Err(SignalBranchRegistryDenial::ExpiredReservation(
                self.branch_id,
            ));
        }
        registry_state
            .cells
            .insert(self.branch_id, Arc::clone(&cell));
        self.consumed = true;
        drop(registry_state);
        if is_fork_destination {
            self.registry
                .counters
                .record_fork_destination_installation();
        }
        Ok(cell)
    }
}

impl<S> Drop for SignalBranchReservation<'_, S> {
    fn drop(&mut self) {
        if !self.consumed {
            self.registry
                .lock_state()
                .reservations
                .remove(&self.branch_id);
        }
    }
}
