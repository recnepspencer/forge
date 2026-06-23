#[test]
fn graph_touch_descriptor_public_boundaries_reject_forged_artifacts() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/graph_touch_descriptor/*.rs");
}
