use std::sync::{Arc, Mutex};

use crate::budget::RuntimeWorldBudgetLimit;
use crate::identity::RuntimeWorldOwnerIdentity;

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

#[derive(Debug)]
struct RecoveryCatalogState {
    owner: RuntimeWorldOwnerIdentity,
    maximum_slots: usize,
    reserved_slots: usize,
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

impl ProductUnpublishedRecoveryCatalog {
    pub(crate) fn new(
        owner: RuntimeWorldOwnerIdentity,
        maximum_slots: RuntimeWorldBudgetLimit,
    ) -> Self {
        Self {
            state: Arc::new(Mutex::new(RecoveryCatalogState {
                owner,
                maximum_slots: maximum_slots.get(),
                reserved_slots: 0,
            })),
        }
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
        if state.reserved_slots >= state.maximum_slots {
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
