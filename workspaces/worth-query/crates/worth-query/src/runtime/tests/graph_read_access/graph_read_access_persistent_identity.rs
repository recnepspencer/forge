use crate::runtime::WorthQueryGraphReadAccessRequirementKind;

use crate::runtime::tests::graph_read_access::support;

use support::graph_index_inventory::runtime_profiles::profile_requiring_store_backed_graph_index;
use support::graph_read_access::persistent_requirements::{
    persistent_predicate_family, persistent_requirement_digest_for_equality_family,
    persistent_requirement_digest_for_named_family,
    persistent_requirement_digest_for_reordered_family, persistent_requirement_workspace,
};

#[test]
fn identical_persistent_requirements_derive_identical_digests() {
    let first_digest = persistent_requirement_digest_for_named_family(
        "graph-read-access.phase-ten.equivalence.first",
        "phase-ten-equivalence",
    );
    let second_digest = persistent_requirement_digest_for_reordered_family(
        "graph-read-access.phase-ten.equivalence.second",
        "phase-ten-equivalence",
    );

    assert_eq!(first_digest, second_digest);
}

#[test]
fn persistent_requirement_identity_ignores_family_display_label() {
    let first_digest = persistent_requirement_digest_for_named_family(
        "graph-read-access.phase-ten.label.first",
        "phase-ten-label-a",
    );
    let second_digest = persistent_requirement_digest_for_named_family(
        "graph-read-access.phase-ten.label.second",
        "phase-ten-label-b",
    );

    assert_eq!(first_digest, second_digest);
}

#[test]
fn different_typed_requirement_identity_changes_persistent_requirement_digest() {
    let presence_digest = persistent_requirement_digest_for_named_family(
        "graph-read-access.phase-ten.near-miss-digest.presence",
        "phase-ten-near-miss-digest",
    );
    let equality_digest = persistent_requirement_digest_for_equality_family(
        "graph-read-access.phase-ten.near-miss-digest.equality",
        "phase-ten-near-miss-digest",
    );

    assert_ne!(presence_digest, equality_digest);
}

#[test]
fn family_index_contract_carries_requirement_identity_without_index_names() {
    let mut workspace = persistent_requirement_workspace(
        "graph-read-access.phase-ten.family-contract",
        profile_requiring_store_backed_graph_index(
            WorthQueryGraphReadAccessRequirementKind::PredicateSupport,
        ),
    );
    let family = persistent_predicate_family(&mut workspace, "phase-ten-family-contract");
    let admission = workspace
        .read_family_intent(&family)
        .review()
        .expect("review should derive")
        .graph_read_access_admission()
        .expect("admission should derive");
    let contract = admission.graph_read_family_index_contract();
    let declaration = admission
        .persistent_index_requirement()
        .expect("persistent declaration should exist");

    assert_eq!(
        contract.read_graph_digest(),
        admission.requirement_set().read_graph_digest().render_hex()
    );
    assert_eq!(
        contract.requirement_set_digest(),
        declaration.requirement_set_digest()
    );
    assert_eq!(
        contract.persistent_requirement_digest(),
        Some(declaration.digest())
    );
    assert_eq!(
        contract.requirement_row_digests().len(),
        admission.requirement_set().rows().len()
    );
    assert_eq!(
        declaration.read_graph_digest(),
        contract.read_graph_digest()
    );
    assert_eq!(
        declaration.access_shape_digest(),
        contract.access_shape_digest()
    );
    assert_eq!(
        declaration.selectivity_shape_digest(),
        contract.selectivity_shape_digest()
    );
}
