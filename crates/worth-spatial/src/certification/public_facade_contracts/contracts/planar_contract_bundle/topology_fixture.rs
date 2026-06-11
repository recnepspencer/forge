use topology::facade::{
    prepare_primitive_construction_query_receipt, TopologyPrimitiveConstructionBirthFamily,
    TopologyPrimitiveConstructionQueryBirthSynopsis,
};
use worth_primitives::{PrimitiveConstructionFamilyContractRegistry, PrimitiveWitnessDescriptor};
use worth_spatial::facade::planar_topology_contract::{
    PlanarTopologyContractCompleteness, PlanarTopologyContractCompletenessContracts,
    PlanarTopologyContractCompletenessReceipt,
};

use super::proof_fixture::{NEIGHBORHOOD, TOPOLOGY};
use super::runtime_handles::topology_contract_handle;

pub(crate) fn topology_contract_receipt(
    world: &'static str,
) -> PlanarTopologyContractCompletenessReceipt {
    let synopsis = TopologyPrimitiveConstructionQueryBirthSynopsis::new(
        TopologyPrimitiveConstructionBirthFamily::ShellWithHole,
        PrimitiveConstructionFamilyContractRegistry::contract_for(
            &PrimitiveWitnessDescriptor::ShellWithHole {
                outer_loop_edge_count: 4,
                hole_loop_edge_counts: vec![4],
            },
        ),
        "topology:bundle".to_string(),
        TOPOLOGY.to_string(),
        "planar_shell_with_hole_body".to_string(),
        8,
        8,
        2,
        0,
        1,
        1,
        1,
    );
    let topology_receipt =
        prepare_primitive_construction_query_receipt(&synopsis).expect("topology query receipt");
    PlanarTopologyContractCompleteness::from_topology_query_receipt(topology_receipt)
        .consume_declared_topology_surfaces("topology.query.declared-surfaces:bundle")
        .within_planar_neighborhood(NEIGHBORHOOD)
        .compile(&PlanarTopologyContractCompletenessContracts::new(
            topology_contract_handle(world),
        ))
        .expect("topology completeness plan")
        .certify()
        .expect("topology completeness receipt")
}
