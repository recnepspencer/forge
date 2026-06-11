#[path = "placement_admission.rs"]
mod placement_admission;
#[path = "placement_constraint_anchors.rs"]
mod placement_constraint_anchors;
#[path = "placement_constraints.rs"]
mod placement_constraints;
#[path = "placement_motion.rs"]
mod placement_motion;
#[path = "placement_motion_anchors.rs"]
mod placement_motion_anchors;
#[path = "placement_types.rs"]
mod placement_types;
#[path = "placement_vocabulary.rs"]
mod placement_vocabulary;

pub(crate) use placement_admission::admit_spatial_placement;
pub(crate) use placement_constraints::{
    apply_anchor_match_constraint_to_placement_with_catalog,
    apply_lies_on_constraint_to_placement_with_catalog,
    apply_points_toward_constraint_to_placement_with_catalog,
};
pub(crate) use placement_motion::{
    apply_move_to_placement_with_catalog, apply_offset_to_placement_with_catalog,
    apply_reorient_to_placement_with_catalog, apply_rotate_to_placement_with_catalog,
};
pub(crate) use placement_types::{AdmittedSpatialPlacement, SpatialPlacementError};
pub use placement_types::{SpatialPlacementConstraintError, SpatialPlacementMotionError};
pub use placement_vocabulary::SpatialPlacementSpec;
