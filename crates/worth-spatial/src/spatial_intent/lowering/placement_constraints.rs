use crate::spatial_intent::constraints::{
    AdmittedSpatialAnchorMatchConstraint, AdmittedSpatialLiesOnConstraint,
    AdmittedSpatialPointsTowardConstraint,
};
use crate::spatial_intent::lowering::placement_anchor_points::SpatialPlacementPointAnchorError;
use crate::spatial_intent::lowering::placement_anchor_progression::{
    lower_supported_point_anchor, lower_supported_point_anchor_with_catalog,
    lower_supported_subject_anchor_with_catalog,
};
use crate::spatial_intent::lowering::SpatialPlacementSpec;
use crate::spatial_intent::refs::{
    SpatialAnchorRef, SpatialGeometricTagFailureClass, SpatialWitnessCatalog,
};
use crate::spatial_intent::resolution::{
    admit_spatial_frame, AdmittedSpatialFrameRef, SpatialFrameError, SpatialWitnessFailureClass,
};

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

impl std::fmt::Display for SpatialPlacementConstraintError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnsupportedLiesOnAnchor => {
                write!(
                    f,
                    "only subject-owned point-like lies-on anchors can lower into placement"
                )
            }
            Self::UnsupportedPointsTowardAnchor => {
                write!(
                    f,
                    "only point-like points-toward anchors can lower into placement"
                )
            }
            Self::UnsupportedAnchorMatch => {
                write!(
                    f,
                    "only subject-owned point anchors matched against point-like target anchors can lower into placement"
                )
            }
            Self::AnchorWitnessFailure(error) => write!(f, "anchor witness failure: {error:?}"),
            Self::AnchorTagFailure(error) => {
                write!(f, "geometric-tag anchor failure: {error:?}")
            }
            Self::InvalidReferenceFrame(error) => write!(f, "{error}"),
            Self::CoincidentTarget => {
                write!(
                    f,
                    "points-toward target must not collapse into the current origin"
                )
            }
        }
    }
}

impl std::error::Error for SpatialPlacementConstraintError {}

pub fn apply_admitted_lies_on_constraint_to_placement(
    placement: SpatialPlacementSpec,
    constraint: &AdmittedSpatialLiesOnConstraint,
) -> Result<SpatialPlacementSpec, SpatialPlacementConstraintError> {
    apply_admitted_lies_on_constraint_to_placement_with_catalog(
        placement,
        constraint,
        &crate::spatial_intent::refs::EmptySpatialWitnessCatalog,
    )
}

pub fn apply_admitted_lies_on_constraint_to_placement_with_catalog(
    placement: SpatialPlacementSpec,
    constraint: &AdmittedSpatialLiesOnConstraint,
    catalog: &impl SpatialWitnessCatalog,
) -> Result<SpatialPlacementSpec, SpatialPlacementConstraintError> {
    match constraint.spec().anchor() {
        SpatialAnchorRef::ShapeOrigin => Ok(placement
            .relative_to(constraint.frame().spec().clone())
            .at([0.0, 0.0, 0.0])),
        SpatialAnchorRef::FeatureOwned(_) => {
            let anchor_world_point = lower_supported_subject_anchor_with_catalog(
                &placement,
                constraint.spec().anchor(),
                catalog,
            )
            .map_err(map_lies_on_anchor_error)?
            .payload()
            .world_point();
            project_subject_anchor_onto_frame_plane(
                placement,
                constraint.frame(),
                anchor_world_point,
            )
        }
        _ => Err(SpatialPlacementConstraintError::UnsupportedLiesOnAnchor),
    }
}

pub fn apply_admitted_points_toward_constraint_to_placement(
    placement: SpatialPlacementSpec,
    constraint: &AdmittedSpatialPointsTowardConstraint,
) -> Result<SpatialPlacementSpec, SpatialPlacementConstraintError> {
    let anchor_world_point = lower_supported_point_anchor(&placement, constraint.spec().anchor())
        .map_err(map_points_toward_anchor_error)?
        .payload()
        .world_point();
    let facing = [
        constraint.target_point()[0] - anchor_world_point[0],
        constraint.target_point()[1] - anchor_world_point[1],
        constraint.target_point()[2] - anchor_world_point[2],
    ];
    if facing.iter().all(|value| value.abs() <= f64::MIN_POSITIVE) {
        return Err(SpatialPlacementConstraintError::CoincidentTarget);
    }
    Ok(placement.facing(facing))
}

pub fn apply_admitted_points_toward_constraint_to_placement_with_catalog(
    placement: SpatialPlacementSpec,
    constraint: &AdmittedSpatialPointsTowardConstraint,
    catalog: &impl SpatialWitnessCatalog,
) -> Result<SpatialPlacementSpec, SpatialPlacementConstraintError> {
    let anchor_world_point =
        lower_supported_point_anchor_with_catalog(&placement, constraint.spec().anchor(), catalog)
            .map_err(map_points_toward_anchor_error)?
            .payload()
            .world_point();
    let facing = [
        constraint.target_point()[0] - anchor_world_point[0],
        constraint.target_point()[1] - anchor_world_point[1],
        constraint.target_point()[2] - anchor_world_point[2],
    ];
    if facing.iter().all(|value| value.abs() <= f64::MIN_POSITIVE) {
        return Err(SpatialPlacementConstraintError::CoincidentTarget);
    }
    Ok(placement.facing(facing))
}

pub fn apply_admitted_anchor_match_constraint_to_placement(
    placement: SpatialPlacementSpec,
    constraint: &AdmittedSpatialAnchorMatchConstraint,
) -> Result<SpatialPlacementSpec, SpatialPlacementConstraintError> {
    let anchor_world_point = lower_supported_subject_anchor_with_catalog(
        &placement,
        constraint.spec().anchor(),
        &crate::spatial_intent::refs::EmptySpatialWitnessCatalog,
    )
    .map_err(map_anchor_match_anchor_error)?
    .payload()
    .world_point();
    let target_world_point =
        lower_supported_point_anchor(&placement, constraint.spec().other_anchor())
            .map_err(map_anchor_match_target_error)?
            .payload()
            .world_point();
    translate_placement_world_offset(
        placement,
        [
            target_world_point[0] - anchor_world_point[0],
            target_world_point[1] - anchor_world_point[1],
            target_world_point[2] - anchor_world_point[2],
        ],
    )
}

pub fn apply_admitted_anchor_match_constraint_to_placement_with_catalog(
    placement: SpatialPlacementSpec,
    constraint: &AdmittedSpatialAnchorMatchConstraint,
    catalog: &impl SpatialWitnessCatalog,
) -> Result<SpatialPlacementSpec, SpatialPlacementConstraintError> {
    let anchor_world_point = lower_supported_subject_anchor_with_catalog(
        &placement,
        constraint.spec().anchor(),
        catalog,
    )
    .map_err(map_anchor_match_anchor_error)?
    .payload()
    .world_point();
    let target_world_point = lower_supported_point_anchor_with_catalog(
        &placement,
        constraint.spec().other_anchor(),
        catalog,
    )
    .map_err(map_anchor_match_target_error)?
    .payload()
    .world_point();
    translate_placement_world_offset(
        placement,
        [
            target_world_point[0] - anchor_world_point[0],
            target_world_point[1] - anchor_world_point[1],
            target_world_point[2] - anchor_world_point[2],
        ],
    )
}

fn map_points_toward_anchor_error(
    error: SpatialPlacementPointAnchorError,
) -> SpatialPlacementConstraintError {
    match error {
        SpatialPlacementPointAnchorError::UnsupportedAnchor => {
            SpatialPlacementConstraintError::UnsupportedPointsTowardAnchor
        }
        SpatialPlacementPointAnchorError::InvalidReferenceFrame(error) => {
            SpatialPlacementConstraintError::InvalidReferenceFrame(error)
        }
        SpatialPlacementPointAnchorError::AnchorWitnessFailure(error) => {
            SpatialPlacementConstraintError::AnchorWitnessFailure(error)
        }
        SpatialPlacementPointAnchorError::AnchorTagFailure(error) => {
            SpatialPlacementConstraintError::AnchorTagFailure(error)
        }
    }
}

fn map_lies_on_anchor_error(
    error: SpatialPlacementPointAnchorError,
) -> SpatialPlacementConstraintError {
    match error {
        SpatialPlacementPointAnchorError::UnsupportedAnchor => {
            SpatialPlacementConstraintError::UnsupportedLiesOnAnchor
        }
        SpatialPlacementPointAnchorError::InvalidReferenceFrame(error) => {
            SpatialPlacementConstraintError::InvalidReferenceFrame(error)
        }
        SpatialPlacementPointAnchorError::AnchorWitnessFailure(error) => {
            SpatialPlacementConstraintError::AnchorWitnessFailure(error)
        }
        SpatialPlacementPointAnchorError::AnchorTagFailure(error) => {
            SpatialPlacementConstraintError::AnchorTagFailure(error)
        }
    }
}

fn map_anchor_match_anchor_error(
    error: SpatialPlacementPointAnchorError,
) -> SpatialPlacementConstraintError {
    match error {
        SpatialPlacementPointAnchorError::UnsupportedAnchor => {
            SpatialPlacementConstraintError::UnsupportedAnchorMatch
        }
        SpatialPlacementPointAnchorError::InvalidReferenceFrame(error) => {
            SpatialPlacementConstraintError::InvalidReferenceFrame(error)
        }
        SpatialPlacementPointAnchorError::AnchorWitnessFailure(error) => {
            SpatialPlacementConstraintError::AnchorWitnessFailure(error)
        }
        SpatialPlacementPointAnchorError::AnchorTagFailure(error) => {
            SpatialPlacementConstraintError::AnchorTagFailure(error)
        }
    }
}

fn map_anchor_match_target_error(
    error: SpatialPlacementPointAnchorError,
) -> SpatialPlacementConstraintError {
    match error {
        SpatialPlacementPointAnchorError::UnsupportedAnchor => {
            SpatialPlacementConstraintError::UnsupportedAnchorMatch
        }
        SpatialPlacementPointAnchorError::InvalidReferenceFrame(error) => {
            SpatialPlacementConstraintError::InvalidReferenceFrame(error)
        }
        SpatialPlacementPointAnchorError::AnchorWitnessFailure(error) => {
            SpatialPlacementConstraintError::AnchorWitnessFailure(error)
        }
        SpatialPlacementPointAnchorError::AnchorTagFailure(error) => {
            SpatialPlacementConstraintError::AnchorTagFailure(error)
        }
    }
}

fn translate_placement_world_offset(
    placement: SpatialPlacementSpec,
    offset: [f64; 3],
) -> Result<SpatialPlacementSpec, SpatialPlacementConstraintError> {
    let reference_frame = admit_spatial_frame(placement.reference_frame().clone())
        .map_err(SpatialPlacementConstraintError::InvalidReferenceFrame)?;
    let origin_world_point = reference_frame.basis().embed_point(placement.origin());
    Ok(placement.at(reference_frame.basis().project_point([
        origin_world_point[0] + offset[0],
        origin_world_point[1] + offset[1],
        origin_world_point[2] + offset[2],
    ])))
}

fn project_subject_anchor_onto_frame_plane(
    placement: SpatialPlacementSpec,
    target_frame: &AdmittedSpatialFrameRef,
    anchor_world_point: [f64; 3],
) -> Result<SpatialPlacementSpec, SpatialPlacementConstraintError> {
    let current_reference_frame = admit_spatial_frame(placement.reference_frame().clone())
        .map_err(SpatialPlacementConstraintError::InvalidReferenceFrame)?;
    let current_origin_world_point = current_reference_frame
        .basis()
        .embed_point(placement.origin());
    let target_anchor_local = target_frame.basis().project_point(anchor_world_point);
    let projected_anchor_world_point =
        target_frame
            .basis()
            .embed_point([target_anchor_local[0], target_anchor_local[1], 0.0]);
    let translated_origin_world_point = [
        current_origin_world_point[0] + projected_anchor_world_point[0] - anchor_world_point[0],
        current_origin_world_point[1] + projected_anchor_world_point[1] - anchor_world_point[1],
        current_origin_world_point[2] + projected_anchor_world_point[2] - anchor_world_point[2],
    ];
    Ok(placement
        .relative_to(target_frame.spec().clone())
        .at(target_frame
            .basis()
            .project_point(translated_origin_world_point)))
}

#[cfg(test)]
#[path = "placement_constraints_tests.rs"]
mod tests;
