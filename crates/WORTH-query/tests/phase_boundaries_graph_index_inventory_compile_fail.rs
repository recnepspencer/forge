#[test]
fn graph_index_inventory_public_boundaries_reject_worthd_artifacts() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/graph_index_inventory/inventory_constructor_private.rs");
    t.compile_fail("tests/ui/graph_index_inventory/support_row_constructor_private.rs");
    t.compile_fail("tests/ui/graph_index_inventory/match_report_constructor_private.rs");
    t.compile_fail("tests/ui/graph_index_inventory/synthetic_matcher_not_exported.rs");
}
