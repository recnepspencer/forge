use crate::runtime::{
    WorthQueryGraphObligationIndex, WorthQueryGraphObligationOperatingWorldSelector,
};

use super::super::fixtures::{
    blocking_registration, catalog, collection_selector, relation_kind_id_selector,
    schema_registration,
};

#[test]
fn graph_obligation_index_digest_is_order_independent() {
    let world = WorthQueryGraphObligationOperatingWorldSelector::any_committed_authority();
    let left = schema_registration("schema", relation_kind_id_selector(), world);
    let right = blocking_registration("blocking", collection_selector(), world);

    let first =
        WorthQueryGraphObligationIndex::from_catalog(&catalog(vec![left.clone(), right.clone()]));
    let second = WorthQueryGraphObligationIndex::from_catalog(&catalog(vec![right, left]));

    assert_eq!(first.index_digest(), second.index_digest());
    assert_eq!(first.registration_count(), 2);
    assert_eq!(first.bucket_count(), 2);
    assert_eq!(
        first
            .entries()
            .iter()
            .map(|entry| entry.entry_digest())
            .collect::<Vec<_>>(),
        second
            .entries()
            .iter()
            .map(|entry| entry.entry_digest())
            .collect::<Vec<_>>()
    );
}
