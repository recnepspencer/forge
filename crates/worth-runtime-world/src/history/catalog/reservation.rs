use std::sync::{Arc, Mutex};

use super::denial::CompositeHistoryCatalogDenial;
use super::entry::CompositeHistoryCatalogEntry;
use super::metadata::{HistoryMetadataCharge, HistoryReservationMetadata};
use super::support::{install_entry, lock_state, release_reservation};
use super::CompositeHistoryCatalogState;
use super::CompositeRuntimeWorldCommit;

/// Rollback custody for the one root entry staged during bootstrap. The
/// guard is armed only after installation and is disarmed after every root
/// artifact has been installed in its owning registry.
#[must_use = "a staged root history entry must be committed or rolled back"]
pub(crate) struct InstalledRootCommitRollback {
    state: Arc<Mutex<CompositeHistoryCatalogState>>,
    identity: crate::identity::CompositeCommitIdentity,
    armed: bool,
}

impl std::fmt::Debug for InstalledRootCommitRollback {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("InstalledRootCommitRollback")
            .field("identity", &self.identity)
            .field("armed", &self.armed)
            .finish_non_exhaustive()
    }
}

impl Drop for InstalledRootCommitRollback {
    fn drop(&mut self) {
        if !self.armed {
            return;
        }
        let mut state = lock_state(&self.state);
        let entry = super::support::remove_installed(&mut state, &self.identity);
        assert!(
            matches!(entry.commit().parent(), super::CompositeCommitParent::Root),
            "bootstrap rollback owns only the root history entry"
        );
        state.root_reserved = false;
        state.root_ever_installed = false;
        self.armed = false;
    }
}

impl InstalledRootCommitRollback {
    pub(super) fn new(
        state: Arc<Mutex<CompositeHistoryCatalogState>>,
        identity: crate::identity::CompositeCommitIdentity,
    ) -> Self {
        Self {
            state,
            identity,
            armed: true,
        }
    }

    pub(crate) fn commit(mut self) {
        self.armed = false;
    }
}

/// Rollback custody for any newly installed commit until the publication CAS
/// is reached. The CAS boundary disarms this guard because a stale product
/// head still leaves a valid immutable commit for recovery/reclamation.
#[must_use = "an installed commit must be committed or rolled back"]
pub(crate) struct InstalledCommitRollback {
    state: Arc<Mutex<CompositeHistoryCatalogState>>,
    identity: crate::identity::CompositeCommitIdentity,
    armed: bool,
}

impl std::fmt::Debug for InstalledCommitRollback {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("InstalledCommitRollback")
            .field("identity", &self.identity)
            .field("armed", &self.armed)
            .finish_non_exhaustive()
    }
}

impl Drop for InstalledCommitRollback {
    fn drop(&mut self) {
        if !self.armed {
            return;
        }
        let mut state = lock_state(&self.state);
        let entry = super::support::remove_installed(&mut state, &self.identity);
        if matches!(entry.commit().parent(), super::CompositeCommitParent::Root) {
            state.root_reserved = false;
            state.root_ever_installed = false;
        }
        self.armed = false;
    }
}

impl InstalledCommitRollback {
    pub(super) fn new(
        state: Arc<Mutex<CompositeHistoryCatalogState>>,
        identity: crate::identity::CompositeCommitIdentity,
    ) -> Self {
        Self {
            state,
            identity,
            armed: true,
        }
    }

    pub(crate) fn commit(mut self) {
        self.armed = false;
    }
}

#[must_use = "reserved commit capacity must be installed or dropped"]
pub(crate) struct ReservedCompositeCommitCapacity {
    state: Arc<Mutex<CompositeHistoryCatalogState>>,
    identity: crate::identity::CompositeCommitIdentity,
    reservation: HistoryReservationMetadata,
    armed: bool,
}

impl std::fmt::Debug for ReservedCompositeCommitCapacity {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ReservedCompositeCommitCapacity")
            .field("identity", &self.identity)
            .field("parent", &self.reservation.parent)
            .field("metadata_charge", &self.reservation.commit_charge)
            .finish_non_exhaustive()
    }
}

impl Drop for ReservedCompositeCommitCapacity {
    fn drop(&mut self) {
        if !self.armed {
            return;
        }
        let mut state = lock_state(&self.state);
        release_reservation(&mut state, &self.identity);
        self.armed = false;
    }
}

impl ReservedCompositeCommitCapacity {
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
