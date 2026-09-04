use std::sync::{Arc, Mutex};

use crate::branch::observation::RuntimeWorldBranchAdmissionDenial;
use crate::budget::RuntimeWorldBudgetLimit;
use crate::identity::{ProductBranchIdentity, RuntimeWorldOwnerIdentity};

use super::{CustodyComponent, OwnerCreatedComponentCustodyRecord};

#[derive(Debug)]
struct CustodyRegistryState {
    owner: RuntimeWorldOwnerIdentity,
    maximum: usize,
    reserved: usize,
    installed: Vec<OwnerCreatedComponentCustodyRecord>,
}

/// The only managed registry of owner-created component branches. It is
/// bounded by the installed custody budget and charged before the owner fork
/// that would create the branch it records.
#[derive(Debug, Clone)]
pub(crate) struct OwnerCreatedComponentCustodyRegistry {
    state: Arc<Mutex<CustodyRegistryState>>,
}

impl OwnerCreatedComponentCustodyRegistry {
    pub(crate) fn new(owner: RuntimeWorldOwnerIdentity, maximum: RuntimeWorldBudgetLimit) -> Self {
        Self {
            state: Arc::new(Mutex::new(CustodyRegistryState {
                owner,
                maximum: maximum.get(),
                reserved: 0,
                installed: Vec::new(),
            })),
        }
    }

    pub(crate) fn owner(&self) -> RuntimeWorldOwnerIdentity {
        self.state
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .owner
    }

    /// Charged before the owner fork that would create the recorded branch;
    /// exhaustion denies pre-effect.
    pub(crate) fn reserve(
        &self,
        component: CustodyComponent,
    ) -> Result<ReservedCustodySlot, RuntimeWorldBranchAdmissionDenial> {
        let mut state = self.state.lock().unwrap_or_else(|error| error.into_inner());
        if state.installed.len().saturating_add(state.reserved) >= state.maximum {
            return Err(RuntimeWorldBranchAdmissionDenial::CustodyCapacityExhausted);
        }
        state.reserved = state
            .reserved
            .checked_add(1)
            .ok_or(RuntimeWorldBranchAdmissionDenial::CustodyCapacityExhausted)?;
        drop(state);
        Ok(ReservedCustodySlot {
            registry: self.clone(),
            component,
            armed: true,
        })
    }

    pub(crate) fn take_for_branch(
        &self,
        branch: &ProductBranchIdentity,
    ) -> Vec<OwnerCreatedComponentCustodyRecord> {
        let mut state = self.state.lock().unwrap_or_else(|error| error.into_inner());
        let mut taken = Vec::new();
        let mut retained = Vec::with_capacity(state.installed.len());
        for record in std::mem::take(&mut state.installed) {
            if record.product_branch() == branch {
                taken.push(record);
            } else {
                retained.push(record);
            }
        }
        state.installed = retained;
        taken
    }

    pub(crate) fn installed(&self) -> usize {
        self.state
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .installed
            .len()
    }
}

/// One charged custody slot. Dropping it uninstalled releases its charge.
#[must_use = "a reserved custody slot is installed or dropped"]
pub(crate) struct ReservedCustodySlot {
    registry: OwnerCreatedComponentCustodyRegistry,
    component: CustodyComponent,
    armed: bool,
}

impl std::fmt::Debug for ReservedCustodySlot {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ReservedCustodySlot")
            .field("component", &self.component)
            .field("armed", &self.armed)
            .finish_non_exhaustive()
    }
}

impl ReservedCustodySlot {
    pub(crate) const fn component(&self) -> CustodyComponent {
        self.component
    }

    pub(crate) fn install(mut self, record: OwnerCreatedComponentCustodyRecord) {
        let mut state = self
            .registry
            .state
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        state.reserved = state
            .reserved
            .checked_sub(1)
            .expect("a live custody reservation owns one slot");
        state.installed.push(record);
        drop(state);
        self.armed = false;
    }
}

impl Drop for ReservedCustodySlot {
    fn drop(&mut self) {
        if !self.armed {
            return;
        }
        let mut state = self
            .registry
            .state
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        state.reserved = state
            .reserved
            .checked_sub(1)
            .expect("a live custody reservation owns one slot");
        self.armed = false;
    }
}
