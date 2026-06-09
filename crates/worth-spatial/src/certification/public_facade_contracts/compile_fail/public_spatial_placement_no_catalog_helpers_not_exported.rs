use worth_spatial::facade::placement::{
    admit_spatial_placement_with_catalog, apply_spatial_placement, AdmittedSpatialPlacement,
    admit_spatial_placement, apply_admitted_anchor_match_constraint_to_placement,
    apply_admitted_anchor_match_constraint_to_placement_with_catalog,
    apply_admitted_lies_on_constraint_to_placement, apply_admitted_move_to_placement,
    apply_admitted_lies_on_constraint_to_placement_with_catalog,
    apply_admitted_move_to_placement_with_catalog, apply_admitted_offset_to_placement,
    apply_admitted_offset_to_placement_with_catalog,
    apply_admitted_points_toward_constraint_to_placement,
    apply_admitted_points_toward_constraint_to_placement_with_catalog,
    apply_admitted_reorient_to_placement, apply_admitted_reorient_to_placement_with_catalog,
    apply_admitted_rotate_to_placement, apply_admitted_rotate_to_placement_with_catalog,
    SpatialPlacementSpec,
};

fn main() {
    let _ = admit_spatial_placement_with_catalog;
    let _ = apply_spatial_placement;
    let _: Option<AdmittedSpatialPlacement> = None;
    let _ = admit_spatial_placement;
    let _ = apply_admitted_move_to_placement;
    let _ = apply_admitted_move_to_placement_with_catalog;
    let _ = apply_admitted_offset_to_placement;
    let _ = apply_admitted_offset_to_placement_with_catalog;
    let _ = apply_admitted_rotate_to_placement;
    let _ = apply_admitted_rotate_to_placement_with_catalog;
    let _ = apply_admitted_reorient_to_placement;
    let _ = apply_admitted_reorient_to_placement_with_catalog;
    let _ = apply_admitted_lies_on_constraint_to_placement;
    let _ = apply_admitted_lies_on_constraint_to_placement_with_catalog;
    let _ = apply_admitted_points_toward_constraint_to_placement;
    let _ = apply_admitted_points_toward_constraint_to_placement_with_catalog;
    let _ = apply_admitted_anchor_match_constraint_to_placement;
    let _ = apply_admitted_anchor_match_constraint_to_placement_with_catalog;
    let _ = SpatialPlacementSpec::world();
}
