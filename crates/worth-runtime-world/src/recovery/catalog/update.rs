use std::sync::Arc;

use super::{
    ProductUnpublishedOwnerEffectsIdentity, ProductUnpublishedOwnerEffectsRecord,
    ProductUnpublishedRecoveryCatalog,
};

/// Exclusive custody while a recovery service performs one owner-local
/// settlement. The record is absent from the visible map only during the
/// owner call, while its slot and metadata charge remain held by this token.
pub(crate) struct ReservedProductUnpublishedRecordUpdate {
    catalog: ProductUnpublishedRecoveryCatalog,
    identity: ProductUnpublishedOwnerEffectsIdentity,
    record: Option<Arc<ProductUnpublishedOwnerEffectsRecord>>,
    armed: bool,
}

impl std::fmt::Debug for ReservedProductUnpublishedRecordUpdate {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ReservedProductUnpublishedRecordUpdate")
            .field("identity", &self.identity)
            .field("armed", &self.armed)
            .finish_non_exhaustive()
    }
}

impl Drop for ReservedProductUnpublishedRecordUpdate {
    fn drop(&mut self) {
        if !self.armed {
            return;
        }
        let Some(record) = self.record.take() else {
            panic!("a live recovery update must retain its record")
        };
        let mut state = self
            .catalog
            .state
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        assert!(state
            .records
            .insert(self.identity.clone(), record)
            .is_none());
        state.updating_identities.remove(&self.identity);
        state.updating_slots = state
            .updating_slots
            .checked_sub(1)
            .expect("a live recovery update owns one slot");
        self.armed = false;
    }
}

impl ReservedProductUnpublishedRecordUpdate {
    pub(super) fn new(
        catalog: ProductUnpublishedRecoveryCatalog,
        identity: ProductUnpublishedOwnerEffectsIdentity,
        record: Arc<ProductUnpublishedOwnerEffectsRecord>,
    ) -> Self {
        Self {
            catalog,
            identity,
            record: Some(record),
            armed: true,
        }
    }

    pub(crate) fn record_mut(&mut self) -> Option<&mut ProductUnpublishedOwnerEffectsRecord> {
        self.record.as_mut().and_then(Arc::get_mut)
    }

    pub(crate) fn finish(mut self) {
        let record = self
            .record
            .take()
            .expect("a recovery update finishes with its retained record");
        let mut state = self
            .catalog
            .state
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        assert!(state
            .records
            .insert(self.identity.clone(), record)
            .is_none());
        state.updating_identities.remove(&self.identity);
        state.updating_slots = state
            .updating_slots
            .checked_sub(1)
            .expect("a live recovery update owns one slot");
        self.armed = false;
    }
}
