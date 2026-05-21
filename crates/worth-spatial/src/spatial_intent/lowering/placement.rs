use crate::spatial_intent::refs::{
    EmptySpatialWitnessCatalog, SpatialAxis, SpatialDirectionWitnessRef, SpatialFrameRef,
    SpatialWitnessCatalog,
};
use crate::spatial_intent::resolution::{
    admit_spatial_frame, resolve_spatial_direction_witness_with_catalog, AdmittedSpatialFrameRef,
    ResolvedSpatialDirectionWitness, SpatialFrameError, SpatialWitnessFailureClass,
};
use worth_geom::facade::Plane;

#[derive(Clone, Debug, PartialEq)]
pub struct SpatialPlacementSpec {
    origin: [f64; 3],
    direction_witness: SpatialDirectionWitnessRef,
    reference_frame: SpatialFrameRef,
}

impl SpatialPlacementSpec {
    pub fn world() -> Self {
        Self {
            origin: [0.0, 0.0, 0.0],
            direction_witness: SpatialDirectionWitnessRef::world_direction([0.0, 0.0, 1.0]),
            reference_frame: SpatialFrameRef::world(),
        }
    }

    pub fn at(self, origin: [f64; 3]) -> Self {
        Self { origin, ..self }
    }

    pub fn facing(self, facing: [f64; 3]) -> Self {
        self.facing_witness(SpatialDirectionWitnessRef::world_direction(facing))
    }

    pub fn facing_witness(self, direction_witness: SpatialDirectionWitnessRef) -> Self {
        Self {
            direction_witness,
            ..self
        }
    }

    pub fn between(self, start: [f64; 3], end: [f64; 3]) -> Self {
        self.at([
            (start[0] + end[0]) * 0.5,
            (start[1] + end[1]) * 0.5,
            (start[2] + end[2]) * 0.5,
        ])
    }

    pub fn relative_to(self, frame: SpatialFrameRef) -> Self {
        Self {
            reference_frame: frame,
            ..self
        }
    }

    pub fn on(self, frame: SpatialFrameRef) -> Self {
        self.relative_to(frame.clone()).aligned_with(frame)
    }

    pub fn r#in(self, frame: SpatialFrameRef) -> Self {
        self.relative_to(frame)
    }

    pub fn inside(self, frame: SpatialFrameRef) -> Self {
        self.relative_to(frame)
    }

    pub fn aligned_with(self, frame: SpatialFrameRef) -> Self {
        self.facing_witness(SpatialDirectionWitnessRef::frame_axis(
            frame.clone(),
            SpatialAxis::W,
        ))
        .relative_to(frame)
    }

    pub fn parallel_to(self, frame: SpatialFrameRef) -> Self {
        self.aligned_with(frame)
    }

    pub fn perpendicular_to(self, frame: SpatialFrameRef) -> Self {
        self.facing_witness(SpatialDirectionWitnessRef::frame_perpendicular_axis(
            frame.clone(),
            SpatialAxis::W,
        ))
        .relative_to(frame)
    }

    pub fn origin(&self) -> [f64; 3] {
        self.origin
    }

    pub fn direction_witness(&self) -> &SpatialDirectionWitnessRef {
        &self.direction_witness
    }

    pub fn reference_frame(&self) -> &SpatialFrameRef {
        &self.reference_frame
    }
}

impl Default for SpatialPlacementSpec {
    fn default() -> Self {
        Self::world()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpatialPlacementFrame {
    origin: [f64; 3],
    u_axis: [f64; 3],
    v_axis: [f64; 3],
    w_axis: [f64; 3],
}

impl SpatialPlacementFrame {
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
}

#[derive(Clone, Debug, PartialEq)]
pub struct AdmittedSpatialPlacement {
    spec: SpatialPlacementSpec,
    frame: SpatialPlacementFrame,
    reference_frame: AdmittedSpatialFrameRef,
    resolved_direction_witness: ResolvedSpatialDirectionWitness,
}

impl AdmittedSpatialPlacement {
    pub fn spec(&self) -> &SpatialPlacementSpec {
        &self.spec
    }

    pub fn origin(&self) -> [f64; 3] {
        self.frame.origin()
    }

    pub fn facing_vector(&self) -> [f64; 3] {
        self.frame.w_axis()
    }

    pub fn frame(&self) -> SpatialPlacementFrame {
        self.frame
    }

    pub fn reference_frame(&self) -> &AdmittedSpatialFrameRef {
        &self.reference_frame
    }

    pub fn resolved_direction_witness(&self) -> &ResolvedSpatialDirectionWitness {
        &self.resolved_direction_witness
    }

    pub fn embed_point(&self, local: [f64; 3]) -> [f64; 3] {
        [
            self.frame.origin[0]
                + self.frame.u_axis[0] * local[0]
                + self.frame.v_axis[0] * local[1]
                + self.frame.w_axis[0] * local[2],
            self.frame.origin[1]
                + self.frame.u_axis[1] * local[0]
                + self.frame.v_axis[1] * local[1]
                + self.frame.w_axis[1] * local[2],
            self.frame.origin[2]
                + self.frame.u_axis[2] * local[0]
                + self.frame.v_axis[2] * local[1]
                + self.frame.w_axis[2] * local[2],
        ]
    }

    pub fn embed_vector(&self, local: [f64; 3]) -> [f64; 3] {
        [
            self.frame.u_axis[0] * local[0]
                + self.frame.v_axis[0] * local[1]
                + self.frame.w_axis[0] * local[2],
            self.frame.u_axis[1] * local[0]
                + self.frame.v_axis[1] * local[1]
                + self.frame.w_axis[1] * local[2],
            self.frame.u_axis[2] * local[0]
                + self.frame.v_axis[2] * local[1]
                + self.frame.w_axis[2] * local[2],
        ]
    }
}

#[derive(Clone, Debug)]
pub struct SpatialPlacementGeometry {
    support_planes: Vec<Plane>,
    vertex_positions: Vec<[f64; 3]>,
}

impl SpatialPlacementGeometry {
    fn new(support_planes: Vec<Plane>, vertex_positions: Vec<[f64; 3]>) -> Self {
        Self {
            support_planes,
            vertex_positions,
        }
    }

    pub fn support_planes(&self) -> &[Plane] {
        &self.support_planes
    }

    pub fn vertex_positions(&self) -> &[[f64; 3]] {
        &self.vertex_positions
    }

    pub fn into_parts(self) -> (Vec<Plane>, Vec<[f64; 3]>) {
        (self.support_planes, self.vertex_positions)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SpatialPlacementError {
    NonFiniteOrigin,
    DirectionWitnessFailure(SpatialWitnessFailureClass),
    InvalidReferenceFrame(SpatialFrameError),
    InvalidEmbeddedPlane,
}

impl std::fmt::Display for SpatialPlacementError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NonFiniteOrigin => write!(f, "placement origin must stay finite"),
            Self::DirectionWitnessFailure(class) => {
                write!(
                    f,
                    "placement direction witness failed with {class:?} semantics"
                )
            }
            Self::InvalidReferenceFrame(error) => write!(f, "{error}"),
            Self::InvalidEmbeddedPlane => write!(f, "embedded support plane became invalid"),
        }
    }
}

impl std::error::Error for SpatialPlacementError {}

pub fn admit_spatial_placement(
    spec: SpatialPlacementSpec,
) -> Result<AdmittedSpatialPlacement, SpatialPlacementError> {
    admit_spatial_placement_with_catalog(spec, &EmptySpatialWitnessCatalog)
}

pub fn admit_spatial_placement_with_catalog(
    spec: SpatialPlacementSpec,
    catalog: &impl SpatialWitnessCatalog,
) -> Result<AdmittedSpatialPlacement, SpatialPlacementError> {
    if spec.origin.iter().any(|value| !value.is_finite()) {
        return Err(SpatialPlacementError::NonFiniteOrigin);
    }
    let reference_frame = admit_spatial_frame(spec.reference_frame.clone())
        .map_err(SpatialPlacementError::InvalidReferenceFrame)?;
    let resolved_direction_witness =
        resolve_spatial_direction_witness_with_catalog(spec.direction_witness.clone(), catalog)
            .map_err(SpatialPlacementError::DirectionWitnessFailure)?;
    let frame_basis = reference_frame.basis();
    let world_origin = frame_basis.embed_point(spec.origin);
    let world_w_axis = resolved_direction_witness.resolved_world_direction();
    let world_frame = admit_spatial_frame(SpatialFrameRef::workplane(
        "placement-facing",
        world_origin,
        world_w_axis,
    ))
    .map_err(SpatialPlacementError::InvalidReferenceFrame)?;
    Ok(AdmittedSpatialPlacement {
        spec,
        frame: SpatialPlacementFrame {
            origin: world_origin,
            u_axis: world_frame.basis().u_axis(),
            v_axis: world_frame.basis().v_axis(),
            w_axis: world_frame.basis().w_axis(),
        },
        reference_frame,
        resolved_direction_witness,
    })
}

pub fn apply_spatial_placement(
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
    Ok(SpatialPlacementGeometry::new(
        support_planes,
        vertex_positions,
    ))
}

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
#[path = "placement_tests.rs"]
mod placement_tests;
