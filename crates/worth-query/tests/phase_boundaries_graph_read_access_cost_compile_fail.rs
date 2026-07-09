#[test]
fn graph_read_access_cost_public_boundaries_reject_worthd_artifacts() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/graph_read_access_cost/cost_estimate_constructor_private.rs");
    t.compile_fail("tests/ui/graph_read_access_cost/cost_status_constructor_private.rs");
    t.compile_fail("tests/ui/graph_read_access_cost/budget_class_constructor_private.rs");
    t.compile_fail("tests/ui/graph_read_access_cost/complexity_contract_constructor_private.rs");
    t.compile_fail("tests/ui/graph_read_access_cost/intrinsic_estimate_constructor_private.rs");
    t.compile_fail("tests/ui/graph_read_access_cost/supported_estimate_constructor_private.rs");
    t.compile_fail("tests/ui/graph_read_access_cost/memory_estimate_default_forbidden.rs");
    t.compile_fail("tests/ui/graph_read_access_cost/budget_check_constructor_private.rs");
    t.compile_fail("tests/ui/graph_read_access_cost/status_kind_upgrade_forbidden.rs");
}
