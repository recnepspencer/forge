use worth_geom::facade::Plane;

use crate::authored_refs::EmptySpatialWitnessCatalog;
use crate::authored_refs::SpatialFrameRef;
use crate::placement::SpatialPlacementSpec;
use crate::witness_resolution::witness_resolution::resolve_spatial_direction_witness_with_catalog;
use crate::witness_resolution::{
    admit_spatial_frame, AdmittedSpatialFrameRef, ResolvedSpatialDirectionWitness,
    SpatialFrameError, SpatialWitnessFailureClass,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PrimitiveConstructionBirthPlacementIssue {
    NonFiniteOrigin,
    DirectionWitnessFailure(SpatialWitnessFailureClass),
    InvalidReferenceFrame(SpatialFrameError),
    InvalidEmbeddedPlane,
}

impl std::fmt::Display for PrimitiveConstructionBirthPlacementIssue {
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

impl std::error::Error for PrimitiveConstructionBirthPlacementIssue {}

#[derive(Clone, Copy, Debug)]
struct PrimitiveConstructionBirthPlacementFrame {
    origin: [f64; 3],
    u_axis: [f64; 3],
    v_axis: [f64; 3],
    w_axis: [f64; 3],
}

#[derive(Clone, Debug)]
struct PrimitiveConstructionBirthPlacement {
    frame: PrimitiveConstructionBirthPlacementFrame,
    #[allow(dead_code)]
    reference_frame: AdmittedSpatialFrameRef,
    #[allow(dead_code)]
    resolved_direction_witness: ResolvedSpatialDirectionWitness,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PrimitiveConstructionBirthPlacementFacts {
    origin: [f64; 3],
    facing_vector: [f64; 3],
}

impl PrimitiveConstructionBirthPlacementFacts {
    pub fn origin(&self) -> [f64; 3] {
        self.origin
    }

    pub fn facing_vector(&self) -> [f64; 3] {
        self.facing_vector
    }
}

impl PrimitiveConstructionBirthPlacement {
    fn facts(&self) -> PrimitiveConstructionBirthPlacementFacts {
        PrimitiveConstructionBirthPlacementFacts {
            origin: self.frame.origin,
            facing_vector: self.frame.w_axis,
        }
    }

    fn embed_point(&self, local: [f64; 3]) -> [f64; 3] {
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

    fn embed_vector(&self, local: [f64; 3]) -> [f64; 3] {
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
pub(crate) struct PrimitiveConstructionBirthPlacementGeometry {
    support_planes: Vec<Plane>,
    vertex_positions: Vec<[f64; 3]>,
}

impl PrimitiveConstructionBirthPlacementGeometry {
    pub(crate) fn into_parts(self) -> (Vec<Plane>, Vec<[f64; 3]>) {
        (self.support_planes, self.vertex_positions)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum PrimitiveConstructionBirthPlacementError {
    InvalidPlacement(PrimitiveConstructionBirthPlacementIssue),
    PlacementGeometry(PrimitiveConstructionBirthPlacementIssue),
}

pub(crate) fn realize_primitive_construction_birth_placement(
    placement_spec: SpatialPlacementSpec,
    support_planes: &[Plane],
    local_vertices: &[[f64; 3]],
) -> Result<PrimitiveConstructionBirthPlacementGeometry, PrimitiveConstructionBirthPlacementError> {
    let placement = admit_primitive_construction_birth_placement(placement_spec)
        .map_err(PrimitiveConstructionBirthPlacementError::InvalidPlacement)?;
    let (support_planes, vertex_positions) =
        apply_primitive_construction_birth_placement(&placement, support_planes, local_vertices)
            .map_err(PrimitiveConstructionBirthPlacementError::PlacementGeometry)?
            .into_parts();
    Ok(PrimitiveConstructionBirthPlacementGeometry {
        support_planes,
        vertex_positions,
    })
}

pub(crate) fn primitive_construction_birth_placement_facts(
    placement_spec: SpatialPlacementSpec,
) -> Result<PrimitiveConstructionBirthPlacementFacts, PrimitiveConstructionBirthPlacementIssue> {
    Ok(admit_primitive_construction_birth_placement(placement_spec)?.facts())
}

fn admit_primitive_construction_birth_placement(
    spec: SpatialPlacementSpec,
) -> Result<PrimitiveConstructionBirthPlacement, PrimitiveConstructionBirthPlacementIssue> {
    if spec.origin().iter().any(|value| !value.is_finite()) {
        return Err(PrimitiveConstructionBirthPlacementIssue::NonFiniteOrigin);
    }
    let reference_frame = admit_spatial_frame(spec.reference_frame().clone())
        .map_err(PrimitiveConstructionBirthPlacementIssue::InvalidReferenceFrame)?;
    let resolved_direction_witness = resolve_spatial_direction_witness_with_catalog(
        spec.direction_witness().clone(),
        &EmptySpatialWitnessCatalog,
    )
    .map_err(PrimitiveConstructionBirthPlacementIssue::DirectionWitnessFailure)?;
    let frame_basis = reference_frame.basis();
    let world_origin = frame_basis.embed_point(spec.origin());
    let world_w_axis = resolved_direction_witness.resolved_world_direction();
    let world_frame = admit_spatial_frame(SpatialFrameRef::workplane(
        "primitive-construction-birth-facing",
        world_origin,
        world_w_axis,
    ))
    .map_err(PrimitiveConstructionBirthPlacementIssue::InvalidReferenceFrame)?;

    Ok(PrimitiveConstructionBirthPlacement {
        frame: PrimitiveConstructionBirthPlacementFrame {
            origin: world_origin,
            u_axis: world_frame.basis().u_axis(),
            v_axis: world_frame.basis().v_axis(),
            w_axis: world_frame.basis().w_axis(),
        },
        reference_frame,
        resolved_direction_witness,
    })
}

fn apply_primitive_construction_birth_placement(
    placement: &PrimitiveConstructionBirthPlacement,
    support_planes: &[Plane],
    local_vertices: &[[f64; 3]],
) -> Result<PrimitiveConstructionBirthPlacementGeometry, PrimitiveConstructionBirthPlacementIssue> {
    let support_planes = support_planes
        .iter()
        .map(|plane| embed_birth_plane(placement, plane))
        .collect::<Result<Vec<_>, _>>()?;
    let vertex_positions = local_vertices
        .iter()
        .copied()
        .map(|point| placement.embed_point(point))
        .collect();
    Ok(PrimitiveConstructionBirthPlacementGeometry {
        support_planes,
        vertex_positions,
    })
}

fn embed_birth_plane(
    placement: &PrimitiveConstructionBirthPlacement,
    plane: &Plane,
) -> Result<Plane, PrimitiveConstructionBirthPlacementIssue> {
    let local_raw_normal = plane.raw_normal();
    let coefficient_scale = local_raw_normal[0]
        .abs()
        .max(local_raw_normal[1].abs())
        .max(local_raw_normal[2].abs());
    if !coefficient_scale.is_finite() || coefficient_scale <= f64::MIN_POSITIVE {
        return Err(PrimitiveConstructionBirthPlacementIssue::InvalidEmbeddedPlane);
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
        return Err(PrimitiveConstructionBirthPlacementIssue::InvalidEmbeddedPlane);
    }
    let point_scale = -normalized_local_offset / normal_length_sq;
    let local_plane_point = [
        normalized_local_normal[0] * point_scale,
        normalized_local_normal[1] * point_scale,
        normalized_local_normal[2] * point_scale,
    ];
    let world_plane_point = placement.embed_point(local_plane_point);
    Plane::from_point_normal(world_plane_point, world_raw_normal)
        .map_err(|_| PrimitiveConstructionBirthPlacementIssue::InvalidEmbeddedPlane)
}
