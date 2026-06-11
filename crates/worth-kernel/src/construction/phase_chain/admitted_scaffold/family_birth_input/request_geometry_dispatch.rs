use super::super::super::request::{
    PrimitiveConstructionGeometry, PrimitiveConstructionPhaseError, PrimitiveConstructionRequest,
};

use super::{families, PrimitiveConstructionAdmittedBirthInput};

pub(super) fn build_request_geometry_birth_input(
    request: &PrimitiveConstructionRequest,
    intent_digest: &str,
) -> Result<PrimitiveConstructionAdmittedBirthInput, PrimitiveConstructionPhaseError> {
    let placement_spec = request.placement_spec();
    match request.geometry() {
        PrimitiveConstructionGeometry::SimplexSolid {
            scale,
            auxiliary_altitude_component,
            ..
        } => families::build_simplex_solid_birth_input(
            placement_spec,
            intent_digest,
            *scale,
            *auxiliary_altitude_component,
        ),
        PrimitiveConstructionGeometry::Orthotope { half_extents, .. } => {
            families::build_orthotope_birth_input(placement_spec, intent_digest, *half_extents)
        }
        PrimitiveConstructionGeometry::RegularPrism {
            sides,
            radius,
            height,
            ..
        } => families::build_regular_prism_birth_input(
            placement_spec,
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
            placement_spec,
            intent_digest,
            *sides,
            *radius,
            *height,
        ),
        PrimitiveConstructionGeometry::WireBody { edge_count, .. } => {
            families::build_wire_body_birth_input(placement_spec, intent_digest, *edge_count)
        }
        PrimitiveConstructionGeometry::ShellWithHole {
            outer_loop_edge_count,
            hole_loop_edge_counts,
            ..
        } => families::build_shell_with_hole_birth_input(
            placement_spec,
            intent_digest,
            *outer_loop_edge_count,
            hole_loop_edge_counts,
        ),
    }
}
