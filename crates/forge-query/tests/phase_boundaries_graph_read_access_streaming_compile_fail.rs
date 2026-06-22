#[test]
fn graph_read_access_streaming_public_boundaries_reject_forged_artifacts() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/graph_read_access_streaming/cursor_constructor_private.rs");
    t.compile_fail("tests/ui/graph_read_access_streaming/page_budget_constructor_private.rs");
    t.compile_fail("tests/ui/graph_read_access_streaming/page_receipt_constructor_private.rs");
    t.compile_fail("tests/ui/graph_read_access_streaming/receipt_constructor_private.rs");
    t.compile_fail("tests/ui/graph_read_access_streaming/plan_constructor_private.rs");
    t.compile_fail("tests/ui/graph_read_access_streaming/session_constructor_private.rs");
}
