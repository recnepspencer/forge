use worth_geom::facade::tangent_frame;
use worth_math::UnitVector3;

use super::{PlanarLocalFrameBasis, PlanarLocalFrameDenial, PlanarLocalFrameDenialKind};

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct PlanarLocalFrameAxes {
    pub(crate) u_axis: [f64; 3],
    pub(crate) v_axis: [f64; 3],
    pub(crate) w_axis: [f64; 3],
}

pub(crate) fn derive_planar_local_frame_axes(
    basis: &PlanarLocalFrameBasis,
) -> Result<PlanarLocalFrameAxes, PlanarLocalFrameDenial> {
    let w_axis = UnitVector3::try_new(basis.normal())
        .map(UnitVector3::as_array)
        .map_err(|_| {
            PlanarLocalFrameDenial::new(
                PlanarLocalFrameDenialKind::InvalidNormal,
                "local frame normal must be finite and non-zero",
            )
        })?;
    let (u_axis, v_axis) = tangent_frame(&w_axis);
    Ok(PlanarLocalFrameAxes {
        u_axis,
        v_axis,
        w_axis,
    })
}
