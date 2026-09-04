use crate::identity::{ProductBranchIdentity, ProductBranchIncarnation};

use super::{
    ProductBranchName, ProductBranchObservation, ProductBranchReferenceCell,
    ProductBranchReferenceSnapshot, ProductBranchRegistry, ProductBranchRegistryDenial,
    ProductBranchRegistryEntry, ProductBranchRegistryState,
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

/// Why an installation from a source observation was refused. The registry's
/// own refusals pass through; the two source refusals are the ones only an
/// installation made under the source head's guard can name.
#[derive(Debug)]
pub(crate) enum ProductBranchSourceInstallDenial {
    Registry(ProductBranchRegistryDenial),
    /// The source branch holds no installed occurrence: it was retired and
    /// not recreated.
    SourceRetired,
    /// The source branch carries a head other than the observed one. The
    /// head it carries comes back so the caller can name what displaced it.
    SourceDisplaced(ProductBranchReferenceSnapshot),
}

/// A refused source-guarded installation hands back the reservation and the
/// cell it was given. Whether the cell's custody is released or retained is
/// the caller's decision, not the registry's.
#[derive(Debug)]
pub(crate) struct ProductBranchSourceInstallFailure {
    pub(crate) reservation: ProductBranchRegistryReservation,
    pub(crate) denial: ProductBranchSourceInstallDenial,
    pub(crate) cell: ProductBranchReferenceCell,
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

    /// Install the root occurrence. Every non-root occurrence is created
    /// from a source and goes through `install_from_source`; there is no
    /// named install that bypasses the source guard.
    pub(crate) fn install_root(
        mut self,
        name: ProductBranchName,
        branch: ProductBranchIdentity,
        lifecycle: ProductBranchIncarnation,
        cell: ProductBranchReferenceCell,
    ) -> Result<(), (Self, ProductBranchRegistryDenial)> {
        let installed = InstalledBranch {
            name,
            branch,
            lifecycle,
            cell,
        };
        if let Err(denial) = self.admits_installation(&installed, true) {
            return Err((self, denial));
        }
        let result = {
            let mut state = self
                .registry
                .state
                .lock()
                .unwrap_or_else(|error| error.into_inner());
            insert_installed(&mut state, installed, true)
        };
        if let Err((denial, _uninstalled)) = result {
            return Err((self, denial));
        }
        self.armed = false;
        self.name = None;
        Ok(())
    }

    /// Install a non-root occurrence created from `source`, and only while
    /// the source branch still carries the head `source` observed. The source
    /// cell's read guard is taken under the registry lock and held across the
    /// insertion, so no publication can move the source between the
    /// currentness check and the installed child: the check and the act are
    /// one step. The lock order, registry then cell, is the one
    /// `root_snapshot` already uses; publication and observation take the
    /// cell lock alone and never the registry's.
    pub(crate) fn install_from_source(
        mut self,
        source: &ProductBranchObservation,
        branch: ProductBranchIdentity,
        lifecycle: ProductBranchIncarnation,
        cell: ProductBranchReferenceCell,
    ) -> Result<(), ProductBranchSourceInstallFailure> {
        let Some(name) = self.name.clone() else {
            return Err(ProductBranchSourceInstallFailure {
                reservation: self,
                denial: ProductBranchSourceInstallDenial::Registry(
                    ProductBranchRegistryDenial::ReservationMissing,
                ),
                cell,
            });
        };
        let installed = InstalledBranch {
            name,
            branch,
            lifecycle,
            cell,
        };
        if let Err(denial) = self.admits_installation(&installed, false) {
            return Err(source_install_failure(
                self,
                ProductBranchSourceInstallDenial::Registry(denial),
                installed,
            ));
        }
        let result = {
            let mut state = self
                .registry
                .state
                .lock()
                .unwrap_or_else(|error| error.into_inner());
            insert_installed_from_source(&mut state, source, installed)
        };
        match result {
            Ok(()) => {
                self.armed = false;
                self.name = None;
                Ok(())
            }
            Err((denial, installed)) => Err(source_install_failure(self, denial, installed)),
        }
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

fn source_install_failure(
    reservation: ProductBranchRegistryReservation,
    denial: ProductBranchSourceInstallDenial,
    installed: InstalledBranch,
) -> ProductBranchSourceInstallFailure {
    ProductBranchSourceInstallFailure {
        reservation,
        denial,
        cell: installed.cell,
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

/// Insert under the registry lock while the source cell provably still
/// carries the observed head. A source with no installed occurrence is
/// retired; one whose cell carries another head, or another occurrence of the
/// same name, is displaced. Either way the occurrence comes back uninstalled.
fn insert_installed_from_source(
    state: &mut ProductBranchRegistryState,
    source: &ProductBranchObservation,
    installed: InstalledBranch,
) -> Result<(), (ProductBranchSourceInstallDenial, InstalledBranch)> {
    let Some(source_cell) = state
        .entries
        .get(source.branch_identity())
        .map(|entry| entry.cell.clone())
    else {
        return Err((ProductBranchSourceInstallDenial::SourceRetired, installed));
    };
    match source_cell.while_current(source, installed, |installed| {
        insert_installed(state, installed, false)
    }) {
        Ok(Ok(())) => Ok(()),
        Ok(Err((denial, installed))) => Err((
            ProductBranchSourceInstallDenial::Registry(denial),
            installed,
        )),
        Err((observed, installed)) => Err((
            ProductBranchSourceInstallDenial::SourceDisplaced(observed),
            installed,
        )),
    }
}

/// Take the reserved slot and install the occurrence, under the registry lock.
/// A root installation consumes the root slot rather than a reserved name. A
/// refused occurrence comes back untouched, with its cell.
fn insert_installed(
    state: &mut ProductBranchRegistryState,
    installed: InstalledBranch,
    root: bool,
) -> Result<(), (ProductBranchRegistryDenial, InstalledBranch)> {
    if let Err(denial) = admits_insertion(state, &installed, root) {
        return Err((denial, installed));
    }
    let InstalledBranch {
        name,
        branch,
        lifecycle,
        cell,
    } = installed;
    state.reserved_branches -= 1;
    state.reserved_names.remove(name.as_str());
    let entry = ProductBranchRegistryEntry { lifecycle, cell };
    // A recreated name is no longer retired: `retired` and `entries` are
    // disjoint, so the two together classify every identity exactly once.
    state.retired.remove(&branch);
    assert!(state.entries.insert(branch.clone(), entry).is_none());
    assert!(state.lifecycles.insert(lifecycle));
    if root {
        state.root = Some(branch);
    }
    Ok(())
}

fn admits_insertion(
    state: &ProductBranchRegistryState,
    installed: &InstalledBranch,
    root: bool,
) -> Result<(), ProductBranchRegistryDenial> {
    if root && state.root.is_some() {
        return Err(ProductBranchRegistryDenial::AlreadyInstalled);
    }
    if state.reserved_branches.checked_sub(1).is_none() {
        return Err(ProductBranchRegistryDenial::ReservationMissing);
    }
    if state.entries.contains_key(&installed.branch) {
        return Err(ProductBranchRegistryDenial::BranchAlreadyInstalled);
    }
    if state.lifecycles.contains(&installed.lifecycle) {
        return Err(ProductBranchRegistryDenial::LifecycleAlreadyInstalled);
    }
    let name = installed.name.as_str();
    if root && state.reserved_names.contains(name) {
        return Err(ProductBranchRegistryDenial::NameAlreadyReserved);
    }
    if !root && !state.reserved_names.contains(name) {
        return Err(ProductBranchRegistryDenial::ReservationMissing);
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
