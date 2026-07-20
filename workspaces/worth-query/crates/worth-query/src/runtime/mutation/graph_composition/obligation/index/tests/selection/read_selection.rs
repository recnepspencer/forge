use crate::runtime::{
    WorthQueryGraphObligationIndex, WorthQueryGraphObligationOperatingWorldDescriptor,
    WorthQueryGraphObligationOperatingWorldSelector, WorthQueryGraphTouchDescriptor,
    WorthQueryGraphTouchReadVerb, WorthQueryGraphTouchSelector, WorthQueryMutationFamily,
};

use super::super::fixtures::{catalog, schema_registration};

#[test]
fn read_descriptor_selects_read_obligations_without_mutation_family_leakage() {
    let world = WorthQueryGraphObligationOperatingWorldSelector::any_committed_authority();
    let descriptor = WorthQueryGraphTouchDescriptor::read_family(
        "TaskEdge",
        [
            WorthQueryGraphTouchReadVerb::ObservesCollection,
            WorthQueryGraphTouchReadVerb::ExposesDerivedTopology,
        ],
    )
    .unwrap();
    let index = WorthQueryGraphObligationIndex::from_catalog(&catalog(vec![
        schema_registration(
            "read-derived-topology",
            WorthQueryGraphTouchSelector::read_verb(
                WorthQueryGraphTouchReadVerb::ExposesDerivedTopology,
            ),
            world,
        ),
        schema_registration(
            "mutation-assertion",
            WorthQueryGraphTouchSelector::mutation_family(WorthQueryMutationFamily::Assertion),
            world,
        ),
    ]));

    let selection = index.select_for_touch(
        &descriptor,
        &WorthQueryGraphObligationOperatingWorldDescriptor::any_committed_authority(),
    );
    let names = selection
        .matched_registrations()
        .iter()
        .map(|registration| registration.rule_identity().name())
        .collect::<Vec<_>>();

    assert_eq!(names, vec!["read-derived-topology"]);
    assert_eq!(selection.matched_obligation_count(), 1);
    assert_eq!(selection.counters().registration_full_scan_count(), 0);
}
