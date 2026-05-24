#[test]
fn domain_declaration_phase_boundaries_hold() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/domain_declaration/golden/*.rs");
    t.compile_fail("tests/ui/domain_declaration/boundaries/*.rs");
}
