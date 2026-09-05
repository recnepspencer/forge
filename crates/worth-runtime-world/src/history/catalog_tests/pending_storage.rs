use super::super::{CompositeHistoryCatalog, CompositeHistoryCatalogDenial};
use super::fixtures::{history_contract, linear_history};
use crate::history::reclamation::{CompositeHistoryReclamationRequest, HistoryReclamationDenial};

#[test]
fn preallocated_slots_are_invisible_until_promotion_and_release_once_on_drop() {
    let (_owner, commits) = linear_history(3);
    let root = &commits[0];
    let child = &commits[1];
    let catalog = CompositeHistoryCatalog::new(
        root.identity().owner_identity(),
        history_contract(2, u64::MAX),
    );
    catalog.append(root.clone()).unwrap();
    let before_metadata = catalog.metadata_ledger();
    let before_counters = catalog.counters();
    let reserved = catalog.reserve(child).unwrap();
    assert_eq!(catalog.len(), 1);
    assert_eq!(catalog.reserved_len(), 1);
    assert!(catalog.lookup(child.identity()).is_none());
    assert!(matches!(
        catalog.protect_product_head(child),
        Err(CompositeHistoryCatalogDenial::UnknownProtectionTarget(_))
    ));
    assert!(matches!(
        catalog.protect_explicit_commit(child),
        Err(CompositeHistoryCatalogDenial::UnknownProtectionTarget(_))
    ));
    assert!(matches!(
        catalog.reserve(&commits[2]),
        Err(CompositeHistoryCatalogDenial::MissingParent(_))
    ));
    assert!(matches!(
        catalog.reclaim_batch(CompositeHistoryReclamationRequest::new(
            root.identity().owner_identity(),
            vec![child.identity().clone()],
            1,
            1,
        )),
        Err(HistoryReclamationDenial::UnknownCandidate(_))
    ));
    // This inspects physical slot residence independently from the public
    // installed count: the eventual ordered map already contains the pending key.
    {
        let state = super::super::support::lock_state(&catalog.state);
        assert!(matches!(state.entries.get(child.identity()), Some(None)));
        assert_eq!(state.entries.len(), 2);
    }
    assert_eq!(
        catalog.counters().reachability_rows_installed(),
        before_counters.reachability_rows_installed()
    );
    drop(reserved);
    assert_eq!(catalog.metadata_ledger(), before_metadata);
    assert_eq!(catalog.reserved_len(), 0);
    assert_eq!(catalog.len(), 1);
    assert_eq!(
        catalog.counters().dependency_decrements(),
        before_counters.dependency_decrements() + 1
    );
    assert!(!super::super::support::lock_state(&catalog.state)
        .entries
        .contains_key(child.identity()));
    let reservation = catalog
        .reserve(child)
        .expect("dropping pending storage releases the exact count and key");
    reservation.install(child.clone()).unwrap();
    assert_eq!(
        catalog.len(),
        2,
        "a reservation spends one catalog slot, not two"
    );
    assert!(catalog.lookup(child.identity()).is_some());
}
