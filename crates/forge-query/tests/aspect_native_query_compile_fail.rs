#[test]
fn aspect_native_query_boundaries_are_compile_time_enforced() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/aspect_native_query/*.rs");
}
