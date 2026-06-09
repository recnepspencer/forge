use worth_spatial::facade::anchor_selection::{
    AdmittedSpatialMove, AdmittedSpatialPointsTowardConstraint, AdmittedSpatialReorient,
    AdmittedSpatialRotate,
    admit_spatial_anchor_match_constraint, admit_spatial_lies_on_constraint, admit_spatial_move,
    admit_spatial_move_with_catalog, admit_spatial_offset, admit_spatial_offset_with_catalog,
    admit_spatial_points_toward_constraint, admit_spatial_points_toward_constraint_with_catalog,
    admit_spatial_reorient, admit_spatial_reorient_with_catalog, admit_spatial_rotate,
    admit_spatial_rotate_with_catalog, author_spatial_anchor_selection,
    author_spatial_anchor_selection_with_catalog, admit_spatial_lies_on_constraint_with_catalog,
    admit_spatial_anchor_match_constraint_with_catalog, AuthorSpatialAnchorSelectionIntent,
    SpatialMoveSpec,
};

fn main() {
    let _: Option<AdmittedSpatialMove> = None;
    let _: Option<AdmittedSpatialPointsTowardConstraint> = None;
    let _: Option<AdmittedSpatialReorient> = None;
    let _: Option<AdmittedSpatialRotate> = None;
    let _ = admit_spatial_move;
    let _ = admit_spatial_move_with_catalog;
    let _ = admit_spatial_offset;
    let _ = admit_spatial_offset_with_catalog;
    let _ = admit_spatial_rotate;
    let _ = admit_spatial_rotate_with_catalog;
    let _ = admit_spatial_reorient;
    let _ = admit_spatial_reorient_with_catalog;
    let _ = admit_spatial_lies_on_constraint;
    let _ = admit_spatial_lies_on_constraint_with_catalog;
    let _ = admit_spatial_points_toward_constraint;
    let _ = admit_spatial_points_toward_constraint_with_catalog;
    let _ = admit_spatial_anchor_match_constraint;
    let _ = admit_spatial_anchor_match_constraint_with_catalog;
    let _ = author_spatial_anchor_selection;
    let _ = author_spatial_anchor_selection_with_catalog;
    let _ = AuthorSpatialAnchorSelectionIntent::Move(SpatialMoveSpec::shape_origin());
}
