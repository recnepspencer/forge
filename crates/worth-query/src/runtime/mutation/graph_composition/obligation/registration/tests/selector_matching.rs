use crate::runtime::{
    WorthQueryAspectMutationOperation, WorthQueryGraphObligationOperatingWorldSelector,
    WorthQueryGraphTouchDescriptor, WorthQueryGraphTouchLifecycleFamily,
    WorthQueryGraphTouchReadVerb, WorthQueryGraphTouchSelector, WorthQueryMutationFamily,
};
use worth_relational::facade::identity::KindId;

use super::fixtures::{
    symbolic_relation_touch_descriptor, symbolic_relation_touch_descriptor_with_relation_kind_id,
    touch,
};

#[test]
fn touch_selectors_do_not_cross_match_unrelated_graph_lanes() {
    let descriptor = symbolic_relation_touch_descriptor("topology.edge", "weight");

    assert!(WorthQueryGraphTouchSelector::relation_kind("topology.edge")
        .unwrap()
        .matches_descriptor(&descriptor));
    assert!(
        WorthQueryGraphTouchSelector::aspect_touch(touch("weight")).matches_descriptor(&descriptor)
    );
    let aspect_selector = WorthQueryGraphTouchSelector::aspect_touch(touch("weight"));
    assert_eq!(
        aspect_selector.terminal_selector_kind_for_boundary(),
        "aspect-touch"
    );
    assert_eq!(
        aspect_selector
            .terminal_selector_value_for_boundary()
            .as_deref(),
        Some("weight:<whole-aspect>")
    );
    assert!(
        WorthQueryGraphTouchSelector::mutation_family(WorthQueryMutationFamily::Delete)
            .matches_descriptor(&descriptor)
    );
    assert!(WorthQueryGraphTouchSelector::lifecycle_family(
        WorthQueryGraphTouchLifecycleFamily::SameBatchSymbolicRelationRetirement
    )
    .matches_descriptor(&descriptor));

    assert!(
        !WorthQueryGraphTouchSelector::relation_kind("topology.face")
            .unwrap()
            .matches_descriptor(&descriptor)
    );
    assert!(
        !WorthQueryGraphTouchSelector::aspect_touch(touch("capacity"))
            .matches_descriptor(&descriptor)
    );
    assert!(
        !WorthQueryGraphTouchSelector::mutation_family(WorthQueryMutationFamily::Update)
            .matches_descriptor(&descriptor)
    );
    assert!(!WorthQueryGraphTouchSelector::relation_kind_id(42).matches_descriptor(&descriptor));
}

#[test]
fn equivalent_selectors_share_stable_identity() {
    let left = WorthQueryGraphTouchSelector::relation_kind("topology.edge").unwrap();
    let right = WorthQueryGraphTouchSelector::collection("topology.edge").unwrap();

    assert_eq!(left.selector_digest(), right.selector_digest());
}

#[test]
fn relation_kind_id_selector_is_not_a_collection_string_alias() {
    let descriptor = symbolic_relation_touch_descriptor_with_relation_kind_id(
        "topology.edge",
        "weight",
        Some(KindId(42)),
    );
    let collection_selector = WorthQueryGraphTouchSelector::relation_kind("42").unwrap();
    let relation_kind_id_selector = WorthQueryGraphTouchSelector::relation_kind_id(42);

    assert_ne!(
        collection_selector.selector_digest(),
        relation_kind_id_selector.selector_digest()
    );
    assert_eq!(
        relation_kind_id_selector.terminal_selector_kind_for_boundary(),
        "relation-kind-id"
    );
    assert_eq!(
        relation_kind_id_selector
            .terminal_selector_value_for_boundary()
            .as_deref(),
        Some("42")
    );
    assert!(relation_kind_id_selector.matches_descriptor(&descriptor));
    assert!(WorthQueryGraphTouchSelector::relational_kind_id(KindId(42))
        .matches_descriptor(&descriptor));
    assert!(!WorthQueryGraphTouchSelector::relation_kind_id(7).matches_descriptor(&descriptor));
    assert!(!collection_selector.matches_descriptor(&descriptor));
}

#[test]
fn declared_mutation_collection_selector_requires_all_declared_facts() {
    let selector = WorthQueryGraphTouchSelector::declared_mutation_collection(
        "topology.primitive_birth",
        WorthQueryMutationFamily::Insert,
        [set_operation("topology.kind")],
        [touch("topology.kind")],
    )
    .expect("declared mutation selector");
    let matching = WorthQueryGraphTouchDescriptor::declared_mutation_collection(
        "topology.primitive_birth",
        WorthQueryMutationFamily::Insert,
        None,
        [
            set_operation("topology.kind"),
            set_operation("topology.structure"),
        ],
        [touch("topology.kind"), touch("topology.structure")],
    )
    .expect("matching descriptor");
    let wrong_mutation = WorthQueryGraphTouchDescriptor::declared_mutation_collection(
        "topology.primitive_birth",
        WorthQueryMutationFamily::Update,
        None,
        [set_operation("topology.kind")],
        [touch("topology.kind")],
    )
    .expect("wrong mutation descriptor");
    let wrong_aspect = WorthQueryGraphTouchDescriptor::declared_mutation_collection(
        "topology.primitive_birth",
        WorthQueryMutationFamily::Insert,
        None,
        [set_operation("topology.structure")],
        [touch("topology.structure")],
    )
    .expect("wrong aspect descriptor");
    let read_shape = WorthQueryGraphTouchDescriptor::read_family(
        "topology.primitive_birth",
        [WorthQueryGraphTouchReadVerb::ObservesCollection],
    )
    .expect("read descriptor");

    assert!(selector.matches_descriptor(&matching));
    assert!(!selector.matches_descriptor(&wrong_mutation));
    assert!(!selector.matches_descriptor(&wrong_aspect));
    assert!(!selector.matches_descriptor(&read_shape));
}

fn set_operation(touch_fixture: &str) -> WorthQueryAspectMutationOperation {
    WorthQueryAspectMutationOperation::set(touch(touch_fixture))
}

#[test]
fn operating_world_selectors_do_not_cross_match_unrelated_lanes() {
    let preview = WorthQueryGraphObligationOperatingWorldSelector::preview();
    let branch = WorthQueryGraphObligationOperatingWorldSelector::branch();
    let committed = WorthQueryGraphObligationOperatingWorldSelector::any_committed_authority();
    let any = WorthQueryGraphObligationOperatingWorldSelector::any_operating_world();

    assert!(preview.matches_operating_world(preview));
    assert!(any.matches_operating_world(preview));
    assert!(any.matches_operating_world(branch));

    assert!(!preview.matches_operating_world(branch));
    assert!(!branch.matches_operating_world(preview));
    assert!(!committed.matches_operating_world(preview));
}
