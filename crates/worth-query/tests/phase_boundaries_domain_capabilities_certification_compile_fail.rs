#[test]
fn certification_surface_boundaries_remain_sealed() {
    let t = trybuild::TestCases::new();

    t.pass("tests/ui/domain_capabilities/golden/domain_capability_certification_surface_readout_compiles.rs");
    t.compile_fail("tests/ui/domain_capabilities/certification/*.rs");
}
