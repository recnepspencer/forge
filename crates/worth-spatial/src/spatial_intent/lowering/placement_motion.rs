use crate::spatial_intent::lowering::placement_anchor_directions::{
    lowered_reorient_facing_from_directional_anchor, SpatialPlacementDirectionalAnchorError,
    SpatialPlacementReorientAnchorMode,
};
use crate::spatial_intent::lowering::placement_anchor_points::SpatialPlacementPointAnchorError;
use crate::spatial_intent::lowering::placement_anchor_progression::{
    lower_supported_point_anchor, lower_supported_point_anchor_with_catalog,
    lower_supported_reorient_anchor, lower_supported_reorient_anchor_with_catalog,
    lower_supported_translation_anchor, lower_supported_translation_anchor_with_catalog,
};
use crate::spatial_intent::lowering::placement_motion_support::{
    rotate_point_about_pivot, rotate_vector, translate_anchor_to_world_point,
    translate_placement_world_offset,
};
use crate::spatial_intent::lowering::{
    admit_spatial_placement, AdmittedSpatialMove, AdmittedSpatialOffset, AdmittedSpatialReorient,
    AdmittedSpatialRotate, SpatialPlacementSpec,
};
use crate::spatial_intent::refs::{
    SpatialAnchorRef, SpatialGeometricTagFailureClass, SpatialWitnessCatalog,
};
use crate::spatial_intent::resolution::{admit_spatial_frame, SpatialWitnessFailureClass};

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

impl std::fmt::Display for SpatialPlacementMotionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnsupportedMoveAnchor => {
                write!(
                    f,
                    "only subject-owned or external-reference point-like move anchors can lower into placement"
                )
            }
            Self::UnsupportedOffsetAnchor => {
                write!(
                    f,
                    "only subject-owned or external-reference point-like offset anchors can lower into placement"
                )
            }
            Self::UnsupportedRotateAnchor => {
                write!(
                    f,
                    "only point-like rotation anchors can lower into placement"
                )
            }
            Self::UnsupportedReorientAnchor => {
                write!(
                    f,
                    "only supported point-like or directional reorientation anchors can lower into placement"
                )
            }
            Self::AmbiguousReorientAnchorMeaning => write!(
                f,
                "feature-owned reorientation anchor is ambiguous between point-like and directional meaning"
            ),
            Self::AnchorWitnessFailure(error) => {
                write!(f, "anchor witness failure: {error:?}")
            }
            Self::AnchorTagFailure(error) => {
                write!(f, "geometric-tag anchor failure: {error:?}")
            }
            Self::InvalidExistingPlacement => {
                write!(
                    f,
                    "existing placement could not be admitted before motion lowering"
                )
            }
        }
    }
}

impl std::error::Error for SpatialPlacementMotionError {}

pub fn apply_admitted_move_to_placement(
    placement: SpatialPlacementSpec,
    motion: &AdmittedSpatialMove,
) -> Result<SpatialPlacementSpec, SpatialPlacementMotionError> {
    let anchor_world_point = lower_supported_translation_anchor(&placement, motion.spec().anchor())
        .map_err(map_move_anchor_error)?
        .payload()
        .world_point();
    translate_anchor_to_world_point(placement, anchor_world_point, motion.destination_point())
        .ok_or(SpatialPlacementMotionError::InvalidExistingPlacement)
}

pub fn apply_admitted_move_to_placement_with_catalog(
    placement: SpatialPlacementSpec,
    motion: &AdmittedSpatialMove,
    catalog: &impl SpatialWitnessCatalog,
) -> Result<SpatialPlacementSpec, SpatialPlacementMotionError> {
    let anchor_world_point = lower_supported_translation_anchor_with_catalog(
        &placement,
        motion.spec().anchor(),
        catalog,
    )
    .map_err(map_move_anchor_error)?
    .payload()
    .world_point();
    translate_anchor_to_world_point(placement, anchor_world_point, motion.destination_point())
        .ok_or(SpatialPlacementMotionError::InvalidExistingPlacement)
}

pub fn apply_admitted_offset_to_placement(
    placement: SpatialPlacementSpec,
    motion: &AdmittedSpatialOffset,
) -> Result<SpatialPlacementSpec, SpatialPlacementMotionError> {
    lower_supported_translation_anchor(&placement, motion.spec().anchor())
        .map_err(map_offset_anchor_error)?;
    translate_placement_world_offset(placement, motion.spec().offset())
        .ok_or(SpatialPlacementMotionError::InvalidExistingPlacement)
}

pub fn apply_admitted_offset_to_placement_with_catalog(
    placement: SpatialPlacementSpec,
    motion: &AdmittedSpatialOffset,
    catalog: &impl SpatialWitnessCatalog,
) -> Result<SpatialPlacementSpec, SpatialPlacementMotionError> {
    lower_supported_translation_anchor_with_catalog(&placement, motion.spec().anchor(), catalog)
        .map_err(map_offset_anchor_error)?;
    translate_placement_world_offset(placement, motion.spec().offset())
        .ok_or(SpatialPlacementMotionError::InvalidExistingPlacement)
}

pub fn apply_admitted_reorient_to_placement(
    placement: SpatialPlacementSpec,
    motion: &AdmittedSpatialReorient,
) -> Result<SpatialPlacementSpec, SpatialPlacementMotionError> {
    match lower_supported_reorient_anchor(&placement, motion.spec().anchor())
        .map_err(map_reorient_directional_anchor_error)?
        .payload()
    {
        SpatialPlacementReorientAnchorMode::PointLike => {
            Ok(placement.facing_witness(motion.spec().direction_witness().clone()))
        }
        SpatialPlacementReorientAnchorMode::Directional(source_world_direction) => {
            let lowered_facing = lowered_reorient_facing_from_directional_anchor(
                &placement,
                *source_world_direction,
                motion.normalized_facing(),
            )
            .map_err(map_reorient_directional_anchor_error)?;
            Ok(placement.facing(lowered_facing))
        }
    }
}

pub fn apply_admitted_reorient_to_placement_with_catalog(
    placement: SpatialPlacementSpec,
    motion: &AdmittedSpatialReorient,
    catalog: &impl SpatialWitnessCatalog,
) -> Result<SpatialPlacementSpec, SpatialPlacementMotionError> {
    match lower_supported_reorient_anchor_with_catalog(&placement, motion.spec().anchor(), catalog)
        .map_err(map_reorient_directional_anchor_error)?
        .payload()
    {
        SpatialPlacementReorientAnchorMode::PointLike => {
            Ok(placement.facing_witness(motion.spec().direction_witness().clone()))
        }
        SpatialPlacementReorientAnchorMode::Directional(source_world_direction) => {
            let lowered_facing = lowered_reorient_facing_from_directional_anchor(
                &placement,
                *source_world_direction,
                motion.normalized_facing(),
            )
            .map_err(map_reorient_directional_anchor_error)?;
            Ok(placement.facing(lowered_facing))
        }
    }
}

pub fn apply_admitted_rotate_to_placement(
    placement: SpatialPlacementSpec,
    motion: &AdmittedSpatialRotate,
) -> Result<SpatialPlacementSpec, SpatialPlacementMotionError> {
    match motion.spec().anchor() {
        SpatialAnchorRef::ShapeOrigin => {
            let facing = admit_spatial_placement(placement.clone())
                .map_err(|_| SpatialPlacementMotionError::InvalidExistingPlacement)?
                .facing_vector();
            Ok(placement.facing(rotate_vector(
                facing,
                motion.normalized_axis(),
                motion.spec().angle_radians(),
            )))
        }
        SpatialAnchorRef::WorldOrigin | SpatialAnchorRef::FrameOrigin(_) => {
            let admitted = admit_spatial_placement(placement.clone())
                .map_err(|_| SpatialPlacementMotionError::InvalidExistingPlacement)?;
            let reference_frame = admit_spatial_frame(placement.reference_frame().clone())
                .map_err(|_| SpatialPlacementMotionError::InvalidExistingPlacement)?;
            let pivot_world_point =
                lower_supported_point_anchor(&placement, motion.spec().anchor())
                    .map_err(map_rotate_anchor_error)?
                    .payload()
                    .world_point();
            let origin_world_point = reference_frame.basis().embed_point(placement.origin());
            let rotated_origin_world_point = rotate_point_about_pivot(
                origin_world_point,
                pivot_world_point,
                motion.normalized_axis(),
                motion.spec().angle_radians(),
            );
            let rotated_facing = rotate_vector(
                admitted.facing_vector(),
                motion.normalized_axis(),
                motion.spec().angle_radians(),
            );
            Ok(placement
                .at(reference_frame
                    .basis()
                    .project_point(rotated_origin_world_point))
                .facing(rotated_facing))
        }
        _ => Err(SpatialPlacementMotionError::UnsupportedRotateAnchor),
    }
}

pub fn apply_admitted_rotate_to_placement_with_catalog(
    placement: SpatialPlacementSpec,
    motion: &AdmittedSpatialRotate,
    catalog: &impl SpatialWitnessCatalog,
) -> Result<SpatialPlacementSpec, SpatialPlacementMotionError> {
    match motion.spec().anchor() {
        SpatialAnchorRef::ShapeOrigin
        | SpatialAnchorRef::WorldOrigin
        | SpatialAnchorRef::FrameOrigin(_) => apply_admitted_rotate_to_placement(placement, motion),
        SpatialAnchorRef::FeatureOwned(_) => {
            let admitted = admit_spatial_placement(placement.clone())
                .map_err(|_| SpatialPlacementMotionError::InvalidExistingPlacement)?;
            let reference_frame = admit_spatial_frame(placement.reference_frame().clone())
                .map_err(|_| SpatialPlacementMotionError::InvalidExistingPlacement)?;
            let pivot_world_point = lower_supported_point_anchor_with_catalog(
                &placement,
                motion.spec().anchor(),
                catalog,
            )
            .map_err(map_rotate_anchor_error)?
            .payload()
            .world_point();
            let origin_world_point = reference_frame.basis().embed_point(placement.origin());
            let rotated_origin_world_point = rotate_point_about_pivot(
                origin_world_point,
                pivot_world_point,
                motion.normalized_axis(),
                motion.spec().angle_radians(),
            );
            let rotated_facing = rotate_vector(
                admitted.facing_vector(),
                motion.normalized_axis(),
                motion.spec().angle_radians(),
            );
            Ok(placement
                .at(reference_frame
                    .basis()
                    .project_point(rotated_origin_world_point))
                .facing(rotated_facing))
        }
        _ => Err(SpatialPlacementMotionError::UnsupportedRotateAnchor),
    }
}

fn map_rotate_anchor_error(error: SpatialPlacementPointAnchorError) -> SpatialPlacementMotionError {
    match error {
        SpatialPlacementPointAnchorError::UnsupportedAnchor => {
            SpatialPlacementMotionError::UnsupportedRotateAnchor
        }
        SpatialPlacementPointAnchorError::InvalidReferenceFrame(_) => {
            SpatialPlacementMotionError::InvalidExistingPlacement
        }
        SpatialPlacementPointAnchorError::AnchorWitnessFailure(error) => {
            SpatialPlacementMotionError::AnchorWitnessFailure(error)
        }
        SpatialPlacementPointAnchorError::AnchorTagFailure(error) => {
            SpatialPlacementMotionError::AnchorTagFailure(error)
        }
    }
}

fn map_reorient_directional_anchor_error(
    error: SpatialPlacementDirectionalAnchorError,
) -> SpatialPlacementMotionError {
    match error {
        SpatialPlacementDirectionalAnchorError::UnsupportedAnchor => {
            SpatialPlacementMotionError::UnsupportedReorientAnchor
        }
        SpatialPlacementDirectionalAnchorError::AmbiguousAnchorMeaning => {
            SpatialPlacementMotionError::AmbiguousReorientAnchorMeaning
        }
        SpatialPlacementDirectionalAnchorError::AnchorWitnessFailure(error) => {
            SpatialPlacementMotionError::AnchorWitnessFailure(error)
        }
        SpatialPlacementDirectionalAnchorError::InvalidExistingPlacement => {
            SpatialPlacementMotionError::InvalidExistingPlacement
        }
    }
}

fn map_move_anchor_error(error: SpatialPlacementPointAnchorError) -> SpatialPlacementMotionError {
    match error {
        SpatialPlacementPointAnchorError::UnsupportedAnchor => {
            SpatialPlacementMotionError::UnsupportedMoveAnchor
        }
        SpatialPlacementPointAnchorError::InvalidReferenceFrame(_) => {
            SpatialPlacementMotionError::InvalidExistingPlacement
        }
        SpatialPlacementPointAnchorError::AnchorWitnessFailure(error) => {
            SpatialPlacementMotionError::AnchorWitnessFailure(error)
        }
        SpatialPlacementPointAnchorError::AnchorTagFailure(error) => {
            SpatialPlacementMotionError::AnchorTagFailure(error)
        }
    }
}

fn map_offset_anchor_error(error: SpatialPlacementPointAnchorError) -> SpatialPlacementMotionError {
    match error {
        SpatialPlacementPointAnchorError::UnsupportedAnchor => {
            SpatialPlacementMotionError::UnsupportedOffsetAnchor
        }
        SpatialPlacementPointAnchorError::InvalidReferenceFrame(_) => {
            SpatialPlacementMotionError::InvalidExistingPlacement
        }
        SpatialPlacementPointAnchorError::AnchorWitnessFailure(error) => {
            SpatialPlacementMotionError::AnchorWitnessFailure(error)
        }
        SpatialPlacementPointAnchorError::AnchorTagFailure(error) => {
            SpatialPlacementMotionError::AnchorTagFailure(error)
        }
    }
}

#[cfg(test)]
#[path = "placement_motion_tests.rs"]
mod tests;
