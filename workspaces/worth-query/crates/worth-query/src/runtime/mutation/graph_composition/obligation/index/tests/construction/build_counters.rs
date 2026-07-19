use crate::runtime::{
    WorthQueryGraphObligationIndex, WorthQueryGraphObligationOperatingWorldSelector,
};

use super::super::fixtures::{
    blocking_registration, catalog, collection_selector, relation_kind_id_selector,
    schema_registration,
};

#[test]
fn index_build_counters_are_stable_and_shape_sensitive() {
    let world = WorthQueryGraphObligationOperatingWorldSelector::any_committed_authority();
    let left = schema_registration("schema", relation_kind_id_selector(), world);
    let right = blocking_registration("blocking", collection_selector(), world);

    let first =
        WorthQueryGraphObligationIndex::from_catalog(&catalog(vec![left.clone(), right.clone()]));
    let reordered = WorthQueryGraphObligationIndex::from_catalog(&catalog(vec![right, left]));
    let smaller =
        WorthQueryGraphObligationIndex::from_catalog(&catalog(vec![schema_registration(
            "schema",
            relation_kind_id_selector(),
            world,
        )]));

    assert_eq!(first.build_counters().registration_count(), 2);
    assert_eq!(first.build_counters().entry_count(), 2);
    assert_eq!(first.build_counters().bucket_count(), 2);
    assert_eq!(first.build_counters().support_row_count(), 6);
    assert_eq!(first.build_counters().complexity_contract_count(), 2);
    assert_eq!(first.build_counters().registration_full_scan_count(), 2);
    assert_eq!(
        first.build_counters().build_digest(),
        reordered.build_counters().build_digest()
    );
    assert_ne!(
        first.build_counters().build_digest(),
        smaller.build_counters().build_digest()
    );
}
