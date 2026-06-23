#[path = "support/trybuild_helpers.rs"]
mod trybuild_helpers;

const RUNTIME_RELOAD_PASS_CASES: &[&str] = &[
    "tests/ui/runtime_authority/pass/projection_contract_facade_types.rs",
    "tests/ui/runtime_authority/pass/query_runtime_reload_facade_types.rs",
    "tests/ui/runtime_authority/pass/hot_reload_enforcement_facade_usage.rs",
];

const RUNTIME_RELOAD_FAIL_CASES: &[&str] = &[
    // Sealed reload evidence and proof carriers.
    "tests/ui/runtime_authority/fail/changed_runtime_facts_fields_not_public.rs",
    "tests/ui/runtime_authority/fail/capability_changed_facts_fields_not_public.rs",
    "tests/ui/runtime_authority/fail/validation_reload_evidence_fields_not_public.rs",
    "tests/ui/runtime_authority/fail/capability_reload_evidence_fields_not_public.rs",
    "tests/ui/runtime_authority/fail/admitted_capability_reload_batch_fields_not_public.rs",
    "tests/ui/runtime_authority/fail/runtime_change_evidence_fields_not_public.rs",
    "tests/ui/runtime_authority/fail/authored_delta_summary_fields_not_public.rs",
    "tests/ui/runtime_authority/fail/touched_authored_semantic_slice_row_fields_not_public.rs",
    "tests/ui/runtime_authority/fail/raw_fact_set_cannot_replace_changed_fact_proof.rs",
    "tests/ui/runtime_authority/fail/classified_change_cannot_replace_admitted_evidence.rs",
    "tests/ui/runtime_authority/fail/authored_structural_runtime_fact_lowering_not_public.rs",
    // Query reload proof ingress.
    "tests/ui/runtime_authority/fail/query_runtime_fact_lowering_input_fields_not_public.rs",
    "tests/ui/runtime_authority/fail/raw_view_binding_id_cannot_replace_query_runtime_fact_lowering.rs",
    "tests/ui/runtime_authority/fail/query_live_rebind_plan_fields_not_public.rs",
    // Runtime change admission boundaries.
    "tests/ui/runtime_authority/fail/raw_capability_request_cannot_replace_reload_evidence.rs",
    "tests/ui/runtime_authority/fail/raw_capability_evidence_cannot_enter_rebind.rs",
    "tests/ui/runtime_authority/fail/raw_validation_evidence_cannot_enter_rebind.rs",
    "tests/ui/runtime_authority/fail/raw_validation_reload_request_cannot_replace_candidate_submission.rs",
    "tests/ui/runtime_authority/fail/appearance_value_map_cannot_replace_reload_evidence.rs",
    "tests/ui/runtime_authority/fail/raw_appearance_package_cannot_replace_reload_evidence.rs",
    "tests/ui/runtime_authority/fail/density_value_map_cannot_enter_rebind_coordinator.rs",
    "tests/ui/runtime_authority/fail/raw_density_package_cannot_enter_rebind_coordinator.rs",
    "tests/ui/runtime_authority/fail/component_changed_fact_constructor_not_public.rs",
    "tests/ui/runtime_authority/fail/replacement_component_registry_fields_not_public.rs",
    // Projection contract and admitted plan boundaries.
    "tests/ui/runtime_authority/fail/projection_contract_fields_not_public.rs",
    "tests/ui/runtime_authority/fail/projection_declaration_cannot_replace_admitted_projection.rs",
    "tests/ui/runtime_authority/fail/app_code_cannot_implement_projection_plan_contract.rs",
    "tests/ui/runtime_authority/fail/projection_rebind_proofs_fields_not_public.rs",
    "tests/ui/runtime_authority/fail/semantic_hot_reload_admission_fields_not_public.rs",
    // Dropdown and page-host rebind ingress boundaries.
    "tests/ui/runtime_authority/fail/raw_selection_mode_cannot_enter_dropdown_projection_rebind.rs",
    "tests/ui/runtime_authority/fail/raw_command_projection_descriptor_cannot_enter_dropdown_projection_rebind.rs",
    "tests/ui/runtime_authority/fail/dropdown_appearance_frame_receipt_fields_not_public.rs",
    "tests/ui/runtime_authority/fail/dropdown_frame_receipt_fields_not_public.rs",
    "tests/ui/runtime_authority/fail/dropdown_selection_interaction_receipt_fields_not_public.rs",
    "tests/ui/runtime_authority/fail/dropdown_selection_state_reconciliation_receipt_fields_not_public.rs",
    "tests/ui/runtime_authority/fail/active_authoring_snapshot_witness_fields_not_public.rs",
    "tests/ui/runtime_authority/fail/header_appearance_frame_receipt_fields_not_public.rs",
    "tests/ui/runtime_authority/fail/header_frame_receipt_fields_not_public.rs",
    "tests/ui/runtime_authority/fail/candidate_snapshot_cannot_plan_page_host.rs",
    "tests/ui/runtime_authority/fail/page_host_frame_receipt_fields_not_public.rs",
    "tests/ui/runtime_authority/fail/raw_content_slot_catalog_cannot_rebind_page_host.rs",
    "tests/ui/runtime_authority/fail/raw_component_mount_map_cannot_rebind_page_host.rs",
    "tests/ui/runtime_authority/fail/app_code_cannot_swap_active_authoring_snapshot.rs",
    "tests/ui/runtime_authority/fail/page_host_slot_receipt_fields_not_public.rs",
    "tests/ui/runtime_authority/fail/header_rebind_receipt_fields_not_public.rs",
    "tests/ui/runtime_authority/fail/page_host_rebind_receipt_fields_not_public.rs",
    "tests/ui/runtime_authority/fail/component_reload_receipt_fields_not_public.rs",
    // Projection rebind completion lanes.
    "tests/ui/runtime_authority/fail/preserved_projection_rebind_cannot_complete_rebuild.rs",
    "tests/ui/runtime_authority/fail/activated_projection_rebind_cannot_complete_preserved.rs",
    // Reload-storm certification remains sealed.
    "tests/ui/runtime_authority/fail/reload_replay_certification_fields_not_public.rs",
];

#[test]
fn runtime_reload_public_types_compile() {
    trybuild_helpers::run_pass_cases(RUNTIME_RELOAD_PASS_CASES);
}

#[test]
fn runtime_reload_boundaries_stay_sealed() {
    trybuild_helpers::run_compile_fail_cases(RUNTIME_RELOAD_FAIL_CASES);
}
