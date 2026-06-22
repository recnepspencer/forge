use crate::runtime::{
    ForgeQueryGraphObligationOperatingWorldSelector, ForgeQueryGraphTouchDescriptor,
    ForgeQueryGraphTouchLifecycleFamily, ForgeQueryGraphTouchReadVerb,
    ForgeQueryGraphTouchSelector, ForgeQueryMutationFamily,
};
use forge_relational::facade::identity::KindId;

use super::fixtures::{
    symbolic_relation_touch_descriptor, symbolic_relation_touch_descriptor_with_relation_kind_id,
};

#[test]
fn touch_selectors_do_not_cross_match_unrelated_graph_lanes() {
    let descriptor = symbolic_relation_touch_descriptor("topology.edge", "weight");

    assert!(ForgeQueryGraphTouchSelector::relation_kind("topology.edge")
        .unwrap()
        .matches_descriptor(&descriptor));
    assert!(ForgeQueryGraphTouchSelector::aspect_path("weight")
        .unwrap()
        .matches_descriptor(&descriptor));
    assert!(
        ForgeQueryGraphTouchSelector::mutation_family(ForgeQueryMutationFamily::Delete)
            .matches_descriptor(&descriptor)
    );
    assert!(ForgeQueryGraphTouchSelector::lifecycle_family(
        ForgeQueryGraphTouchLifecycleFamily::SameBatchSymbolicRelationRetirement
    )
    .matches_descriptor(&descriptor));

    assert!(
        !ForgeQueryGraphTouchSelector::relation_kind("topology.face")
            .unwrap()
            .matches_descriptor(&descriptor)
    );
    assert!(!ForgeQueryGraphTouchSelector::aspect_path("capacity")
        .unwrap()
        .matches_descriptor(&descriptor));
    assert!(
        !ForgeQueryGraphTouchSelector::mutation_family(ForgeQueryMutationFamily::Update)
            .matches_descriptor(&descriptor)
    );
    assert!(!ForgeQueryGraphTouchSelector::relation_kind_id(42).matches_descriptor(&descriptor));
}

#[test]
fn equivalent_selectors_share_stable_identity() {
    let left = ForgeQueryGraphTouchSelector::relation_kind("topology.edge").unwrap();
    let right = ForgeQueryGraphTouchSelector::collection("topology.edge").unwrap();

    assert_eq!(left.selector_digest(), right.selector_digest());
}

#[test]
fn relation_kind_id_selector_is_not_a_collection_string_alias() {
    let descriptor = symbolic_relation_touch_descriptor_with_relation_kind_id(
        "topology.edge",
        "weight",
        Some(KindId(42)),
    );
    let collection_selector = ForgeQueryGraphTouchSelector::relation_kind("42").unwrap();
    let relation_kind_id_selector = ForgeQueryGraphTouchSelector::relation_kind_id(42);

    assert_ne!(
        collection_selector.selector_digest(),
        relation_kind_id_selector.selector_digest()
    );
    assert_eq!(
        relation_kind_id_selector.selector_kind(),
        "relation-kind-id"
    );
    assert_eq!(
        relation_kind_id_selector.selector_value().as_deref(),
        Some("42")
    );
    assert!(relation_kind_id_selector.matches_descriptor(&descriptor));
    assert!(ForgeQueryGraphTouchSelector::relational_kind_id(KindId(42))
        .matches_descriptor(&descriptor));
    assert!(!ForgeQueryGraphTouchSelector::relation_kind_id(7).matches_descriptor(&descriptor));
    assert!(!collection_selector.matches_descriptor(&descriptor));
}

#[test]
fn declared_mutation_collection_selector_requires_all_declared_facts() {
    let selector = ForgeQueryGraphTouchSelector::declared_mutation_collection(
        "topology.primitive_birth",
        ForgeQueryMutationFamily::Insert,
        ["set:topology.kind"],
        ["topology.kind"],
    )
    .expect("declared mutation selector");
    let matching = ForgeQueryGraphTouchDescriptor::declared_mutation_collection(
        "topology.primitive_birth",
        ForgeQueryMutationFamily::Insert,
        None,
        ["set:topology.kind", "set:topology.structure"],
        ["topology.kind", "topology.structure"],
    )
    .expect("matching descriptor");
    let wrong_mutation = ForgeQueryGraphTouchDescriptor::declared_mutation_collection(
        "topology.primitive_birth",
        ForgeQueryMutationFamily::Update,
        None,
        ["set:topology.kind"],
        ["topology.kind"],
    )
    .expect("wrong mutation descriptor");
    let wrong_aspect = ForgeQueryGraphTouchDescriptor::declared_mutation_collection(
        "topology.primitive_birth",
        ForgeQueryMutationFamily::Insert,
        None,
        ["set:topology.structure"],
        ["topology.structure"],
    )
    .expect("wrong aspect descriptor");
    let read_shape = ForgeQueryGraphTouchDescriptor::read_family(
        "topology.primitive_birth",
        [ForgeQueryGraphTouchReadVerb::ObservesCollection],
    )
    .expect("read descriptor");

    assert!(selector.matches_descriptor(&matching));
    assert!(!selector.matches_descriptor(&wrong_mutation));
    assert!(!selector.matches_descriptor(&wrong_aspect));
    assert!(!selector.matches_descriptor(&read_shape));
}

#[test]
fn operating_world_selectors_do_not_cross_match_unrelated_lanes() {
    let preview = ForgeQueryGraphObligationOperatingWorldSelector::preview();
    let branch = ForgeQueryGraphObligationOperatingWorldSelector::branch();
    let committed = ForgeQueryGraphObligationOperatingWorldSelector::any_committed_authority();
    let any = ForgeQueryGraphObligationOperatingWorldSelector::any_operating_world();

    assert!(preview.matches_operating_world(preview));
    assert!(any.matches_operating_world(preview));
    assert!(any.matches_operating_world(branch));

    assert!(!preview.matches_operating_world(branch));
    assert!(!branch.matches_operating_world(preview));
    assert!(!committed.matches_operating_world(preview));
}
