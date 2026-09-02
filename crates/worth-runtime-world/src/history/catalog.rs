mod counters;
mod denial;
mod entry;
mod metadata;
mod reachability;
mod reservation;
mod support;
mod traversal;

use std::collections::BTreeMap;
use std::num::NonZeroUsize;
use std::sync::{Arc, Mutex};

use crate::budget::RuntimeWorldBudgetLimit;
use crate::identity::{CompositeCommitIdentity, RuntimeWorldOwnerIdentity};

use super::reclamation::{
    CompositeHistoryReclamationRequest, HistoryReclamationDenial, HistoryReclamationOutcome,
};
use super::retention::{
    CompositeHistoryProtectionObligation, ExplicitCommitHistoryProtectionObligation,
    HistoryProtectionClass, ProductHeadHistoryProtectionObligation,
};
use super::{CompositeCommitParent, CompositeRuntimeWorldCommit};

pub(crate) use counters::HistoryCatalogCounters;
pub(crate) use denial::CompositeHistoryCatalogDenial;
pub(crate) use entry::CompositeHistoryCatalogEntry;
pub(crate) use metadata::HistoryMetadataLedger;
pub(super) use metadata::HistoryReservationMetadata;
pub(in crate::history) use reachability::{
    lock_index, HistoryReachabilityHandle, HistoryReachabilityIndex,
};
pub(crate) use reservation::ReservedCompositeCommitCapacity;
use support::{
    lock_state, prevalidate_candidate_prefix, remove_installed, validate_owner,
    validate_parent_for_reservation,
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
    reservations: BTreeMap<CompositeCommitIdentity, HistoryReservationMetadata>,
    metadata: HistoryMetadataLedger,
    reachability: HistoryReachabilityHandle,
    counters: counters::HistoryCatalogCountersHandle,
    root: Option<CompositeCommitIdentity>,
    root_reserved: bool,
    root_ever_installed: bool,
}

impl CompositeHistoryCatalog {
    pub(crate) fn new(
        owner: RuntimeWorldOwnerIdentity,
        contract: RuntimeWorldHistoryCatalogContract,
    ) -> Self {
        let counters = counters::new_handle();
        let reachability = Arc::new(Mutex::new(HistoryReachabilityIndex::new(Arc::clone(
            &counters,
        ))));
        Self {
            state: Arc::new(Mutex::new(CompositeHistoryCatalogState {
                owner,
                limits: contract,
                entries: BTreeMap::new(),
                reservations: BTreeMap::new(),
                metadata: HistoryMetadataLedger::default(),
                reachability,
                counters,
                root: None,
                root_reserved: false,
                root_ever_installed: false,
            })),
        }
    }

    pub(crate) fn reserve(
        &self,
        commit: &CompositeRuntimeWorldCommit,
    ) -> Result<ReservedCompositeCommitCapacity, CompositeHistoryCatalogDenial> {
        self.reserve_commit_capacity(commit.identity().clone(), commit.parent().clone())
    }

    /// Reserve immutable commit and metadata capacity before a commit exists.
    /// The owner supplies the identity and exact parent; installation later
    /// accepts only the matching immutable commit.
    pub(crate) fn reserve_commit_capacity(
        &self,
        identity: CompositeCommitIdentity,
        parent: CompositeCommitParent,
    ) -> Result<ReservedCompositeCommitCapacity, CompositeHistoryCatalogDenial> {
        let mut state = lock_state(&self.state);
        validate_owner(&state, identity.owner_identity())?;
        if state.entries.contains_key(&identity) || state.reservations.contains_key(&identity) {
            return Err(CompositeHistoryCatalogDenial::DuplicateCommit);
        }
        if matches!(parent, CompositeCommitParent::Root)
            && (state.root.is_some() || state.root_reserved || state.root_ever_installed)
        {
            return Err(CompositeHistoryCatalogDenial::RootAlreadyInstalled);
        }
        validate_parent_for_reservation(&mut state, &parent)?;
        let maximum_commits = state.limits.maximum_commits().get();
        let installed_commits = state.entries.len();
        let reserved_commits = state.reservations.len();
        if installed_commits >= maximum_commits
            || reserved_commits >= maximum_commits.saturating_sub(installed_commits)
        {
            return Err(CompositeHistoryCatalogDenial::CommitCapacityExhausted {
                maximum: maximum_commits,
            });
        }

        let commit_charge = metadata::HistoryMetadataCharge::for_parent(&parent)
            .map_err(|_| CompositeHistoryCatalogDenial::ArithmeticOverflow)?;
        let reservation_charge = metadata::HistoryReservationCharge::for_parent(&parent)
            .map_err(|_| CompositeHistoryCatalogDenial::ArithmeticOverflow)?;
        let preview = {
            counters::lock_counters(&state.counters).record_metadata_reservation_check();
            state
                .metadata
                .preview_reservation(
                    reservation_charge,
                    commit_charge,
                    state.limits.maximum_metadata_bytes().get(),
                )
                .map_err(|denial| match denial {
                    metadata::HistoryMetadataLedgerDenial::ArithmeticOverflow => {
                        CompositeHistoryCatalogDenial::ArithmeticOverflow
                    }
                    metadata::HistoryMetadataLedgerDenial::Capacity {
                        maximum,
                        used,
                        requested,
                    } => CompositeHistoryCatalogDenial::MetadataCapacityExhausted {
                        maximum,
                        used,
                        requested,
                    },
                })?
        };

        if let CompositeCommitParent::Ordinary(parent) = &parent {
            let mut reachability = lock_index(&state.reachability);
            reachability.increment_descendant_dependency(parent.commit())?;
        }
        state.metadata.reserve_confirmed(preview);
        counters::lock_counters(&state.counters).record_metadata_reservation();
        let reservation = HistoryReservationMetadata {
            parent: parent.clone(),
            commit_charge,
            reservation_charge,
        };
        assert!(state
            .reservations
            .insert(identity.clone(), reservation.clone())
            .is_none());
        if matches!(parent, CompositeCommitParent::Root) {
            state.root_reserved = true;
        }
        Ok(ReservedCompositeCommitCapacity::new(
            Arc::clone(&self.state),
            identity,
            reservation,
        ))
    }

    pub(crate) fn append(
        &self,
        commit: Arc<CompositeRuntimeWorldCommit>,
    ) -> Result<CompositeHistoryCatalogEntry, CompositeHistoryCatalogDenial> {
        let reservation = self.reserve(commit.as_ref())?;
        reservation.install(commit)
    }

    pub(crate) fn lookup(
        &self,
        identity: &CompositeCommitIdentity,
    ) -> Option<Arc<CompositeRuntimeWorldCommit>> {
        let state = lock_state(&self.state);
        counters::lock_counters(&state.counters).record_entry_lookup();
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

    pub(crate) fn arm_installed_root_rollback(
        &self,
        identity: &CompositeCommitIdentity,
    ) -> reservation::InstalledRootCommitRollback {
        let state = lock_state(&self.state);
        assert!(
            state.entries.get(identity).is_some_and(|entry| matches!(
                entry.commit().parent(),
                CompositeCommitParent::Root
            )),
            "root rollback must be armed for an installed root entry"
        );
        reservation::InstalledRootCommitRollback::new(Arc::clone(&self.state), identity.clone())
    }

    pub(crate) fn arm_installed_commit_rollback(
        &self,
        identity: &CompositeCommitIdentity,
    ) -> reservation::InstalledCommitRollback {
        assert!(
            lock_state(&self.state).entries.contains_key(identity),
            "commit rollback must be armed for an installed entry"
        );
        reservation::InstalledCommitRollback::new(Arc::clone(&self.state), identity.clone())
    }

    pub(crate) fn metadata_ledger(&self) -> HistoryMetadataLedger {
        lock_state(&self.state).metadata
    }

    pub(crate) fn lookup_count(&self) -> u64 {
        self.counters().entry_lookups()
    }

    pub(crate) fn counters(&self) -> HistoryCatalogCounters {
        let state = lock_state(&self.state);
        let counters = *counters::lock_counters(&state.counters);
        counters
    }

    fn protect_exact(
        &self,
        identity: CompositeCommitIdentity,
        class: HistoryProtectionClass,
    ) -> Result<CompositeHistoryProtectionObligation, CompositeHistoryCatalogDenial> {
        let state = lock_state(&self.state);
        validate_owner(&state, identity.owner_identity())?;
        if !state.entries.contains_key(&identity) {
            return Err(CompositeHistoryCatalogDenial::UnknownProtectionTarget(
                identity,
            ));
        }
        {
            let mut reachability = lock_index(&state.reachability);
            reachability.increment_direct_protection(&identity)?;
        }
        Ok(CompositeHistoryProtectionObligation::new(
            Arc::clone(&state.reachability),
            identity,
            class,
        ))
    }

    /// Issue the only cross-module history protection capability used by a
    /// product reference. Callers cannot choose another protection class.
    pub(crate) fn protect_product_head(
        &self,
        commit: &CompositeRuntimeWorldCommit,
    ) -> Result<ProductHeadHistoryProtectionObligation, CompositeHistoryCatalogDenial> {
        self.protect_exact(
            commit.identity().clone(),
            HistoryProtectionClass::ProductHead,
        )
        .map(ProductHeadHistoryProtectionObligation::issued)
    }

    /// Issue the exact history protection carried by a live commit-bound
    /// consumer such as a managed product-branch observation.
    pub(crate) fn protect_explicit_commit(
        &self,
        commit: &CompositeRuntimeWorldCommit,
    ) -> Result<ExplicitCommitHistoryProtectionObligation, CompositeHistoryCatalogDenial> {
        self.protect_exact(
            commit.identity().clone(),
            HistoryProtectionClass::ExplicitObligation,
        )
        .map(ExplicitCommitHistoryProtectionObligation::issued)
    }

    /// Walk one parent chain up to an explicit caller bound. Reclamation does
    /// not call this method; its reachability decision is index-local.
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
            let Some(parent) = support::ordinary_parent_identity(entry.commit().parent()) else {
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
        let maximum_reclaims = request.maximum_reclaims();
        if maximum_reclaims == 0 {
            return Ok(HistoryReclamationOutcome::new(0));
        }
        let candidate_count = request.candidate_commits().len().min(maximum_reclaims);
        let candidates = &request.candidate_commits()[..candidate_count];
        prevalidate_candidate_prefix(&mut state, candidates)?;

        let mut outcome = HistoryReclamationOutcome::new(maximum_reclaims);
        for candidate in candidates {
            outcome.examined_one();
            let reachability = {
                let mut index = lock_index(&state.reachability);
                index
                    .lookup(candidate)
                    .expect("prevalidated candidate has a reachability row")
            };
            if reachability.direct_protections() > 0 {
                outcome.record_skipped_protected();
                continue;
            }
            if reachability.descendant_dependencies() > 0 {
                outcome.record_skipped_with_descendant_dependencies();
                continue;
            }
            if request.age_ticks() == 0 {
                outcome.record_skipped_too_young();
                continue;
            }
            let entry = remove_installed(&mut state, candidate);
            outcome.reclaimed_one(candidate.clone(), entry.metadata_charge().total());
        }
        Ok(outcome)
    }
}
