use crate::placement::SpatialPlacementSpec;
use crate::witness_resolution::{
    AdmittedSpatialFrameRef, ResolvedSpatialDirectionWitness, SpatialFrameError,
    SpatialWitnessFailureClass,
};
#[cfg(test)]
use worth_geom::facade::Plane;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpatialPlacementFrame {
    origin: [f64; 3],
    u_axis: [f64; 3],
    v_axis: [f64; 3],
    w_axis: [f64; 3],
}

impl SpatialPlacementFrame {
    pub(crate) fn new(
        origin: [f64; 3],
        u_axis: [f64; 3],
        v_axis: [f64; 3],
        w_axis: [f64; 3],
    ) -> Self {
        Self {
            origin,
            u_axis,
            v_axis,
            w_axis,
        }
    }

    #[cfg(test)]
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
    pub(crate) fn from_parts(
        spec: SpatialPlacementSpec,
        frame: SpatialPlacementFrame,
        reference_frame: AdmittedSpatialFrameRef,
        resolved_direction_witness: ResolvedSpatialDirectionWitness,
    ) -> Self {
        Self {
            spec,
            frame,
            reference_frame,
            resolved_direction_witness,
        }
    }

    #[cfg(test)]
    pub fn spec(&self) -> &SpatialPlacementSpec {
        &self.spec
    }

    #[cfg(test)]
    pub fn origin(&self) -> [f64; 3] {
        self.frame.origin()
    }

    pub fn facing_vector(&self) -> [f64; 3] {
        self.frame.w_axis()
    }

    pub fn frame(&self) -> SpatialPlacementFrame {
        self.frame
    }

    #[cfg(test)]
    pub fn reference_frame(&self) -> &AdmittedSpatialFrameRef {
        &self.reference_frame
    }

    #[cfg(test)]
    pub fn resolved_direction_witness(&self) -> &ResolvedSpatialDirectionWitness {
        &self.resolved_direction_witness
    }

    #[cfg(test)]
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

    #[cfg(test)]
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

#[cfg(test)]
#[derive(Clone, Debug)]
pub struct SpatialPlacementGeometry {
    support_planes: Vec<Plane>,
    vertex_positions: Vec<[f64; 3]>,
}

#[cfg(test)]
impl SpatialPlacementGeometry {
    pub(crate) fn from_parts(support_planes: Vec<Plane>, vertex_positions: Vec<[f64; 3]>) -> Self {
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
    #[cfg(test)]
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
            #[cfg(test)]
            Self::InvalidEmbeddedPlane => write!(f, "embedded support plane became invalid"),
        }
    }
}

impl std::error::Error for SpatialPlacementError {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialPlacementMotionError {
    UnsupportedMoveAnchor,
    UnsupportedOffsetAnchor,
    UnsupportedRotateAnchor,
    UnsupportedReorientAnchor,
    AmbiguousReorientAnchorMeaning,
    AnchorWitnessFailure(SpatialWitnessFailureClass),
    AnchorTagFailure(crate::authored_refs::SpatialGeometricTagFailureClass),
    InvalidExistingPlacement,
}

impl std::fmt::Display for SpatialPlacementMotionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnsupportedMoveAnchor => write!(f, "unsupported move anchor"),
            Self::UnsupportedOffsetAnchor => write!(f, "unsupported offset anchor"),
            Self::UnsupportedRotateAnchor => write!(f, "unsupported rotate anchor"),
            Self::UnsupportedReorientAnchor => write!(f, "unsupported reorient anchor"),
            Self::AmbiguousReorientAnchorMeaning => {
                write!(f, "ambiguous reorient anchor meaning")
            }
            Self::AnchorWitnessFailure(error) => write!(f, "anchor witness failure: {error:?}"),
            Self::AnchorTagFailure(error) => write!(f, "anchor tag failure: {error:?}"),
            Self::InvalidExistingPlacement => write!(f, "invalid existing placement"),
        }
    }
}

impl std::error::Error for SpatialPlacementMotionError {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialPlacementConstraintError {
    UnsupportedLiesOnAnchor,
    UnsupportedPointsTowardAnchor,
    UnsupportedAnchorMatch,
    AnchorWitnessFailure(SpatialWitnessFailureClass),
    AnchorTagFailure(crate::authored_refs::SpatialGeometricTagFailureClass),
    InvalidReferenceFrame(SpatialFrameError),
    CoincidentTarget,
}

impl std::fmt::Display for SpatialPlacementConstraintError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnsupportedLiesOnAnchor => write!(f, "unsupported lies-on anchor"),
            Self::UnsupportedPointsTowardAnchor => {
                write!(f, "unsupported points-toward anchor")
            }
            Self::UnsupportedAnchorMatch => write!(f, "unsupported anchor match"),
            Self::AnchorWitnessFailure(error) => write!(f, "anchor witness failure: {error:?}"),
            Self::AnchorTagFailure(error) => write!(f, "anchor tag failure: {error:?}"),
            Self::InvalidReferenceFrame(error) => write!(f, "invalid reference frame: {error}"),
            Self::CoincidentTarget => write!(f, "coincident target"),
        }
    }
}

impl std::error::Error for SpatialPlacementConstraintError {}
