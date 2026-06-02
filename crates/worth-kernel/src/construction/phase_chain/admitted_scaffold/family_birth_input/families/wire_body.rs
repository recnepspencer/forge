use super::super::birth_scaffold::{
    lower_family_birth_scaffold_plan, PrimitiveConstructionBirthScaffoldPlan,
};
use super::super::error_mapping::map_support_plane;
use super::super::geometry::{planar_support_plane, wire_body_vertices};
use super::super::scalar_admission::admit_polygon_edge_count;
use super::super::topology_counts::PrimitiveConstructionTopologyCounts;
use crate::construction::request::{PrimitiveConstructionFamily, PrimitiveConstructionPhaseError};
use worth_spatial::facade::{AdmittedSpatialPlacement, PrimitiveConstructionBirthScaffoldInput};

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
    lower_family_birth_scaffold_plan(
        intent_digest,
        placement,
        PrimitiveConstructionBirthScaffoldPlan::from_direct_planar_support(
            PrimitiveConstructionFamily::WireBody,
            "wire_body",
            support_planes,
            wire_body_vertices(admitted.edge_count, 1.5),
            PrimitiveConstructionTopologyCounts::new(
                admitted.edge_count as usize,
                admitted.edge_count as usize,
                1,
                1,
                0,
                0,
                1,
            ),
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
