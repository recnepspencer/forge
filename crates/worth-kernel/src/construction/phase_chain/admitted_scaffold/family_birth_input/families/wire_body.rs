use super::super::birth_scaffold::{
    lower_family_birth_scaffold_plan, PrimitiveConstructionBirthScaffoldPlan,
};
use super::super::error_mapping::map_support_plane;
use super::super::geometry::{planar_support_plane, wire_body_vertices};
use super::super::scalar_admission::admit_polygon_edge_count;
use super::super::topology_counts::PrimitiveConstructionTopologyCounts;
use crate::construction::request::{PrimitiveConstructionFamily, PrimitiveConstructionPhaseError};
use worth_primitives::{PrimitiveConstructionFamilyContractRegistry, PrimitiveWitnessDescriptor};
use worth_spatial::facade::birth::PrimitiveConstructionBirthScaffoldInput;
use worth_spatial::facade::placement::AdmittedSpatialPlacement;

struct AdmittedWireBodyBirthParameters {
    edge_count: u32,
}

pub(in super::super) fn build_wire_body_birth_input(
    placement: &AdmittedSpatialPlacement,
    intent_digest: &str,
    edge_count: u32,
) -> Result<PrimitiveConstructionBirthScaffoldInput, PrimitiveConstructionPhaseError> {
    let admitted = admit_wire_body_birth_parameters(edge_count)?;
    let support_planes = vec![planar_support_plane().map_err(map_support_plane)?];
    let birth_contract = PrimitiveConstructionFamilyContractRegistry::contract_for(
        &PrimitiveWitnessDescriptor::WireBody {
            edge_count: admitted.edge_count,
        },
    );
    lower_family_birth_scaffold_plan(
        intent_digest,
        placement,
        PrimitiveConstructionBirthScaffoldPlan::from_direct_planar_support(
            PrimitiveConstructionFamily::WireBody,
            birth_contract,
            "wire_body",
            support_planes,
            wire_body_vertices(admitted.edge_count, 1.5),
            PrimitiveConstructionTopologyCounts::from_contract(birth_contract.topology_contract()),
        ),
    )
}

fn admit_wire_body_birth_parameters(
    edge_count: u32,
) -> Result<AdmittedWireBodyBirthParameters, PrimitiveConstructionPhaseError> {
    Ok(AdmittedWireBodyBirthParameters {
        edge_count: admit_polygon_edge_count(PrimitiveConstructionFamily::WireBody, edge_count)?,
    })
}
