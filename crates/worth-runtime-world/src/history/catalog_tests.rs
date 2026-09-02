use crate::history::reclamation::{CompositeHistoryReclamationRequest, HistoryReclamationDenial};
use crate::history::{CompositeCommitParent, OrdinaryParent};
use crate::lifecycle::owner::RuntimeWorldOwnerConstructionContract;
use std::num::NonZeroUsize;
use std::sync::Arc;

use super::{
    CompositeHistoryCatalog, CompositeHistoryCatalogDenial, RuntimeWorldHistoryCatalogContract,
};

#[path = "catalog_tests/support.rs"]
mod support;
use support::{commit_chain, history_contract, metadata_limit};

#[test]
fn reservation_is_bounded_move_only_and_root_is_one_shot() {
    let mut owner = RuntimeWorldOwnerConstructionContract::new().expect("World owner");
    let catalog = CompositeHistoryCatalog::new(owner.owner_identity(), history_contract(2, 8));
    let root_identity = owner
        .issuer_mut()
        .composite_commit()
        .expect("root identity");
    let reservation = catalog
        .reserve(root_identity, CompositeCommitParent::Root, 4)
        .expect("root reservation");
    assert_eq!(catalog.reserved_len(), 1);
    assert_eq!(catalog.reserved_metadata_bytes(), 4);

    let second_root = owner
        .issuer_mut()
        .composite_commit()
        .expect("second identity");
    assert!(matches!(
        catalog.reserve(second_root, CompositeCommitParent::Root, 1),
        Err(CompositeHistoryCatalogDenial::RootAlreadyInstalled)
    ));
    drop(reservation);
    assert_eq!(catalog.reserved_len(), 0);
    assert_eq!(catalog.reserved_metadata_bytes(), 0);

    let retry_root = owner
        .issuer_mut()
        .composite_commit()
        .expect("retry identity");
    let retry = catalog
        .reserve(retry_root, CompositeCommitParent::Root, 4)
        .expect("dropped reservation releases root capacity");
    drop(retry);
    assert_eq!(catalog.root(), None);
}

#[test]
fn history_keeps_equal_basis_occurrences_distinct_and_traversal_explicit() {
    let (owner, root, ordinary, leaf) = commit_chain();
    assert_eq!(root.basis().identity(), ordinary.basis().identity());
    assert_eq!(ordinary.basis().identity(), leaf.basis().identity());
    assert_ne!(root.identity(), ordinary.identity());
    assert_ne!(ordinary.identity(), leaf.identity());

    let catalog = CompositeHistoryCatalog::new(
        owner.owner_identity(),
        history_contract(4, metadata_limit(root.as_ref(), 4)),
    );
    catalog.append(root.clone()).expect("root install");
    catalog.append(ordinary.clone()).expect("ordinary install");
    catalog.append(leaf.clone()).expect("leaf install");
    assert_eq!(catalog.len(), 3);
    assert_eq!(catalog.root(), Some(root.identity().clone()));
    assert_eq!(catalog.lookup_count(), 0);
    assert_eq!(
        catalog
            .lookup(ordinary.identity())
            .as_ref()
            .map(|commit| commit.identity()),
        Some(ordinary.identity())
    );
    assert_eq!(catalog.lookup_count(), 1);

    let traversal = catalog
        .trace_ancestry(leaf.identity().clone(), NonZeroUsize::new(2).unwrap())
        .expect("bounded traversal");
    assert_eq!(traversal.visited_count(), 2);
    assert_eq!(traversal.commits()[0].identity(), leaf.identity());
    assert_eq!(traversal.commits()[1].identity(), ordinary.identity());
    assert_eq!(traversal.next_parent(), Some(root.identity()));
    assert!(!traversal.is_complete());
    assert_eq!(catalog.lookup_count(), 1, "traversal does not hide lookups");

    let root_traversal = catalog
        .trace_ancestry(root.identity().clone(), NonZeroUsize::new(1).unwrap())
        .expect("root traversal");
    assert!(root_traversal.is_complete());
    assert!(matches!(
        catalog.append(root),
        Err(CompositeHistoryCatalogDenial::DuplicateCommit)
    ));
}

#[test]
fn reclamation_is_prevalidated_bounded_and_protects_complete_ancestry() {
    let (mut owner, root, ordinary, leaf) = commit_chain();
    let catalog = CompositeHistoryCatalog::new(
        owner.owner_identity(),
        history_contract(4, metadata_limit(root.as_ref(), 4)),
    );
    for commit in [&root, &ordinary, &leaf] {
        catalog.append(Arc::clone(commit)).expect("chain install");
    }

    let protected = CompositeHistoryReclamationRequest::new(
        owner.owner_identity(),
        vec![leaf.identity().clone()],
        vec![
            root.identity().clone(),
            ordinary.identity().clone(),
            leaf.identity().clone(),
        ],
        3,
        1,
    );
    let outcome = catalog.reclaim_batch(protected).expect("protected batch");
    assert_eq!(outcome.examined(), 3);
    assert_eq!(outcome.skipped_protected(), 3);
    assert!(outcome.reclaimed_commits().is_empty());
    assert_eq!(catalog.len(), 3);

    let unknown_identity = owner
        .issuer_mut()
        .composite_commit()
        .expect("unknown identity");
    let prevalidated = CompositeHistoryReclamationRequest::new(
        owner.owner_identity(),
        Vec::new(),
        vec![leaf.identity().clone(), unknown_identity.clone()],
        2,
        1,
    );
    assert_eq!(
        catalog.reclaim_batch(prevalidated),
        Err(HistoryReclamationDenial::UnknownCandidate(unknown_identity))
    );
    assert!(catalog.lookup(leaf.identity()).is_some());

    let blocked_parent = CompositeHistoryReclamationRequest::new(
        owner.owner_identity(),
        Vec::new(),
        vec![root.identity().clone()],
        1,
        1,
    );
    let blocked = catalog
        .reclaim_batch(blocked_parent)
        .expect("parent is protected by child");
    assert_eq!(blocked.skipped_with_children(), 1);

    let too_young = CompositeHistoryReclamationRequest::new(
        owner.owner_identity(),
        Vec::new(),
        vec![leaf.identity().clone()],
        1,
        0,
    );
    assert_eq!(
        catalog
            .reclaim_batch(too_young)
            .unwrap()
            .skipped_too_young(),
        1
    );

    let leaf_reclaim = CompositeHistoryReclamationRequest::new(
        owner.owner_identity(),
        Vec::new(),
        vec![leaf.identity().clone(), ordinary.identity().clone()],
        1,
        1,
    );
    let leaf_outcome = catalog
        .reclaim_batch(leaf_reclaim)
        .expect("bounded leaf reclaim");
    assert_eq!(leaf_outcome.reclaimed_commits(), &[leaf.identity().clone()]);
    assert_eq!(catalog.len(), 2);
    assert!(catalog.lookup(leaf.identity()).is_none());

    let ordinary_reclaim = CompositeHistoryReclamationRequest::new(
        owner.owner_identity(),
        Vec::new(),
        vec![ordinary.identity().clone()],
        1,
        1,
    );
    catalog
        .reclaim_batch(ordinary_reclaim)
        .expect("now-leafless parent reclaim");
    assert_eq!(catalog.len(), 1);

    let root_reclaim = CompositeHistoryReclamationRequest::new(
        owner.owner_identity(),
        Vec::new(),
        vec![root.identity().clone()],
        1,
        1,
    );
    catalog
        .reclaim_batch(root_reclaim)
        .expect("unreachable root reclaim");
    assert_eq!(catalog.root(), None);
    let replacement = owner
        .issuer_mut()
        .composite_commit()
        .expect("replacement id");
    assert!(matches!(
        catalog.reserve(replacement, CompositeCommitParent::Root, 1),
        Err(CompositeHistoryCatalogDenial::RootAlreadyInstalled)
    ));
}

#[test]
fn history_capacity_denies_before_installation() {
    let (owner, root, ordinary, _) = commit_chain();
    let catalog = CompositeHistoryCatalog::new(
        owner.owner_identity(),
        history_contract(1, u64::try_from(root.metadata_bytes()).unwrap()),
    );
    catalog.append(root.clone()).expect("root install");
    let denial = catalog.append(ordinary);
    assert!(matches!(
        denial,
        Err(CompositeHistoryCatalogDenial::CommitCapacityExhausted { maximum: 1 })
    ));
    assert_eq!(catalog.len(), 1);
    assert_eq!(catalog.metadata_bytes(), root.metadata_bytes());
}

#[test]
fn metadata_capacity_denies_before_reservation() {
    let (owner, root, ordinary, _) = commit_chain();
    let catalog = CompositeHistoryCatalog::new(
        owner.owner_identity(),
        history_contract(2, u64::try_from(root.metadata_bytes()).unwrap()),
    );
    catalog.append(root.clone()).expect("root install");
    let denial = catalog.append(ordinary);
    assert!(matches!(
        denial,
        Err(CompositeHistoryCatalogDenial::MetadataCapacityExhausted {
            maximum,
            used,
            requested,
        }) if maximum == root.metadata_bytes()
            && used == root.metadata_bytes()
            && requested == root.metadata_bytes()
    ));
    assert_eq!(catalog.reserved_len(), 0);
    assert_eq!(catalog.metadata_bytes(), root.metadata_bytes());
}

#[test]
fn duplicate_reclamation_candidates_are_rejected_before_any_removal() {
    let (owner, root, ordinary, leaf) = commit_chain();
    let catalog = CompositeHistoryCatalog::new(
        owner.owner_identity(),
        history_contract(3, metadata_limit(root.as_ref(), 3)),
    );
    catalog.append(root.clone()).expect("root install");
    catalog.append(ordinary).expect("ordinary install");
    catalog.append(leaf.clone()).expect("leaf install");
    let request = CompositeHistoryReclamationRequest::new(
        owner.owner_identity(),
        Vec::new(),
        vec![leaf.identity().clone(), leaf.identity().clone()],
        2,
        1,
    );
    assert_eq!(
        catalog.reclaim_batch(request),
        Err(HistoryReclamationDenial::DuplicateCandidate(
            leaf.identity().clone()
        ))
    );
    assert_eq!(catalog.len(), 3);
    assert!(catalog.lookup(leaf.identity()).is_some());
}

#[test]
fn failed_slot_install_releases_capacity_without_publishing_the_wrong_commit() {
    let (owner, root, ordinary, _) = commit_chain();
    let catalog = CompositeHistoryCatalog::new(
        owner.owner_identity(),
        history_contract(2, metadata_limit(root.as_ref(), 2)),
    );
    let slot = catalog
        .reserve(
            root.identity().clone(),
            CompositeCommitParent::Root,
            root.metadata_bytes(),
        )
        .expect("root slot");
    assert!(matches!(
        slot.install(ordinary),
        Err(CompositeHistoryCatalogDenial::ReservationCommitMismatch)
    ));
    assert_eq!(catalog.reserved_len(), 0);
    assert_eq!(catalog.len(), 0);
    catalog
        .append(root)
        .expect("released root slot can be retried");
}

#[test]
fn foreign_and_missing_parent_inputs_are_denied_without_reservation() {
    let mut owner = RuntimeWorldOwnerConstructionContract::new().expect("World owner");
    let mut foreign = RuntimeWorldOwnerConstructionContract::new().expect("foreign owner");
    let catalog = CompositeHistoryCatalog::new(owner.owner_identity(), history_contract(2, 8));
    let foreign_identity = foreign
        .issuer_mut()
        .composite_commit()
        .expect("foreign identity");
    assert!(matches!(
        catalog.reserve(foreign_identity, CompositeCommitParent::Root, 1),
        Err(CompositeHistoryCatalogDenial::ForeignOwner { .. })
    ));

    let missing_parent = owner
        .issuer_mut()
        .composite_commit()
        .expect("parent identity");
    let child = owner
        .issuer_mut()
        .composite_commit()
        .expect("child identity");
    let denial = catalog.reserve(
        child,
        CompositeCommitParent::Ordinary(OrdinaryParent::new(missing_parent.clone())),
        1,
    );
    assert!(matches!(
        denial,
        Err(CompositeHistoryCatalogDenial::MissingParent(parent)) if parent == missing_parent
    ));
    assert_eq!(catalog.reserved_len(), 0);
}
