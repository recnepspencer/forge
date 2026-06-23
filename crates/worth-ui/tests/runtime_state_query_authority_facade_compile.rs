#[path = "support/trybuild_helpers.rs"]
mod trybuild_helpers;

#[test]
fn runtime_state_query_boundary_stays_sealed() {
    trybuild_helpers::run_compile_fail_cases(&[
        "tests/ui/runtime_authority/fail/durable_state_inventory_fields_not_public.rs",
        "tests/ui/runtime_authority/fail/durable_state_reconciliation_plan_fields_not_public.rs",
        "tests/ui/runtime_authority/fail/durable_state_reconciliation_receipt_fields_not_public.rs",
        "tests/ui/runtime_authority/fail/local_query_result_state_enum_cannot_replace_query_binding_posture.rs",
        "tests/ui/runtime_authority/fail/local_subscription_recovery_path_cannot_replace_query_rebind.rs",
    ]);
}
