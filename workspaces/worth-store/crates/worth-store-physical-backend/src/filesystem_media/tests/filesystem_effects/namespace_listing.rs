use super::super::super::*;
use super::fixture::{created, owner, staged_path};

#[test]
fn bounded_listing_never_materializes_the_whole_directory() {
    let (_root, owner) = owner();
    for value in 1..=19 {
        drop(created(owner.create_new(&staged_path(&owner, value))));
    }
    let NamespaceDirectoryListingResult::Opened(mut listing) =
        owner.begin_directory_listing(owner.namespace_directory())
    else {
        panic!("namespace listing must open");
    };
    let first = listing.next_batch(7);
    let second = listing.next_batch(7);
    let third = listing.next_batch(7);
    assert_eq!(observed_len(&first), 7);
    assert_eq!(observed_len(&second), 7);
    assert_eq!(observed_len(&third), 6);
    assert!(matches!(
        third.result(),
        NamespaceEntryBatchResult::Observed(batch) if batch.exhausted()
    ));
    assert!(matches!(
        listing.next_batch(MAX_DIRECTORY_BATCH_ENTRIES + 1).result(),
        NamespaceEntryBatchResult::Failed(_)
    ));
    let counters = owner.counters();
    assert_eq!(counters.listing_batches(), 3);
    assert_eq!(counters.listing_entries(), 20);
    assert!(counters.is_conserved());
}

fn observed_len(outcome: &NamespaceEntryBatchOutcome) -> usize {
    match outcome.result() {
        NamespaceEntryBatchResult::Observed(batch) => batch.entries().len(),
        other => panic!("listing failed: {other:?}"),
    }
}
