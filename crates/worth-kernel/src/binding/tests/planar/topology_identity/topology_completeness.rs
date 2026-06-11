use topology::facade::{
    prepare_primitive_construction_query_receipt, TopologyConstructionQueryFactKind,
    TopologyPrimitiveConstructionBirthFamily, TopologyPrimitiveConstructionQueryBirthSynopsis,
};
use worth_primitives::{PrimitiveConstructionFamilyContractRegistry, PrimitiveWitnessDescriptor};
use worth_spatial::facade::planar_topology_contract::{
    PlanarTopologyContractCompleteness, PlanarTopologyContractCompletenessContracts,
};

use super::super::bundle_closeout::runtime_handles::topology_contract_handle;

#[test]
fn kernel_consumes_topology_completeness_without_raw_topology_spelunking() {
    let topology_receipt = prepare_primitive_construction_query_receipt(
        &TopologyPrimitiveConstructionQueryBirthSynopsis::new(
            TopologyPrimitiveConstructionBirthFamily::ShellWithHole,
            PrimitiveConstructionFamilyContractRegistry::contract_for(
                &PrimitiveWitnessDescriptor::ShellWithHole {
                    outer_loop_edge_count: 4,
                    hole_loop_edge_counts: vec![4],
                },
            ),
            "kernel-topology-scaffold".to_string(),
            "topology:kernel-completeness".to_string(),
            "planar_shell_with_hole_body".to_string(),
            8,
            8,
            2,
            0,
            1,
            1,
            1,
        ),
    )
    .expect("topology query receipt");

    assert!(topology_receipt
        .row_for(TopologyConstructionQueryFactKind::ValidationSurface)
        .is_some());

    let receipt = PlanarTopologyContractCompleteness::from_topology_query_receipt(topology_receipt)
        .consume_declared_topology_surfaces("topology.query.declared-surfaces:kernel")
        .within_planar_neighborhood("neighborhood:kernel-topology-completeness")
        .compile(&PlanarTopologyContractCompletenessContracts::new(
            topology_contract_handle(),
        ))
        .expect("topology completeness plan")
        .certify()
        .expect("topology completeness receipt");

    assert_eq!(
        receipt.basis().topology_basis_identity(),
        "topology:kernel-completeness"
    );
    assert_eq!(receipt.counters().inspected_required_fact_rows(), 7);
    assert!(!receipt.fact_digest().is_empty());
}
