#[test]
fn spatial_public_boundary_rejects_internal_constructor_bypass() {
    let t = trybuild::TestCases::new();
    let compile_fail = "src/certification/public_facade_contracts/compile_fail";
    t.compile_fail(format!(
        "{compile_fail}/public_spatial_authority_constructor_not_exported.rs"
    ));
}
