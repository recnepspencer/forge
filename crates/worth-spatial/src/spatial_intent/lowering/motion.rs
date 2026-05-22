use crate::spatial_intent::refs::{
    EmptySpatialWitnessCatalog, SpatialAnchorRef, SpatialAxis, SpatialDirectionWitnessRef,
    SpatialFrameRef, SpatialPointWitnessRef, SpatialWitnessCatalog,
};
use crate::spatial_intent::resolution::{
    resolve_spatial_direction_witness_with_catalog, resolve_spatial_point_witness_with_catalog,
    ResolvedSpatialDirectionWitness, ResolvedSpatialPointWitness, SpatialWitnessFailureClass,
};

#[derive(Clone, Debug, PartialEq)]
pub struct SpatialMoveSpec {
    anchor: SpatialAnchorRef,
    destination_witness: SpatialPointWitnessRef,
}

impl SpatialMoveSpec {
    pub fn shape_origin() -> Self {
        Self {
            anchor: SpatialAnchorRef::shape_origin(),
            destination_witness: SpatialPointWitnessRef::world_point([0.0, 0.0, 0.0]),
        }
    }

    pub fn from(self, anchor: SpatialAnchorRef) -> Self {
        Self { anchor, ..self }
    }

    pub fn to(self, destination: [f64; 3]) -> Self {
        self.to_witness(SpatialPointWitnessRef::world_point(destination))
    }

    pub fn to_witness(self, destination_witness: SpatialPointWitnessRef) -> Self {
        Self {
            destination_witness,
            ..self
        }
    }

    pub fn anchor(&self) -> &SpatialAnchorRef {
        &self.anchor
    }

    pub fn destination_witness(&self) -> &SpatialPointWitnessRef {
        &self.destination_witness
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SpatialRotateSpec {
    anchor: SpatialAnchorRef,
    axis_witness: SpatialDirectionWitnessRef,
    angle_radians: f64,
}

impl SpatialRotateSpec {
    pub fn shape_origin() -> Self {
        Self {
            anchor: SpatialAnchorRef::shape_origin(),
            axis_witness: SpatialDirectionWitnessRef::world_direction([0.0, 0.0, 1.0]),
            angle_radians: 0.0,
        }
    }

    pub fn about(self, anchor: SpatialAnchorRef) -> Self {
        Self { anchor, ..self }
    }

    pub fn around(self, axis: [f64; 3]) -> Self {
        self.around_witness(SpatialDirectionWitnessRef::world_direction(axis))
    }

    pub fn around_witness(self, axis_witness: SpatialDirectionWitnessRef) -> Self {
        Self {
            axis_witness,
            ..self
        }
    }

    pub fn by_radians(self, angle_radians: f64) -> Self {
        Self {
            angle_radians,
            ..self
        }
    }

    pub fn anchor(&self) -> &SpatialAnchorRef {
        &self.anchor
    }

    pub fn axis_witness(&self) -> &SpatialDirectionWitnessRef {
        &self.axis_witness
    }

    pub fn angle_radians(&self) -> f64 {
        self.angle_radians
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SpatialReorientSpec {
    anchor: SpatialAnchorRef,
    direction_witness: SpatialDirectionWitnessRef,
}

impl SpatialReorientSpec {
    pub fn shape_origin() -> Self {
        Self {
            anchor: SpatialAnchorRef::shape_origin(),
            direction_witness: SpatialDirectionWitnessRef::world_direction([0.0, 0.0, 1.0]),
        }
    }

    pub fn about(self, anchor: SpatialAnchorRef) -> Self {
        Self { anchor, ..self }
    }

    pub fn toward(self, facing: [f64; 3]) -> Self {
        self.toward_witness(SpatialDirectionWitnessRef::world_direction(facing))
    }

    pub fn toward_witness(self, direction_witness: SpatialDirectionWitnessRef) -> Self {
        Self {
            direction_witness,
            ..self
        }
    }

    pub fn aligned_with(self, frame: SpatialFrameRef) -> Self {
        self.toward_witness(SpatialDirectionWitnessRef::frame_axis(
            frame,
            SpatialAxis::W,
        ))
    }

    pub fn parallel_to(self, frame: SpatialFrameRef) -> Self {
        self.aligned_with(frame)
    }

    pub fn perpendicular_to(self, frame: SpatialFrameRef) -> Self {
        self.toward_witness(SpatialDirectionWitnessRef::frame_perpendicular_axis(
            frame,
            SpatialAxis::W,
        ))
    }

    pub fn anchor(&self) -> &SpatialAnchorRef {
        &self.anchor
    }

    pub fn direction_witness(&self) -> &SpatialDirectionWitnessRef {
        &self.direction_witness
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SpatialOffsetSpec {
    anchor: SpatialAnchorRef,
    offset: [f64; 3],
}

impl SpatialOffsetSpec {
    pub fn shape_origin() -> Self {
        Self {
            anchor: SpatialAnchorRef::shape_origin(),
            offset: [0.0, 0.0, 0.0],
        }
    }

    pub fn from(self, anchor: SpatialAnchorRef) -> Self {
        Self { anchor, ..self }
    }

    pub fn by(self, offset: [f64; 3]) -> Self {
        Self { offset, ..self }
    }

    pub fn translated_by(self, offset: [f64; 3]) -> Self {
        self.by(offset)
    }

    pub fn offset_by(self, offset: [f64; 3]) -> Self {
        self.by(offset)
    }

    pub fn anchor(&self) -> &SpatialAnchorRef {
        &self.anchor
    }

    pub fn offset(&self) -> [f64; 3] {
        self.offset
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AdmittedSpatialMove {
    spec: SpatialMoveSpec,
    resolved_destination_witness: ResolvedSpatialPointWitness,
}

impl AdmittedSpatialMove {
    pub fn spec(&self) -> &SpatialMoveSpec {
        &self.spec
    }

    pub fn resolved_destination_witness(&self) -> &ResolvedSpatialPointWitness {
        &self.resolved_destination_witness
    }

    pub fn destination_point(&self) -> [f64; 3] {
        self.resolved_destination_witness.resolved_world_point()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AdmittedSpatialRotate {
    spec: SpatialRotateSpec,
    resolved_axis_witness: ResolvedSpatialDirectionWitness,
}

impl AdmittedSpatialRotate {
    pub fn spec(&self) -> &SpatialRotateSpec {
        &self.spec
    }

    pub fn resolved_axis_witness(&self) -> &ResolvedSpatialDirectionWitness {
        &self.resolved_axis_witness
    }

    pub fn normalized_axis(&self) -> [f64; 3] {
        self.resolved_axis_witness.resolved_world_direction()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AdmittedSpatialReorient {
    spec: SpatialReorientSpec,
    resolved_direction_witness: ResolvedSpatialDirectionWitness,
}

impl AdmittedSpatialReorient {
    pub fn spec(&self) -> &SpatialReorientSpec {
        &self.spec
    }

    pub fn resolved_direction_witness(&self) -> &ResolvedSpatialDirectionWitness {
        &self.resolved_direction_witness
    }

    pub fn normalized_facing(&self) -> [f64; 3] {
        self.resolved_direction_witness.resolved_world_direction()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AdmittedSpatialOffset {
    spec: SpatialOffsetSpec,
}

impl AdmittedSpatialOffset {
    pub fn spec(&self) -> &SpatialOffsetSpec {
        &self.spec
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpatialMotionError {
    DestinationWitnessFailure(SpatialWitnessFailureClass),
    RotationWitnessFailure(SpatialWitnessFailureClass),
    NonFiniteRotationAngle,
    DirectionWitnessFailure(SpatialWitnessFailureClass),
    NonFiniteOffset,
}

impl std::fmt::Display for SpatialMotionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DestinationWitnessFailure(class) => {
                write!(f, "destination witness failed with {class:?} semantics")
            }
            Self::RotationWitnessFailure(class) => {
                write!(f, "rotation axis witness failed with {class:?} semantics")
            }
            Self::NonFiniteRotationAngle => write!(f, "rotation angle must stay finite"),
            Self::DirectionWitnessFailure(class) => {
                write!(f, "direction witness failed with {class:?} semantics")
            }
            Self::NonFiniteOffset => write!(f, "offset vector must stay finite"),
        }
    }
}

impl std::error::Error for SpatialMotionError {}

pub fn admit_spatial_move(
    spec: SpatialMoveSpec,
) -> Result<AdmittedSpatialMove, SpatialMotionError> {
    admit_spatial_move_with_catalog(spec, &EmptySpatialWitnessCatalog)
}

pub fn admit_spatial_move_with_catalog(
    spec: SpatialMoveSpec,
    catalog: &impl SpatialWitnessCatalog,
) -> Result<AdmittedSpatialMove, SpatialMotionError> {
    let resolved_destination_witness =
        resolve_spatial_point_witness_with_catalog(spec.destination_witness.clone(), catalog)
            .map_err(SpatialMotionError::DestinationWitnessFailure)?;
    Ok(AdmittedSpatialMove {
        spec,
        resolved_destination_witness,
    })
}

pub fn admit_spatial_rotate(
    spec: SpatialRotateSpec,
) -> Result<AdmittedSpatialRotate, SpatialMotionError> {
    admit_spatial_rotate_with_catalog(spec, &EmptySpatialWitnessCatalog)
}

pub fn admit_spatial_rotate_with_catalog(
    spec: SpatialRotateSpec,
    catalog: &impl SpatialWitnessCatalog,
) -> Result<AdmittedSpatialRotate, SpatialMotionError> {
    if !spec.angle_radians.is_finite() {
        return Err(SpatialMotionError::NonFiniteRotationAngle);
    }
    let resolved_axis_witness =
        resolve_spatial_direction_witness_with_catalog(spec.axis_witness.clone(), catalog)
            .map_err(SpatialMotionError::RotationWitnessFailure)?;
    Ok(AdmittedSpatialRotate {
        spec,
        resolved_axis_witness,
    })
}

pub fn admit_spatial_reorient(
    spec: SpatialReorientSpec,
) -> Result<AdmittedSpatialReorient, SpatialMotionError> {
    admit_spatial_reorient_with_catalog(spec, &EmptySpatialWitnessCatalog)
}

pub fn admit_spatial_reorient_with_catalog(
    spec: SpatialReorientSpec,
    catalog: &impl SpatialWitnessCatalog,
) -> Result<AdmittedSpatialReorient, SpatialMotionError> {
    let resolved_direction_witness =
        resolve_spatial_direction_witness_with_catalog(spec.direction_witness.clone(), catalog)
            .map_err(SpatialMotionError::DirectionWitnessFailure)?;
    Ok(AdmittedSpatialReorient {
        spec,
        resolved_direction_witness,
    })
}

pub fn admit_spatial_offset(
    spec: SpatialOffsetSpec,
) -> Result<AdmittedSpatialOffset, SpatialMotionError> {
    if spec.offset.iter().any(|value| !value.is_finite()) {
        return Err(SpatialMotionError::NonFiniteOffset);
    }
    Ok(AdmittedSpatialOffset { spec })
}

#[cfg(test)]
#[path = "motion_tests.rs"]
mod motion_tests;
