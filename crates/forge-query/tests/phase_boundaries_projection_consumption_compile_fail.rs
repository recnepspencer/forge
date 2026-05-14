#[test]
fn projection_consumption_compile_fail_boundaries_hold() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/projection_consumption/*.rs");
}
