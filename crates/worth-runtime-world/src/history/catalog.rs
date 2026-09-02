mod denial;
mod entry;
mod reservation;
mod support;
mod traversal;

use std::collections::{BTreeMap, BTreeSet};
use std::num::NonZeroUsize;
use std::sync::{Arc, Mutex};

use crate::budget::RuntimeWorldBudgetLimit;
use crate::identity::{CompositeCommitIdentity, RuntimeWorldOwnerIdentity};

use super::reclamation::{
    CompositeHistoryReclamationRequest, HistoryReclamationDenial, HistoryReclamationOutcome,
};
use super::{CompositeCommitParent, CompositeRuntimeWorldCommit};

pub(crate) use denial::CompositeHistoryCatalogDenial;
pub(crate) use entry::CompositeHistoryCatalogEntry;
pub(crate) use reservation::ReservedCompositeHistorySlot;
use support::{
    install_entry, lock_state, ordinary_parent_identity, protected_ancestry, release_reservation,
    remove_child_index, validate_candidate, validate_owner, validate_parent_for_reservation,
};
pub(crate) use traversal::CompositeHistoryTraversal;

/// Installed limits consumed by the immutable history owner.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct RuntimeWorldHistoryCatalogContract {
    maximum_commits: RuntimeWorldBudgetLimit,
    maximum_metadata_bytes: RuntimeWorldBudgetLimit,
}

impl RuntimeWorldHistoryCatalogContract {
    pub(crate) const fn installed(
        maximum_commits: RuntimeWorldBudgetLimit,
        maximum_metadata_bytes: RuntimeWorldBudgetLimit,
    ) -> Self {
        Self {
            maximum_commits,
            maximum_metadata_bytes,
        }
    }

    pub(crate) const fn maximum_commits(self) -> RuntimeWorldBudgetLimit {
        self.maximum_commits
    }

    pub(crate) const fn maximum_metadata_bytes(self) -> RuntimeWorldBudgetLimit {
        self.maximum_metadata_bytes
    }
}

#[cfg(test)]
#[path = "catalog_tests.rs"]
mod tests;

/// Immutable single-parent history owned by one Runtime World owner.
#[derive(Debug, Clone)]
pub(crate) struct CompositeHistoryCatalog {
    state: Arc<Mutex<CompositeHistoryCatalogState>>,
}

#[derive(Debug)]
pub(super) struct CompositeHistoryCatalogState {
    owner: RuntimeWorldOwnerIdentity,
    limits: RuntimeWorldHistoryCatalogContract,
    entries: BTreeMap<CompositeCommitIdentity, CompositeHistoryCatalogEntry>,
    children: BTreeMap<CompositeCommitIdentity, BTreeSet<CompositeCommitIdentity>>,
    reservations: BTreeMap<CompositeCommitIdentity, reservation::ReservedHistoryMetadata>,
    metadata_bytes: usize,
    reserved_metadata_bytes: usize,
    root: Option<CompositeCommitIdentity>,
    root_reserved: bool,
    root_ever_installed: bool,
    lookup_count: u64,
}

impl CompositeHistoryCatalog {
    pub(crate) fn new(
        owner: RuntimeWorldOwnerIdentity,
        contract: RuntimeWorldHistoryCatalogContract,
    ) -> Self {
        Self {
            state: Arc::new(Mutex::new(CompositeHistoryCatalogState {
                owner,
                limits: contract,
                entries: BTreeMap::new(),
                children: BTreeMap::new(),
                reservations: BTreeMap::new(),
                metadata_bytes: 0,
                reserved_metadata_bytes: 0,
                root: None,
                root_reserved: false,
                root_ever_installed: false,
                lookup_count: 0,
            })),
        }
    }

    pub(crate) fn reserve(
        &self,
        identity: CompositeCommitIdentity,
        parent: CompositeCommitParent,
        metadata_bytes: usize,
    ) -> Result<ReservedCompositeHistorySlot, CompositeHistoryCatalogDenial> {
        let mut state = lock_state(&self.state);
        validate_owner(&state, identity.owner_identity())?;
        validate_parent_for_reservation(&state, &parent)?;
        if state.entries.contains_key(&identity) || state.reservations.contains_key(&identity) {
            return Err(CompositeHistoryCatalogDenial::DuplicateCommit);
        }
        if matches!(parent, CompositeCommitParent::Root)
            && (state.root.is_some() || state.root_reserved || state.root_ever_installed)
        {
            return Err(CompositeHistoryCatalogDenial::RootAlreadyInstalled);
        }
        let population = state
            .entries
            .len()
            .checked_add(state.reservations.len())
            .expect("catalog population cannot exceed addressable memory");
        if population >= state.limits.maximum_commits().get() {
            return Err(CompositeHistoryCatalogDenial::CommitCapacityExhausted {
                maximum: state.limits.maximum_commits().get(),
            });
        }
        let used_metadata = state
            .metadata_bytes
            .checked_add(state.reserved_metadata_bytes)
            .ok_or(CompositeHistoryCatalogDenial::MetadataSizeOverflow {
                requested: metadata_bytes,
            })?;
        let combined_metadata = used_metadata.checked_add(metadata_bytes).ok_or(
            CompositeHistoryCatalogDenial::MetadataSizeOverflow {
                requested: metadata_bytes,
            },
        )?;
        if combined_metadata > state.limits.maximum_metadata_bytes().get() {
            return Err(CompositeHistoryCatalogDenial::MetadataCapacityExhausted {
                maximum: state.limits.maximum_metadata_bytes().get(),
                used: used_metadata,
                requested: metadata_bytes,
            });
        }
        state.reserved_metadata_bytes =
            state
                .reserved_metadata_bytes
                .checked_add(metadata_bytes)
                .ok_or(CompositeHistoryCatalogDenial::MetadataSizeOverflow {
                    requested: metadata_bytes,
                })?;
        if matches!(parent, CompositeCommitParent::Root) {
            state.root_reserved = true;
        }
        state.reservations.insert(
            identity.clone(),
            reservation::ReservedHistoryMetadata {
                parent: parent.clone(),
                metadata_bytes,
            },
        );
        Ok(ReservedCompositeHistorySlot::new(
            Arc::clone(&self.state),
            identity,
            parent,
            metadata_bytes,
        ))
    }

    pub(crate) fn append(
        &self,
        commit: Arc<CompositeRuntimeWorldCommit>,
    ) -> Result<CompositeHistoryCatalogEntry, CompositeHistoryCatalogDenial> {
        let reservation = self.reserve(
            commit.identity().clone(),
            commit.parent().clone(),
            commit.metadata_bytes(),
        )?;
        reservation.install(commit)
    }

    pub(crate) fn lookup(
        &self,
        identity: &CompositeCommitIdentity,
    ) -> Option<Arc<CompositeRuntimeWorldCommit>> {
        let mut state = lock_state(&self.state);
        state.lookup_count = state.lookup_count.saturating_add(1);
        state
            .entries
            .get(identity)
            .map(|entry| Arc::clone(&entry.commit))
    }

    pub(crate) fn root(&self) -> Option<CompositeCommitIdentity> {
        lock_state(&self.state).root.clone()
    }

    pub(crate) fn len(&self) -> usize {
        lock_state(&self.state).entries.len()
    }

    pub(crate) fn reserved_len(&self) -> usize {
        lock_state(&self.state).reservations.len()
    }

    pub(crate) fn metadata_bytes(&self) -> usize {
        lock_state(&self.state).metadata_bytes
    }

    pub(crate) fn reserved_metadata_bytes(&self) -> usize {
        lock_state(&self.state).reserved_metadata_bytes
    }

    pub(crate) fn lookup_count(&self) -> u64 {
        lock_state(&self.state).lookup_count
    }

    /// Walk one parent chain up to an explicit caller bound.
    pub(crate) fn trace_ancestry(
        &self,
        start: CompositeCommitIdentity,
        maximum_commits: NonZeroUsize,
    ) -> Result<CompositeHistoryTraversal, CompositeHistoryCatalogDenial> {
        let state = lock_state(&self.state);
        validate_owner(&state, start.owner_identity())?;
        let mut current = start;
        let mut commits = Vec::with_capacity(maximum_commits.get().min(state.entries.len()));
        for _ in 0..maximum_commits.get() {
            let entry = state
                .entries
                .get(&current)
                .ok_or_else(|| CompositeHistoryCatalogDenial::MissingParent(current.clone()))?;
            commits.push(Arc::clone(&entry.commit));
            let Some(parent) = ordinary_parent_identity(entry.commit.parent()) else {
                return Ok(CompositeHistoryTraversal {
                    commits,
                    next_parent: None,
                });
            };
            current = parent;
        }
        Ok(CompositeHistoryTraversal {
            commits,
            next_parent: Some(current),
        })
    }

    pub(crate) fn reclaim_batch(
        &self,
        request: CompositeHistoryReclamationRequest,
    ) -> Result<HistoryReclamationOutcome, HistoryReclamationDenial> {
        let mut state = lock_state(&self.state);
        validate_owner(&state, request.owner()).map_err(HistoryReclamationDenial::Catalog)?;
        let protected = protected_ancestry(&state, request.protected_commits())?;
        let candidates = request
            .candidate_commits()
            .iter()
            .take(request.maximum_reclaims());
        let mut distinct_candidates = BTreeSet::new();
        for candidate in candidates.clone() {
            validate_candidate(&state, candidate)?;
            if !distinct_candidates.insert(candidate.clone()) {
                return Err(HistoryReclamationDenial::DuplicateCandidate(
                    candidate.clone(),
                ));
            }
        }

        let mut outcome = HistoryReclamationOutcome::new(request.maximum_reclaims());
        for candidate in candidates {
            outcome.examined_one();
            let reachable = protected.contains(candidate);
            if !super::reclamation::HistoryReclamationEligibility::new(
                request.age_ticks(),
                reachable,
            )
            .is_eligible()
            {
                if reachable {
                    outcome.record_skipped_protected();
                } else {
                    outcome.record_skipped_too_young();
                }
                continue;
            }
            if state
                .children
                .get(candidate)
                .is_some_and(|children| !children.is_empty())
            {
                outcome.record_skipped_with_children();
                continue;
            }
            let (metadata_bytes, parent) = {
                let entry = state
                    .entries
                    .get(candidate)
                    .expect("candidate was validated before reclamation");
                (entry.metadata_bytes(), entry.commit.parent().clone())
            };
            state.entries.remove(candidate);
            state.children.remove(candidate);
            remove_child_index(&mut state, &parent, candidate);
            if matches!(parent, CompositeCommitParent::Root)
                && state.root.as_ref() == Some(candidate)
            {
                state.root = None;
            }
            state.metadata_bytes -= metadata_bytes;
            outcome.reclaimed_one(candidate.clone(), metadata_bytes);
        }
        Ok(outcome)
    }
}
