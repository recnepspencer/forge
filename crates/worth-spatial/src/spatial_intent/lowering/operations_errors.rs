use crate::spatial_intent::refs::SpatialGeometricTagFailureClass;
use crate::spatial_intent::resolution::{SpatialFrameError, SpatialWitnessFailureClass};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialPlacementMotionError {
    UnsupportedMoveAnchor,
    UnsupportedOffsetAnchor,
    UnsupportedRotateAnchor,
    UnsupportedReorientAnchor,
    AmbiguousReorientAnchorMeaning,
    AnchorWitnessFailure(SpatialWitnessFailureClass),
    AnchorTagFailure(SpatialGeometricTagFailureClass),
    InvalidExistingPlacement,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialPlacementConstraintError {
    UnsupportedLiesOnAnchor,
    UnsupportedPointsTowardAnchor,
    UnsupportedAnchorMatch,
    AnchorWitnessFailure(SpatialWitnessFailureClass),
    AnchorTagFailure(SpatialGeometricTagFailureClass),
    InvalidReferenceFrame(SpatialFrameError),
    CoincidentTarget,
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
