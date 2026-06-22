use topology::facade::{
    prepare_primitive_construction_query_admitted_handoff_from_synopsis,
    TopologyConstructionQueryAdmittedHandoffError, TopologyPrimitiveConstructionBirthFamily,
    TopologyPrimitiveConstructionQueryAdmittedHandoff,
    TopologyPrimitiveConstructionQueryBirthSynopsis,
};
use worth_primitives::PrimitiveConstructionFamilyKey;

use super::PrimitiveConstructionAdmittedBirthTopologyTruth;

pub(super) fn prepare_primitive_construction_topology_ready_birth(
    birth_topology_truth: &PrimitiveConstructionAdmittedBirthTopologyTruth,
) -> Result<
    TopologyPrimitiveConstructionQueryAdmittedHandoff,
    TopologyConstructionQueryAdmittedHandoffError,
> {
    build_topology_query_admitted_handoff(birth_topology_truth)
}

pub(crate) fn prepare_primitive_construction_topology_query_admitted_handoff(
    birth_topology_truth: &PrimitiveConstructionAdmittedBirthTopologyTruth,
) -> Result<
    TopologyPrimitiveConstructionQueryAdmittedHandoff,
    TopologyConstructionQueryAdmittedHandoffError,
> {
    build_topology_query_admitted_handoff(birth_topology_truth)
}

fn build_topology_query_admitted_handoff(
    birth_topology_truth: &PrimitiveConstructionAdmittedBirthTopologyTruth,
) -> Result<
    TopologyPrimitiveConstructionQueryAdmittedHandoff,
    TopologyConstructionQueryAdmittedHandoffError,
> {
    let topology_query_birth_synopsis = build_topology_query_birth_synopsis(birth_topology_truth);
    prepare_primitive_construction_query_admitted_handoff_from_synopsis(
        &topology_query_birth_synopsis,
        birth_topology_truth.consequence_digest(),
        birth_topology_truth.birth_mapping_digest(),
        birth_topology_truth.supported_loop_count(),
        birth_topology_truth.supported_body_count(),
    )
}

fn build_topology_query_birth_synopsis(
    birth_topology_truth: &PrimitiveConstructionAdmittedBirthTopologyTruth,
) -> TopologyPrimitiveConstructionQueryBirthSynopsis {
    TopologyPrimitiveConstructionQueryBirthSynopsis::new(
        topology_family_from_spatial_family(birth_topology_truth.family()),
        birth_topology_truth.birth_contract(),
        birth_topology_truth.scaffold_digest().to_string(),
        birth_topology_truth.birth_digest().to_string(),
        birth_topology_truth.topology_birth_class().to_string(),
        birth_topology_truth.supported_vertex_count(),
        birth_topology_truth.supported_edge_count(),
        birth_topology_truth.supported_loop_count(),
        birth_topology_truth.supported_wire_count(),
        birth_topology_truth.supported_face_count(),
        birth_topology_truth.supported_shell_count(),
        birth_topology_truth.supported_body_count(),
    )
}

fn topology_family_from_spatial_family(
    family: PrimitiveConstructionFamilyKey,
) -> TopologyPrimitiveConstructionBirthFamily {
    match family {
        PrimitiveConstructionFamilyKey::SimplexSolid => {
            TopologyPrimitiveConstructionBirthFamily::SimplexSolid
        }
        PrimitiveConstructionFamilyKey::Orthotope => {
            TopologyPrimitiveConstructionBirthFamily::Orthotope
        }
        PrimitiveConstructionFamilyKey::RegularPrism => {
            TopologyPrimitiveConstructionBirthFamily::RegularPrism
        }
        PrimitiveConstructionFamilyKey::RegularPyramid => {
            TopologyPrimitiveConstructionBirthFamily::RegularPyramid
        }
        PrimitiveConstructionFamilyKey::WireBody => {
            TopologyPrimitiveConstructionBirthFamily::WireBody
        }
        PrimitiveConstructionFamilyKey::ShellWithHole => {
            TopologyPrimitiveConstructionBirthFamily::ShellWithHole
        }
    }
}
