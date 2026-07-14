use crate::runtime::{
    WorthQueryGraphObligationIndex, WorthQueryGraphObligationOperatingWorldDescriptor,
    WorthQueryGraphObligationOperatingWorldSelector,
};

use super::super::fixtures::{
    catalog, impossible_collection_selector, schema_registration,
    symbolic_relation_retirement_descriptor,
};

#[test]
fn no_match_selection_reports_lookup_work_without_fake_candidates() {
    let descriptor = symbolic_relation_retirement_descriptor();
    let registrations = (0..16)
        .map(|index| {
            schema_registration(
                &format!("impossible-{index}"),
                impossible_collection_selector(),
                WorthQueryGraphObligationOperatingWorldSelector::preview(),
            )
        })
        .collect::<Vec<_>>();
    let index = WorthQueryGraphObligationIndex::from_catalog(&catalog(registrations));

    let selection = index.select_for_touch(
        &descriptor,
        &WorthQueryGraphObligationOperatingWorldDescriptor::any_committed_authority(),
    );

    assert_eq!(selection.matched_obligation_count(), 0);
    assert_eq!(selection.counters().candidate_registration_count(), 0);
    assert_eq!(selection.counters().matched_bucket_count(), 0);
    assert_eq!(
        selection.counters().attempted_bucket_lookup_count(),
        selection.counters().touch_lookup_key_count()
            * selection.counters().operating_world_lookup_key_count()
    );
    assert_eq!(selection.counters().registration_full_scan_count(), 0);
}
