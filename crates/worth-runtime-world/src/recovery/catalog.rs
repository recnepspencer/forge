use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, Mutex};

use crate::budget::RuntimeWorldBudgetLimit;
use crate::identity::{ProductUnpublishedOwnerEffectsIdentity, RuntimeWorldOwnerIdentity};

use super::product_unpublished::{
    ProductUnpublishedOwnerEffectsRecord, ProductUnpublishedRecoveryHandle,
};

#[path = "catalog/update.rs"]
mod update;
use update::ReservedProductUnpublishedRecordUpdate;

#[cfg(test)]
#[path = "catalog_tests.rs"]
mod tests;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RecoveryCatalogDenial {
    ForeignOwner {
        expected: RuntimeWorldOwnerIdentity,
        actual: RuntimeWorldOwnerIdentity,
    },
    CapacityExhausted {
        maximum: usize,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RecoveryRecordRemovalDenial {
    CallerCapabilityLive,
    NotEligible,
}

#[derive(Debug)]
struct RecoveryCatalogState {
    owner: RuntimeWorldOwnerIdentity,
    maximum_slots: usize,
    maximum_metadata_bytes: usize,
    reserved_slots: usize,
    updating_slots: usize,
    updating_identities: BTreeSet<ProductUnpublishedOwnerEffectsIdentity>,
    metadata_bytes: usize,
    records:
        BTreeMap<ProductUnpublishedOwnerEffectsIdentity, Arc<ProductUnpublishedOwnerEffectsRecord>>,
}

/// Runtime World capacity for records whose owner effects outlived product
/// publication. It is a real bounded resource, not copied budget metadata.
#[derive(Debug, Clone)]
pub(crate) struct ProductUnpublishedRecoveryCatalog {
    state: Arc<Mutex<RecoveryCatalogState>>,
}

#[must_use = "an unpublished recovery slot must be retained or dropped"]
pub(crate) struct ReservedProductUnpublishedSlot {
    catalog: ProductUnpublishedRecoveryCatalog,
    armed: bool,
}

impl std::fmt::Debug for ReservedProductUnpublishedSlot {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ReservedProductUnpublishedSlot")
            .field("armed", &self.armed)
            .finish_non_exhaustive()
    }
}

impl Drop for ReservedProductUnpublishedSlot {
    fn drop(&mut self) {
        if !self.armed {
            return;
        }
        let mut state = self
            .catalog
            .state
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        state.reserved_slots = state
            .reserved_slots
            .checked_sub(1)
            .expect("a live recovery reservation owns one slot");
        self.armed = false;
    }
}

impl ReservedProductUnpublishedSlot {
    pub(crate) fn catalog_affinity(&self) -> usize {
        self.catalog.affinity()
    }

    pub(crate) fn install_record(
        self,
        identity: ProductUnpublishedOwnerEffectsIdentity,
        metadata_bytes: usize,
        record: Arc<ProductUnpublishedOwnerEffectsRecord>,
    ) -> Result<(), (Self, RecoveryCatalogDenial)> {
        let catalog = self.catalog.clone();
        catalog.install_reserved_record(self, identity, metadata_bytes, record)
    }
}

impl ProductUnpublishedRecoveryCatalog {
    pub(crate) fn new(
        owner: RuntimeWorldOwnerIdentity,
        maximum_slots: RuntimeWorldBudgetLimit,
    ) -> Self {
        Self {
            state: Arc::new(Mutex::new(RecoveryCatalogState {
                owner,
                maximum_slots: maximum_slots.get(),
                maximum_metadata_bytes: usize::MAX,
                reserved_slots: 0,
                updating_slots: 0,
                updating_identities: BTreeSet::new(),
                metadata_bytes: 0,
                records: BTreeMap::new(),
            })),
        }
    }

    pub(crate) fn new_with_metadata(
        owner: RuntimeWorldOwnerIdentity,
        maximum_slots: RuntimeWorldBudgetLimit,
        maximum_metadata_bytes: RuntimeWorldBudgetLimit,
    ) -> Self {
        Self {
            state: Arc::new(Mutex::new(RecoveryCatalogState {
                owner,
                maximum_slots: maximum_slots.get(),
                maximum_metadata_bytes: maximum_metadata_bytes.get(),
                reserved_slots: 0,
                updating_slots: 0,
                updating_identities: BTreeSet::new(),
                metadata_bytes: 0,
                records: BTreeMap::new(),
            })),
        }
    }

    pub(crate) fn affinity(&self) -> usize {
        Arc::as_ptr(&self.state) as usize
    }

    pub(crate) fn reserve_product_unpublished(
        &self,
        owner: RuntimeWorldOwnerIdentity,
    ) -> Result<ReservedProductUnpublishedSlot, RecoveryCatalogDenial> {
        let mut state = self.state.lock().unwrap_or_else(|error| error.into_inner());
        if owner != state.owner {
            return Err(RecoveryCatalogDenial::ForeignOwner {
                expected: state.owner,
                actual: owner,
            });
        }
        if state
            .records
            .len()
            .saturating_add(state.reserved_slots)
            .saturating_add(state.updating_slots)
            >= state.maximum_slots
        {
            return Err(RecoveryCatalogDenial::CapacityExhausted {
                maximum: state.maximum_slots,
            });
        }
        state.reserved_slots += 1;
        Ok(ReservedProductUnpublishedSlot {
            catalog: self.clone(),
            armed: true,
        })
    }

    fn install_record_locked(
        &self,
        state: &mut RecoveryCatalogState,
        identity: ProductUnpublishedOwnerEffectsIdentity,
        metadata_bytes: usize,
        record: Arc<ProductUnpublishedOwnerEffectsRecord>,
    ) -> Result<(), RecoveryCatalogDenial> {
        if identity.owner_identity() != state.owner {
            return Err(RecoveryCatalogDenial::ForeignOwner {
                expected: state.owner,
                actual: identity.owner_identity(),
            });
        }
        if state.records.contains_key(&identity) {
            return Err(RecoveryCatalogDenial::CapacityExhausted {
                maximum: state.maximum_slots,
            });
        }
        if record.catalog_affinity() != self.affinity() {
            return Err(RecoveryCatalogDenial::ForeignOwner {
                expected: state.owner,
                actual: identity.owner_identity(),
            });
        }
        let Some(metadata_after) = state.metadata_bytes.checked_add(metadata_bytes) else {
            return Err(RecoveryCatalogDenial::CapacityExhausted {
                maximum: state.maximum_slots,
            });
        };
        if metadata_after > state.maximum_metadata_bytes {
            return Err(RecoveryCatalogDenial::CapacityExhausted {
                maximum: state.maximum_slots,
            });
        }
        if state.reserved_slots == 0 {
            return Err(RecoveryCatalogDenial::CapacityExhausted {
                maximum: state.maximum_slots,
            });
        }
        state.reserved_slots -= 1;
        state.metadata_bytes = metadata_after;
        assert!(state.records.insert(identity, record).is_none());
        Ok(())
    }

    pub(crate) fn install_reserved_record(
        &self,
        slot: ReservedProductUnpublishedSlot,
        identity: ProductUnpublishedOwnerEffectsIdentity,
        metadata_bytes: usize,
        record: Arc<ProductUnpublishedOwnerEffectsRecord>,
    ) -> Result<(), (ReservedProductUnpublishedSlot, RecoveryCatalogDenial)> {
        match self.install_record_from_slot(slot, identity, metadata_bytes, record) {
            Ok(()) => Ok(()),
            Err((slot, denial)) => Err((slot, denial)),
        }
    }

    fn install_record_from_slot(
        &self,
        mut slot: ReservedProductUnpublishedSlot,
        identity: ProductUnpublishedOwnerEffectsIdentity,
        metadata_bytes: usize,
        record: Arc<ProductUnpublishedOwnerEffectsRecord>,
    ) -> Result<(), (ReservedProductUnpublishedSlot, RecoveryCatalogDenial)> {
        let result = {
            let mut state = self.state.lock().unwrap_or_else(|error| error.into_inner());
            self.install_record_locked(&mut state, identity, metadata_bytes, record)
        };
        match result {
            Ok(()) => {
                slot.armed = false;
                Ok(())
            }
            Err(denial) => Err((slot, denial)),
        }
    }

    pub(crate) fn lookup_record(
        &self,
        handle: &ProductUnpublishedRecoveryHandle,
    ) -> Option<Arc<ProductUnpublishedOwnerEffectsRecord>> {
        if handle.catalog_affinity() != self.affinity() {
            return None;
        }
        let state = self.state.lock().unwrap_or_else(|error| error.into_inner());
        state.records.get(handle.identity()).cloned()
    }

    pub(crate) fn take_record_for_update(
        &self,
        handle: &ProductUnpublishedRecoveryHandle,
    ) -> Option<ReservedProductUnpublishedRecordUpdate> {
        if handle.catalog_affinity() != self.affinity() {
            return None;
        }
        let mut state = self.state.lock().unwrap_or_else(|error| error.into_inner());
        let record = state.records.remove(handle.identity())?;
        state.updating_slots = state
            .updating_slots
            .checked_add(1)
            .expect("a bounded recovery update counter cannot overflow");
        assert!(state.updating_identities.insert(handle.identity().clone()));
        Some(ReservedProductUnpublishedRecordUpdate::new(
            self.clone(),
            handle.identity().clone(),
            record,
        ))
    }

    pub(crate) fn remove_record_if_exclusive<F>(
        &self,
        handle: &ProductUnpublishedRecoveryHandle,
        eligible: F,
    ) -> Result<Option<Arc<ProductUnpublishedOwnerEffectsRecord>>, RecoveryRecordRemovalDenial>
    where
        F: FnOnce(&ProductUnpublishedOwnerEffectsRecord) -> bool,
    {
        if handle.catalog_affinity() != self.affinity() {
            return Ok(None);
        }
        let mut state = self.state.lock().unwrap_or_else(|error| error.into_inner());
        let Some(record) = state.records.get(handle.identity()) else {
            return Ok(None);
        };
        if Arc::strong_count(record) != 1 {
            return Err(RecoveryRecordRemovalDenial::CallerCapabilityLive);
        }
        if !eligible(record) {
            return Err(RecoveryRecordRemovalDenial::NotEligible);
        }
        let record = state
            .records
            .remove(handle.identity())
            .expect("the checked recovery record remains installed");
        state.metadata_bytes = state
            .metadata_bytes
            .checked_sub(record.metadata_bytes())
            .expect("a catalog record owns its metadata charge");
        Ok(Some(record))
    }

    pub(crate) fn contains_handle(&self, handle: &ProductUnpublishedRecoveryHandle) -> bool {
        self.lookup_record(handle).is_some()
    }

    pub(crate) fn identities(&self) -> Vec<ProductUnpublishedOwnerEffectsIdentity> {
        let state = self.state.lock().unwrap_or_else(|error| error.into_inner());
        state
            .records
            .keys()
            .chain(state.updating_identities.iter())
            .cloned()
            .collect()
    }

    pub(crate) fn installed_slots(&self) -> usize {
        let state = self.state.lock().unwrap_or_else(|error| error.into_inner());
        state.records.len().saturating_add(state.updating_slots)
    }

    pub(crate) fn metadata_bytes(&self) -> usize {
        self.state
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .metadata_bytes
    }

    pub(crate) fn drain_records(&self) -> Vec<Arc<ProductUnpublishedOwnerEffectsRecord>> {
        let mut state = self.state.lock().unwrap_or_else(|error| error.into_inner());
        assert_eq!(
            state.updating_slots, 0,
            "catalog shutdown cannot drain while a recovery update owns custody"
        );
        assert_eq!(
            state.reserved_slots, 0,
            "catalog shutdown cannot drain while a recovery reservation owns custody"
        );
        assert!(
            state.updating_identities.is_empty(),
            "catalog shutdown cannot drain while an update identity is live"
        );
        state.metadata_bytes = 0;
        std::mem::take(&mut state.records).into_values().collect()
    }

    pub(crate) fn reserved_slots(&self) -> usize {
        self.state
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .reserved_slots
    }

    pub(crate) fn maximum_slots(&self) -> usize {
        self.state
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .maximum_slots
    }
}

pub(crate) use ProductUnpublishedRecoveryCatalog as RecoveryCatalog;
