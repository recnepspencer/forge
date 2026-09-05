use std::sync::Arc;

use crate::identity::ProductUnpublishedOwnerEffectsIdentity;
use crate::publication::ActiveAttemptRecord;

use super::{ProductUnpublishedRecoveryCatalog, ReservedProductUnpublishedSlot};

impl ReservedProductUnpublishedSlot {
    /// Choose an explicit retained terminal through the same owner record
    /// and reservation conversion used by caller abandonment.
    pub(crate) fn retain_active(
        mut self,
        identity: &ProductUnpublishedOwnerEffectsIdentity,
    ) -> super::ProductUnpublishedOwnerEffects {
        let (record, removed, permits) = {
            let mut state = self.catalog.locked_state();
            let active = state
                .active
                .get(identity)
                .expect("the caller owns a registered attempt");
            let permits = active.abandon();
            let record = active
                .materialize_abandoned(self.catalog.affinity())
                .expect("retained active evidence is representable");
            state.reserved_slots -= 1;
            state.reserved_metadata_bytes -= self.reserved_metadata_bytes;
            state.metadata_bytes += self.reserved_metadata_bytes;
            // Acquire the caller's capability before exposing the record to
            // cleanup. Releasing admission first would allow close or another
            // caller to remove the record before this terminal received it.
            assert!(state
                .records
                .insert(identity.clone(), Arc::clone(&record))
                .is_none());
            let removed = state.active.remove(identity);
            self.armed = false;
            (record, removed, permits)
        };
        drop(removed);
        drop(permits);
        super::ProductUnpublishedOwnerEffects::from_catalog_record(record)
    }

    /// Install the owner record while the slot is still reserved, before any
    /// component effect. Live attempts and abandoned records use one budget.
    pub(crate) fn register_active(&self, record: Arc<ActiveAttemptRecord>) {
        let mut state = self.catalog.locked_state();
        assert_eq!(record.identity().owner_identity(), state.owner);
        assert!(!state.records.contains_key(record.identity()));
        assert!(!state.active.contains_key(record.identity()));
        assert!(state
            .active
            .insert(record.identity().clone(), record)
            .is_none());
    }

    pub(crate) fn remove_active(&self, identity: &ProductUnpublishedOwnerEffectsIdentity) {
        let record = self.catalog.locked_state().active.remove(identity);
        drop(record);
    }

    /// Convert the existing slot charge to retained custody without allocating
    /// a record, installing history, acquiring pins, or contacting an owner.
    pub(crate) fn abandon_active(mut self, identity: &ProductUnpublishedOwnerEffectsIdentity) {
        let permits = {
            let mut state = self.catalog.locked_state();
            let record = state
                .active
                .get(identity)
                .expect("the caller owns a registered attempt");
            let permits = record.abandon();
            state.reserved_slots -= 1;
            state.reserved_metadata_bytes -= self.reserved_metadata_bytes;
            state.abandoned_slots += 1;
            state.metadata_bytes += self.reserved_metadata_bytes;
            self.armed = false;
            permits
        };
        drop(permits);
    }
}

impl ProductUnpublishedRecoveryCatalog {
    /// Convert under one catalog selection. A competing lookup sees either the
    /// complete original custody or the complete retained row. The slot and
    /// byte charge remain unchanged, and destructors run outside the lock.
    pub(super) fn materialize_abandoned(&self, identity: &ProductUnpublishedOwnerEffectsIdentity) {
        let removed = {
            let mut state = self.locked_state();
            let Some(active) = state.active.get(identity) else {
                return;
            };
            let Some(record) = active.materialize_abandoned(self.affinity()) else {
                return;
            };
            assert!(!state.records.contains_key(identity));
            state.records.insert(identity.clone(), record);
            state.abandoned_slots -= 1;
            state.active.remove(identity)
        };
        drop(removed);
    }
}
