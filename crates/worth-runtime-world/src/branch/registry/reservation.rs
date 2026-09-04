use crate::identity::{ProductBranchIdentity, ProductBranchIncarnation};

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
        lifecycle: ProductBranchIncarnation,
        cell: ProductBranchReferenceCell,
    ) -> Result<(), (Self, ProductBranchRegistryDenial)> {
        self.install_reserved(
            InstalledBranch {
                name,
                branch,
                lifecycle,
                cell,
            },
            true,
        )
    }

    pub(crate) fn install(
        self,
        branch: ProductBranchIdentity,
        lifecycle: ProductBranchIncarnation,
        cell: ProductBranchReferenceCell,
    ) -> Result<(), (Self, ProductBranchRegistryDenial)> {
        let Some(name) = self.name.clone() else {
            return Err((self, ProductBranchRegistryDenial::ReservationMissing));
        };
        self.install_reserved(
            InstalledBranch {
                name,
                branch,
                lifecycle,
                cell,
            },
            false,
        )
    }

    fn install_reserved(
        mut self,
        installed: InstalledBranch,
        root: bool,
    ) -> Result<(), (Self, ProductBranchRegistryDenial)> {
        if let Err(denial) = self.admits_installation(&installed, root) {
            return Err((self, denial));
        }
        let result = {
            let mut state = self
                .registry
                .state
                .lock()
                .unwrap_or_else(|error| error.into_inner());
            insert_installed(&mut state, installed, root)
        };
        if let Err(denial) = result {
            return Err((self, denial));
        }
        self.armed = false;
        self.name = None;
        Ok(())
    }

    /// Every identity axis the reservation can settle on its own, before it
    /// takes the registry lock: the posture it was issued for, the owner of
    /// each identity, and the head the cell actually carries.
    fn admits_installation(
        &self,
        installed: &InstalledBranch,
        root: bool,
    ) -> Result<(), ProductBranchRegistryDenial> {
        if self.root != root {
            return Err(ProductBranchRegistryDenial::ReservationMissing);
        }
        if installed.branch.owner_identity() != self.owner
            || installed.lifecycle.owner_identity() != self.owner
        {
            return Err(ProductBranchRegistryDenial::IdentityMismatch);
        }
        if !root && self.name.as_ref() != Some(&installed.name) {
            return Err(ProductBranchRegistryDenial::IdentityMismatch);
        }
        // The identity is the owner plus this exact normalized name, so the
        // installed key and the reserved name cannot disagree.
        if installed.branch.name() != &installed.name {
            return Err(ProductBranchRegistryDenial::IdentityMismatch);
        }
        let snapshot = installed.cell.atomic_snapshot();
        if snapshot.owner_identity() != self.owner
            || snapshot.branch_identity() != &installed.branch
            || snapshot.lifecycle_incarnation() != installed.lifecycle
        {
            return Err(ProductBranchRegistryDenial::IdentityMismatch);
        }
        Ok(())
    }
}

/// One product-branch occurrence as it is handed to the registry: the name it
/// was reserved under, the identity that name issues, the incarnation that
/// distinguishes this occurrence, and the reference cell holding its head.
pub(crate) struct InstalledBranch {
    pub(crate) name: ProductBranchName,
    pub(crate) branch: ProductBranchIdentity,
    pub(crate) lifecycle: ProductBranchIncarnation,
    pub(crate) cell: ProductBranchReferenceCell,
}

/// Take the reserved slot and install the occurrence, under the registry lock.
/// A root installation consumes the root slot rather than a reserved name.
fn insert_installed(
    state: &mut ProductBranchRegistryState,
    installed: InstalledBranch,
    root: bool,
) -> Result<(), ProductBranchRegistryDenial> {
    let InstalledBranch {
        name,
        branch,
        lifecycle,
        cell,
    } = installed;
    let snapshot = cell.atomic_snapshot();
    let basis = snapshot.basis().identity().clone();
    let commit = snapshot.selected_commit().clone();
    if root && state.root.is_some() {
        return Err(ProductBranchRegistryDenial::AlreadyInstalled);
    }
    if state.reserved_branches.checked_sub(1).is_none() {
        return Err(ProductBranchRegistryDenial::ReservationMissing);
    }
    if state.entries.contains_key(&branch) {
        return Err(ProductBranchRegistryDenial::BranchAlreadyInstalled);
    }
    if state.lifecycles.contains(&lifecycle) {
        return Err(ProductBranchRegistryDenial::LifecycleAlreadyInstalled);
    }
    if root && state.reserved_names.contains(name.as_str()) {
        return Err(ProductBranchRegistryDenial::NameAlreadyReserved);
    }
    if !root && !state.reserved_names.contains(name.as_str()) {
        return Err(ProductBranchRegistryDenial::ReservationMissing);
    }
    if basis.owner_identity() != state.owner || commit.owner_identity() != state.owner {
        return Err(ProductBranchRegistryDenial::IdentityMismatch);
    }
    state.reserved_branches -= 1;
    state.reserved_names.remove(name.as_str());
    let entry = ProductBranchRegistryEntry {
        lifecycle,
        basis: basis.clone(),
        commit: commit.clone(),
        cell,
    };
    // A recreated name is no longer retired: `retired` and `entries` are
    // disjoint, so the two together classify every identity exactly once.
    state.retired.remove(&branch);
    assert!(state.entries.insert(branch.clone(), entry).is_none());
    assert!(state.lifecycles.insert(lifecycle));
    let candidates = state.basis_commits.entry(basis).or_default();
    let count = candidates.entry(commit).or_insert(0);
    *count = count
        .checked_add(1)
        .expect("live branch count cannot overflow a bounded registry");
    if root {
        state.root = Some(branch);
    }
    Ok(())
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
