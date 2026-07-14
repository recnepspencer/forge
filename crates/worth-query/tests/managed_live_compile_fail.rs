#[test]
fn managed_live_resource_boundaries_are_compile_time_enforced() {
    let cases = trybuild::TestCases::new();
    cases.compile_fail("tests/ui/managed_live/*.rs");
}
