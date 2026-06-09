use crate::authored_refs::{
    SpatialAnchorRef, SpatialAxis, SpatialDirectionWitnessRef, SpatialFrameRef,
    SpatialPointWitnessRef,
};
use crate::witness_resolution::{SpatialFrameError, SpatialWitnessFailureClass};

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
pub struct SpatialPointsTowardConstraintSpec {
    anchor: SpatialAnchorRef,
    target_witness: SpatialPointWitnessRef,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SpatialLiesOnConstraintSpec {
    anchor: SpatialAnchorRef,
    frame: SpatialFrameRef,
}

impl SpatialLiesOnConstraintSpec {
    pub fn new(anchor: SpatialAnchorRef, frame: SpatialFrameRef) -> Self {
        Self { anchor, frame }
    }

    pub fn anchor(&self) -> &SpatialAnchorRef {
        &self.anchor
    }

    pub fn frame(&self) -> &SpatialFrameRef {
        &self.frame
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SpatialAnchorMatchConstraintSpec {
    anchor: SpatialAnchorRef,
    other_anchor: SpatialAnchorRef,
}

impl SpatialAnchorMatchConstraintSpec {
    pub fn new(anchor: SpatialAnchorRef, other_anchor: SpatialAnchorRef) -> Self {
        Self {
            anchor,
            other_anchor,
        }
    }

    pub fn anchor(&self) -> &SpatialAnchorRef {
        &self.anchor
    }

    pub fn other_anchor(&self) -> &SpatialAnchorRef {
        &self.other_anchor
    }
}

impl SpatialPointsTowardConstraintSpec {
    pub fn new(anchor: SpatialAnchorRef, target_point: [f64; 3]) -> Self {
        Self::with_witness(anchor, SpatialPointWitnessRef::world_point(target_point))
    }

    pub fn with_witness(anchor: SpatialAnchorRef, target_witness: SpatialPointWitnessRef) -> Self {
        Self {
            anchor,
            target_witness,
        }
    }

    pub fn anchor(&self) -> &SpatialAnchorRef {
        &self.anchor
    }

    pub fn target_witness(&self) -> &SpatialPointWitnessRef {
        &self.target_witness
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpatialConstraintError {
    TargetWitnessFailure(SpatialWitnessFailureClass),
    InvalidFrame(SpatialFrameError),
}

impl std::fmt::Display for SpatialConstraintError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TargetWitnessFailure(class) => {
                write!(f, "target witness failed with {class:?} semantics")
            }
            Self::InvalidFrame(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for SpatialConstraintError {}
