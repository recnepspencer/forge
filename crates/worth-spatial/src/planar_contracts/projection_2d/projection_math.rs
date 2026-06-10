use worth_math::linalg::dot;

use super::{
    ProjectPointToCertifiedPlane2DBasis, ProjectPointToCertifiedPlane2DDenial,
    ProjectPointToCertifiedPlane2DDenialKind,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct CertifiedPlaneProjection2D {
    pub(crate) point_2d: [f64; 2],
    pub(crate) signed_distance_to_plane: f64,
}

pub(crate) fn project_point_to_certified_plane_2d(
    basis: &ProjectPointToCertifiedPlane2DBasis,
) -> Result<CertifiedPlaneProjection2D, ProjectPointToCertifiedPlane2DDenial> {
    let local_delta = basis.local_delta_from_frame_origin();
    let signed_distance_to_plane = normalize_signed_zero(dot(local_delta, basis.w_axis()));
    if signed_distance_to_plane.to_bits() != 0.0f64.to_bits() {
        return Err(ProjectPointToCertifiedPlane2DDenial::new(
            ProjectPointToCertifiedPlane2DDenialKind::OffPlanePoint,
            "certified plane projection requires an exactly on-plane local delta",
        ));
    }
    Ok(CertifiedPlaneProjection2D {
        point_2d: [
            normalize_signed_zero(dot(local_delta, basis.u_axis())),
            normalize_signed_zero(dot(local_delta, basis.v_axis())),
        ],
        signed_distance_to_plane,
    })
}

fn normalize_signed_zero(value: f64) -> f64 {
    if value == 0.0 {
        0.0
    } else {
        value
    }
}
