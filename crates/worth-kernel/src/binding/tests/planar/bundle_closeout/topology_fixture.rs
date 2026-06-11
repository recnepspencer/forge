use topology::facade::{
    prepare_primitive_construction_query_receipt, TopologyPrimitiveConstructionBirthFamily,
    TopologyPrimitiveConstructionQueryBirthSynopsis,
};
use worth_primitives::{PrimitiveConstructionFamilyContractRegistry, PrimitiveWitnessDescriptor};
use worth_spatial::facade::planar_topology_contract::{
    PlanarTopologyContractCompleteness, PlanarTopologyContractCompletenessContracts,
    PlanarTopologyContractCompletenessReceipt,
};

use super::runtime_handles::topology_contract_handle;

pub(crate) fn kernel_topology_contract_receipt() -> PlanarTopologyContractCompletenessReceipt {
    let synopsis = TopologyPrimitiveConstructionQueryBirthSynopsis::new(
        TopologyPrimitiveConstructionBirthFamily::ShellWithHole,
        PrimitiveConstructionFamilyContractRegistry::contract_for(
            &PrimitiveWitnessDescriptor::ShellWithHole {
                outer_loop_edge_count: 4,
                hole_loop_edge_counts: vec![4],
            },
        ),
        "topology-scaffold:kernel-bundle".to_string(),
        "topology:kernel-bundle".to_string(),
        "planar_shell_with_hole_body".to_string(),
        8,
        8,
        2,
        0,
        1,
        1,
        1,
    );
    PlanarTopologyContractCompleteness::from_topology_query_receipt(
        prepare_primitive_construction_query_receipt(&synopsis).expect("topology query receipt"),
    )
    .consume_declared_topology_surfaces("topology.query.declared-surfaces:kernel-bundle")
    .within_planar_neighborhood("neighborhood:kernel-bundle")
    .compile(&PlanarTopologyContractCompletenessContracts::new(
        topology_contract_handle(),
    ))
    .expect("topology completeness plan")
    .certify()
    .expect("topology completeness receipt")
}
