use topology::facade::{
    prepare_primitive_construction_query_receipt, TopologyConstructionQueryReceiptError,
};
use worth_spatial::facade::planar_topology_contract::{
    PlanarTopologyContractCompleteness, PlanarTopologyContractCompletenessBasis,
    PlanarTopologyContractCompletenessContracts, PlanarTopologyContractCompletenessDenialKind,
};

use super::contract_subject::{
    malformed_shell_with_missing_loop_basis_synopsis, topology_query_receipt, DECLARED_SURFACES,
    NEIGHBORHOOD,
};
use super::runtime_handles::topology_contract_handle;

#[test]
fn topology_to_spatial_planar_contract_completeness_blocks_incomplete_loop_shell_and_orientation_basis(
) {
    assert_spatial_denies_incomplete_topology_contract(
        "topology-missing-shell",
        topology_query_receipt("topology-missing-shell", 1, 0, 0, 1),
        PlanarTopologyContractCompletenessDenialKind::MissingShellBasis,
    );
    assert_malformed_shell_counts_rejected_before_spatial_contract();

    let complete_topology_contract =
        PlanarTopologyContractCompleteness::from_topology_query_receipt(topology_query_receipt(
            "topology-complete-orientation-basis",
            2,
            1,
            1,
            1,
        ))
        .consume_declared_topology_surfaces(DECLARED_SURFACES)
        .within_planar_neighborhood(NEIGHBORHOOD)
        .compile(&PlanarTopologyContractCompletenessContracts::new(
            topology_contract_handle("topology-complete-orientation-basis"),
        ))
        .expect("complete topology basis should reach planar contract certification")
        .certify()
        .expect("complete topology receipt");
    assert_eq!(
        complete_topology_contract
            .basis()
            .topology_query_receipt()
            .row_for(topology::facade::TopologyConstructionQueryFactKind::FaceOrientation)
            .expect("face orientation row")
            .fact_count(),
        1
    );
}

#[test]
fn topology_query_boundary_rejects_malformed_shell_counts_before_spatial_contract() {
    assert_malformed_shell_counts_rejected_before_spatial_contract();
}

fn assert_malformed_shell_counts_rejected_before_spatial_contract() {
    let malformed_shell = malformed_shell_with_missing_loop_basis_synopsis();
    let error = prepare_primitive_construction_query_receipt(&malformed_shell)
        .expect_err("malformed shell counts must not receive a topology Query receipt");
    match error {
        TopologyConstructionQueryReceiptError::UnsupportedBirthClass(reason) => {
            assert_eq!(
                reason,
                "only admitted primitive construction birth plans may cross the topology Query-native construction receipt boundary"
            );
        }
    }
}

#[test]
fn topology_contract_completeness_reports_missing_query_owned_receipt() {
    let denial = match PlanarTopologyContractCompletenessBasis::builder()
        .declared_query_surface(DECLARED_SURFACES)
        .planar_neighborhood(NEIGHBORHOOD)
        .build()
    {
        Ok(_) => panic!("missing topology receipt must deny"),
        Err(denial) => denial,
    };

    assert_eq!(
        denial.kind(),
        PlanarTopologyContractCompletenessDenialKind::MissingTopologyReceipt
    );
    assert_eq!(denial.kind().as_str(), "missing-topology-receipt");
}

#[test]
fn topology_contract_completeness_requires_declared_query_surface_and_neighborhood() {
    let missing_surface = match PlanarTopologyContractCompleteness::from_topology_query_receipt(
        topology_query_receipt("topology-missing-surface", 2, 1, 1, 1),
    )
    .within_planar_neighborhood(NEIGHBORHOOD)
    .compile(&PlanarTopologyContractCompletenessContracts::new(
        topology_contract_handle("topology-missing-surface"),
    )) {
        Ok(_) => panic!("missing declared Query surface must deny"),
        Err(denial) => denial,
    };
    assert_eq!(
        missing_surface.kind(),
        PlanarTopologyContractCompletenessDenialKind::MissingDeclaredQuerySurface
    );

    let missing_neighborhood =
        match PlanarTopologyContractCompleteness::from_topology_query_receipt(
            topology_query_receipt("topology-missing-neighborhood", 2, 1, 1, 1),
        )
        .consume_declared_topology_surfaces(DECLARED_SURFACES)
        .compile(&PlanarTopologyContractCompletenessContracts::new(
            topology_contract_handle("topology-missing-neighborhood"),
        )) {
            Ok(_) => panic!("missing neighborhood must deny"),
            Err(denial) => denial,
        };
    assert_eq!(
        missing_neighborhood.kind(),
        PlanarTopologyContractCompletenessDenialKind::MissingNeighborhoodBasis
    );
}

fn assert_spatial_denies_incomplete_topology_contract(
    world: &'static str,
    receipt: topology::facade::TopologyPrimitiveConstructionQueryReceipt,
    expected: PlanarTopologyContractCompletenessDenialKind,
) {
    let denial = match PlanarTopologyContractCompleteness::from_topology_query_receipt(receipt)
        .consume_declared_topology_surfaces(DECLARED_SURFACES)
        .within_planar_neighborhood(NEIGHBORHOOD)
        .compile(&PlanarTopologyContractCompletenessContracts::new(
            topology_contract_handle(world),
        )) {
        Ok(_) => panic!("incomplete topology must deny before spatial facts"),
        Err(denial) => denial,
    };

    assert_eq!(denial.kind(), expected);
    assert_eq!(denial.counters().rejected_missing_fact_rows(), 1);
}
