use crate::runtime::{
    ForgeQueryGraphObligationIndex, ForgeQueryGraphObligationOperatingWorldDescriptor,
    ForgeQueryGraphObligationOperatingWorldSelector,
};

use super::super::fixtures::{
    catalog, relation_kind_id_selector, schema_registration,
    symbolic_relation_retirement_descriptor,
};

#[test]
fn operating_world_selection_matches_specific_world_plus_any_world() {
    let descriptor = symbolic_relation_retirement_descriptor();
    let index = ForgeQueryGraphObligationIndex::from_catalog(&catalog(vec![
        schema_registration(
            "committed",
            relation_kind_id_selector(),
            ForgeQueryGraphObligationOperatingWorldSelector::any_committed_authority(),
        ),
        schema_registration(
            "preview",
            relation_kind_id_selector(),
            ForgeQueryGraphObligationOperatingWorldSelector::preview(),
        ),
        schema_registration(
            "branch",
            relation_kind_id_selector(),
            ForgeQueryGraphObligationOperatingWorldSelector::branch(),
        ),
        schema_registration(
            "configured-domain-handle",
            relation_kind_id_selector(),
            ForgeQueryGraphObligationOperatingWorldSelector::configured_domain_handle(),
        ),
        schema_registration(
            "any-world",
            relation_kind_id_selector(),
            ForgeQueryGraphObligationOperatingWorldSelector::any_operating_world(),
        ),
    ]));

    assert_selected_names(
        &index,
        &descriptor,
        ForgeQueryGraphObligationOperatingWorldDescriptor::any_committed_authority(),
        &["any-world", "committed"],
    );
    assert_selected_names(
        &index,
        &descriptor,
        ForgeQueryGraphObligationOperatingWorldDescriptor::preview(),
        &["any-world", "preview"],
    );
    assert_selected_names(
        &index,
        &descriptor,
        ForgeQueryGraphObligationOperatingWorldDescriptor::branch(),
        &["any-world", "branch"],
    );
    assert_selected_names(
        &index,
        &descriptor,
        ForgeQueryGraphObligationOperatingWorldDescriptor::configured_domain_handle(),
        &["any-world", "configured-domain-handle"],
    );
}

fn assert_selected_names(
    index: &ForgeQueryGraphObligationIndex,
    descriptor: &crate::runtime::ForgeQueryGraphTouchDescriptor,
    world: ForgeQueryGraphObligationOperatingWorldDescriptor,
    expected_names: &[&str],
) {
    let selection = index.select_for_touch(descriptor, &world);
    let mut names = selection
        .matched_registrations()
        .iter()
        .map(|registration| registration.rule_identity().name())
        .collect::<Vec<_>>();
    names.sort();
    assert_eq!(names, expected_names);
}
