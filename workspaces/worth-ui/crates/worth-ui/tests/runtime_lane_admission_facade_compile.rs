#[path = "trybuild_support.rs"]
mod trybuild_support;
fn runtime_lane_admission_compile_fail(path: &str) {
    trybuild_support::new_test_cases().compile_fail(path);
}

#[test]
fn execution_lane_support_fields_are_not_publicly_mintable() {
    runtime_lane_admission_compile_fail(
        "tests/ui/runtime_authority/fail/execution_lane_support_fields_not_public.rs",
    );
}

#[test]
fn lane_admission_fields_are_not_publicly_mintable() {
    runtime_lane_admission_compile_fail(
        "tests/ui/runtime_authority/fail/lane_admission_fields_not_public.rs",
    );
}

#[test]
fn lane_adapter_hook_fields_are_not_publicly_mintable() {
    runtime_lane_admission_compile_fail(
        "tests/ui/runtime_authority/fail/lane_adapter_hook_fields_not_public.rs",
    );
}

#[test]
fn unsupported_hook_denial_fields_are_not_publicly_mintable() {
    runtime_lane_admission_compile_fail(
        "tests/ui/runtime_authority/fail/unsupported_hook_denial_fields_not_public.rs",
    );
}

#[test]
fn private_component_lane_string_cannot_enter_lane_admission() {
    runtime_lane_admission_compile_fail(
        "tests/ui/runtime_authority/fail/private_component_lane_string_cannot_enter_lane_admission.rs",
    );
}

#[test]
fn extension_hook_cannot_override_active_plan_truth() {
    runtime_lane_admission_compile_fail(
        "tests/ui/runtime_authority/fail/extension_hook_cannot_override_active_plan_truth.rs",
    );
}
