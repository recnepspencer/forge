use forge_query::facade::runtime::ForgeQueryGraphObligationIndex;

use super::support::{
    committed_world, graph_mutation_touch, registration_catalog, unrelated_touch,
};

#[test]
fn selection_replay_is_pure_for_equivalent_touch_world_and_index() {
    let first_index = ForgeQueryGraphObligationIndex::from_catalog(&registration_catalog());
    let second_index = ForgeQueryGraphObligationIndex::from_catalog(&registration_catalog());
    let touch = graph_mutation_touch();
    let world = committed_world();

    let first = first_index.select_for_touch(&touch, &world);
    let replay = second_index.select_for_touch(&touch, &world);

    assert_eq!(first.selection_digest(), replay.selection_digest());
    assert_eq!(
        first.matched_obligation_count(),
        replay.matched_obligation_count()
    );
    assert_eq!(first.counters().registration_full_scan_count(), 0);
    assert_eq!(
        first.counters().attempted_bucket_lookup_count(),
        first.counters().touch_lookup_key_count()
            * first.counters().operating_world_lookup_key_count()
    );
}

#[test]
fn false_fire_touch_does_not_select_unrelated_graph_obligations() {
    let index = ForgeQueryGraphObligationIndex::from_catalog(&registration_catalog());
    let selection = index.select_for_touch(&unrelated_touch(), &committed_world());

    assert_eq!(selection.matched_obligation_count(), 0);
    assert_eq!(selection.counters().registration_full_scan_count(), 0);
    assert!(
        selection.counters().attempted_bucket_lookup_count() > 0,
        "no-match selections must still report lookup effort"
    );
}
