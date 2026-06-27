#[path = "trybuild_support.rs"]
mod trybuild_support;
#[test]
fn durable_state_inventory_fields_are_not_publicly_mintable() {
    runtime_authority_compile_fail(
        "tests/ui/runtime_authority/fail/durable_state_inventory_fields_not_public.rs",
    );
}

#[test]
fn durable_state_reconciliation_plan_fields_are_not_publicly_mintable() {
    runtime_authority_compile_fail(
        "tests/ui/runtime_authority/fail/durable_state_reconciliation_plan_fields_not_public.rs",
    );
}

#[test]
fn durable_state_reconciliation_receipt_fields_are_not_publicly_mintable() {
    runtime_authority_compile_fail(
        "tests/ui/runtime_authority/fail/durable_state_reconciliation_receipt_fields_not_public.rs",
    );
}

#[test]
fn durable_state_reconciliation_counters_are_not_publicly_mintable() {
    runtime_authority_compile_fail(
        "tests/ui/runtime_authority/fail/durable_state_reconciliation_counters_not_publicly_mintable.rs",
    );
}

#[test]
fn durable_state_carry_forward_fields_are_not_publicly_mintable() {
    runtime_authority_compile_fail(
        "tests/ui/runtime_authority/fail/durable_state_carry_forward_fields_not_public.rs",
    );
}

#[test]
fn durable_state_replacement_fields_are_not_publicly_mintable() {
    runtime_authority_compile_fail(
        "tests/ui/runtime_authority/fail/durable_state_replacement_fields_not_public.rs",
    );
}

#[test]
fn local_query_result_state_enum_cannot_replace_query_binding_posture() {
    runtime_authority_compile_fail(
        "tests/ui/runtime_authority/fail/local_query_result_state_enum_cannot_replace_query_binding_posture.rs",
    );
}

#[test]
fn local_subscription_recovery_path_cannot_replace_query_rebind_plan() {
    runtime_authority_compile_fail(
        "tests/ui/runtime_authority/fail/local_subscription_recovery_path_cannot_replace_query_rebind.rs",
    );
}

fn runtime_authority_compile_fail(path: &str) {
    trybuild_support::new_test_cases().compile_fail(path);
}

