mod reservation;

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

use crate::identity::{ProductBranchIdentity, ProductBranchIncarnation, RuntimeWorldOwnerIdentity};

use super::{
    ProductBranchName, ProductBranchObservation, ProductBranchReferenceCell,
    ProductBranchReferenceSnapshot,
};

pub(crate) use reservation::{
    ProductBranchRegistryReservation, ProductBranchSourceInstallDenial,
    ProductBranchSourceInstallFailure,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ProductBranchRegistryDenial {
    ForeignOwner,
    CapacityExhausted,
    AlreadyInstalled,
    ReservationMissing,
    IdentityMismatch,
    NameAlreadyReserved,
    NameAlreadyInstalled,
    BranchAlreadyInstalled,
    LifecycleAlreadyInstalled,
    AlreadyRetired,
    UnknownBranch,
}

#[derive(Debug)]
struct ProductBranchRegistryState {
    owner: RuntimeWorldOwnerIdentity,
    maximum_branches: usize,
    reserved_branches: usize,
    reserved_names: HashSet<String>,
    /// Keyed by the owner-plus-normalized-name identity, so the installed name
    /// index and the branch index are one map rather than two authorities.
    entries: HashMap<ProductBranchIdentity, ProductBranchRegistryEntry>,
    lifecycles: HashSet<ProductBranchIncarnation>,
    /// Identities that held an installed incarnation and no longer do. Under a
    /// name-keyed identity this, not an ordinal high-water mark, is what
    /// separates "retired" from "never installed". It is disjoint from
    /// `entries`: recreating a name takes its identity back out.
    ///
    /// INT-BLOCK-1. This set is bounded by the number of distinct names ever
    /// retired, not by the branch budget, so it grows over a long-lived owner.
    /// It cannot be bounded under the frozen retirement seam: `retire` is given
    /// only a name-keyed identity, and separating "retired" from "never
    /// installed" from a name alone requires remembering the names. The seam
    /// that bounds it is
    /// `retire_product_branch(owner, branch, incarnation)` — with the
    /// occurrence named, `AlreadyRetired` is derivable from the bounded
    /// `lifecycles` set and the owner's monotonic incarnation cursor, and this
    /// field is deleted rather than replaced.
    retired: HashSet<ProductBranchIdentity>,
    root: Option<ProductBranchIdentity>,
}

#[derive(Debug)]
struct ProductBranchRegistryEntry {
    lifecycle: ProductBranchIncarnation,
    cell: ProductBranchReferenceCell,
}

/// The only managed owner registry for Runtime World product branches.
///
/// The registry owns only product-reference cells and the name, incarnation,
/// and root indexes over them. It keeps no copy of a head: the cell is the
/// one authority for what a branch carries, and exact reuse resolves the
/// commit from the observation a cell issued. It never owns a component
/// branch and never calls a component owner while its mutex is held.
#[derive(Debug, Clone)]
pub(crate) struct ProductBranchRegistry {
    state: Arc<Mutex<ProductBranchRegistryState>>,
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
                reserved_names: HashSet::new(),
                entries: HashMap::new(),
                lifecycles: HashSet::new(),
                retired: HashSet::new(),
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
        reservation::reserve_slot(&mut state)?;
        Ok(ProductBranchRegistryReservation::root(self.clone(), owner))
    }

    pub(crate) fn reserve_branch(
        &self,
        owner: RuntimeWorldOwnerIdentity,
        name: ProductBranchName,
    ) -> Result<ProductBranchRegistryReservation, ProductBranchRegistryDenial> {
        let mut state = self.state.lock().unwrap_or_else(|error| error.into_inner());
        if owner != state.owner {
            return Err(ProductBranchRegistryDenial::ForeignOwner);
        }
        if state
            .entries
            .contains_key(&ProductBranchIdentity::issued(owner, name.clone()))
        {
            return Err(ProductBranchRegistryDenial::NameAlreadyInstalled);
        }
        if !state.reserved_names.insert(name.as_str().to_owned()) {
            return Err(ProductBranchRegistryDenial::NameAlreadyReserved);
        }
        if let Err(denial) = reservation::reserve_slot(&mut state) {
            state.reserved_names.remove(name.as_str());
            return Err(denial);
        }
        Ok(ProductBranchRegistryReservation::named(
            self.clone(),
            owner,
            name,
        ))
    }

    pub(crate) fn root_cell(&self) -> Option<ProductBranchReferenceCell> {
        let state = self.state.lock().unwrap_or_else(|error| error.into_inner());
        state
            .root
            .as_ref()
            .and_then(|branch| state.entries.get(branch))
            .map(|entry| entry.cell.clone())
    }

    pub(crate) fn root_snapshot(&self) -> Option<ProductBranchReferenceSnapshot> {
        let state = self.state.lock().unwrap_or_else(|error| error.into_inner());
        state
            .root
            .as_ref()
            .and_then(|branch| state.entries.get(branch))
            .map(|entry| entry.cell.atomic_snapshot())
    }

    pub(crate) fn root_branch(&self) -> Option<ProductBranchIdentity> {
        self.state
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .root
            .clone()
    }

    pub(crate) fn root_name(&self) -> Option<ProductBranchName> {
        let state = self.state.lock().unwrap_or_else(|error| error.into_inner());
        state
            .root
            .as_ref()
            .filter(|branch| state.entries.contains_key(*branch))
            .map(|branch| branch.name().clone())
    }

    pub(crate) fn branch_cell(
        &self,
        branch: &ProductBranchIdentity,
    ) -> Option<ProductBranchReferenceCell> {
        self.state
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .entries
            .get(branch)
            .map(|entry| entry.cell.clone())
    }

    pub(crate) fn branch_count(&self) -> usize {
        self.state
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .entries
            .len()
    }

    pub(crate) fn reserved_branch_count(&self) -> usize {
        self.state
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .reserved_branches
    }

    /// Release one installed product reference. The retired incarnation is
    /// returned with the cell because it, not the name-keyed identity, is what
    /// the custody records of this occurrence are keyed by.
    pub(crate) fn retire(
        &self,
        owner: RuntimeWorldOwnerIdentity,
        branch: &ProductBranchIdentity,
    ) -> Result<(ProductBranchReferenceCell, ProductBranchIncarnation), ProductBranchRegistryDenial>
    {
        let mut state = self.state.lock().unwrap_or_else(|error| error.into_inner());
        if owner != state.owner || branch.owner_identity() != state.owner {
            return Err(ProductBranchRegistryDenial::ForeignOwner);
        }
        let entry = match release_installed_entry(&mut state, branch) {
            Some(entry) => entry,
            None => {
                return Err(if state.retired.contains(branch) {
                    ProductBranchRegistryDenial::AlreadyRetired
                } else {
                    ProductBranchRegistryDenial::UnknownBranch
                });
            }
        };
        Ok((entry.cell, entry.lifecycle))
    }

    /// Release every installed non-root product reference and report how many
    /// product-head references that released. Close owns this: the registry is
    /// the only holder of an installed branch's reference cell, so the count is
    /// what close actually let go of, not what a budget says could exist.
    ///
    /// The root is deliberately excluded. It is the world's own reference
    /// rather than a branch a caller created, and the close report describes
    /// created branches.
    pub(crate) fn release_non_root_branches(&self) -> usize {
        let mut state = self.state.lock().unwrap_or_else(|error| error.into_inner());
        let root = state.root.clone();
        let branches: Vec<ProductBranchIdentity> = state
            .entries
            .keys()
            .filter(|branch| root.as_ref() != Some(*branch))
            .cloned()
            .collect();
        for branch in &branches {
            release_installed_entry(&mut state, branch)
                .expect("a branch just read out of the index is still installed");
        }
        branches.len()
    }
}

/// Take one installed occurrence out of every index that names it. Retirement
/// and close both release a product reference, and both release exactly this
/// much: one authority, so neither can leave an index the other clears.
fn release_installed_entry(
    state: &mut ProductBranchRegistryState,
    branch: &ProductBranchIdentity,
) -> Option<ProductBranchRegistryEntry> {
    let entry = state.entries.remove(branch)?;
    state.lifecycles.remove(&entry.lifecycle);
    state.retired.insert(branch.clone());
    if state.root.as_ref() == Some(branch) {
        state.root = None;
    }
    Some(entry)
}

#[cfg(test)]
#[path = "registry_tests.rs"]
mod tests;
