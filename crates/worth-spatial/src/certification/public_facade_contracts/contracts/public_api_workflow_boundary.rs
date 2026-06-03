#[test]
fn spatial_workflow_boundary_no_longer_teaches_preview_or_continuity_sidecars() {
    let spatial_intent_mod = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/spatial_intent/mod.rs"
    ));
    let facade = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/facade/mod.rs"));

    assert!(!spatial_intent_mod.contains("mod preview;"));
    assert!(!spatial_intent_mod.contains("mod continuity;"));
    assert!(!facade.contains("prepare_spatial_intent_preview"));
    assert!(!facade.contains("assess_spatial_identity_continuity_from_analysis"));
    assert!(!facade.contains("assess_spatial_identity_continuity_from_resolution"));
    assert!(facade.contains("pub mod arbitration;"));
}
