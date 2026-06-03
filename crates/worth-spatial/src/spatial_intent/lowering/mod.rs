mod anchors;
mod lowered_intents;
mod motion;
mod operations;
mod operations_errors;
mod placement;
mod transforms;

pub use lowered_intents::{
    lower_admitted_anchor_match_constraint_intent,
    lower_admitted_anchor_match_constraint_intent_with_catalog,
    lower_admitted_lies_on_constraint_intent,
    lower_admitted_lies_on_constraint_intent_with_catalog, lower_admitted_move_intent,
    lower_admitted_move_intent_with_catalog, lower_admitted_offset_intent,
    lower_admitted_offset_intent_with_catalog, lower_admitted_points_toward_constraint_intent,
    lower_admitted_points_toward_constraint_intent_with_catalog, lower_admitted_reorient_intent,
    lower_admitted_reorient_intent_with_catalog, lower_admitted_rotate_intent,
    lower_admitted_rotate_intent_with_catalog, SpatialLoweringDenial,
};
pub use motion::{
    admit_spatial_move, admit_spatial_move_with_catalog, admit_spatial_offset,
    admit_spatial_reorient, admit_spatial_reorient_with_catalog, admit_spatial_rotate,
    admit_spatial_rotate_with_catalog, AdmittedSpatialMove, AdmittedSpatialOffset,
    AdmittedSpatialReorient, AdmittedSpatialRotate, SpatialMotionError, SpatialMoveSpec,
    SpatialOffsetSpec, SpatialReorientSpec, SpatialRotateSpec,
};
pub use operations::{
    apply_admitted_anchor_match_constraint_to_placement,
    apply_admitted_anchor_match_constraint_to_placement_with_catalog,
    apply_admitted_lies_on_constraint_to_placement,
    apply_admitted_lies_on_constraint_to_placement_with_catalog, apply_admitted_move_to_placement,
    apply_admitted_move_to_placement_with_catalog, apply_admitted_offset_to_placement,
    apply_admitted_offset_to_placement_with_catalog,
    apply_admitted_points_toward_constraint_to_placement,
    apply_admitted_points_toward_constraint_to_placement_with_catalog,
    apply_admitted_reorient_to_placement, apply_admitted_reorient_to_placement_with_catalog,
    apply_admitted_rotate_to_placement, apply_admitted_rotate_to_placement_with_catalog,
    SpatialPlacementConstraintError, SpatialPlacementMotionError,
};
pub use placement::{
    admit_spatial_placement, admit_spatial_placement_with_catalog, apply_spatial_placement,
    AdmittedSpatialPlacement, SpatialPlacementError, SpatialPlacementFrame,
    SpatialPlacementGeometry, SpatialPlacementSpec,
};
