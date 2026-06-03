#[test]
fn spatial_public_boundary_rejects_internal_constructor_bypass() {
    let t = trybuild::TestCases::new();
    let compile_fail = "src/certification/public_facade_contracts/compile_fail";
    t.compile_fail(format!(
        "{compile_fail}/public_spatial_admitted_witness_request_artifacts_not_exported.rs"
    ));
    t.compile_fail(format!(
        "{compile_fail}/public_spatial_top_level_witness_helpers_not_exported.rs"
    ));
    t.compile_fail(format!(
        "{compile_fail}/public_spatial_top_level_refs_not_exported.rs"
    ));
    t.compile_fail(format!(
        "{compile_fail}/public_spatial_top_level_lowering_runtime_products_not_exported.rs"
    ));
    t.compile_fail(format!(
        "{compile_fail}/public_spatial_top_level_arbitration_runtime_products_not_exported.rs"
    ));
    t.compile_fail(format!(
        "{compile_fail}/public_spatial_preview_and_continuity_helpers_not_exported.rs"
    ));
    t.compile_fail(format!(
        "{compile_fail}/public_spatial_top_level_bindings_report_products_not_exported.rs"
    ));
    t.compile_fail(format!(
        "{compile_fail}/public_spatial_flat_semantic_facade_not_exported.rs"
    ));
    t.compile_fail(format!(
        "{compile_fail}/public_spatial_test_support_not_exported.rs"
    ));
}
