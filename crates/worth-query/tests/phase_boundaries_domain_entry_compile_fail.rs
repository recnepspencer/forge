#[test]
fn domain_entry_phase_boundaries_hold() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/domain_entry/golden/*.rs");
    t.compile_fail("tests/ui/domain_entry/boundaries/*.rs");
}
