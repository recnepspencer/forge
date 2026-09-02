use std::sync::{Arc, Mutex};

use crate::identity::{
    ProductBranchIdentity, ProductBranchLifecycleIncarnation, RuntimeWorldOwnerIdentity,
};

use super::{ProductBranchName, ProductBranchReferenceCell, ProductBranchReferenceSnapshot};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ProductBranchRegistryDenial {
    ForeignOwner,
    CapacityExhausted,
    AlreadyInstalled,
    ReservationMissing,
    IdentityMismatch,
}

#[derive(Debug)]
struct ProductBranchRegistryState {
    owner: RuntimeWorldOwnerIdentity,
    maximum_branches: usize,
    reserved_branches: usize,
    root: Option<ProductBranchRegistryEntry>,
}

#[derive(Debug, Clone)]
struct ProductBranchRegistryEntry {
    name: ProductBranchName,
    branch: ProductBranchIdentity,
    lifecycle: ProductBranchLifecycleIncarnation,
    cell: ProductBranchReferenceCell,
}

/// The only managed owner registry for Runtime World product branches.
#[derive(Debug, Clone)]
pub(crate) struct ProductBranchRegistry {
    state: Arc<Mutex<ProductBranchRegistryState>>,
}

#[must_use = "a branch reservation must be installed or dropped"]
pub(crate) struct ProductBranchRegistryReservation {
    registry: ProductBranchRegistry,
    owner: RuntimeWorldOwnerIdentity,
    armed: bool,
}

impl std::fmt::Debug for ProductBranchRegistryReservation {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ProductBranchRegistryReservation")
            .field("owner", &self.owner)
            .field("armed", &self.armed)
            .finish_non_exhaustive()
    }
}

impl Drop for ProductBranchRegistryReservation {
    fn drop(&mut self) {
        if !self.armed {
            return;
        }
        let mut state = self
            .registry
            .state
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        state.reserved_branches = state
            .reserved_branches
            .checked_sub(1)
            .expect("a live branch reservation owns one slot");
        self.armed = false;
    }
}

impl ProductBranchRegistry {
    pub(crate) fn new(
        owner: RuntimeWorldOwnerIdentity,
        maximum_branches: crate::budget::RuntimeWorldBudgetLimit,
    ) -> Self {
        Self {
            state: Arc::new(Mutex::new(ProductBranchRegistryState {
                owner,
                maximum_branches: maximum_branches.get(),
                reserved_branches: 0,
                root: None,
            })),
        }
    }

    pub(crate) fn reserve_root(
        &self,
        owner: RuntimeWorldOwnerIdentity,
    ) -> Result<ProductBranchRegistryReservation, ProductBranchRegistryDenial> {
        let mut state = self.state.lock().unwrap_or_else(|error| error.into_inner());
        if owner != state.owner {
            return Err(ProductBranchRegistryDenial::ForeignOwner);
        }
        if state.root.is_some() {
            return Err(ProductBranchRegistryDenial::AlreadyInstalled);
        }
        if state.reserved_branches >= state.maximum_branches {
            return Err(ProductBranchRegistryDenial::CapacityExhausted);
        }
        state.reserved_branches += 1;
        Ok(ProductBranchRegistryReservation {
            registry: self.clone(),
            owner,
            armed: true,
        })
    }

    pub(crate) fn root_cell(&self) -> Option<ProductBranchReferenceCell> {
        self.state
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .root
            .as_ref()
            .map(|entry| entry.cell.clone())
    }

    pub(crate) fn root_snapshot(&self) -> Option<ProductBranchReferenceSnapshot> {
        self.state
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .root
            .as_ref()
            .map(|entry| entry.cell.atomic_snapshot())
    }

    pub(crate) fn root_branch(&self) -> Option<ProductBranchIdentity> {
        self.state
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .root
            .as_ref()
            .map(|entry| entry.branch.clone())
    }

    pub(crate) fn root_name(&self) -> Option<ProductBranchName> {
        self.state
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .root
            .as_ref()
            .map(|entry| entry.name.clone())
    }
}

impl ProductBranchRegistryReservation {
    pub(crate) fn install_root(
        mut self,
        name: ProductBranchName,
        branch: ProductBranchIdentity,
        lifecycle: ProductBranchLifecycleIncarnation,
        cell: ProductBranchReferenceCell,
    ) -> Result<(), (Self, ProductBranchRegistryDenial)> {
        if branch.owner_identity() != self.owner || lifecycle.owner_identity() != self.owner {
            return Err((self, ProductBranchRegistryDenial::IdentityMismatch));
        }
        let result = {
            let mut state = self
                .registry
                .state
                .lock()
                .unwrap_or_else(|error| error.into_inner());
            if state.root.is_some() {
                Err(ProductBranchRegistryDenial::AlreadyInstalled)
            } else if state.reserved_branches.checked_sub(1).is_none() {
                Err(ProductBranchRegistryDenial::ReservationMissing)
            } else {
                state.reserved_branches -= 1;
                state.root = Some(ProductBranchRegistryEntry {
                    name,
                    branch,
                    lifecycle,
                    cell,
                });
                Ok(())
            }
        };
        if let Err(denial) = result {
            return Err((self, denial));
        }
        self.armed = false;
        Ok(())
    }
}
