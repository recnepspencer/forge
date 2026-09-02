//! Exact component-pin registry vocabulary and transferred publication proof.

use crate::basis::AdmittedCompositeRuntimeWorldBasis;
use crate::identity::RuntimeWorldOwnerIdentity;

use super::component_obligation::PublicationRetentionObligation;
use super::obligation_transfer::ComponentBasisObligationTransferDestination;
use super::unique_component_pin::{ExactComponentBasis, ExactComponentBasisKey};

mod owner;

pub(crate) use super::obligation_transfer::RetentionTransferDenial;
#[allow(unused_imports)]
pub(crate) use owner::RuntimeWorldRetentionOwner;

/// Why the Runtime World could not issue an exact component dependency claim.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum RetentionObligationDenial {
    ForeignOwner {
        expected: RuntimeWorldOwnerIdentity,
        actual: RuntimeWorldOwnerIdentity,
    },
    InvalidComponentPair,
    UniquePinCapacityExhausted {
        maximum_unique_component_pins: usize,
    },
    InFlightAcquisitionCapacityExhausted {
        maximum_in_flight_reservations: usize,
    },
    DependencyCountExhausted,
    LeaseIdentityExhausted,
    Relational(worth_relational::facade::branch::RelationalBranchBasisDenial),
    Signal(worth_signal::facade::branch::SignalBranchRetentionAcquisitionDenial),
    OwnerOperationPanicked,
}

/// Counts only named structural work performed by this registry. No byte or
/// time estimate is invented for a component owner's lease.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct RetentionCostSnapshot {
    unique_pin_hits: u64,
    owner_acquisition_contacts: u64,
    owner_release_contacts: u64,
    owner_drop_releases: u64,
    dependency_acquires: u64,
    dependency_releases: u64,
    single_flight_joins: u64,
    batch_admitted: u64,
    batch_denied: u64,
    flights_started: u64,
    relational_contacts: u64,
    relational_successes: u64,
    relational_denials: u64,
    signal_contacts: u64,
    signal_successes: u64,
    signal_denials: u64,
    rollbacks: u64,
    reclamation_entries_examined: u64,
    reclamation_entries_reclaimed: u64,
}

impl RetentionCostSnapshot {
    pub(crate) const fn unique_pin_hits(self) -> u64 {
        self.unique_pin_hits
    }
    pub(crate) const fn owner_acquisition_contacts(self) -> u64 {
        self.owner_acquisition_contacts
    }
    pub(crate) const fn owner_release_contacts(self) -> u64 {
        self.owner_release_contacts
    }
    pub(crate) const fn owner_drop_releases(self) -> u64 {
        self.owner_drop_releases
    }
    pub(crate) const fn dependency_acquires(self) -> u64 {
        self.dependency_acquires
    }
    pub(crate) const fn dependency_releases(self) -> u64 {
        self.dependency_releases
    }
    pub(crate) const fn single_flight_joins(self) -> u64 {
        self.single_flight_joins
    }
    pub(crate) const fn batch_admitted(self) -> u64 {
        self.batch_admitted
    }
    pub(crate) const fn batch_denied(self) -> u64 {
        self.batch_denied
    }
    pub(crate) const fn flights_started(self) -> u64 {
        self.flights_started
    }
    pub(crate) const fn relational_contacts(self) -> u64 {
        self.relational_contacts
    }
    pub(crate) const fn relational_successes(self) -> u64 {
        self.relational_successes
    }
    pub(crate) const fn relational_denials(self) -> u64 {
        self.relational_denials
    }
    pub(crate) const fn signal_contacts(self) -> u64 {
        self.signal_contacts
    }
    pub(crate) const fn signal_successes(self) -> u64 {
        self.signal_successes
    }
    pub(crate) const fn signal_denials(self) -> u64 {
        self.signal_denials
    }
    pub(crate) const fn rollbacks(self) -> u64 {
        self.rollbacks
    }

    pub(super) fn record_component_contact(&mut self, component: ExactComponentBasis<'_>) {
        match component {
            ExactComponentBasis::Relational(_) => {
                self.relational_contacts = self.relational_contacts.saturating_add(1);
            }
            ExactComponentBasis::Signal(_) => {
                self.signal_contacts = self.signal_contacts.saturating_add(1);
            }
        }
    }

    pub(super) fn record_component_outcome(
        &mut self,
        component: ExactComponentBasis<'_>,
        succeeded: bool,
    ) {
        match component {
            ExactComponentBasis::Relational(_) if succeeded => {
                self.relational_successes = self.relational_successes.saturating_add(1);
            }
            ExactComponentBasis::Relational(_) => {
                self.relational_denials = self.relational_denials.saturating_add(1);
            }
            ExactComponentBasis::Signal(_) if succeeded => {
                self.signal_successes = self.signal_successes.saturating_add(1);
            }
            ExactComponentBasis::Signal(_) => {
                self.signal_denials = self.signal_denials.saturating_add(1);
            }
        }
    }
    pub(crate) const fn reclamation_entries_examined(self) -> u64 {
        self.reclamation_entries_examined
    }
    pub(crate) const fn reclamation_entries_reclaimed(self) -> u64 {
        self.reclamation_entries_reclaimed
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct RetentionReclamationReport {
    requested: usize,
    examined: usize,
    reclaimed: usize,
    remaining_unique_pins: usize,
}

impl RetentionReclamationReport {
    pub(crate) const fn requested(self) -> usize {
        self.requested
    }
    pub(crate) const fn examined(self) -> usize {
        self.examined
    }
    pub(crate) const fn reclaimed(self) -> usize {
        self.reclaimed
    }
    pub(crate) const fn remaining_unique_pins(self) -> usize {
        self.remaining_unique_pins
    }
}

/// Receipt carrying the one transferred publication obligation. The keys are
/// copied as evidence only; the obligation remains the sole release authority.
#[derive(Debug)]
pub(crate) struct RetentionTransferReceipt {
    relational: ExactComponentBasisKey,
    signal: ExactComponentBasisKey,
    destination: ComponentBasisObligationTransferDestination,
    obligation: PublicationRetentionObligation,
}

#[derive(Debug)]
pub(crate) struct RetentionTransferFailure {
    obligation: PublicationRetentionObligation,
    denial: RetentionTransferDenial,
}

impl RetentionTransferFailure {
    pub(crate) fn denial(&self) -> RetentionTransferDenial {
        self.denial
    }
    pub(crate) fn obligation(&self) -> &PublicationRetentionObligation {
        &self.obligation
    }
}

impl RetentionTransferReceipt {
    pub(crate) fn from_publication(
        obligation: PublicationRetentionObligation,
        basis: &AdmittedCompositeRuntimeWorldBasis,
        destination: ComponentBasisObligationTransferDestination,
    ) -> Result<Self, RetentionTransferFailure> {
        if !obligation.matches_basis(basis) {
            return Err(RetentionTransferFailure {
                obligation,
                denial: RetentionTransferDenial::BasisMismatch,
            });
        }
        let relational = obligation.relational().key().clone();
        let signal = obligation.signal().key().clone();
        match obligation.try_transfer_to(destination) {
            Ok(obligation) => Ok(Self {
                relational,
                signal,
                destination,
                obligation,
            }),
            Err((obligation, denial)) => Err(RetentionTransferFailure { obligation, denial }),
        }
    }

    pub(crate) const fn destination(&self) -> ComponentBasisObligationTransferDestination {
        self.destination
    }

    pub(crate) fn obligation(&self) -> &PublicationRetentionObligation {
        &self.obligation
    }

    pub(crate) fn matches_basis(&self, basis: &AdmittedCompositeRuntimeWorldBasis) -> bool {
        self.obligation.matches_basis(basis)
            && self.relational
                == super::unique_component_pin::ExactComponentPinRequest::relational(
                    basis,
                    super::ComponentBasisDependencyClass::ActivePublicationAttempt,
                )
                .key()
            && self.signal
                == super::unique_component_pin::ExactComponentPinRequest::signal(
                    basis,
                    super::ComponentBasisDependencyClass::ActivePublicationAttempt,
                )
                .key()
    }
}
