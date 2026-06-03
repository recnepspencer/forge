#[test]
fn spatial_arbitration_boundary_no_longer_teaches_runtime_or_materialization_sidecars() {
    let arbitration_mod = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/spatial_intent/arbitration/mod.rs"
    ));
    let facade = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/facade/mod.rs"));

    assert!(!arbitration_mod.contains("mod materialization;"));
    assert!(!arbitration_mod.contains("mod materialization_vocab;"));
    assert!(!arbitration_mod.contains("mod progression;"));
    assert!(!arbitration_mod.contains("mod runtime_declaration;"));
    assert!(!arbitration_mod.contains("declare_spatial_arbitration_runtime"));
    assert!(!arbitration_mod.contains("materialize_spatial_arbitration_support_report"));
    assert!(!facade.contains("declare_spatial_arbitration_runtime"));
    assert!(!facade.contains("materialize_spatial_arbitration_support_report"));
    assert!(!facade.contains("SpatialArbitrationRuntimeDeclaration"));
    assert!(!facade.contains("SpatialArbitrationSupportMaterialization"));
    assert!(!facade.contains("SpatialArbitrationMaterializationProfilePlan"));
    assert!(!facade.contains("SpatialArbitrationMaterializationDenial"));
    assert!(facade.contains("pub mod arbitration;"));
}
