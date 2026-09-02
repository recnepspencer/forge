use std::collections::BTreeSet;
use std::sync::{Arc, Mutex, MutexGuard};

use crate::identity::{CompositeCommitIdentity, RuntimeWorldOwnerIdentity};

use super::super::reclamation::HistoryReclamationDenial;
use super::denial::CompositeHistoryCatalogDenial;
use super::entry::CompositeHistoryCatalogEntry;
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
    state: &CompositeHistoryCatalogState,
    parent: &CompositeCommitParent,
) -> Result<(), CompositeHistoryCatalogDenial> {
    if let CompositeCommitParent::Ordinary(parent) = parent {
        validate_owner(state, parent.commit().owner_identity())?;
        if !state.entries.contains_key(parent.commit()) {
            return Err(CompositeHistoryCatalogDenial::MissingParent(
                parent.commit().clone(),
            ));
        }
    }
    Ok(())
}

pub(super) fn install_entry(
    state: &mut CompositeHistoryCatalogState,
    entry: CompositeHistoryCatalogEntry,
) {
    let identity = entry.identity().clone();
    let parent = entry.commit().parent().clone();
    if matches!(parent, CompositeCommitParent::Root) {
        state.root = Some(identity.clone());
        state.root_ever_installed = true;
    } else if let CompositeCommitParent::Ordinary(parent) = parent {
        state
            .children
            .entry(parent.commit().clone())
            .or_default()
            .insert(identity.clone());
    }
    state.entries.insert(identity, entry);
}

pub(super) fn release_reservation(
    state: &mut CompositeHistoryCatalogState,
    identity: &CompositeCommitIdentity,
    metadata_bytes: usize,
) {
    if let Some(reservation) = state.reservations.remove(identity) {
        debug_assert_eq!(reservation.metadata_bytes, metadata_bytes);
        state.reserved_metadata_bytes -= reservation.metadata_bytes;
        if matches!(reservation.parent, CompositeCommitParent::Root) {
            state.root_reserved = false;
        }
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

pub(super) fn validate_candidate(
    state: &CompositeHistoryCatalogState,
    candidate: &CompositeCommitIdentity,
) -> Result<(), HistoryReclamationDenial> {
    if candidate.owner_identity() != state.owner {
        return Err(HistoryReclamationDenial::ForeignCandidate {
            expected: state.owner,
            actual: candidate.owner_identity(),
        });
    }
    if !state.entries.contains_key(candidate) {
        return Err(HistoryReclamationDenial::UnknownCandidate(
            candidate.clone(),
        ));
    }
    Ok(())
}

pub(super) fn remove_child_index(
    state: &mut CompositeHistoryCatalogState,
    parent: &CompositeCommitParent,
    child: &CompositeCommitIdentity,
) {
    let CompositeCommitParent::Ordinary(parent) = parent else {
        return;
    };
    let remove_index = state
        .children
        .get_mut(parent.commit())
        .map(|children| {
            children.remove(child);
            children.is_empty()
        })
        .unwrap_or(false);
    if remove_index {
        state.children.remove(parent.commit());
    }
}

pub(super) fn protected_ancestry(
    state: &CompositeHistoryCatalogState,
    protected_commits: &[CompositeCommitIdentity],
) -> Result<BTreeSet<CompositeCommitIdentity>, HistoryReclamationDenial> {
    let mut protected = BTreeSet::new();
    for protected_commit in protected_commits {
        validate_owner(state, protected_commit.owner_identity())
            .map_err(HistoryReclamationDenial::Catalog)?;
        let mut current = protected_commit.clone();
        loop {
            if !protected.insert(current.clone()) {
                break;
            }
            let entry = state
                .entries
                .get(&current)
                .ok_or_else(|| HistoryReclamationDenial::UnknownProtected(current.clone()))?;
            let Some(parent) = ordinary_parent_identity(entry.commit().parent()) else {
                break;
            };
            current = parent;
        }
    }
    Ok(protected)
}
