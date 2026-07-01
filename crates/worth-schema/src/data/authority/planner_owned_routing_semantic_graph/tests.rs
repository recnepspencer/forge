use super::{
    admit_planner_decision_trace_identity, admit_planner_derived_diagnostic_contract_identity,
    admit_planner_public_proof_identity, admit_planner_selected_family_identity,
    admit_planner_selected_product_identity, admit_planner_selected_route_identity,
    admit_planner_witness_identity,
    admitted_explanation_input::admit_planner_admitted_explanation_input, PlannerMismatchLocus,
    PlannerWitnessRole,
};

#[test]
fn planner_explanation_identity_preserves_authority_distinctions() {
    let input =
        admit_planner_admitted_explanation_input("worth-topo", "selected-route-packet-digest")
            .expect("input");
    let family =
        admit_planner_selected_family_identity(&input, "query-backed-consumer").expect("family");
    let route =
        admit_planner_selected_route_identity(&family, "ordinary-query-route").expect("route");
    let product = admit_planner_selected_product_identity(&route, "topology-query-closeout")
        .expect("product");
    let witness = admit_planner_witness_identity(
        &route,
        PlannerWitnessRole::DenialOrAdvisory,
        PlannerMismatchLocus::SelectedProduct,
        "same textual reason",
    )
    .expect("witness");
    let trace =
        admit_planner_decision_trace_identity(&route, "same textual reason").expect("trace");
    let public_proof = admit_planner_public_proof_identity(&route, &product, "same textual reason")
        .expect("public proof");
    let diagnostic =
        admit_planner_derived_diagnostic_contract_identity(&input, "same textual reason")
            .expect("diagnostic");

    assert_ne!(family.identity_digest(), route.identity_digest());
    assert_ne!(route.identity_digest(), product.identity_digest());
    assert_ne!(route.identity_digest(), witness.identity_digest());
    assert_ne!(witness.identity_digest(), trace.identity_digest());
    assert_ne!(trace.identity_digest(), public_proof.identity_digest());
    assert_ne!(public_proof.identity_digest(), diagnostic.identity_digest());
}

#[test]
fn rendered_strings_cannot_mint_route_explanation_identity() {
    let input = admit_planner_admitted_explanation_input("worth-spatial", "planner-packet-digest")
        .expect("input");
    let family = admit_planner_selected_family_identity(&input, "evidence-lookup").expect("family");
    let route =
        admit_planner_selected_route_identity(&family, "lookup-selected-route").expect("route");
    let product = admit_planner_selected_product_identity(&route, "evidence-lookup-closeout")
        .expect("product");

    let first =
        admit_planner_public_proof_identity(&route, &product, "rendered-output").expect("proof");
    let second =
        admit_planner_public_proof_identity(&route, &product, "rendered-output").expect("proof");
    let witness = admit_planner_witness_identity(
        &route,
        PlannerWitnessRole::QuerySupportPosture,
        PlannerMismatchLocus::QuerySupportPosture,
        "rendered-output",
    )
    .expect("witness");

    assert_eq!(first.identity_digest(), second.identity_digest());
    assert_ne!(first.identity_digest(), witness.identity_digest());
}
