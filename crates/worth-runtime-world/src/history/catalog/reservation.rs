use std::sync::{Arc, Mutex};

use super::denial::CompositeHistoryCatalogDenial;
use super::entry::CompositeHistoryCatalogEntry;
use super::metadata::{HistoryMetadataCharge, HistoryReservationMetadata};
use super::support::{install_entry, lock_state, release_reservation};
use super::CompositeHistoryCatalogState;
use super::CompositeRuntimeWorldCommit;

#[must_use = "a reserved history slot must be installed or dropped"]
pub(crate) struct ReservedCompositeHistorySlot {
    state: Arc<Mutex<CompositeHistoryCatalogState>>,
    identity: crate::identity::CompositeCommitIdentity,
    reservation: HistoryReservationMetadata,
    armed: bool,
}

impl std::fmt::Debug for ReservedCompositeHistorySlot {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ReservedCompositeHistorySlot")
            .field("identity", &self.identity)
            .field("parent", &self.reservation.parent)
            .field("metadata_charge", &self.reservation.commit_charge)
            .finish_non_exhaustive()
    }
}

impl Drop for ReservedCompositeHistorySlot {
    fn drop(&mut self) {
        if !self.armed {
            return;
        }
        let mut state = lock_state(&self.state);
        release_reservation(&mut state, &self.identity);
        self.armed = false;
    }
}

impl ReservedCompositeHistorySlot {
    pub(super) fn new(
        state: Arc<Mutex<CompositeHistoryCatalogState>>,
        identity: crate::identity::CompositeCommitIdentity,
        reservation: HistoryReservationMetadata,
    ) -> Self {
        Self {
            state,
            identity,
            reservation,
            armed: true,
        }
    }

    pub(crate) fn install(
        mut self,
        commit: Arc<CompositeRuntimeWorldCommit>,
    ) -> Result<CompositeHistoryCatalogEntry, CompositeHistoryCatalogDenial> {
        let actual_charge = HistoryMetadataCharge::for_commit(commit.as_ref())
            .map_err(|_| CompositeHistoryCatalogDenial::ArithmeticOverflow)?;
        let mut state = lock_state(&self.state);
        let Some(reservation) = state.reservations.get(&self.identity) else {
            return Err(CompositeHistoryCatalogDenial::ReservationMissing);
        };
        if commit.identity() != &self.identity {
            return Err(CompositeHistoryCatalogDenial::ReservationCommitMismatch);
        }
        if commit.parent() != &reservation.parent || commit.parent() != &self.reservation.parent {
            return Err(CompositeHistoryCatalogDenial::ReservationParentMismatch);
        }
        if actual_charge != reservation.commit_charge
            || actual_charge != self.reservation.commit_charge
        {
            return Err(CompositeHistoryCatalogDenial::ReservationChargeMismatch);
        }

        let reservation = state
            .reservations
            .remove(&self.identity)
            .expect("the reservation was present immediately before installation");
        state
            .metadata
            .promote(reservation.reservation_charge, reservation.commit_charge);
        super::counters::lock_counters(&state.counters).record_metadata_promotion();
        if matches!(reservation.parent, super::CompositeCommitParent::Root) {
            state.root_reserved = false;
        }
        let entry = CompositeHistoryCatalogEntry {
            commit,
            metadata_charge: reservation.commit_charge,
        };
        install_entry(&mut state, entry.clone());
        self.armed = false;
        Ok(entry)
    }
}
