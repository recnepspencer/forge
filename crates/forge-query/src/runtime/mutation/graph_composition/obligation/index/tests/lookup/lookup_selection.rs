use crate::runtime::{
    ForgeQueryGraphObligationIndex, ForgeQueryGraphObligationOperatingWorldDescriptor,
    ForgeQueryGraphObligationOperatingWorldSelector, ForgeQueryGraphTouchLifecycleFamily,
    ForgeQueryGraphTouchSelector, ForgeQueryMutationFamily,
};

use super::super::fixtures::{
    blocking_registration, catalog, collection_selector, multi_component_descriptor,
    relation_kind_id_selector, schema_registration, symbolic_relation_retirement_descriptor,
    unrelated_collection_selector,
};

#[test]
fn selection_matches_only_bucketed_touch_and_world_obligations() {
    let descriptor = symbolic_relation_retirement_descriptor();
    let index = ForgeQueryGraphObligationIndex::from_catalog(&catalog(vec![
        schema_registration(
            "relation-kind-id",
            relation_kind_id_selector(),
            ForgeQueryGraphObligationOperatingWorldSelector::any_committed_authority(),
        ),
        blocking_registration(
            "collection-any-world",
            collection_selector(),
            ForgeQueryGraphObligationOperatingWorldSelector::any_operating_world(),
        ),
        schema_registration(
            "unrelated-collection",
            unrelated_collection_selector(),
            ForgeQueryGraphObligationOperatingWorldSelector::any_committed_authority(),
        ),
        schema_registration(
            "preview-only",
            relation_kind_id_selector(),
            ForgeQueryGraphObligationOperatingWorldSelector::preview(),
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

    assert_eq!(selection.matched_obligation_count(), 2);
    assert!(names.contains(&"relation-kind-id"));
    assert!(names.contains(&"collection-any-world"));
    assert!(!names.contains(&"unrelated-collection"));
    assert!(!names.contains(&"preview-only"));
    assert_eq!(selection.counters().registration_full_scan_count(), 0);
}

#[test]
fn selection_derives_keys_across_multi_component_descriptors() {
    let descriptor = multi_component_descriptor();
    let world = ForgeQueryGraphObligationOperatingWorldSelector::any_committed_authority();
    let index = ForgeQueryGraphObligationIndex::from_catalog(&catalog(vec![
        schema_registration("relation-kind-77", relation_kind_id_selector(), world),
        schema_registration(
            "relation-kind-88",
            ForgeQueryGraphTouchSelector::relation_kind_id(88),
            world,
        ),
        schema_registration(
            "collection-face",
            ForgeQueryGraphTouchSelector::relation_kind("topology.face").unwrap(),
            world,
        ),
        schema_registration(
            "declared-aspect-operation",
            ForgeQueryGraphTouchSelector::declared_aspect_operation("set:capacity").unwrap(),
            world,
        ),
        schema_registration(
            "aspect-capacity",
            ForgeQueryGraphTouchSelector::aspect_path("capacity").unwrap(),
            world,
        ),
        schema_registration(
            "aspect-boundary",
            ForgeQueryGraphTouchSelector::aspect_path("boundary").unwrap(),
            world,
        ),
        schema_registration(
            "mutation-update",
            ForgeQueryGraphTouchSelector::mutation_family(ForgeQueryMutationFamily::Update),
            world,
        ),
        schema_registration(
            "lifecycle-followup",
            ForgeQueryGraphTouchSelector::lifecycle_family(
                ForgeQueryGraphTouchLifecycleFamily::SameBatchSymbolicRelationFollowup,
            ),
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
        .collect::<std::collections::BTreeSet<_>>();

    assert_eq!(selection.matched_obligation_count(), 8);
    for expected in [
        "relation-kind-77",
        "relation-kind-88",
        "collection-face",
        "declared-aspect-operation",
        "aspect-capacity",
        "aspect-boundary",
        "mutation-update",
        "lifecycle-followup",
    ] {
        assert!(
            names.contains(expected),
            "missing expected obligation {expected}"
        );
    }
    assert_eq!(
        selection.counters().attempted_bucket_lookup_count(),
        selection.counters().touch_lookup_key_count()
            * selection.counters().operating_world_lookup_key_count()
    );
}
