use topology::facade::{
    prepare_primitive_construction_query_receipt, TopologyPrimitiveConstructionBirthFamily,
    TopologyPrimitiveConstructionQueryBirthSynopsis, TopologyPrimitiveConstructionQueryReceipt,
};
use worth_primitives::{PrimitiveConstructionFamilyContractRegistry, PrimitiveWitnessDescriptor};
use worth_spatial::facade::planar_topology_contract::{
    PlanarTopologyContractCompleteness, PlanarTopologyContractCompletenessContracts,
    PlanarTopologyContractCompletenessReceipt,
};

use super::runtime_handles::topology_contract_handle;

pub(crate) const DECLARED_SURFACES: &str = "topology.query.declared-surfaces:phase14";
pub(crate) const NEIGHBORHOOD: &str = "neighborhood:phase14";
pub(crate) const TOPOLOGY: &str = "topology:phase14";

pub(crate) fn topology_query_receipt(
    world: &'static str,
    loop_count: usize,
    face_count: usize,
    shell_count: usize,
    body_count: usize,
) -> TopologyPrimitiveConstructionQueryReceipt {
    let (family, birth_contract, topology_birth_class, vertex_count, edge_count, wire_count) =
        if face_count == 0 && shell_count == 0 {
            (
                TopologyPrimitiveConstructionBirthFamily::WireBody,
                PrimitiveConstructionFamilyContractRegistry::contract_for(
                    &PrimitiveWitnessDescriptor::WireBody { edge_count: 8 },
                ),
                "planar_wire_body",
                8,
                8,
                1,
            )
        } else {
            (
                TopologyPrimitiveConstructionBirthFamily::ShellWithHole,
                PrimitiveConstructionFamilyContractRegistry::contract_for(
                    &PrimitiveWitnessDescriptor::ShellWithHole {
                        outer_loop_edge_count: 4,
                        hole_loop_edge_counts: vec![4],
                    },
                ),
                "planar_shell_with_hole_body",
                8,
                8,
                0,
            )
        };
    let synopsis = TopologyPrimitiveConstructionQueryBirthSynopsis::new(
        family,
        birth_contract,
        format!("topology-scaffold:{world}"),
        TOPOLOGY.to_string(),
        topology_birth_class.to_string(),
        vertex_count,
        edge_count,
        loop_count,
        wire_count,
        face_count,
        shell_count,
        body_count,
    );
    prepare_primitive_construction_query_receipt(&synopsis).expect("topology query receipt")
}

pub(crate) fn complete_topology_contract_receipt(
    world: &'static str,
) -> PlanarTopologyContractCompletenessReceipt {
    PlanarTopologyContractCompleteness::from_topology_query_receipt(topology_query_receipt(
        world, 2, 1, 1, 1,
    ))
    .consume_declared_topology_surfaces(DECLARED_SURFACES)
    .within_planar_neighborhood(NEIGHBORHOOD)
    .compile(&PlanarTopologyContractCompletenessContracts::new(
        topology_contract_handle(world),
    ))
    .expect("topology completeness plan")
    .certify()
    .expect("topology completeness receipt")
}

pub(crate) fn malformed_shell_with_missing_loop_basis_synopsis(
) -> TopologyPrimitiveConstructionQueryBirthSynopsis {
    TopologyPrimitiveConstructionQueryBirthSynopsis::new(
        TopologyPrimitiveConstructionBirthFamily::ShellWithHole,
        PrimitiveConstructionFamilyContractRegistry::contract_for(
            &PrimitiveWitnessDescriptor::ShellWithHole {
                outer_loop_edge_count: 4,
                hole_loop_edge_counts: vec![4],
            },
        ),
        "topology-scaffold:malformed-shell".to_string(),
        "topology:malformed-shell".to_string(),
        "planar_shell_with_hole_body".to_string(),
        8,
        8,
        0,
        0,
        1,
        1,
        1,
    )
}
