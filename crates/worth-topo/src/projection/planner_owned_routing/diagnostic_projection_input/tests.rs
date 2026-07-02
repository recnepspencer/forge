use schema::facade::topology_authoring::MilestoneOnePrimitiveCase;

use crate::derived_invalidation_route_input::admit_topology_invalidation_route_input;
use crate::derived_topology::compiled_product_consumer_cutover::{
    build_derived_equivalence_contract, DerivedEquivalenceContractReport,
};
use crate::derived_topology::materialized_graph::TopologyMaterializer;
use crate::derived_topology::traversal_views::bootstrap_topology_interpretation;
use crate::replay_undo_semantic_graph::current_topology_invalidation_proof;
use crate::test_support::schema_topology_authoring_boundary::seed_milestone_one_primitive_through_schema_execution;
use crate::validation::reference_integrity::milestone_one_runtime_builder;

use super::{
    admit_topology_derived_read_diagnostic_input, derive_topology_validation_report,
    selected_route_authority::TopologyDerivedReadDiagnosticSelectedRouteAuthority,
};

#[test]
fn mismatched_selected_route_authority_cannot_mint_derived_read_diagnostic_input() {
    let proof = current_topology_invalidation_proof().expect("current invalidation proof");
    let route_input =
        admit_topology_invalidation_route_input(proof.touched_closure(), proof.selected_plan())
            .expect("route input");
    let (read_basis, materialized, interpreted, validation, expected_report) = diagnostic_world();
    let authority =
        authority_with_hostile_product_digest(&expected_report, "selected-route.current");

    let error = admit_topology_derived_read_diagnostic_input(
        &route_input,
        &authority,
        &read_basis,
        &materialized,
        &interpreted,
        &validation,
    )
    .expect_err("mismatched selected-route authority must fail");

    assert_eq!(
        error.detail(),
        format!(
            "derived-read diagnostic input rejected mismatched selected product identity: expected hostile-product, observed {}",
            expected_report
                .compiled_product_identity_digest()
                .expect("compiled product"),
        )
    );
}

fn diagnostic_world() -> (
    schema::facade::topology_authoring::DerivedTopologyReadBasis,
    crate::derived_topology::materialized_graph::MaterializedTopologyView,
    crate::derived_topology::traversal_views::InterpretedTopologyView,
    crate::validation::DerivedTopologyValidationReport,
    DerivedEquivalenceContractReport,
) {
    let mut runtime = milestone_one_runtime_builder()
        .expect(" milestone one runtime builder")
        .build();
    let verified = seed_milestone_one_primitive_through_schema_execution(
        &mut runtime,
        "phase-six-diagnostic-inputs",
        &MilestoneOnePrimitiveCase::SheetDisk { edge_count: 5 },
    )
    .expect("verified primitive");
    let read_view = runtime
        .read_truth()
        .read_snapshot(verified.snapshot())
        .expect("snapshot read");
    let materialized =
        TopologyMaterializer::materialize_from_truth(&read_view).expect("materialized");
    let interpreted = bootstrap_topology_interpretation(&materialized);
    let validation =
        derive_topology_validation_report(&materialized, &interpreted).expect("validation");
    let expected_report = build_derived_equivalence_contract(
        verified.read_basis(),
        &materialized,
        &interpreted,
        &validation,
    );

    (
        verified.read_basis().clone(),
        materialized,
        interpreted,
        validation,
        expected_report,
    )
}

fn authority_with_hostile_product_digest(
    report: &DerivedEquivalenceContractReport,
    selected_route_identity_digest: &str,
) -> TopologyDerivedReadDiagnosticSelectedRouteAuthority {
    TopologyDerivedReadDiagnosticSelectedRouteAuthority::from_selected_route_identities(
        selected_route_identity_digest,
        report
            .selected_equivalence_family_identity()
            .expect("selected family")
            .as_str(),
        "hostile-product",
        report
            .equivalence_policy_identity_digest()
            .expect("equivalence policy"),
        report
            .selected_compatibility_basis_identity_digest()
            .expect("compatibility basis"),
        report
            .selected_reuse_basis_identity_digest()
            .expect("reuse basis"),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    )
}
