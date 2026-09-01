use std::collections::BTreeMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, MutexGuard};

use crate::state::SignalBranchId;

use super::branch_execution_cell::{SignalBranchCellAdmissionDenial, SignalBranchExecutionCell};
use super::cell_incarnation::SignalBranchCellIncarnation;
use super::counters::SignalOwnerServiceCounters;
use super::lifecycle_state::{
    SignalOwnerAdmissionMismatch, SignalOwnerLifecycleIdentity, SignalOwnerLifecycleState,
    SignalOwnerOperationAdmission,
};
use super::operation_control::SignalOwnerOperationBoundary;

#[path = "branch_registry/retirement.rs"]
mod retirement;
pub(crate) use retirement::SignalBranchRetirement;
#[path = "branch_registry/prepared_installation.rs"]
mod prepared;
pub(crate) use prepared::{
    SignalInstalledBranchCell, SignalPreparedBranchCell, SignalPreparedBranchInstallation,
};
#[path = "branch_registry/capacity.rs"]
mod capacity;
#[path = "branch_registry/close_cleanup.rs"]
mod close_cleanup;

pub(super) struct SignalBranchCellConstruction(());

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SignalBranchRegistryDenial {
    ForeignOwner,
    ExpiredAdmission,
    DuplicateBranch(SignalBranchId),
    UnknownBranch(SignalBranchId),
    LiveCapacityExhausted { maximum_live_branches: usize },
    ReservationCapacityExhausted { maximum_reservations: usize },
    RetirementInProgress(SignalBranchId),
    ExpiredRetirement(SignalBranchId),
    TargetCellDenied(SignalBranchCellAdmissionDenial),
    OwnerMetadataOrdering,
    OwnerReentry,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SignalBranchRegistryPoisonRecovery {
    PreservedCanonicalMembership,
}

#[derive(Debug)]
enum SignalBranchRegistryEntry<S> {
    Reserved,
    Live(Arc<SignalBranchExecutionCell<S>>),
    Retiring(Arc<SignalBranchExecutionCell<S>>),
}

#[derive(Debug)]
struct SignalBranchRegistryState<S> {
    entries: BTreeMap<SignalBranchId, SignalBranchRegistryEntry<S>>,
    live_count: usize,
    reservation_count: usize,
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
                entries: BTreeMap::new(),
                live_count: 0,
                reservation_count: 0,
            }),
            counters: lifecycle.counters(),
            recovered_poison: AtomicBool::new(false),
        }
    }

    pub(crate) fn lookup(
        &self,
        admission: &SignalOwnerOperationAdmission<'_>,
        branch_id: SignalBranchId,
    ) -> Result<Arc<SignalBranchExecutionCell<S>>, SignalBranchRegistryDenial> {
        self.validate_admission(admission)?;
        admission.reach_operation_boundary(SignalOwnerOperationBoundary::BranchRegistryLookup);
        let _metadata_hold = admission
            .hold_owner_metadata()
            .map_err(map_metadata_hold_denial)?;
        self.counters.record_branch_registry_lookup();
        match self.lock_state().entries.get(&branch_id) {
            Some(SignalBranchRegistryEntry::Reserved) => {
                Err(SignalBranchRegistryDenial::UnknownBranch(branch_id))
            }
            Some(SignalBranchRegistryEntry::Live(cell)) => Ok(Arc::clone(cell)),
            Some(SignalBranchRegistryEntry::Retiring(_)) => {
                Err(SignalBranchRegistryDenial::RetirementInProgress(branch_id))
            }
            None => Err(SignalBranchRegistryDenial::UnknownBranch(branch_id)),
        }
    }

    pub(crate) fn reserve<'a>(
        &'a self,
        admission: &'a SignalOwnerOperationAdmission<'_>,
        branch_id: SignalBranchId,
    ) -> Result<SignalBranchReservation<'a, S>, SignalBranchRegistryDenial> {
        self.validate_admission(admission)?;
        admission.reach_operation_boundary(SignalOwnerOperationBoundary::BranchRegistryReservation);
        let _metadata_hold = admission
            .hold_owner_metadata()
            .map_err(map_metadata_hold_denial)?;
        let mut state = self.lock_state();
        self.validate_available_identity(&state, branch_id)?;
        self.validate_capacity(&state)?;
        let displaced = state
            .entries
            .insert(branch_id, SignalBranchRegistryEntry::Reserved);
        debug_assert!(
            displaced.is_none(),
            "accepted reservation identity must be vacant"
        );
        state.reservation_count += 1;
        self.counters.record_branch_registry_reservation();
        Ok(SignalBranchReservation {
            registry: self,
            admission,
            branch_id,
            prepared_cell_incarnation: None,
            consumed: false,
        })
    }

    pub(crate) fn live_cells_in_identity_order(
        &self,
        admission: &SignalOwnerOperationAdmission<'_>,
    ) -> Result<Vec<Arc<SignalBranchExecutionCell<S>>>, SignalBranchRegistryDenial> {
        self.validate_admission(admission)?;
        let _metadata_hold = admission
            .hold_owner_metadata()
            .map_err(map_metadata_hold_denial)?;
        let state = self.lock_state();
        let mut cells = Vec::with_capacity(state.live_count);
        for entry in state.entries.values() {
            self.counters.record_branch_registry_entry_scanned();
            if let SignalBranchRegistryEntry::Live(cell) = entry {
                cells.push(Arc::clone(cell));
            }
        }
        Ok(cells)
    }

    pub(crate) fn begin_retirement<'a>(
        &'a self,
        admission: &'a SignalOwnerOperationAdmission<'_>,
        branch_id: SignalBranchId,
    ) -> Result<SignalBranchRetirement<'a, S>, SignalBranchRegistryDenial> {
        self.validate_admission(admission)?;
        let _metadata_hold = admission
            .hold_owner_metadata()
            .map_err(map_metadata_hold_denial)?;
        let cell = {
            let mut state = self.lock_state();
            let entry = state
                .entries
                .get_mut(&branch_id)
                .ok_or(SignalBranchRegistryDenial::UnknownBranch(branch_id))?;
            let cell = match entry {
                SignalBranchRegistryEntry::Reserved => {
                    return Err(SignalBranchRegistryDenial::UnknownBranch(branch_id));
                }
                SignalBranchRegistryEntry::Live(cell) => Arc::clone(cell),
                SignalBranchRegistryEntry::Retiring(_) => {
                    return Err(SignalBranchRegistryDenial::RetirementInProgress(branch_id));
                }
            };
            *entry = SignalBranchRegistryEntry::Retiring(Arc::clone(&cell));
            cell
        };
        let retirement = SignalBranchRetirement {
            registry: self,
            admission,
            branch_id,
            cell,
            completed: false,
        };
        Ok(retirement)
    }

    pub(crate) fn poison_recovery(&self) -> Option<SignalBranchRegistryPoisonRecovery> {
        self.recovered_poison
            .load(Ordering::Acquire)
            .then_some(SignalBranchRegistryPoisonRecovery::PreservedCanonicalMembership)
    }

    fn validate_admission(
        &self,
        admission: &SignalOwnerOperationAdmission<'_>,
    ) -> Result<(), SignalBranchRegistryDenial> {
        admission
            .authorize(
                self.owner_runtime_instance_id,
                self.owner_lifecycle_identity,
            )
            .map_err(SignalBranchRegistryDenial::from)
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

fn map_metadata_hold_denial(
    denial: super::lifecycle_state::SignalOwnerMetadataHoldDenial,
) -> SignalBranchRegistryDenial {
    match denial {
        super::lifecycle_state::SignalOwnerMetadataHoldDenial::BranchCellOrMetadataAlreadyHeld => {
            SignalBranchRegistryDenial::OwnerMetadataOrdering
        }
        super::lifecycle_state::SignalOwnerMetadataHoldDenial::ExecutingThreadReentry => {
            SignalBranchRegistryDenial::OwnerReentry
        }
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
    admission: &'a SignalOwnerOperationAdmission<'a>,
    branch_id: SignalBranchId,
    prepared_cell_incarnation: Option<SignalBranchCellIncarnation>,
    consumed: bool,
}

impl<'a, S> SignalBranchReservation<'a, S> {
    pub(crate) fn admission(&self) -> &'a SignalOwnerOperationAdmission<'a> {
        self.admission
    }

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

    pub(crate) fn prepare_fork_destination_cell(
        &mut self,
        state: S,
    ) -> Result<SignalPreparedBranchCell<S>, SignalBranchRegistryDenial> {
        self.prepare_cell_state(state, true)
    }

    pub(crate) fn matches_prepared_fork_destination(
        &self,
        prepared: &SignalPreparedBranchCell<S>,
    ) -> bool {
        prepared.is_fork_destination
            && self.prepared_cell_incarnation == Some(prepared.cell.incarnation())
    }

    pub(crate) fn bind_prepared_fork_destination(
        self,
        prepared: SignalPreparedBranchCell<S>,
    ) -> SignalPreparedBranchInstallation<'a, S> {
        assert!(
            self.matches_prepared_fork_destination(&prepared),
            "prepared fork destination must match its exact reservation"
        );
        SignalPreparedBranchInstallation {
            reservation: self,
            cell: prepared.cell,
            is_fork_destination: true,
        }
    }

    fn install_cell(
        self,
        state: S,
        is_fork_destination: bool,
    ) -> Result<Arc<SignalBranchExecutionCell<S>>, SignalBranchRegistryDenial> {
        self.prepare_cell(state, is_fork_destination)?.install()
    }

    fn prepare_cell(
        mut self,
        state: S,
        is_fork_destination: bool,
    ) -> Result<SignalPreparedBranchInstallation<'a, S>, SignalBranchRegistryDenial> {
        let prepared = self.prepare_cell_state(state, is_fork_destination)?;
        Ok(SignalPreparedBranchInstallation {
            reservation: self,
            cell: prepared.cell,
            is_fork_destination: prepared.is_fork_destination,
        })
    }

    fn prepare_cell_state(
        &mut self,
        state: S,
        is_fork_destination: bool,
    ) -> Result<SignalPreparedBranchCell<S>, SignalBranchRegistryDenial> {
        self.registry.validate_admission(self.admission)?;
        let cell = Arc::new(SignalBranchExecutionCell::new(
            SignalBranchCellConstruction(()),
            state,
            self.registry.owner_runtime_instance_id,
            self.registry.owner_lifecycle_identity,
            self.branch_id,
            Arc::clone(&self.registry.counters),
        ));
        if is_fork_destination {
            self.registry.counters.record_fork_destination_preparation();
        }
        self.prepared_cell_incarnation = Some(cell.incarnation());
        Ok(SignalPreparedBranchCell {
            cell,
            is_fork_destination,
        })
    }
}

impl<S> Drop for SignalBranchReservation<'_, S> {
    fn drop(&mut self) {
        if !self.consumed {
            debug_assert!(
                self.admission.permits_owner_lock_acquisition(),
                "branch reservation cleanup must run after target-cell release"
            );
            let mut state = self.registry.lock_state();
            if matches!(
                state.entries.get(&self.branch_id),
                Some(SignalBranchRegistryEntry::Reserved)
            ) {
                state.entries.remove(&self.branch_id);
                state.reservation_count = state
                    .reservation_count
                    .checked_sub(1)
                    .expect("dropping a Signal branch reservation must release capacity");
            }
        }
    }
}
