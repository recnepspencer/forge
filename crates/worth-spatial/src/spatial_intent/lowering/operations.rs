use crate::spatial_intent::constraints::{
    AdmittedSpatialAnchorMatchConstraint, AdmittedSpatialLiesOnConstraint,
    AdmittedSpatialPointsTowardConstraint,
};
use crate::spatial_intent::lowering::lowered_intents::{
    lower_admitted_anchor_match_constraint_semantic_intent_with_catalog,
    lower_admitted_lies_on_constraint_semantic_intent_with_catalog,
    lower_admitted_move_semantic_intent_with_catalog,
    lower_admitted_offset_semantic_intent_with_catalog,
    lower_admitted_points_toward_constraint_semantic_intent_with_catalog,
    lower_admitted_reorient_semantic_intent_with_catalog,
    lower_admitted_rotate_semantic_intent_with_catalog, LoweredSpatialOperation,
    SpatialLoweringDenial,
};
pub use crate::spatial_intent::lowering::operations_errors::{
    SpatialPlacementConstraintError, SpatialPlacementMotionError,
};
use crate::spatial_intent::lowering::{
    AdmittedSpatialMove, AdmittedSpatialOffset, AdmittedSpatialReorient, AdmittedSpatialRotate,
    SpatialPlacementSpec,
};
use crate::spatial_intent::refs::{EmptySpatialWitnessCatalog, SpatialWitnessCatalog};
use crate::spatial_intent::resolution::SpatialFrameError;

pub fn apply_admitted_move_to_placement(
    placement: SpatialPlacementSpec,
    motion: &AdmittedSpatialMove,
) -> Result<SpatialPlacementSpec, SpatialPlacementMotionError> {
    apply_admitted_move_to_placement_with_catalog(placement, motion, &EmptySpatialWitnessCatalog)
}
pub fn apply_admitted_offset_to_placement(
    placement: SpatialPlacementSpec,
    motion: &AdmittedSpatialOffset,
) -> Result<SpatialPlacementSpec, SpatialPlacementMotionError> {
    apply_admitted_offset_to_placement_with_catalog(placement, motion, &EmptySpatialWitnessCatalog)
}
pub fn apply_admitted_rotate_to_placement(
    placement: SpatialPlacementSpec,
    motion: &AdmittedSpatialRotate,
) -> Result<SpatialPlacementSpec, SpatialPlacementMotionError> {
    apply_admitted_rotate_to_placement_with_catalog(placement, motion, &EmptySpatialWitnessCatalog)
}
pub fn apply_admitted_reorient_to_placement(
    placement: SpatialPlacementSpec,
    motion: &AdmittedSpatialReorient,
) -> Result<SpatialPlacementSpec, SpatialPlacementMotionError> {
    apply_admitted_reorient_to_placement_with_catalog(
        placement,
        motion,
        &EmptySpatialWitnessCatalog,
    )
}
pub fn apply_admitted_lies_on_constraint_to_placement(
    placement: SpatialPlacementSpec,
    constraint: &AdmittedSpatialLiesOnConstraint,
) -> Result<SpatialPlacementSpec, SpatialPlacementConstraintError> {
    apply_admitted_lies_on_constraint_to_placement_with_catalog(
        placement,
        constraint,
        &EmptySpatialWitnessCatalog,
    )
}
pub fn apply_admitted_points_toward_constraint_to_placement(
    placement: SpatialPlacementSpec,
    constraint: &AdmittedSpatialPointsTowardConstraint,
) -> Result<SpatialPlacementSpec, SpatialPlacementConstraintError> {
    apply_admitted_points_toward_constraint_to_placement_with_catalog(
        placement,
        constraint,
        &EmptySpatialWitnessCatalog,
    )
}
pub fn apply_admitted_anchor_match_constraint_to_placement(
    placement: SpatialPlacementSpec,
    constraint: &AdmittedSpatialAnchorMatchConstraint,
) -> Result<SpatialPlacementSpec, SpatialPlacementConstraintError> {
    apply_admitted_anchor_match_constraint_to_placement_with_catalog(
        placement,
        constraint,
        &EmptySpatialWitnessCatalog,
    )
}

pub fn apply_admitted_move_to_placement_with_catalog(
    placement: SpatialPlacementSpec,
    motion: &AdmittedSpatialMove,
    catalog: &impl SpatialWitnessCatalog,
) -> Result<SpatialPlacementSpec, SpatialPlacementMotionError> {
    let lowered =
        lower_admitted_move_semantic_intent_with_catalog(placement.clone(), motion, catalog)
            .map_err(map_move_denial)?;
    apply_lowered_intent_to_placement(placement, lowered.operation())
        .ok_or(SpatialPlacementMotionError::InvalidExistingPlacement)
}
pub fn apply_admitted_offset_to_placement_with_catalog(
    placement: SpatialPlacementSpec,
    motion: &AdmittedSpatialOffset,
    catalog: &impl SpatialWitnessCatalog,
) -> Result<SpatialPlacementSpec, SpatialPlacementMotionError> {
    let lowered =
        lower_admitted_offset_semantic_intent_with_catalog(placement.clone(), motion, catalog)
            .map_err(map_offset_denial)?;
    apply_lowered_intent_to_placement(placement, lowered.operation())
        .ok_or(SpatialPlacementMotionError::InvalidExistingPlacement)
}
pub fn apply_admitted_rotate_to_placement_with_catalog(
    placement: SpatialPlacementSpec,
    motion: &AdmittedSpatialRotate,
    catalog: &impl SpatialWitnessCatalog,
) -> Result<SpatialPlacementSpec, SpatialPlacementMotionError> {
    let lowered =
        lower_admitted_rotate_semantic_intent_with_catalog(placement.clone(), motion, catalog)
            .map_err(map_rotate_denial)?;
    apply_lowered_intent_to_placement(placement, lowered.operation())
        .ok_or(SpatialPlacementMotionError::InvalidExistingPlacement)
}
pub fn apply_admitted_reorient_to_placement_with_catalog(
    placement: SpatialPlacementSpec,
    motion: &AdmittedSpatialReorient,
    catalog: &impl SpatialWitnessCatalog,
) -> Result<SpatialPlacementSpec, SpatialPlacementMotionError> {
    let lowered =
        lower_admitted_reorient_semantic_intent_with_catalog(placement.clone(), motion, catalog)
            .map_err(map_reorient_denial)?;
    apply_lowered_intent_to_placement(placement, lowered.operation())
        .ok_or(SpatialPlacementMotionError::InvalidExistingPlacement)
}
pub fn apply_admitted_lies_on_constraint_to_placement_with_catalog(
    placement: SpatialPlacementSpec,
    constraint: &AdmittedSpatialLiesOnConstraint,
    catalog: &impl SpatialWitnessCatalog,
) -> Result<SpatialPlacementSpec, SpatialPlacementConstraintError> {
    let lowered = lower_admitted_lies_on_constraint_semantic_intent_with_catalog(
        placement.clone(),
        constraint,
        catalog,
    )
    .map_err(map_lies_on_denial)?;
    apply_lowered_intent_to_placement(placement, lowered.operation()).ok_or(
        SpatialPlacementConstraintError::InvalidReferenceFrame(SpatialFrameError::InvalidNormal),
    )
}
pub fn apply_admitted_points_toward_constraint_to_placement_with_catalog(
    placement: SpatialPlacementSpec,
    constraint: &AdmittedSpatialPointsTowardConstraint,
    catalog: &impl SpatialWitnessCatalog,
) -> Result<SpatialPlacementSpec, SpatialPlacementConstraintError> {
    let lowered = lower_admitted_points_toward_constraint_semantic_intent_with_catalog(
        placement.clone(),
        constraint,
        catalog,
    )
    .map_err(map_points_toward_denial)?;
    apply_lowered_intent_to_placement(placement, lowered.operation()).ok_or(
        SpatialPlacementConstraintError::InvalidReferenceFrame(SpatialFrameError::InvalidNormal),
    )
}
pub fn apply_admitted_anchor_match_constraint_to_placement_with_catalog(
    placement: SpatialPlacementSpec,
    constraint: &AdmittedSpatialAnchorMatchConstraint,
    catalog: &impl SpatialWitnessCatalog,
) -> Result<SpatialPlacementSpec, SpatialPlacementConstraintError> {
    let lowered = lower_admitted_anchor_match_constraint_semantic_intent_with_catalog(
        placement.clone(),
        constraint,
        catalog,
    )
    .map_err(map_anchor_match_denial)?;
    apply_lowered_intent_to_placement(placement, lowered.operation()).ok_or(
        SpatialPlacementConstraintError::InvalidReferenceFrame(SpatialFrameError::InvalidNormal),
    )
}

fn apply_lowered_intent_to_placement(
    placement: SpatialPlacementSpec,
    operation: &LoweredSpatialOperation,
) -> Option<SpatialPlacementSpec> {
    match operation {
        LoweredSpatialOperation::Move {
            anchor_world_point,
            target_world_point,
        } => crate::spatial_intent::lowering::transforms::translate_anchor_to_world_point(
            placement,
            *anchor_world_point,
            *target_world_point,
        ),
        LoweredSpatialOperation::Offset { offset } => {
            crate::spatial_intent::lowering::transforms::translate_placement_world_offset(
                placement, *offset,
            )
        }
        LoweredSpatialOperation::RotateFacingOnly {
            source_facing,
            axis,
            angle_radians,
        } => Some(placement.clone().facing(
            crate::spatial_intent::lowering::transforms::rotate_vector(
                *source_facing,
                *axis,
                *angle_radians,
            ),
        )),
        LoweredSpatialOperation::RotateAroundPivot {
            source_facing,
            pivot_world_point,
            axis,
            angle_radians,
        } => crate::spatial_intent::lowering::transforms::rotate_origin_and_facing(
            placement.clone(),
            *source_facing,
            *pivot_world_point,
            *axis,
            *angle_radians,
        ),
        LoweredSpatialOperation::ReorientPointLike {
            target_direction, ..
        } => Some(placement.facing_witness(target_direction.to_witness())),
        LoweredSpatialOperation::ReorientDirectional {
            source_world_direction,
            target_world_direction,
        } => crate::spatial_intent::lowering::transforms::rotate_facing_to_align_source(
            &placement,
            *source_world_direction,
            *target_world_direction,
        )
        .map(|facing| placement.facing(facing)),
        LoweredSpatialOperation::LiesOnShapeOrigin { target_frame } => Some(
            placement
                .relative_to(target_frame.spec().clone())
                .at([0.0, 0.0, 0.0]),
        ),
        LoweredSpatialOperation::LiesOnProjected {
            target_frame,
            anchor_world_point,
        } => crate::spatial_intent::lowering::transforms::project_subject_anchor_onto_frame_plane(
            placement,
            target_frame,
            *anchor_world_point,
        ),
        LoweredSpatialOperation::PointsToward {
            anchor_world_point,
            target_world_point,
        } => Some(placement.facing([
            target_world_point[0] - anchor_world_point[0],
            target_world_point[1] - anchor_world_point[1],
            target_world_point[2] - anchor_world_point[2],
        ])),
        LoweredSpatialOperation::AnchorMatch {
            anchor_world_point,
            target_world_point,
        } => crate::spatial_intent::lowering::transforms::translate_anchor_to_world_point(
            placement,
            *anchor_world_point,
            *target_world_point,
        ),
    }
}

fn map_move_denial(denial: SpatialLoweringDenial) -> SpatialPlacementMotionError {
    match denial {
        SpatialLoweringDenial::WitnessFailure(value) => {
            SpatialPlacementMotionError::AnchorWitnessFailure(value)
        }
        SpatialLoweringDenial::TagFailure(value) => {
            SpatialPlacementMotionError::AnchorTagFailure(value)
        }
        SpatialLoweringDenial::InvalidExistingPlacement
        | SpatialLoweringDenial::InvalidReferenceFrame(_) => {
            SpatialPlacementMotionError::InvalidExistingPlacement
        }
        _ => SpatialPlacementMotionError::UnsupportedMoveAnchor,
    }
}
fn map_offset_denial(denial: SpatialLoweringDenial) -> SpatialPlacementMotionError {
    match denial {
        SpatialLoweringDenial::WitnessFailure(value) => {
            SpatialPlacementMotionError::AnchorWitnessFailure(value)
        }
        SpatialLoweringDenial::TagFailure(value) => {
            SpatialPlacementMotionError::AnchorTagFailure(value)
        }
        SpatialLoweringDenial::InvalidExistingPlacement
        | SpatialLoweringDenial::InvalidReferenceFrame(_) => {
            SpatialPlacementMotionError::InvalidExistingPlacement
        }
        _ => SpatialPlacementMotionError::UnsupportedOffsetAnchor,
    }
}
fn map_rotate_denial(denial: SpatialLoweringDenial) -> SpatialPlacementMotionError {
    match denial {
        SpatialLoweringDenial::WitnessFailure(value) => {
            SpatialPlacementMotionError::AnchorWitnessFailure(value)
        }
        SpatialLoweringDenial::TagFailure(value) => {
            SpatialPlacementMotionError::AnchorTagFailure(value)
        }
        SpatialLoweringDenial::InvalidExistingPlacement
        | SpatialLoweringDenial::InvalidReferenceFrame(_) => {
            SpatialPlacementMotionError::InvalidExistingPlacement
        }
        _ => SpatialPlacementMotionError::UnsupportedRotateAnchor,
    }
}
fn map_reorient_denial(denial: SpatialLoweringDenial) -> SpatialPlacementMotionError {
    match denial {
        SpatialLoweringDenial::Ambiguous => {
            SpatialPlacementMotionError::AmbiguousReorientAnchorMeaning
        }
        SpatialLoweringDenial::WitnessFailure(value) => {
            SpatialPlacementMotionError::AnchorWitnessFailure(value)
        }
        SpatialLoweringDenial::InvalidExistingPlacement
        | SpatialLoweringDenial::InvalidReferenceFrame(_) => {
            SpatialPlacementMotionError::InvalidExistingPlacement
        }
        _ => SpatialPlacementMotionError::UnsupportedReorientAnchor,
    }
}
fn map_lies_on_denial(denial: SpatialLoweringDenial) -> SpatialPlacementConstraintError {
    match denial {
        SpatialLoweringDenial::WitnessFailure(value) => {
            SpatialPlacementConstraintError::AnchorWitnessFailure(value)
        }
        SpatialLoweringDenial::TagFailure(value) => {
            SpatialPlacementConstraintError::AnchorTagFailure(value)
        }
        SpatialLoweringDenial::InvalidReferenceFrame(value) => {
            SpatialPlacementConstraintError::InvalidReferenceFrame(value)
        }
        _ => SpatialPlacementConstraintError::UnsupportedLiesOnAnchor,
    }
}
fn map_points_toward_denial(denial: SpatialLoweringDenial) -> SpatialPlacementConstraintError {
    match denial {
        SpatialLoweringDenial::Coincident => SpatialPlacementConstraintError::CoincidentTarget,
        SpatialLoweringDenial::WitnessFailure(value) => {
            SpatialPlacementConstraintError::AnchorWitnessFailure(value)
        }
        SpatialLoweringDenial::TagFailure(value) => {
            SpatialPlacementConstraintError::AnchorTagFailure(value)
        }
        SpatialLoweringDenial::InvalidReferenceFrame(value) => {
            SpatialPlacementConstraintError::InvalidReferenceFrame(value)
        }
        _ => SpatialPlacementConstraintError::UnsupportedPointsTowardAnchor,
    }
}
fn map_anchor_match_denial(denial: SpatialLoweringDenial) -> SpatialPlacementConstraintError {
    match denial {
        SpatialLoweringDenial::WitnessFailure(value) => {
            SpatialPlacementConstraintError::AnchorWitnessFailure(value)
        }
        SpatialLoweringDenial::TagFailure(value) => {
            SpatialPlacementConstraintError::AnchorTagFailure(value)
        }
        SpatialLoweringDenial::InvalidReferenceFrame(value) => {
            SpatialPlacementConstraintError::InvalidReferenceFrame(value)
        }
        _ => SpatialPlacementConstraintError::UnsupportedAnchorMatch,
    }
}
