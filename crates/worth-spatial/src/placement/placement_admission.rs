#![cfg_attr(not(any(test, feature = "test-support-lowering")), allow(dead_code))]

#[cfg(test)]
use super::placement_types::SpatialPlacementGeometry;
use super::placement_types::{
    AdmittedSpatialPlacement, SpatialPlacementError, SpatialPlacementFrame,
};
use crate::authored_refs::{EmptySpatialWitnessCatalog, SpatialFrameRef, SpatialWitnessCatalog};
use crate::placement::SpatialPlacementSpec;
use crate::witness_resolution::admit_spatial_frame;
use crate::witness_resolution::witness_resolution::resolve_spatial_direction_witness_with_catalog;
#[cfg(test)]
use worth_geom::facade::Plane;

pub(crate) fn admit_spatial_placement(
    spec: SpatialPlacementSpec,
) -> Result<AdmittedSpatialPlacement, SpatialPlacementError> {
    admit_spatial_placement_with_catalog(spec, &EmptySpatialWitnessCatalog)
}

pub(crate) fn admit_spatial_placement_with_catalog(
    spec: SpatialPlacementSpec,
    catalog: &impl SpatialWitnessCatalog,
) -> Result<AdmittedSpatialPlacement, SpatialPlacementError> {
    if spec.origin().iter().any(|value| !value.is_finite()) {
        return Err(SpatialPlacementError::NonFiniteOrigin);
    }
    let reference_frame = admit_spatial_frame(spec.reference_frame().clone())
        .map_err(SpatialPlacementError::InvalidReferenceFrame)?;
    let resolved_direction_witness =
        resolve_spatial_direction_witness_with_catalog(spec.direction_witness().clone(), catalog)
            .map_err(SpatialPlacementError::DirectionWitnessFailure)?;
    let frame_basis = reference_frame.basis();
    let world_origin = frame_basis.embed_point(spec.origin());
    let world_w_axis = resolved_direction_witness.resolved_world_direction();
    let world_frame = admit_spatial_frame(SpatialFrameRef::workplane(
        "placement-facing",
        world_origin,
        world_w_axis,
    ))
    .map_err(SpatialPlacementError::InvalidReferenceFrame)?;
    Ok(AdmittedSpatialPlacement::from_parts(
        spec,
        SpatialPlacementFrame::new(
            world_origin,
            world_frame.basis().u_axis(),
            world_frame.basis().v_axis(),
            world_frame.basis().w_axis(),
        ),
        reference_frame,
        resolved_direction_witness,
    ))
}

#[cfg(test)]
pub(crate) fn apply_spatial_placement(
    placement: &AdmittedSpatialPlacement,
    support_planes: &[Plane],
    vertex_positions: &[[f64; 3]],
) -> Result<SpatialPlacementGeometry, SpatialPlacementError> {
    let support_planes = support_planes
        .iter()
        .map(|plane| embed_plane(placement, plane))
        .collect::<Result<Vec<_>, _>>()?;
    let vertex_positions = vertex_positions
        .iter()
        .copied()
        .map(|point| placement.embed_point(point))
        .collect();
    Ok(SpatialPlacementGeometry::from_parts(
        support_planes,
        vertex_positions,
    ))
}

#[cfg(test)]
fn embed_plane(
    placement: &AdmittedSpatialPlacement,
    plane: &Plane,
) -> Result<Plane, SpatialPlacementError> {
    let local_raw_normal = plane.raw_normal();
    let coefficient_scale = local_raw_normal[0]
        .abs()
        .max(local_raw_normal[1].abs())
        .max(local_raw_normal[2].abs());
    if !coefficient_scale.is_finite() || coefficient_scale <= f64::MIN_POSITIVE {
        return Err(SpatialPlacementError::InvalidEmbeddedPlane);
    }
    let normalized_local_normal = [
        local_raw_normal[0] / coefficient_scale,
        local_raw_normal[1] / coefficient_scale,
        local_raw_normal[2] / coefficient_scale,
    ];
    let normalized_local_offset = plane.raw_offset() / coefficient_scale;
    let world_raw_normal = placement.embed_vector(normalized_local_normal);
    let normal_length_sq = normalized_local_normal[0] * normalized_local_normal[0]
        + normalized_local_normal[1] * normalized_local_normal[1]
        + normalized_local_normal[2] * normalized_local_normal[2];
    if !normal_length_sq.is_finite() || normal_length_sq <= f64::MIN_POSITIVE {
        return Err(SpatialPlacementError::InvalidEmbeddedPlane);
    }
    let point_scale = -normalized_local_offset / normal_length_sq;
    let local_plane_point = [
        normalized_local_normal[0] * point_scale,
        normalized_local_normal[1] * point_scale,
        normalized_local_normal[2] * point_scale,
    ];
    let world_plane_point = placement.embed_point(local_plane_point);
    Plane::from_point_normal(world_plane_point, world_raw_normal)
        .map_err(|_| SpatialPlacementError::InvalidEmbeddedPlane)
}

#[cfg(test)]
#[path = "internal/placement_tests.rs"]
mod placement_tests;
