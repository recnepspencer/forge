#[path = "trybuild_support.rs"]
mod trybuild_support;
fn runtime_plan_swap_pass(path: &str) {
    trybuild_support::new_test_cases().pass(path);
}

fn runtime_plan_swap_fail(path: &str) {
    trybuild_support::new_test_cases().compile_fail(path);
}

#[test]
fn plan_swap_facade_types_are_importable() {
    runtime_plan_swap_pass("tests/ui/runtime_plan_swap/pass/plan_swap_facade_types.rs");
}

#[test]
fn plan_swap_receipt_fields_are_not_publicly_mintable() {
    runtime_plan_swap_fail(
        "tests/ui/runtime_plan_swap/fail/plan_swap_receipt_fields_not_public.rs",
    );
}

#[test]
fn prior_valid_plan_observation_fields_are_not_publicly_mintable() {
    runtime_plan_swap_fail("tests/ui/runtime_plan_swap/fail/prior_valid_plan_fields_not_public.rs");
}

#[test]
fn plan_swap_rollback_fields_are_not_publicly_mintable() {
    runtime_plan_swap_fail(
        "tests/ui/runtime_plan_swap/fail/plan_swap_rollback_fields_not_public.rs",
    );
}

#[test]
fn plan_swap_counters_are_not_publicly_mintable() {
    runtime_plan_swap_fail(
        "tests/ui/runtime_plan_swap/fail/plan_swap_counters_not_publicly_mintable.rs",
    );
}

#[test]
fn injected_swap_denials_are_not_public_facade_variants() {
    runtime_plan_swap_fail(
        "tests/ui/runtime_plan_swap/fail/injected_denial_variants_not_public.rs",
    );
}
