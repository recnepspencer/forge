use topology::facade::TopologyConstructionQueryFactKind;
use worth_spatial::facade::planar_topology_contract::{
    PlanarTopologyContractCompleteness, PlanarTopologyContractCompletenessContracts,
};

use super::contract_subject::{
    complete_topology_contract_receipt, topology_query_receipt, DECLARED_SURFACES, NEIGHBORHOOD,
    TOPOLOGY,
};
use super::runtime_handles::topology_contract_handle;

#[test]
fn topology_query_projection_consumption_feeds_planar_contract_without_raw_topology_spelunking() {
    let topology_receipt = topology_query_receipt("topology-query-consumption", 2, 1, 1, 1);
    assert_required_topology_contract_fact_floor(&topology_receipt);

    let contracts = PlanarTopologyContractCompletenessContracts::new(topology_contract_handle(
        "topology-query-consumption",
    ));
    let plan = PlanarTopologyContractCompleteness::from_topology_query_receipt(topology_receipt)
        .consume_declared_topology_surfaces(DECLARED_SURFACES)
        .within_planar_neighborhood(NEIGHBORHOOD)
        .compile(&contracts)
        .expect("topology completeness plan");

    assert_eq!(plan.inspected_topology_rows(), 14);
    let receipt = plan.certify().expect("topology completeness receipt");
    assert_eq!(receipt.basis().topology_basis_identity(), TOPOLOGY);
    assert_eq!(receipt.counters().inspected_topology_fact_rows(), 12);
    assert_eq!(receipt.counters().inspected_required_fact_rows(), 7);
    assert!(!receipt.fact_digest().is_empty());
}

#[test]
fn mb_m6_2_high_valence_contract_runs_topology_completeness_before_identity() {
    let receipt = complete_topology_contract_receipt("topology-high-valence");

    assert_eq!(
        receipt
            .basis()
            .topology_query_receipt()
            .row_for(TopologyConstructionQueryFactKind::LoopClosure)
            .expect("loop closure")
            .fact_count(),
        2
    );
    assert_eq!(
        receipt.basis().declared_query_surface_identity(),
        DECLARED_SURFACES
    );
}

fn assert_required_topology_contract_fact_floor(
    receipt: &topology::facade::TopologyPrimitiveConstructionQueryReceipt,
) {
    assert_topology_fact_count(
        receipt,
        TopologyConstructionQueryFactKind::LoopMembership,
        2,
    );
    assert_topology_fact_count(receipt, TopologyConstructionQueryFactKind::LoopClosure, 2);
    assert_topology_fact_count(
        receipt,
        TopologyConstructionQueryFactKind::ShellMembership,
        1,
    );
    assert_topology_fact_count(receipt, TopologyConstructionQueryFactKind::ShellClosure, 1);
    assert_topology_fact_count(
        receipt,
        TopologyConstructionQueryFactKind::FaceOrientation,
        1,
    );
    assert_topology_fact_count(
        receipt,
        TopologyConstructionQueryFactKind::PlanarNeighborhoodBasis,
        1,
    );
    assert_topology_fact_count(
        receipt,
        TopologyConstructionQueryFactKind::ValidationSurface,
        1,
    );
}

fn assert_topology_fact_count(
    receipt: &topology::facade::TopologyPrimitiveConstructionQueryReceipt,
    kind: TopologyConstructionQueryFactKind,
    expected_count: usize,
) {
    assert_eq!(
        receipt
            .row_for(kind)
            .unwrap_or_else(|| panic!("missing topology fact row: {}", kind.as_str()))
            .fact_count(),
        expected_count
    );
}
