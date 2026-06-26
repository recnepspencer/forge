fn runtime_diagnostics_compile_pass(path: &str) {
    trybuild::TestCases::new().pass(path);
}

fn runtime_diagnostics_compile_fail(path: &str) {
    trybuild::TestCases::new().compile_fail(path);
}

#[test]
fn runtime_diagnostics_facade_types_are_importable() {
    runtime_diagnostics_compile_pass(
        "tests/ui/runtime_diagnostics/pass/diagnostic_facade_types.rs",
    );
}

#[test]
fn diagnostics_projection_facade_types_are_importable() {
    runtime_diagnostics_compile_pass(
        "tests/ui/runtime_diagnostics/pass/diagnostics_projection_facade_types.rs",
    );
}

#[test]
fn diagnostic_report_fields_are_not_publicly_mintable() {
    runtime_diagnostics_compile_fail(
        "tests/ui/runtime_diagnostics/fail/diagnostic_report_fields_not_public.rs",
    );
}

#[test]
fn diagnostics_projection_fields_are_not_publicly_mintable() {
    runtime_diagnostics_compile_fail(
        "tests/ui/runtime_diagnostics/fail/diagnostics_projection_fields_not_public.rs",
    );
}

#[test]
fn diagnostics_projection_rejects_freeform_query_status_rows() {
    runtime_diagnostics_compile_fail(
        "tests/ui/runtime_diagnostics/fail/diagnostics_projection_rejects_freeform_query_status_rows.rs",
    );
}

#[test]
fn raw_strings_cannot_replace_diagnostic_codes() {
    runtime_diagnostics_compile_fail(
        "tests/ui/runtime_diagnostics/fail/raw_strings_cannot_replace_diagnostic_codes.rs",
    );
}

#[test]
fn projection_hook_cannot_mint_runtime_truth() {
    runtime_diagnostics_compile_fail(
        "tests/ui/runtime_diagnostics/fail/projection_hook_cannot_mint_runtime_truth.rs",
    );
}

#[test]
fn richness_policy_fields_are_not_publicly_mintable() {
    runtime_diagnostics_compile_fail(
        "tests/ui/runtime_diagnostics/fail/richness_policy_fields_not_public.rs",
    );
}
