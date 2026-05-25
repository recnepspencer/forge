#[test]
fn domain_capability_dx_boundaries_hold() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/domain_capabilities/golden/*.rs");
    t.compile_fail("tests/ui/domain_capabilities/dx_boundaries/*.rs");
}
