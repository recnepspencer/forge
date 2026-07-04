use super::{admit_topology_invalidation_route_input, current_topology_invalidation_route_input};
use crate::derived_topology::invalidation_plan::selection::selection_test_fixtures::{
    admitted_legality_support, admitted_query_support, catalog_closeout,
    unrelated_geometry_touched_closure,
};
use crate::derived_topology::invalidation_plan::selection::{
    DerivedInvalidationDensityPolicy, DerivedInvalidationSelectedPlan,
};
use crate::replay_undo_semantic_graph::current_topology_invalidation_proof;

#[test]
fn invalidation_route_input_carries_exact_touched_fact_identity_aspect_scope_and_selected_family() {
    let proof = current_topology_invalidation_proof()
        .expect("current topology invalidation proof should assemble");
    let explicit_route_input =
        admit_topology_invalidation_route_input(proof.touched_closure(), proof.selected_plan())
            .expect("explicit invalidation route input should admit");
    let current_route_input = current_topology_invalidation_route_input()
        .expect("current invalidation route input should admit");

    assert_eq!(
        current_route_input, explicit_route_input,
        "current helper should lower the same route input as explicit admitted authority",
    );
    assert_eq!(
        explicit_route_input.touched_closure_digest(),
        proof.touched_closure().closure_digest()
    );
    assert_eq!(
        explicit_route_input.selected_plan_digest(),
        proof.selected_plan().selected_plan_digest()
    );
    assert_eq!(
        explicit_route_input.routing_contract_digest(),
        proof.selected_plan().routing_contract().contract_digest()
    );
    assert_eq!(
        explicit_route_input.touched_closure().basis_digest(),
        proof.touched_closure().basis_digest()
    );
    assert_eq!(
        explicit_route_input
            .touched_closure()
            .touch_descriptor_digest(),
        proof.touched_closure().touch_descriptor_digest()
    );
    assert_eq!(
        explicit_route_input.touched_closure().counters(),
        proof.touched_closure().counters()
    );
    assert_eq!(
        explicit_route_input.selected_rows(),
        proof.selected_plan().selected_rows()
    );
    assert_eq!(
        explicit_route_input.execution_admission(),
        proof.selected_plan().execution_admission()
    );
    assert_eq!(
        explicit_route_input.touched_closure().semantic_family_key(),
        proof.touched_closure().semantic_family_key()
    );
}

#[test]
fn mismatched_selected_plan_cannot_mint_invalidation_route_input() {
    let proof = current_topology_invalidation_proof()
        .expect("current topology invalidation proof should assemble");
    let mismatched_selected_plan = unrelated_geometry_selected_plan();
    let error =
        admit_topology_invalidation_route_input(proof.touched_closure(), &mismatched_selected_plan)
            .expect_err("selected plan from a different touched closure should be rejected");

    assert!(error.detail().contains("touched closure digest"));
    assert!(error
        .detail()
        .contains(mismatched_selected_plan.touched_closure_digest()));
    assert!(error
        .detail()
        .contains(proof.touched_closure().closure_digest()));
}

fn unrelated_geometry_selected_plan() -> DerivedInvalidationSelectedPlan {
    DerivedInvalidationSelectedPlan::lower(
        &catalog_closeout(),
        &unrelated_geometry_touched_closure(),
        &admitted_query_support(),
        &admitted_legality_support(),
        DerivedInvalidationDensityPolicy::Sparse,
    )
    .expect("unrelated geometry selected plan should lower")
}
