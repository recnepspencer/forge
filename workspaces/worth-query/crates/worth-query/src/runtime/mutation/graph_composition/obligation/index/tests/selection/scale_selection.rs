use crate::runtime::{
    WorthQueryGraphObligationIndex, WorthQueryGraphObligationOperatingWorldDescriptor,
    WorthQueryGraphObligationOperatingWorldSelector,
};

use super::super::fixtures::{
    catalog, relation_kind_id_selector, schema_registration,
    symbolic_relation_retirement_descriptor, unrelated_collection_selector,
};

#[test]
fn selection_cost_stays_bound_to_lookup_keys_not_catalog_breadth() {
    let descriptor = symbolic_relation_retirement_descriptor();
    let mut registrations = (0..512)
        .map(|index| {
            schema_registration(
                &format!("unrelated-{index}"),
                unrelated_collection_selector(),
                WorthQueryGraphObligationOperatingWorldSelector::preview(),
            )
        })
        .collect::<Vec<_>>();
    registrations.push(schema_registration(
        "matching",
        relation_kind_id_selector(),
        WorthQueryGraphObligationOperatingWorldSelector::any_committed_authority(),
    ));
    let index = WorthQueryGraphObligationIndex::from_catalog(&catalog(registrations));

    let selection = index.select_for_touch(
        &descriptor,
        &WorthQueryGraphObligationOperatingWorldDescriptor::any_committed_authority(),
    );

    assert_eq!(selection.matched_obligation_count(), 1);
    assert_eq!(selection.counters().candidate_registration_count(), 1);
    assert_eq!(selection.counters().matched_bucket_count(), 1);
    assert_eq!(selection.counters().registration_full_scan_count(), 0);
    assert_eq!(
        selection.counters().attempted_bucket_lookup_count(),
        selection.counters().touch_lookup_key_count()
            * selection.counters().operating_world_lookup_key_count()
    );
}
