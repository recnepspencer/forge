use crate::runtime::{
    ForgeQueryGraphObligationIndex, ForgeQueryGraphObligationOperatingWorldDescriptor,
    ForgeQueryGraphObligationOperatingWorldSelector, ForgeQueryGraphTouchDescriptor,
    ForgeQueryGraphTouchReadVerb, ForgeQueryGraphTouchSelector, ForgeQueryMutationFamily,
};

use super::super::fixtures::{catalog, schema_registration};

#[test]
fn read_descriptor_selects_read_obligations_without_mutation_family_leakage() {
    let world = ForgeQueryGraphObligationOperatingWorldSelector::any_committed_authority();
    let descriptor = ForgeQueryGraphTouchDescriptor::read_family(
        "TaskEdge",
        [
            ForgeQueryGraphTouchReadVerb::ObservesCollection,
            ForgeQueryGraphTouchReadVerb::ExposesDerivedTopology,
        ],
    )
    .unwrap();
    let index = ForgeQueryGraphObligationIndex::from_catalog(&catalog(vec![
        schema_registration(
            "read-derived-topology",
            ForgeQueryGraphTouchSelector::read_verb(
                ForgeQueryGraphTouchReadVerb::ExposesDerivedTopology,
            ),
            world,
        ),
        schema_registration(
            "mutation-assertion",
            ForgeQueryGraphTouchSelector::mutation_family(ForgeQueryMutationFamily::Assertion),
            world,
        ),
    ]));

    let selection = index.select_for_touch(
        &descriptor,
        &ForgeQueryGraphObligationOperatingWorldDescriptor::any_committed_authority(),
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
