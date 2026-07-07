use crate::authored_refs::SpatialFrameRef;
use worth_geom::facade::tangent_frame;
use worth_math::{FinitePoint3, UnitVector3};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpatialFrameBasis {
    origin: [f64; 3],
    u_axis: [f64; 3],
    v_axis: [f64; 3],
    w_axis: [f64; 3],
}

impl SpatialFrameBasis {
    pub fn origin(&self) -> [f64; 3] {
        self.origin
    }

    pub fn u_axis(&self) -> [f64; 3] {
        self.u_axis
    }

    pub fn v_axis(&self) -> [f64; 3] {
        self.v_axis
    }

    pub fn w_axis(&self) -> [f64; 3] {
        self.w_axis
    }

    pub fn embed_point(&self, local: [f64; 3]) -> [f64; 3] {
        [
            self.origin[0]
                + self.u_axis[0] * local[0]
                + self.v_axis[0] * local[1]
                + self.w_axis[0] * local[2],
            self.origin[1]
                + self.u_axis[1] * local[0]
                + self.v_axis[1] * local[1]
                + self.w_axis[1] * local[2],
            self.origin[2]
                + self.u_axis[2] * local[0]
                + self.v_axis[2] * local[1]
                + self.w_axis[2] * local[2],
        ]
    }

    pub fn project_vector(&self, world: [f64; 3]) -> [f64; 3] {
        [
            self.u_axis[0] * world[0] + self.u_axis[1] * world[1] + self.u_axis[2] * world[2],
            self.v_axis[0] * world[0] + self.v_axis[1] * world[1] + self.v_axis[2] * world[2],
            self.w_axis[0] * world[0] + self.w_axis[1] * world[1] + self.w_axis[2] * world[2],
        ]
    }

    pub fn project_point(&self, world: [f64; 3]) -> [f64; 3] {
        self.project_vector([
            world[0] - self.origin[0],
            world[1] - self.origin[1],
            world[2] - self.origin[2],
        ])
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AdmittedSpatialFrameRef {
    spec: SpatialFrameRef,
    basis: SpatialFrameBasis,
}

impl AdmittedSpatialFrameRef {
    pub fn spec(&self) -> &SpatialFrameRef {
        &self.spec
    }

    pub fn basis(&self) -> SpatialFrameBasis {
        self.basis
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpatialFrameError {
    NonFiniteOrigin,
    InvalidNormal,
}

impl std::fmt::Display for SpatialFrameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NonFiniteOrigin => write!(f, "frame origin must stay finite"),
            Self::InvalidNormal => write!(f, "frame normal must stay finite and non-zero"),
        }
    }
}

impl std::error::Error for SpatialFrameError {}

pub(crate) fn admit_spatial_frame(
    spec: SpatialFrameRef,
) -> Result<AdmittedSpatialFrameRef, SpatialFrameError> {
    let basis = match &spec {
        SpatialFrameRef::World | SpatialFrameRef::ShapeLocal => SpatialFrameBasis {
            origin: [0.0, 0.0, 0.0],
            u_axis: [1.0, 0.0, 0.0],
            v_axis: [0.0, 1.0, 0.0],
            w_axis: [0.0, 0.0, 1.0],
        },
        SpatialFrameRef::Workplane { origin, normal, .. }
        | SpatialFrameRef::FeatureLocal { origin, normal, .. } => {
            let origin = FinitePoint3::try_new(*origin)
                .map(FinitePoint3::as_array)
                .map_err(|_| SpatialFrameError::NonFiniteOrigin)?;
            let w_axis = UnitVector3::try_new(*normal)
                .map(UnitVector3::as_array)
                .map_err(|_| SpatialFrameError::InvalidNormal)?;
            let (u_axis, v_axis) = tangent_frame(&w_axis);
            SpatialFrameBasis {
                origin,
                u_axis,
                v_axis,
                w_axis,
            }
        }
    };
    Ok(AdmittedSpatialFrameRef { spec, basis })
}

#[cfg(test)]
mod tests {
    use super::admit_spatial_frame;
    use crate::authored_refs::SpatialFrameRef;

    #[test]
    fn admitted_spatial_frame_embeds_workplane_local_coordinates() {
        let frame = admit_spatial_frame(SpatialFrameRef::workplane(
            "wp-1",
            [10.0, 0.0, 3.0],
            [1.0, 0.0, 0.0],
        ))
        .expect("frame");

        assert_eq!(frame.basis().origin(), [10.0, 0.0, 3.0]);
        assert_eq!(frame.basis().w_axis(), [1.0, 0.0, 0.0]);
        assert_eq!(frame.basis().embed_point([0.0, 0.0, 2.0]), [12.0, 0.0, 3.0]);
    }
}
