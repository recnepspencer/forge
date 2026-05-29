#[test]
fn domain_handle_phase_boundaries_hold() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/domain_handle/golden/*.rs");
    t.compile_fail("tests/ui/domain_handle/boundaries/*.rs");
}
