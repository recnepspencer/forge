use std::sync::{Arc, Mutex, MutexGuard};

use crate::identity::{CompositeCommitIdentity, RuntimeWorldOwnerIdentity};

use super::super::reclamation::HistoryReclamationDenial;
use super::counters::lock_counters;
use super::denial::CompositeHistoryCatalogDenial;
use super::entry::CompositeHistoryCatalogEntry;
use super::reachability::lock_index;
use super::{CompositeCommitParent, CompositeHistoryCatalogState};

pub(super) fn lock_state(
    state: &Arc<Mutex<CompositeHistoryCatalogState>>,
) -> MutexGuard<'_, CompositeHistoryCatalogState> {
    state
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

pub(super) fn validate_owner(
    state: &CompositeHistoryCatalogState,
    actual: RuntimeWorldOwnerIdentity,
) -> Result<(), CompositeHistoryCatalogDenial> {
    lock_counters(&state.counters).record_owner_validation();
    if actual == state.owner {
        Ok(())
    } else {
        Err(CompositeHistoryCatalogDenial::ForeignOwner {
            expected: state.owner,
            actual,
        })
    }
}

pub(super) fn validate_parent_for_reservation(
    state: &mut CompositeHistoryCatalogState,
    parent: &CompositeCommitParent,
) -> Result<(), CompositeHistoryCatalogDenial> {
    lock_counters(&state.counters).record_parent_validation();
    let CompositeCommitParent::Ordinary(parent) = parent else {
        return Ok(());
    };
    if parent.commit().owner_identity() != state.owner {
        return Err(CompositeHistoryCatalogDenial::ForeignParent {
            expected: state.owner,
            actual: parent.commit().owner_identity(),
        });
    }
    if !state.entries.contains_key(parent.commit()) {
        return Err(CompositeHistoryCatalogDenial::MissingParent(
            parent.commit().clone(),
        ));
    }
    Ok(())
}

pub(super) fn install_entry(
    state: &mut CompositeHistoryCatalogState,
    entry: CompositeHistoryCatalogEntry,
) {
    let identity = entry.identity().clone();
    let parent = entry.commit().parent().clone();
    {
        let mut reachability = lock_index(&state.reachability);
        reachability.install(identity.clone());
    }
    assert!(state.entries.insert(identity.clone(), entry).is_none());
    if matches!(parent, CompositeCommitParent::Root) {
        state.root = Some(identity);
        state.root_ever_installed = true;
    }
}

pub(super) fn release_reservation(
    state: &mut CompositeHistoryCatalogState,
    identity: &CompositeCommitIdentity,
) {
    let Some(reservation) = state.reservations.remove(identity) else {
        return;
    };
    state.metadata.release_reservation(&reservation);
    lock_counters(&state.counters).record_metadata_release();
    if let CompositeCommitParent::Ordinary(parent) = &reservation.parent {
        let mut reachability = lock_index(&state.reachability);
        reachability.decrement_descendant_dependency(parent.commit());
    }
    if matches!(reservation.parent, CompositeCommitParent::Root) {
        state.root_reserved = false;
    }
}

pub(super) fn ordinary_parent_identity(
    parent: &CompositeCommitParent,
) -> Option<CompositeCommitIdentity> {
    match parent {
        CompositeCommitParent::Root => None,
        CompositeCommitParent::Ordinary(parent) => Some(parent.commit().clone()),
    }
}

pub(super) fn prevalidate_candidate_prefix(
    state: &mut CompositeHistoryCatalogState,
    candidates: &[CompositeCommitIdentity],
) -> Result<(), HistoryReclamationDenial> {
    for candidate in candidates {
        lock_counters(&state.counters).record_candidate_validation();
        if candidate.owner_identity() != state.owner {
            return Err(HistoryReclamationDenial::ForeignCandidate {
                expected: state.owner,
                actual: candidate.owner_identity(),
            });
        }
    }
    for (index, candidate) in candidates.iter().enumerate() {
        if candidates[..index].iter().any(|prior| prior == candidate) {
            return Err(HistoryReclamationDenial::DuplicateCandidate(
                candidate.clone(),
            ));
        }
    }
    for candidate in candidates {
        if !state.entries.contains_key(candidate) {
            return Err(HistoryReclamationDenial::UnknownCandidate(
                candidate.clone(),
            ));
        }
    }
    Ok(())
}

pub(super) fn remove_installed(
    state: &mut CompositeHistoryCatalogState,
    identity: &CompositeCommitIdentity,
) -> CompositeHistoryCatalogEntry {
    let entry = state
        .entries
        .remove(identity)
        .expect("prevalidated candidate remains installed during reclamation");
    let parent = entry.commit().parent().clone();
    {
        let mut reachability = lock_index(&state.reachability);
        reachability.remove_installed(identity);
        if let CompositeCommitParent::Ordinary(parent) = &parent {
            reachability.decrement_descendant_dependency(parent.commit());
        }
    }
    state.metadata.release_installed(entry.metadata_charge());
    lock_counters(&state.counters).record_metadata_release();
    if matches!(parent, CompositeCommitParent::Root) && state.root.as_ref() == Some(identity) {
        state.root = None;
    }
    entry
}
