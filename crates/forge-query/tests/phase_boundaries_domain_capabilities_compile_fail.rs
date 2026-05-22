#[test]
fn domain_capability_phase_boundaries_hold() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/domain_capabilities/boundaries/**/*.rs");
}
