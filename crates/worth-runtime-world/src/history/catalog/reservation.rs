use std::sync::{Arc, Mutex};

use super::denial::CompositeHistoryCatalogDenial;
use super::entry::CompositeHistoryCatalogEntry;
use super::{
    install_entry, lock_state, release_reservation, CompositeCommitParent,
    CompositeHistoryCatalogState, CompositeRuntimeWorldCommit,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ReservedHistoryMetadata {
    pub(super) parent: CompositeCommitParent,
    pub(super) metadata_bytes: usize,
}

#[must_use = "a reserved history slot must be installed or dropped"]
pub(crate) struct ReservedCompositeHistorySlot {
    state: Arc<Mutex<CompositeHistoryCatalogState>>,
    identity: crate::identity::CompositeCommitIdentity,
    parent: CompositeCommitParent,
    metadata_bytes: usize,
    armed: bool,
}

impl std::fmt::Debug for ReservedCompositeHistorySlot {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ReservedCompositeHistorySlot")
            .field("identity", &self.identity)
            .field("parent", &self.parent)
            .field("metadata_bytes", &self.metadata_bytes)
            .finish_non_exhaustive()
    }
}

impl Drop for ReservedCompositeHistorySlot {
    fn drop(&mut self) {
        if !self.armed {
            return;
        }
        let mut state = lock_state(&self.state);
        release_reservation(&mut state, &self.identity, self.metadata_bytes);
        self.armed = false;
    }
}

impl ReservedCompositeHistorySlot {
    pub(crate) fn new(
        state: Arc<Mutex<CompositeHistoryCatalogState>>,
        identity: crate::identity::CompositeCommitIdentity,
        parent: CompositeCommitParent,
        metadata_bytes: usize,
    ) -> Self {
        Self {
            state,
            identity,
            parent,
            metadata_bytes,
            armed: true,
        }
    }

    pub(crate) fn install(
        mut self,
        commit: Arc<CompositeRuntimeWorldCommit>,
    ) -> Result<CompositeHistoryCatalogEntry, CompositeHistoryCatalogDenial> {
        let actual_metadata_bytes = commit.metadata_bytes();
        let mut state = lock_state(&self.state);
        let Some(reservation) = state.reservations.get(&self.identity) else {
            return Err(CompositeHistoryCatalogDenial::ReservationMissing);
        };
        if commit.identity() != &self.identity {
            return Err(CompositeHistoryCatalogDenial::ReservationCommitMismatch);
        }
        if commit.parent() != &reservation.parent || commit.parent() != &self.parent {
            return Err(CompositeHistoryCatalogDenial::ReservationParentMismatch);
        }
        if actual_metadata_bytes > reservation.metadata_bytes {
            return Err(CompositeHistoryCatalogDenial::ReservationMetadataTooSmall {
                reserved: reservation.metadata_bytes,
                actual: actual_metadata_bytes,
            });
        }

        let reservation = state
            .reservations
            .remove(&self.identity)
            .expect("the reservation was present immediately before installation");
        state.reserved_metadata_bytes -= reservation.metadata_bytes;
        state.metadata_bytes = state
            .metadata_bytes
            .checked_add(actual_metadata_bytes)
            .expect("reserved metadata capacity makes installation addition bounded");
        if matches!(reservation.parent, CompositeCommitParent::Root) {
            state.root_reserved = false;
        }
        let entry = CompositeHistoryCatalogEntry {
            commit,
            metadata_bytes: actual_metadata_bytes,
        };
        install_entry(&mut state, entry.clone());
        self.armed = false;
        Ok(entry)
    }
}
