#[path = "support/trybuild_helpers.rs"]
mod trybuild_helpers;

const RUNTIME_SUPPORT_PASS_CASES: &[&str] = &[
    "tests/ui/runtime_diagnostics/pass/diagnostic_facade_types.rs",
    "tests/ui/runtime_diagnostics/pass/diagnostics_projection_facade_types.rs",
    "tests/ui/runtime_file_rust_replacement_parity/pass/parity_public_types_are_importable.rs",
    "tests/ui/runtime_identity_state_query_certification/pass/identity_state_query_certification_facade_types.rs",
    "tests/ui/runtime_lane_frame_cost_certification/pass/lane_frame_cost_certification_facade_types.rs",
    "tests/ui/runtime_measurement/pass/measurement_facade_types.rs",
    "tests/ui/runtime_reload_counter_boundary/pass/reload_counter_boundary_facade_types.rs",
    "tests/ui/runtime_reload_storm_certification/pass/reload_storm_certification_facade_types.rs",
    "tests/ui/runtime_steady_frame_counter_boundary/pass/steady_frame_counter_boundary_facade_types.rs",
];

const RUNTIME_SUPPORT_FAIL_CASES: &[&str] = &[
    "tests/ui/runtime_diagnostics/fail/diagnostic_report_fields_not_public.rs",
    "tests/ui/runtime_diagnostics/fail/projection_hook_cannot_mint_runtime_truth.rs",
    "tests/ui/runtime_file_rust_replacement_parity/fail/parity_receipt_fields_not_public.rs",
    "tests/ui/runtime_file_rust_replacement_parity/fail/rust_cannot_construct_replacement_candidate.rs",
    "tests/ui/runtime_measurement/fail/measurement_counter_fields_not_public.rs",
    "tests/ui/runtime_measurement/fail/uncertified_counter_packet_cannot_lower_to_foundational.rs",
    "tests/ui/runtime_reload_counter_boundary/fail/reload_counter_receipt_fields_not_public.rs",
    "tests/ui/runtime_reload_counter_boundary/fail/raw_reload_counter_receipt_cannot_lower_to_foundational.rs",
    "tests/ui/runtime_steady_frame_counter_boundary/fail/steady_frame_receipt_fields_not_public.rs",
    "tests/ui/runtime_steady_frame_counter_boundary/fail/raw_steady_frame_receipt_cannot_lower_to_foundational.rs",
];

#[test]
fn runtime_support_public_types_compile() {
    trybuild_helpers::run_pass_cases(RUNTIME_SUPPORT_PASS_CASES);
}

#[test]
fn runtime_support_boundaries_stay_sealed() {
    trybuild_helpers::run_compile_fail_cases(RUNTIME_SUPPORT_FAIL_CASES);
}
