use crate::runtime::{
    WorthQueryGraphObligationIndex, WorthQueryGraphObligationOperatingWorldDescriptor,
    WorthQueryGraphObligationOperatingWorldSelector,
};

use super::super::fixtures::{
    catalog, lifecycle_selector, relation_kind_id_selector, schema_registration,
    symbolic_relation_retirement_descriptor, unrelated_collection_selector,
};

#[test]
fn selection_counters_report_bucketed_work_instead_of_catalog_scan() {
    let descriptor = symbolic_relation_retirement_descriptor();
    let mut registrations = (0..24)
        .map(|index| {
            schema_registration(
                &format!("unrelated-{index}"),
                unrelated_collection_selector(),
                WorthQueryGraphObligationOperatingWorldSelector::any_committed_authority(),
            )
        })
        .collect::<Vec<_>>();
    registrations.push(schema_registration(
        "matching-relation-kind",
        relation_kind_id_selector(),
        WorthQueryGraphObligationOperatingWorldSelector::any_committed_authority(),
    ));
    registrations.push(schema_registration(
        "matching-lifecycle-any-world",
        lifecycle_selector(),
        WorthQueryGraphObligationOperatingWorldSelector::any_operating_world(),
    ));
    let index = WorthQueryGraphObligationIndex::from_catalog(&catalog(registrations));

    let selection = index.select_for_touch(
        &descriptor,
        &WorthQueryGraphObligationOperatingWorldDescriptor::any_committed_authority(),
    );

    assert_eq!(selection.matched_obligation_count(), 2);
    assert_eq!(
        selection.counters().attempted_bucket_lookup_count(),
        selection.counters().touch_lookup_key_count()
            * selection.counters().operating_world_lookup_key_count()
    );
    assert_eq!(selection.counters().candidate_registration_count(), 2);
    assert_eq!(selection.counters().deduplicated_candidate_count(), 2);
    assert_eq!(selection.counters().matched_bucket_count(), 2);
    assert_eq!(selection.counters().visited_bucket_count(), 2);
    assert_eq!(selection.counters().registration_full_scan_count(), 0);
    assert!(selection.counters().touch_lookup_key_count() >= 5);
    assert_eq!(selection.counters().operating_world_lookup_key_count(), 2);
}
