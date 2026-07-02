#[path = "trybuild_support.rs"]
mod trybuild_support;

#[test]
fn graph_touch_compile_failures_prevent_raw_or_generic_touch_construction() {
    let tests = trybuild_support::new_test_cases();
    tests.compile_fail("tests/ui/graph_touch/external_callers_cannot_forge_touch_descriptors.rs");
}
