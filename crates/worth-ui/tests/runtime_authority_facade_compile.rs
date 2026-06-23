#[path = "support/trybuild_helpers.rs"]
mod trybuild_helpers;

const RUNTIME_AUTHORITY_PASS_CASES: &[&str] =
    &["tests/ui/runtime_authority/pass/runtime_facade_types.rs"];

const RUNTIME_AUTHORITY_FAIL_CASES: &[&str] = &[
    "tests/ui/runtime_authority/fail/runtime_internal_module_is_not_public.rs",
    "tests/ui/runtime_authority/fail/runtime_launch_fields_not_public.rs",
    "tests/ui/runtime_authority/fail/runtime_fact_id_raw_constructor_not_public.rs",
    "tests/ui/runtime_authority/fail/runtime_host_authority_not_cloneable.rs",
    "tests/ui/runtime_authority/fail/replacement_candidate_bundle_fields_not_public.rs",
    "tests/ui/runtime_authority/fail/unadmitted_candidate_cannot_enter_admitted_boundary.rs",
    "tests/ui/runtime_authority/fail/active_runtime_state_not_constructible_from_app_local_parts.rs",
    "tests/ui/runtime_authority/fail/forged_last_valid_receipt_not_installable.rs",
    "tests/ui/runtime_authority/fail/pending_execution_plan_lowering_input_fields_not_public.rs",
    "tests/ui/runtime_authority/fail/raw_source_cannot_enter_plan_lowering.rs",
    "tests/ui/runtime_authority/fail/execution_plan_fields_not_public.rs",
    "tests/ui/runtime_authority/fail/plan_lookup_index_fields_not_public.rs",
    "tests/ui/runtime_authority/fail/frame_path_cannot_scan_artifact_tree_from_execution_plan.rs",
    "tests/ui/runtime_authority/fail/pointer_identity_cannot_replace_plan_equivalence.rs",
    "tests/ui/runtime_authority/fail/node_replacement_plan_fields_not_public.rs",
    "tests/ui/runtime_authority/fail/runtime_handle_fields_not_public.rs",
    "tests/ui/runtime_authority/fail/stale_runtime_handle_cannot_replace_fresh_plan_receipt.rs",
    "tests/ui/runtime_authority/fail/frame_boundary_fields_not_public.rs",
    "tests/ui/runtime_authority/fail/execution_plan_inspection_fields_not_public.rs",
    "tests/ui/runtime_authority/fail/extension_hook_cannot_override_active_plan_truth.rs",
    "tests/ui/runtime_authority/fail/component_interaction_receipt_fields_not_public.rs",
    "tests/ui/runtime_authority/fail/interaction_receipt_fields_not_public.rs",
    "tests/ui/runtime_authority/fail/interaction_activation_request_fields_not_public.rs",
    "tests/ui/runtime_authority/fail/interaction_payload_fields_not_public.rs",
    "tests/ui/runtime_authority/fail/interaction_field_fields_not_public.rs",
    "tests/ui/runtime_authority/fail/primitive_proof_receipt_fields_not_public.rs",
    "tests/ui/runtime_authority/fail/primitive_nested_authority_types_not_mintable.rs",
    "tests/ui/runtime_authority/fail/event_region_receipt_fields_not_public.rs",
    "tests/ui/runtime_authority/fail/event_dispatch_receipt_fields_not_public.rs",
    "tests/ui/runtime_authority/fail/pointer_frame_receipt_fields_not_public.rs",
];

#[test]
fn runtime_authority_public_types_compile() {
    trybuild_helpers::run_pass_cases(RUNTIME_AUTHORITY_PASS_CASES);
}

#[test]
fn runtime_authority_boundaries_stay_sealed() {
    trybuild_helpers::run_compile_fail_cases(RUNTIME_AUTHORITY_FAIL_CASES);
}
