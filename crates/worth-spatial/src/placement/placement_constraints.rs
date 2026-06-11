#![cfg_attr(not(any(test, feature = "test-support-lowering")), allow(dead_code))]

use super::placement_constraint_anchors::{
    lower_anchor_match_target_with_catalog, lower_points_toward_anchor_with_catalog,
    lower_subject_anchor, ConstraintAnchorDenial,
};
use super::placement_types::SpatialPlacementConstraintError;
use crate::anchor_selection::{
    SpatialAnchorMatchConstraintSpec, SpatialLiesOnConstraintSpec,
    SpatialPointsTowardConstraintSpec,
};
use crate::authored_refs::SpatialWitnessCatalog;
use crate::placement::SpatialPlacementSpec;
use crate::witness_resolution::{admit_spatial_frame, AdmittedSpatialFrameRef, SpatialFrameError};

pub(crate) fn apply_lies_on_constraint_to_placement_with_catalog(
    placement: SpatialPlacementSpec,
    spec: &SpatialLiesOnConstraintSpec,
    frame: &AdmittedSpatialFrameRef,
    catalog: &impl SpatialWitnessCatalog,
) -> Result<SpatialPlacementSpec, SpatialPlacementConstraintError> {
    match spec.anchor() {
        crate::authored_refs::SpatialAnchorRef::ShapeOrigin => Ok(placement
            .relative_to(frame.spec().clone())
            .at([0.0, 0.0, 0.0])),
        _ => {
            let anchor = lower_subject_anchor(&placement, spec.anchor(), catalog)
                .map_err(map_lies_on_anchor_failure)?;
            project_subject_anchor_onto_frame_plane(placement, frame, anchor).ok_or(
                SpatialPlacementConstraintError::InvalidReferenceFrame(
                    SpatialFrameError::InvalidNormal,
                ),
            )
        }
    }
}

pub(crate) fn apply_points_toward_constraint_to_placement_with_catalog(
    placement: SpatialPlacementSpec,
    spec: &SpatialPointsTowardConstraintSpec,
    target_point: [f64; 3],
    catalog: &impl SpatialWitnessCatalog,
) -> Result<SpatialPlacementSpec, SpatialPlacementConstraintError> {
    let anchor = lower_points_toward_anchor_with_catalog(&placement, spec.anchor(), catalog)
        .map_err(map_points_toward_anchor_failure)?;
    if points_are_coincident(anchor, target_point) {
        Err(SpatialPlacementConstraintError::CoincidentTarget)
    } else {
        Ok(placement.facing([
            target_point[0] - anchor[0],
            target_point[1] - anchor[1],
            target_point[2] - anchor[2],
        ]))
    }
}

pub(crate) fn apply_anchor_match_constraint_to_placement_with_catalog(
    placement: SpatialPlacementSpec,
    spec: &SpatialAnchorMatchConstraintSpec,
    catalog: &impl SpatialWitnessCatalog,
) -> Result<SpatialPlacementSpec, SpatialPlacementConstraintError> {
    let anchor = lower_subject_anchor(&placement, spec.anchor(), catalog)
        .map_err(map_anchor_match_failure)?;
    let target = lower_anchor_match_target_with_catalog(&placement, spec.other_anchor(), catalog)
        .map_err(map_anchor_match_failure)?;
    translate_anchor_to_world_point(placement, anchor, target).ok_or(
        SpatialPlacementConstraintError::InvalidReferenceFrame(SpatialFrameError::InvalidNormal),
    )
}

fn map_lies_on_anchor_failure(denial: ConstraintAnchorDenial) -> SpatialPlacementConstraintError {
    match denial {
        ConstraintAnchorDenial::WitnessFailure(value) => {
            SpatialPlacementConstraintError::AnchorWitnessFailure(value)
        }
        ConstraintAnchorDenial::TagFailure(value) => {
            SpatialPlacementConstraintError::AnchorTagFailure(value)
        }
        ConstraintAnchorDenial::InvalidReferenceFrame(value) => {
            SpatialPlacementConstraintError::InvalidReferenceFrame(value)
        }
        _ => SpatialPlacementConstraintError::UnsupportedLiesOnAnchor,
    }
}

fn map_points_toward_anchor_failure(
    denial: ConstraintAnchorDenial,
) -> SpatialPlacementConstraintError {
    match denial {
        ConstraintAnchorDenial::WitnessFailure(value) => {
            SpatialPlacementConstraintError::AnchorWitnessFailure(value)
        }
        ConstraintAnchorDenial::TagFailure(value) => {
            SpatialPlacementConstraintError::AnchorTagFailure(value)
        }
        ConstraintAnchorDenial::InvalidReferenceFrame(value) => {
            SpatialPlacementConstraintError::InvalidReferenceFrame(value)
        }
        _ => SpatialPlacementConstraintError::UnsupportedPointsTowardAnchor,
    }
}

fn map_anchor_match_failure(denial: ConstraintAnchorDenial) -> SpatialPlacementConstraintError {
    match denial {
        ConstraintAnchorDenial::WitnessFailure(value) => {
            SpatialPlacementConstraintError::AnchorWitnessFailure(value)
        }
        ConstraintAnchorDenial::TagFailure(value) => {
            SpatialPlacementConstraintError::AnchorTagFailure(value)
        }
        ConstraintAnchorDenial::InvalidReferenceFrame(value) => {
            SpatialPlacementConstraintError::InvalidReferenceFrame(value)
        }
        _ => SpatialPlacementConstraintError::UnsupportedAnchorMatch,
    }
}

fn points_are_coincident(a: [f64; 3], b: [f64; 3]) -> bool {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
        .iter()
        .all(|value| value.abs() <= f64::MIN_POSITIVE)
}

fn translate_anchor_to_world_point(
    placement: SpatialPlacementSpec,
    anchor_world_point: [f64; 3],
    target_world_point: [f64; 3],
) -> Option<SpatialPlacementSpec> {
    let reference_frame = admit_spatial_frame(placement.reference_frame().clone()).ok()?;
    let origin_world_point = reference_frame.basis().embed_point(placement.origin());
    Some(placement.at(reference_frame.basis().project_point([
        origin_world_point[0] + target_world_point[0] - anchor_world_point[0],
        origin_world_point[1] + target_world_point[1] - anchor_world_point[1],
        origin_world_point[2] + target_world_point[2] - anchor_world_point[2],
    ])))
}

fn project_subject_anchor_onto_frame_plane(
    placement: SpatialPlacementSpec,
    target_frame: &AdmittedSpatialFrameRef,
    anchor_world_point: [f64; 3],
) -> Option<SpatialPlacementSpec> {
    let current_reference_frame = admit_spatial_frame(placement.reference_frame().clone()).ok()?;
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
    Some(
        placement
            .relative_to(target_frame.spec().clone())
            .at(target_frame
                .basis()
                .project_point(translated_origin_world_point)),
    )
}
