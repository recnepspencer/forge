const FIXTURES: &[&str] = &[
    "tests/fixtures/planner_owned_routing_semantic_graph/public_constructors_not_exported.rs",
    "tests/fixtures/planner_owned_routing_semantic_graph/internal_constructors_not_exported_without_feature.rs",
    "tests/fixtures/planner_owned_routing_semantic_graph/public_selected_route_not_from_raw_strings.rs",
    "tests/fixtures/planner_owned_routing_semantic_graph/public_selected_route_not_from_truth_basis_digest.rs",
    "tests/fixtures/planner_owned_routing_semantic_graph/public_witness_not_from_raw_strings.rs",
];

#[test]
fn planner_owned_routing_public_facade_rejects_raw_identity_minting() {
    let trybuild = trybuild::TestCases::new();
    for fixture in FIXTURES {
        trybuild.compile_fail(fixture);
    }
}
