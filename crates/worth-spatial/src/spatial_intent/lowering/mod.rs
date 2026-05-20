mod motion;
mod placement;
mod placement_constraints;
mod placement_motion;

pub use motion::{
    admit_spatial_move, admit_spatial_move_with_catalog, admit_spatial_offset,
    admit_spatial_reorient, admit_spatial_reorient_with_catalog, admit_spatial_rotate,
    admit_spatial_rotate_with_catalog, AdmittedSpatialMove, AdmittedSpatialOffset,
    AdmittedSpatialReorient, AdmittedSpatialRotate, SpatialMotionError, SpatialMoveSpec,
    SpatialOffsetSpec, SpatialReorientSpec, SpatialRotateSpec,
};
pub use placement::{
    admit_spatial_placement, admit_spatial_placement_with_catalog, apply_spatial_placement,
    AdmittedSpatialPlacement, SpatialPlacementError, SpatialPlacementFrame,
    SpatialPlacementGeometry, SpatialPlacementSpec,
};
pub use placement_constraints::{
    apply_admitted_anchor_match_constraint_to_placement,
    apply_admitted_lies_on_constraint_to_placement,
    apply_admitted_points_toward_constraint_to_placement, SpatialPlacementConstraintError,
};
pub use placement_motion::{
    apply_admitted_move_to_placement, apply_admitted_offset_to_placement,
    apply_admitted_reorient_to_placement, apply_admitted_rotate_to_placement,
    SpatialPlacementMotionError,
};
