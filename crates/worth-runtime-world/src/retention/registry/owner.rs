use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, Condvar, Mutex};

use worth_relational::facade::branch::{
    RelationalBranchBasisPort, RelationalBranchRetentionLease, RelationalOwnerServicePorts,
};
use worth_signal::facade::branch::{
    SignalBranchBasisPort, SignalBranchRetentionLease, SignalOwnerServicePorts,
};

use crate::basis::AdmittedCompositeRuntimeWorldBasis;
use crate::budget::RuntimeWorldBudgetLimit;
use crate::history::CompositeRuntimeWorldCommit;
use crate::identity::RuntimeWorldOwnerIdentity;

use super::super::component_obligation::{
    ObservationRetentionObligation, PublicationRetentionObligation,
    RetainedPartialRetentionObligation, RetentionReleaseDenial,
};
use super::super::dependency_counts::ComponentBasisDependencyCounts;
use super::super::unique_component_pin::{ComponentBasisLeaseIdentity, ExactComponentBasisKey};
use super::super::ComponentBasisDependencyClass;
use super::{RetentionCostSnapshot, RetentionObligationDenial, RetentionReclamationReport};

mod acquisition;
mod batch_acquisition;
mod claim_lifecycle;

#[cfg(test)]
mod tests;

#[derive(Debug)]
enum ComponentOwnerLease {
    Relational(RelationalBranchRetentionLease),
    Signal(SignalBranchRetentionLease),
}

#[derive(Debug)]
struct PinEntry {
    owner_lease: Option<ComponentOwnerLease>,
    counts: ComponentBasisDependencyCounts,
    lease_identity: ComponentBasisLeaseIdentity,
}

#[derive(Debug, Clone)]
enum FlightCompletion {
    Pending,
    Acquired,
    Released,
    AcquisitionDenied(RetentionObligationDenial),
}

#[derive(Debug)]
struct PinFlight {
    completion: Mutex<FlightCompletion>,
    wake: Condvar,
}

impl PinFlight {
    fn new() -> Self {
        Self {
            completion: Mutex::new(FlightCompletion::Pending),
            wake: Condvar::new(),
        }
    }

    fn finish(&self, completion: FlightCompletion) {
        let mut current = self
            .completion
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        *current = completion;
        self.wake.notify_all();
    }
}

struct RegistryState<D, I, T>
where
    D: Copy + Ord + fmt::Debug + Send + Sync + 'static,
    I: Copy + Ord + Send + Sync + 'static,
    T: Copy + Ord + Send + Sync + 'static,
{
    owner_identity: RuntimeWorldOwnerIdentity,
    relational_port: RelationalBranchBasisPort,
    signal_port: SignalBranchBasisPort<D, I, T>,
    maximum_unique_pins: usize,
    maximum_in_flight_reservations: usize,
    unique_slots: usize,
    active_reservations: usize,
    active_obligations: usize,
    next_lease_ordinal: u64,
    entries: HashMap<ExactComponentBasisKey, PinEntry>,
    flights: HashMap<ExactComponentBasisKey, Arc<PinFlight>>,
    costs: RetentionCostSnapshot,
}

struct OwnerReleaseFailure {
    reason: RetentionReleaseDenial,
    lease: ComponentOwnerLease,
}

/// Concrete Runtime World retention authority. It stores only already-issued
/// weak component service ports and never recreates a component owner.
pub(crate) struct RuntimeWorldRetentionOwner<D, I, T>
where
    D: Copy + Ord + fmt::Debug + Send + Sync + 'static,
    I: Copy + Ord + Send + Sync + 'static,
    T: Copy + Ord + Send + Sync + 'static,
{
    state: Arc<Mutex<RegistryState<D, I, T>>>,
}

impl<D, I, T> Clone for RuntimeWorldRetentionOwner<D, I, T>
where
    D: Copy + Ord + fmt::Debug + Send + Sync + 'static,
    I: Copy + Ord + Send + Sync + 'static,
    T: Copy + Ord + Send + Sync + 'static,
{
    fn clone(&self) -> Self {
        Self {
            state: Arc::clone(&self.state),
        }
    }
}

impl<D, I, T> fmt::Debug for RuntimeWorldRetentionOwner<D, I, T>
where
    D: Copy + Ord + fmt::Debug + Send + Sync + 'static,
    I: Copy + Ord + Send + Sync + 'static,
    T: Copy + Ord + Send + Sync + 'static,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RuntimeWorldRetentionOwner")
            .finish_non_exhaustive()
    }
}

impl<D, I, T> RuntimeWorldRetentionOwner<D, I, T>
where
    D: Copy + Ord + fmt::Debug + Send + Sync + 'static,
    I: Copy + Ord + Send + Sync + 'static,
    T: Copy + Ord + Send + Sync + 'static,
{
    pub(crate) fn new(
        owner_identity: RuntimeWorldOwnerIdentity,
        relational_port: RelationalBranchBasisPort,
        signal_port: SignalBranchBasisPort<D, I, T>,
        unique_pin_limit: RuntimeWorldBudgetLimit,
        reservation_limit: RuntimeWorldBudgetLimit,
    ) -> Self {
        let state = RegistryState {
            owner_identity,
            relational_port,
            signal_port,
            maximum_unique_pins: unique_pin_limit.get(),
            maximum_in_flight_reservations: reservation_limit.get(),
            unique_slots: 0,
            active_reservations: 0,
            active_obligations: 0,
            next_lease_ordinal: 0,
            entries: HashMap::new(),
            flights: HashMap::new(),
            costs: RetentionCostSnapshot::default(),
        };
        Self {
            state: Arc::new(Mutex::new(state)),
        }
    }

    pub(crate) fn from_component_services<E, Ctx>(
        owner_identity: RuntimeWorldOwnerIdentity,
        relational: &RelationalOwnerServicePorts,
        signal: &SignalOwnerServicePorts<D, I, E, Ctx, T>,
        unique_pin_limit: RuntimeWorldBudgetLimit,
        reservation_limit: RuntimeWorldBudgetLimit,
    ) -> Self {
        Self::new(
            owner_identity,
            relational.basis_port(),
            signal.basis_port(),
            unique_pin_limit,
            reservation_limit,
        )
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, RegistryState<D, I, T>> {
        self.state.lock().unwrap_or_else(|error| error.into_inner())
    }

    pub(crate) fn issue_observation(
        &self,
        commit: &CompositeRuntimeWorldCommit,
    ) -> Result<ObservationRetentionObligation, RetentionObligationDenial> {
        self.issue_pair(
            commit.basis(),
            ComponentBasisDependencyClass::AdmittedObservation,
        )
        .map(|pair| ObservationRetentionObligation::owner_issued(commit, pair))
    }

    pub(crate) fn issue_publication(
        &self,
        basis: &AdmittedCompositeRuntimeWorldBasis,
    ) -> Result<PublicationRetentionObligation, RetentionObligationDenial> {
        self.issue_pair(
            basis,
            ComponentBasisDependencyClass::ActivePublicationAttempt,
        )
        .map(PublicationRetentionObligation::owner_issued)
    }

    pub(crate) fn issue_retained_partial(
        &self,
        basis: &AdmittedCompositeRuntimeWorldBasis,
    ) -> Result<RetainedPartialRetentionObligation, RetentionObligationDenial> {
        self.issue_pair(
            basis,
            ComponentBasisDependencyClass::ProductUnpublishedOwnerEffects,
        )
        .map(RetainedPartialRetentionObligation::owner_issued)
    }

    pub(crate) fn active_component_obligation_count(&self) -> usize {
        self.lock().active_obligations
    }

    pub(crate) fn unique_pin_count(&self) -> usize {
        self.lock().unique_slots
    }

    pub(crate) fn in_flight_acquisition_count(&self) -> usize {
        self.lock().active_reservations
    }

    pub(crate) fn cost_snapshot(&self) -> RetentionCostSnapshot {
        self.lock().costs
    }

    pub(crate) fn reclaim(&self, requested: usize) -> RetentionReclamationReport {
        let mut state = self.lock();
        let keys: Vec<_> = state.entries.keys().take(requested).cloned().collect();
        let mut reclaimed = 0;
        for key in &keys {
            let eligible = state
                .entries
                .get(key)
                .is_some_and(|entry| entry.owner_lease.is_none() && entry.counts.is_zero());
            state.costs.reclamation_entries_examined =
                state.costs.reclamation_entries_examined.saturating_add(1);
            if eligible && state.entries.remove(key).is_some() {
                state.unique_slots -= 1;
                reclaimed += 1;
                state.costs.reclamation_entries_reclaimed =
                    state.costs.reclamation_entries_reclaimed.saturating_add(1);
            }
        }
        RetentionReclamationReport {
            requested,
            examined: keys.len(),
            reclaimed,
            remaining_unique_pins: state.unique_slots,
        }
    }
}
