use crate::identity::{ProductBranchIdentity, ProductBranchLifecycleIncarnation};

use super::{
    ProductBranchName, ProductBranchReferenceCell, ProductBranchRegistry,
    ProductBranchRegistryDenial, ProductBranchRegistryEntry, ProductBranchRegistryState,
};

#[must_use = "a branch reservation must be installed or dropped"]
pub(crate) struct ProductBranchRegistryReservation {
    pub(super) registry: ProductBranchRegistry,
    owner: crate::identity::RuntimeWorldOwnerIdentity,
    name: Option<ProductBranchName>,
    root: bool,
    armed: bool,
}

impl std::fmt::Debug for ProductBranchRegistryReservation {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ProductBranchRegistryReservation")
            .field("owner", &self.owner)
            .field("name", &self.name)
            .field("root", &self.root)
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
        if let Some(name) = &self.name {
            state.reserved_names.remove(name.as_str());
        }
        state.reserved_branches = state
            .reserved_branches
            .checked_sub(1)
            .expect("a live branch reservation owns one slot");
        self.armed = false;
    }
}

impl ProductBranchRegistryReservation {
    pub(super) fn root(
        registry: ProductBranchRegistry,
        owner: crate::identity::RuntimeWorldOwnerIdentity,
    ) -> Self {
        Self {
            registry,
            owner,
            name: None,
            root: true,
            armed: true,
        }
    }

    pub(super) fn named(
        registry: ProductBranchRegistry,
        owner: crate::identity::RuntimeWorldOwnerIdentity,
        name: ProductBranchName,
    ) -> Self {
        Self {
            registry,
            owner,
            name: Some(name),
            root: false,
            armed: true,
        }
    }

    pub(crate) fn install_root(
        self,
        name: ProductBranchName,
        branch: ProductBranchIdentity,
        lifecycle: ProductBranchLifecycleIncarnation,
        cell: ProductBranchReferenceCell,
    ) -> Result<(), (Self, ProductBranchRegistryDenial)> {
        self.install_reserved(name, branch, lifecycle, cell, true)
    }

    pub(crate) fn install(
        self,
        branch: ProductBranchIdentity,
        lifecycle: ProductBranchLifecycleIncarnation,
        cell: ProductBranchReferenceCell,
    ) -> Result<(), (Self, ProductBranchRegistryDenial)> {
        let Some(name) = self.name.clone() else {
            return Err((self, ProductBranchRegistryDenial::ReservationMissing));
        };
        self.install_reserved(name, branch, lifecycle, cell, false)
    }

    fn install_reserved(
        mut self,
        name: ProductBranchName,
        branch: ProductBranchIdentity,
        lifecycle: ProductBranchLifecycleIncarnation,
        cell: ProductBranchReferenceCell,
        root: bool,
    ) -> Result<(), (Self, ProductBranchRegistryDenial)> {
        if self.root != root {
            return Err((self, ProductBranchRegistryDenial::ReservationMissing));
        }
        if branch.owner_identity() != self.owner || lifecycle.owner_identity() != self.owner {
            return Err((self, ProductBranchRegistryDenial::IdentityMismatch));
        }
        if !root && self.name.as_ref() != Some(&name) {
            return Err((self, ProductBranchRegistryDenial::IdentityMismatch));
        }
        let snapshot = cell.atomic_snapshot();
        if snapshot.owner_identity() != self.owner
            || snapshot.branch_identity() != &branch
            || snapshot.lifecycle_incarnation() != lifecycle
        {
            return Err((self, ProductBranchRegistryDenial::IdentityMismatch));
        }
        let basis = snapshot.basis().identity().clone();
        let commit = snapshot.selected_commit().clone();
        let installed_root = branch.clone();
        let result = {
            let mut state = self
                .registry
                .state
                .lock()
                .unwrap_or_else(|error| error.into_inner());
            if root && state.root.is_some() {
                Err(ProductBranchRegistryDenial::AlreadyInstalled)
            } else if state.reserved_branches.checked_sub(1).is_none() {
                Err(ProductBranchRegistryDenial::ReservationMissing)
            } else if state.entries.contains_key(&branch) {
                Err(ProductBranchRegistryDenial::BranchAlreadyInstalled)
            } else if state.lifecycles.contains(&lifecycle) {
                Err(ProductBranchRegistryDenial::LifecycleAlreadyInstalled)
            } else if root && state.reserved_names.contains(name.as_str()) {
                Err(ProductBranchRegistryDenial::NameAlreadyReserved)
            } else if state.names.contains_key(name.as_str()) {
                Err(ProductBranchRegistryDenial::NameAlreadyInstalled)
            } else if !self.root && !state.reserved_names.contains(name.as_str()) {
                Err(ProductBranchRegistryDenial::ReservationMissing)
            } else if basis.owner_identity() != state.owner
                || commit.owner_identity() != state.owner
            {
                Err(ProductBranchRegistryDenial::IdentityMismatch)
            } else {
                state.reserved_branches -= 1;
                state.reserved_names.remove(name.as_str());
                let entry = ProductBranchRegistryEntry {
                    name: name.clone(),
                    lifecycle,
                    basis: basis.clone(),
                    commit: commit.clone(),
                    cell,
                };
                assert!(state.entries.insert(branch.clone(), entry).is_none());
                assert!(state
                    .names
                    .insert(name.as_str().to_owned(), branch)
                    .is_none());
                assert!(state.lifecycles.insert(lifecycle));
                let candidates = state.basis_commits.entry(basis).or_default();
                let count = candidates.entry(commit).or_insert(0);
                *count = count
                    .checked_add(1)
                    .expect("live branch count cannot overflow a bounded registry");
                if root {
                    state.root = Some(installed_root);
                }
                Ok(())
            }
        };
        if let Err(denial) = result {
            return Err((self, denial));
        }
        self.armed = false;
        self.name = None;
        Ok(())
    }
}

pub(super) fn reserve_slot(
    state: &mut ProductBranchRegistryState,
) -> Result<(), ProductBranchRegistryDenial> {
    if state.entries.len().saturating_add(state.reserved_branches) >= state.maximum_branches {
        return Err(ProductBranchRegistryDenial::CapacityExhausted);
    }
    state.reserved_branches = state
        .reserved_branches
        .checked_add(1)
        .ok_or(ProductBranchRegistryDenial::CapacityExhausted)?;
    Ok(())
}
