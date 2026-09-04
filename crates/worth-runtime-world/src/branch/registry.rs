mod reservation;

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

use crate::basis::AdmittedCompositeRuntimeWorldBasis;
use crate::identity::{
    CompositeBasisKey, CompositeCommitIdentity, ProductBranchIdentity, ProductBranchIncarnation,
    RuntimeWorldOwnerIdentity,
};

use super::{ProductBranchName, ProductBranchReferenceCell, ProductBranchReferenceSnapshot};

pub(crate) use reservation::ProductBranchRegistryReservation;

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
    retired: HashSet<ProductBranchIdentity>,
    basis_commits: HashMap<CompositeBasisKey, HashMap<CompositeCommitIdentity, usize>>,
    root: Option<ProductBranchIdentity>,
}

#[derive(Debug)]
struct ProductBranchRegistryEntry {
    lifecycle: ProductBranchIncarnation,
    basis: CompositeBasisKey,
    commit: CompositeCommitIdentity,
    cell: ProductBranchReferenceCell,
}

/// The only managed owner registry for Runtime World product branches.
///
/// The registry owns only product-reference cells and their local indexes. It
/// never owns a component branch and never calls a component owner while its
/// mutex is held.
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
                basis_commits: HashMap::new(),
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

    /// Resolve a basis only when one installed product branch occurrence
    /// supplies an unambiguous exact commit. A basis with multiple commit
    /// occurrences is deliberately denied rather than selecting a heuristic.
    pub(crate) fn commit_for_basis(
        &self,
        basis: &AdmittedCompositeRuntimeWorldBasis,
    ) -> Option<CompositeCommitIdentity> {
        let state = self.state.lock().unwrap_or_else(|error| error.into_inner());
        if basis.owner_identity() != state.owner {
            return None;
        }
        let candidates = state.basis_commits.get(basis.identity())?;
        (candidates.len() == 1).then(|| {
            candidates
                .keys()
                .next()
                .expect("one candidate exists")
                .clone()
        })
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
        let entry = match state.entries.remove(branch) {
            Some(entry) => entry,
            None => {
                return Err(if state.retired.contains(branch) {
                    ProductBranchRegistryDenial::AlreadyRetired
                } else {
                    ProductBranchRegistryDenial::UnknownBranch
                });
            }
        };
        state.lifecycles.remove(&entry.lifecycle);
        state.retired.insert(branch.clone());
        remove_basis_candidate(&mut state, &entry.basis, &entry.commit);
        if state.root.as_ref() == Some(branch) {
            state.root = None;
        }
        Ok((entry.cell, entry.lifecycle))
    }
}

fn remove_basis_candidate(
    state: &mut ProductBranchRegistryState,
    basis: &CompositeBasisKey,
    commit: &CompositeCommitIdentity,
) {
    let mut remove_basis = false;
    if let Some(candidates) = state.basis_commits.get_mut(basis) {
        if let Some(count) = candidates.get_mut(commit) {
            if *count <= 1 {
                candidates.remove(commit);
            } else {
                *count -= 1;
            }
        }
        remove_basis = candidates.is_empty();
    }
    if remove_basis {
        state.basis_commits.remove(basis);
    }
}

#[cfg(test)]
#[path = "registry_tests.rs"]
mod tests;
