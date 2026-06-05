use crate::construction::request::{
    PrimitiveConstructionGeometry, PrimitiveConstructionPhaseError, PrimitiveConstructionRequest,
};
use worth_spatial::facade::birth::PrimitiveConstructionBirthScaffoldInput;
use worth_spatial::facade::placement::AdmittedSpatialPlacement;

mod birth_scaffold;
mod error_mapping;
mod families;
pub(crate) mod geometry;
mod scalar_admission;
mod spatial_family_bridge;
mod topology_counts;

pub(super) fn build_family_birth_input(
    request: &PrimitiveConstructionRequest,
    placement: &AdmittedSpatialPlacement,
    intent_digest: &str,
) -> Result<PrimitiveConstructionBirthScaffoldInput, PrimitiveConstructionPhaseError> {
    match request.geometry() {
        PrimitiveConstructionGeometry::SimplexSolid {
            scale,
            auxiliary_altitude_component,
            ..
        } => families::build_simplex_solid_birth_input(
            placement,
            intent_digest,
            *scale,
            *auxiliary_altitude_component,
        ),
        PrimitiveConstructionGeometry::Orthotope { half_extents, .. } => {
            families::build_orthotope_birth_input(placement, intent_digest, *half_extents)
        }
        PrimitiveConstructionGeometry::RegularPrism {
            sides,
            radius,
            height,
            ..
        } => families::build_regular_prism_birth_input(
            placement,
            intent_digest,
            *sides,
            *radius,
            *height,
        ),
        PrimitiveConstructionGeometry::RegularPyramid {
            sides,
            radius,
            height,
            ..
        } => families::build_regular_pyramid_birth_input(
            placement,
            intent_digest,
            *sides,
            *radius,
            *height,
        ),
        PrimitiveConstructionGeometry::WireBody { edge_count, .. } => {
            families::build_wire_body_birth_input(placement, intent_digest, *edge_count)
        }
        PrimitiveConstructionGeometry::ShellWithHole {
            outer_loop_edge_count,
            hole_loop_edge_counts,
            ..
        } => families::build_shell_with_hole_birth_input(
            placement,
            intent_digest,
            *outer_loop_edge_count,
            hole_loop_edge_counts,
        ),
    }
}
