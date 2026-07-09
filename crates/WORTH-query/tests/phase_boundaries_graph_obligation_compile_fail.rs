#[test]
fn graph_obligation_public_boundaries_reject_worthd_artifacts() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/graph_obligation/dispatch/*.rs");
    t.compile_fail("tests/ui/graph_obligation/index/*.rs");
    t.compile_fail("tests/ui/graph_obligation/registration/*.rs");
    t.compile_fail("tests/ui/graph_obligation/consumer_kit/*.rs");
}
