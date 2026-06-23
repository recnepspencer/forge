#[test]
fn graph_read_bypass_public_boundaries_reject_forged_artifacts() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/graph_read_bypass/*.rs");
}
