#[path = "support/trybuild_helpers.rs"]
mod trybuild_helpers;

#[test]
fn runtime_plan_swap_public_types_compile() {
    trybuild_helpers::run_pass_cases(&[
        "tests/ui/runtime_plan_swap/pass/plan_swap_facade_types.rs",
    ]);
}

#[test]
fn runtime_plan_swap_boundary_stays_sealed() {
    trybuild_helpers::run_compile_fail_cases(&[
        "tests/ui/runtime_plan_swap/fail/plan_swap_receipt_fields_not_public.rs",
        "tests/ui/runtime_plan_swap/fail/prior_valid_plan_fields_not_public.rs",
        "tests/ui/runtime_plan_swap/fail/plan_swap_rollback_fields_not_public.rs",
        "tests/ui/runtime_plan_swap/fail/injected_denial_variants_not_public.rs",
    ]);
}
