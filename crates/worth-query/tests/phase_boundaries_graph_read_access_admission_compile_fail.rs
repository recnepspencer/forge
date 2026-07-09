#[test]
fn graph_read_access_admission_public_boundaries_reject_worthd_artifacts() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/graph_read_access_admission/access_admission_constructor_private.rs");
    t.compile_fail("tests/ui/graph_read_access_admission/admitted_plan_constructor_private.rs");
    t.compile_fail("tests/ui/graph_read_access_admission/admitted_plan_from_admission_private.rs");
    t.compile_fail("tests/ui/graph_read_access_admission/access_case_constructor_private.rs");
    t.compile_fail(
        "tests/ui/graph_read_access_admission/budget_exceeded_denial_constructor_private.rs",
    );
    t.compile_fail(
        "tests/ui/graph_read_access_admission/execution_counters_constructor_private.rs",
    );
    t.compile_fail("tests/ui/graph_read_access_admission/inventory_match_constructor_private.rs");
    t.compile_fail("tests/ui/graph_read_access_admission/live_access_plan_constructor_private.rs");
    t.compile_fail(
        "tests/ui/graph_read_access_admission/live_access_receipt_constructor_private.rs",
    );
    t.compile_fail(
        "tests/ui/graph_read_access_admission/live_mutation_delta_scope_constructor_private.rs",
    );
    t.compile_fail("tests/ui/graph_read_access_admission/plan_consumption_constructor_private.rs");
    t.compile_fail("tests/ui/graph_read_access_admission/posture_from_string_forbidden.rs");
}
