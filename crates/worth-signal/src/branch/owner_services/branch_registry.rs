use std::collections::{BTreeMap, BTreeSet};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, MutexGuard};

use crate::state::SignalBranchId;

use super::branch_execution_cell::{SignalBranchCellAdmissionDenial, SignalBranchExecutionCell};
use super::counters::SignalOwnerServiceCounters;
use super::lifecycle_state::{
    SignalOwnerAdmissionMismatch, SignalOwnerLifecycleIdentity, SignalOwnerLifecycleState,
    SignalOwnerOperationAdmission,
};

#[path = "branch_registry/retirement.rs"]
mod retirement;
pub(crate) use retirement::SignalBranchRetirement;

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
    RetirementInProgress(SignalBranchId),
    ExpiredRetirement(SignalBranchId),
    TargetCellDenied(SignalBranchCellAdmissionDenial),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SignalBranchRegistryPoisonRecovery {
    PreservedCanonicalMembership,
}

#[derive(Debug)]
enum SignalBranchRegistryEntry<S> {
    Live(Arc<SignalBranchExecutionCell<S>>),
    Retiring(Arc<SignalBranchExecutionCell<S>>),
}

#[derive(Debug)]
struct SignalBranchRegistryState<S> {
    cells: BTreeMap<SignalBranchId, SignalBranchRegistryEntry<S>>,
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
    recovered_poison: AtomicBool,
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
            recovered_poison: AtomicBool::new(false),
        }
    }

    pub(crate) fn lookup(
        &self,
        admission: &SignalOwnerOperationAdmission,
        branch_id: SignalBranchId,
    ) -> Result<Arc<SignalBranchExecutionCell<S>>, SignalBranchRegistryDenial> {
        self.validate_admission(admission)?;
        self.counters.record_branch_registry_lookup();
        match self.lock_state().cells.get(&branch_id) {
            Some(SignalBranchRegistryEntry::Live(cell)) => Ok(Arc::clone(cell)),
            Some(SignalBranchRegistryEntry::Retiring(_)) => {
                Err(SignalBranchRegistryDenial::RetirementInProgress(branch_id))
            }
            None => Err(SignalBranchRegistryDenial::UnknownBranch(branch_id)),
        }
    }

    pub(crate) fn reserve<'a>(
        &'a self,
        admission: &'a SignalOwnerOperationAdmission,
        branch_id: SignalBranchId,
    ) -> Result<SignalBranchReservation<'a, S>, SignalBranchRegistryDenial> {
        self.validate_admission(admission)?;
        let mut state = self.lock_state();
        self.validate_available_identity(&state, branch_id)?;
        self.validate_capacity(&state)?;
        let inserted = state.reservations.insert(branch_id);
        debug_assert!(inserted, "accepted reservation identity must be vacant");
        self.counters.record_branch_registry_reservation();
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

    pub(crate) fn live_cells_in_identity_order(
        &self,
        admission: &SignalOwnerOperationAdmission,
    ) -> Result<Vec<Arc<SignalBranchExecutionCell<S>>>, SignalBranchRegistryDenial> {
        self.validate_admission(admission)?;
        let state = self.lock_state();
        let mut cells = Vec::with_capacity(state.cells.len());
        for entry in state.cells.values() {
            self.counters.record_branch_registry_entry_scanned();
            if let SignalBranchRegistryEntry::Live(cell) = entry {
                cells.push(Arc::clone(cell));
            }
        }
        Ok(cells)
    }

    pub(crate) fn begin_retirement<'a>(
        &'a self,
        admission: &'a SignalOwnerOperationAdmission,
        branch_id: SignalBranchId,
    ) -> Result<SignalBranchRetirement<'a, S>, SignalBranchRegistryDenial> {
        self.validate_admission(admission)?;
        let cell = {
            let mut state = self.lock_state();
            let entry = state
                .cells
                .get_mut(&branch_id)
                .ok_or(SignalBranchRegistryDenial::UnknownBranch(branch_id))?;
            let cell = match entry {
                SignalBranchRegistryEntry::Live(cell) => Arc::clone(cell),
                SignalBranchRegistryEntry::Retiring(_) => {
                    return Err(SignalBranchRegistryDenial::RetirementInProgress(branch_id));
                }
            };
            *entry = SignalBranchRegistryEntry::Retiring(Arc::clone(&cell));
            cell
        };
        let mut retirement = SignalBranchRetirement {
            registry: self,
            admission,
            branch_id,
            cell,
            cell_marked_retiring: false,
            cell_retired: false,
            completed: false,
        };
        retirement
            .cell
            .begin_retirement()
            .map_err(SignalBranchRegistryDenial::TargetCellDenied)?;
        retirement.cell_marked_retiring = true;
        Ok(retirement)
    }

    pub(crate) fn poison_recovery(&self) -> Option<SignalBranchRegistryPoisonRecovery> {
        self.recovered_poison
            .load(Ordering::Acquire)
            .then_some(SignalBranchRegistryPoisonRecovery::PreservedCanonicalMembership)
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
        if let Some(entry) = state.cells.get(&branch_id) {
            return match entry {
                SignalBranchRegistryEntry::Live(_) => {
                    Err(SignalBranchRegistryDenial::DuplicateBranch(branch_id))
                }
                SignalBranchRegistryEntry::Retiring(_) => {
                    Err(SignalBranchRegistryDenial::RetirementInProgress(branch_id))
                }
            };
        }
        if state.reservations.contains(&branch_id) {
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
        match self.state.lock() {
            Ok(state) => state,
            Err(poisoned) => {
                self.recovered_poison.store(true, Ordering::Release);
                poisoned.into_inner()
            }
        }
    }

    #[cfg(test)]
    pub(super) fn poison_state_for_test(&self) {
        let _state = self.lock_state();
        panic!("inject branch-registry poison");
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
        registry_state.cells.insert(
            self.branch_id,
            SignalBranchRegistryEntry::Live(Arc::clone(&cell)),
        );
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
