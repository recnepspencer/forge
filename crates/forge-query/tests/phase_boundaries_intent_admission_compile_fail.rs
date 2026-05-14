#[test]
fn intent_admission_dx_boundaries_hold() {
    let t = trybuild::TestCases::new();
    t.compile_fail(
        "tests/ui/intent_admission/runtime_intent_admission_review_constructor_private.rs",
    );
    t.compile_fail("tests/ui/intent_admission/admitted_runtime_intent_constructor_private.rs");
    t.compile_fail("tests/ui/intent_admission/raw_request_cannot_mint_admitted_plan.rs");
    t.compile_fail("tests/ui/intent_admission/family_inventory_row_constructor_private.rs");
    t.compile_fail("tests/ui/intent_admission/family_inventory_constructor_private.rs");
    t.compile_fail("tests/ui/intent_admission/support_row_constructor_private.rs");
    t.compile_fail("tests/ui/intent_admission/support_matrix_constructor_private.rs");
    t.compile_fail("tests/ui/intent_admission/coverage_row_constructor_private.rs");
    t.compile_fail("tests/ui/intent_admission/coverage_inventory_constructor_private.rs");
    t.compile_fail("tests/ui/intent_admission/authoritative_plan_constructor_private.rs");
    t.compile_fail("tests/ui/intent_admission/effect_plan_constructor_private.rs");
    t.compile_fail("tests/ui/intent_admission/authoritative_handoff_constructor_private.rs");
    t.compile_fail("tests/ui/intent_admission/effect_handoff_constructor_private.rs");
    t.compile_fail(
        "tests/ui/intent_admission/authoritative_execution_binding_constructor_private.rs",
    );
    t.compile_fail("tests/ui/intent_admission/effect_execution_binding_constructor_private.rs");
    t.compile_fail("tests/ui/intent_admission/advisory_decision_constructor_private.rs");
    t.compile_fail("tests/ui/intent_admission/violation_decision_constructor_private.rs");
    t.compile_fail("tests/ui/intent_admission/advisory_stop_constructor_private.rs");
    t.compile_fail("tests/ui/intent_admission/violation_stop_constructor_private.rs");
}
