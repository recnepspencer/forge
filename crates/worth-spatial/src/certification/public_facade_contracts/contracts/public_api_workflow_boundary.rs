#[test]
fn spatial_workflow_boundary_no_longer_teaches_preview_or_continuity_sidecars() {
    let lib = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));
    let facade = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/facade/mod.rs"));
    let anchor_selection_authoring = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/anchor_selection/query_native_authoring.rs"
    ));
    let anchor_selection_facade = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/facade/anchor_selection.rs"
    ));
    let certification_file =
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/certification.rs"));
    let placement_mod = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/placement/mod.rs"));
    let placement_motion = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/placement/placement_motion.rs"
    ));
    let placement_constraints = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/placement/placement_constraints.rs"
    ));
    let placement_motion_anchors = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/placement/placement_motion_anchors.rs"
    ));
    let placement_constraint_anchors = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/placement/placement_constraint_anchors.rs"
    ));

    assert!(!lib.contains("mod spatial_domain;"));
    assert!(!facade.contains("prepare_spatial_intent_preview"));
    assert!(!facade.contains("assess_spatial_identity_continuity_from_analysis"));
    assert!(!facade.contains("assess_spatial_identity_continuity_from_resolution"));
    assert!(!facade.contains("pub mod arbitration;"));
    assert!(!anchor_selection_authoring.contains("certification::support::"));
    assert!(!anchor_selection_facade.contains("certification::support::"));
    assert!(!certification_file.contains("pub mod support;"));
    assert!(placement_mod.contains("mod placement_admission;"));
    assert!(!placement_mod.contains("mod placement_anchors;"));
    assert!(placement_mod.contains("mod placement_motion_anchors;"));
    assert!(placement_mod.contains("mod placement_constraint_anchors;"));
    assert!(placement_mod.contains("mod placement_motion;"));
    assert!(placement_mod.contains("mod placement_constraints;"));
    assert!(!placement_mod.contains("mod placement_transforms;"));
    assert!(!placement_mod.contains("internal/"));
    assert!(!placement_motion.contains("PlacementApplicationDenial"));
    assert!(!placement_constraints.contains("PlacementApplicationDenial"));
    assert!(!placement_motion_anchors.contains("AdmittedSpatialPlacement"));
    assert!(!placement_constraint_anchors.contains("LoweredReorientAnchor"));
    assert!(placement_motion.contains("rotate_origin_and_facing"));
    assert!(placement_constraints.contains("project_subject_anchor_onto_frame_plane"));
}
