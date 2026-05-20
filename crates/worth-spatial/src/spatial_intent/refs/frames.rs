use worth_geom::facade::tangent_frame;

#[derive(Clone, Debug, PartialEq)]
pub enum SpatialFrameRef {
    World,
    ShapeLocal,
    Workplane {
        name: String,
        origin: [f64; 3],
        normal: [f64; 3],
    },
    FeatureLocal {
        name: String,
        origin: [f64; 3],
        normal: [f64; 3],
    },
}

impl SpatialFrameRef {
    pub fn world() -> Self {
        Self::World
    }

    pub fn shape_local() -> Self {
        Self::ShapeLocal
    }

    pub fn workplane(name: impl Into<String>, origin: [f64; 3], normal: [f64; 3]) -> Self {
        Self::Workplane {
            name: name.into(),
            origin,
            normal,
        }
    }

    pub fn feature_local(name: impl Into<String>, origin: [f64; 3], normal: [f64; 3]) -> Self {
        Self::FeatureLocal {
            name: name.into(),
            origin,
            normal,
        }
    }
}

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

    pub fn embed_vector(&self, local: [f64; 3]) -> [f64; 3] {
        [
            self.u_axis[0] * local[0] + self.v_axis[0] * local[1] + self.w_axis[0] * local[2],
            self.u_axis[1] * local[0] + self.v_axis[1] * local[1] + self.w_axis[1] * local[2],
            self.u_axis[2] * local[0] + self.v_axis[2] * local[1] + self.w_axis[2] * local[2],
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

pub fn admit_spatial_frame(
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
            if origin.iter().any(|value| !value.is_finite()) {
                return Err(SpatialFrameError::NonFiniteOrigin);
            }
            let w_axis = normalize(*normal).ok_or(SpatialFrameError::InvalidNormal)?;
            let (u_axis, v_axis) = tangent_frame(&w_axis);
            SpatialFrameBasis {
                origin: *origin,
                u_axis,
                v_axis,
                w_axis,
            }
        }
    };
    Ok(AdmittedSpatialFrameRef { spec, basis })
}

fn normalize(vector: [f64; 3]) -> Option<[f64; 3]> {
    if vector.iter().any(|value| !value.is_finite()) {
        return None;
    }
    let magnitude_sq = vector[0] * vector[0] + vector[1] * vector[1] + vector[2] * vector[2];
    if magnitude_sq <= f64::MIN_POSITIVE {
        return None;
    }
    let magnitude = magnitude_sq.sqrt();
    Some([
        vector[0] / magnitude,
        vector[1] / magnitude,
        vector[2] / magnitude,
    ])
}

#[cfg(test)]
mod tests {
    use super::{admit_spatial_frame, SpatialFrameRef};

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
