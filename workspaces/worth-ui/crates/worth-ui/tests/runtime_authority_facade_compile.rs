fn runtime_authority_compile_pass(path: &str) {
    trybuild::TestCases::new().pass(path);
}

fn runtime_authority_compile_fail(path: &str) {
    trybuild::TestCases::new().compile_fail(path);
}

#[test]
fn runtime_authority_facade_types_are_importable() {
    runtime_authority_compile_pass("tests/ui/runtime_authority/pass/runtime_facade_types.rs");
}

#[test]
fn runtime_internal_module_is_not_public() {
    runtime_authority_compile_fail(
        "tests/ui/runtime_authority/fail/runtime_internal_module_is_not_public.rs",
    );
}

#[test]
fn runtime_launch_fields_are_not_publicly_mintable() {
    runtime_authority_compile_fail(
        "tests/ui/runtime_authority/fail/runtime_launch_fields_not_public.rs",
    );
}

#[test]
fn runtime_host_authority_is_not_publicly_cloneable() {
    runtime_authority_compile_fail(
        "tests/ui/runtime_authority/fail/runtime_host_authority_not_cloneable.rs",
    );
}

#[test]
fn replacement_candidate_packets_are_not_publicly_cloneable() {
    runtime_authority_compile_fail(
        "tests/ui/runtime_authority/fail/replacement_candidate_packets_not_cloneable.rs",
    );
}

#[test]
fn replacement_candidate_bundle_fields_are_not_publicly_mintable() {
    runtime_authority_compile_fail(
        "tests/ui/runtime_authority/fail/replacement_candidate_bundle_fields_not_public.rs",
    );
}

#[test]
fn unadmitted_candidate_cannot_enter_admitted_boundary() {
    runtime_authority_compile_fail(
        "tests/ui/runtime_authority/fail/unadmitted_candidate_cannot_enter_admitted_boundary.rs",
    );
}

#[test]
fn unadmitted_candidate_cannot_enter_runtime_comparison() {
    runtime_authority_compile_fail(
        "tests/ui/runtime_authority/fail/unadmitted_candidate_cannot_enter_runtime_comparison.rs",
    );
}

#[test]
fn unadmitted_candidate_cannot_enter_impact_classification() {
    runtime_authority_compile_fail(
        "tests/ui/runtime_authority/fail/unadmitted_candidate_cannot_enter_impact_classification.rs",
    );
}

#[test]
fn unadmitted_candidate_cannot_enter_impact_narrowing() {
    runtime_authority_compile_fail(
        "tests/ui/runtime_authority/fail/unadmitted_candidate_cannot_enter_impact_narrowing.rs",
    );
}

#[test]
fn runtime_impact_narrowing_fields_are_not_publicly_mintable() {
    runtime_authority_compile_fail(
        "tests/ui/runtime_authority/fail/runtime_impact_narrowing_fields_not_public.rs",
    );
}

#[test]
fn impact_lookup_counters_are_not_publicly_mintable() {
    runtime_authority_compile_fail(
        "tests/ui/runtime_authority/fail/impact_lookup_counters_not_publicly_mintable.rs",
    );
}

#[test]
fn admitted_candidate_fields_are_not_publicly_mintable() {
    runtime_authority_compile_fail(
        "tests/ui/runtime_authority/fail/admitted_candidate_fields_not_public.rs",
    );
}

#[test]
fn active_runtime_state_is_not_constructible_from_app_local_parts() {
    runtime_authority_compile_fail(
        "tests/ui/runtime_authority/fail/active_runtime_state_not_constructible_from_app_local_parts.rs",
    );
}

#[test]
fn forged_last_valid_receipt_is_not_installable_as_active_truth() {
    runtime_authority_compile_fail(
        "tests/ui/runtime_authority/fail/forged_last_valid_receipt_not_installable.rs",
    );
}

#[test]
fn pending_activation_is_not_publicly_mintable() {
    runtime_authority_compile_fail(
        "tests/ui/runtime_authority/fail/pending_activation_not_publicly_mintable.rs",
    );
}

#[test]
fn staged_replacement_fields_are_not_publicly_mintable() {
    runtime_authority_compile_fail(
        "tests/ui/runtime_authority/fail/staged_replacement_fields_not_public.rs",
    );
}

#[test]
fn activation_readiness_is_not_publicly_mintable() {
    runtime_authority_compile_fail(
        "tests/ui/runtime_authority/fail/activation_readiness_not_publicly_mintable.rs",
    );
}

#[test]
fn activation_staging_report_fields_are_not_publicly_mintable() {
    runtime_authority_compile_fail(
        "tests/ui/runtime_authority/fail/activation_staging_report_fields_not_public.rs",
    );
}

#[test]
fn activation_staging_counters_are_not_publicly_mintable() {
    runtime_authority_compile_fail(
        "tests/ui/runtime_authority/fail/activation_staging_counters_not_publicly_mintable.rs",
    );
}

#[test]
fn frame_boundary_fields_are_not_publicly_mintable() {
    runtime_authority_compile_fail(
        "tests/ui/runtime_authority/fail/frame_boundary_fields_not_public.rs",
    );
}

#[test]
fn ready_activation_fields_are_not_publicly_mintable() {
    runtime_authority_compile_fail(
        "tests/ui/runtime_authority/fail/ready_activation_fields_not_public.rs",
    );
}

#[test]
fn activation_gate_receipt_fields_are_not_publicly_mintable() {
    runtime_authority_compile_fail(
        "tests/ui/runtime_authority/fail/activation_gate_receipt_fields_not_public.rs",
    );
}

#[test]
fn pending_execution_plan_lowering_input_fields_are_not_publicly_mintable() {
    runtime_authority_compile_fail(
        "tests/ui/runtime_authority/fail/pending_execution_plan_lowering_input_fields_not_public.rs",
    );
}

#[test]
fn raw_source_cannot_enter_plan_lowering() {
    runtime_authority_compile_fail(
        "tests/ui/runtime_authority/fail/raw_source_cannot_enter_plan_lowering.rs",
    );
}

#[test]
fn admitted_candidate_cannot_bypass_activation_readiness() {
    runtime_authority_compile_fail(
        "tests/ui/runtime_authority/fail/admitted_candidate_cannot_bypass_activation_readiness.rs",
    );
}

#[test]
fn execution_plan_input_fields_are_not_publicly_mintable() {
    runtime_authority_compile_fail(
        "tests/ui/runtime_authority/fail/execution_plan_input_fields_not_public.rs",
    );
}

#[test]
fn plan_node_input_fields_are_not_publicly_mintable() {
    runtime_authority_compile_fail(
        "tests/ui/runtime_authority/fail/plan_node_input_fields_not_public.rs",
    );
}

#[test]
fn plan_lowering_counters_are_not_publicly_mintable() {
    runtime_authority_compile_fail(
        "tests/ui/runtime_authority/fail/plan_lowering_counters_not_publicly_mintable.rs",
    );
}

#[test]
fn unresolved_capability_string_cannot_enter_plan_node_input() {
    runtime_authority_compile_fail(
        "tests/ui/runtime_authority/fail/unresolved_capability_string_cannot_enter_plan_node_input.rs",
    );
}

#[test]
fn unregistered_component_hook_family_is_not_public_plan_input() {
    runtime_authority_compile_fail(
        "tests/ui/runtime_authority/fail/unregistered_component_hook_family_not_public_plan_input.rs",
    );
}

#[test]
fn runtime_handle_fields_are_not_publicly_mintable() {
    runtime_authority_compile_fail(
        "tests/ui/runtime_authority/fail/runtime_handle_fields_not_public.rs",
    );
}

#[test]
fn runtime_handle_allocation_counters_are_not_publicly_mintable() {
    runtime_authority_compile_fail(
        "tests/ui/runtime_authority/fail/runtime_handle_allocation_counters_not_publicly_mintable.rs",
    );
}

#[test]
fn frame_path_cannot_resolve_component_or_command_by_string() {
    runtime_authority_compile_fail(
        "tests/ui/runtime_authority/fail/frame_path_cannot_resolve_component_or_command_by_string.rs",
    );
}

#[test]
fn stale_runtime_handle_cannot_replace_fresh_plan_receipt() {
    runtime_authority_compile_fail(
        "tests/ui/runtime_authority/fail/stale_runtime_handle_cannot_replace_fresh_plan_receipt.rs",
    );
}

#[test]
fn execution_plan_fields_are_not_publicly_mintable() {
    runtime_authority_compile_fail(
        "tests/ui/runtime_authority/fail/execution_plan_fields_not_public.rs",
    );
}

#[test]
fn plan_lookup_index_fields_are_not_publicly_mintable() {
    runtime_authority_compile_fail(
        "tests/ui/runtime_authority/fail/plan_lookup_index_fields_not_public.rs",
    );
}

#[test]
fn plan_topology_counters_are_not_publicly_mintable() {
    runtime_authority_compile_fail(
        "tests/ui/runtime_authority/fail/plan_topology_counters_not_publicly_mintable.rs",
    );
}

#[test]
fn frame_path_cannot_scan_artifact_tree_from_execution_plan() {
    runtime_authority_compile_fail(
        "tests/ui/runtime_authority/fail/frame_path_cannot_scan_artifact_tree_from_execution_plan.rs",
    );
}

#[test]
fn execution_plan_digest_fields_are_not_publicly_mintable() {
    runtime_authority_compile_fail(
        "tests/ui/runtime_authority/fail/execution_plan_digest_fields_not_public.rs",
    );
}

#[test]
fn execution_plan_equivalence_fields_are_not_publicly_mintable() {
    runtime_authority_compile_fail(
        "tests/ui/runtime_authority/fail/execution_plan_equivalence_fields_not_public.rs",
    );
}

#[test]
fn plan_equivalence_counters_are_not_publicly_mintable() {
    runtime_authority_compile_fail(
        "tests/ui/runtime_authority/fail/plan_equivalence_counters_not_publicly_mintable.rs",
    );
}

#[test]
fn pointer_identity_cannot_replace_plan_equivalence() {
    runtime_authority_compile_fail(
        "tests/ui/runtime_authority/fail/pointer_identity_cannot_replace_plan_equivalence.rs",
    );
}

#[test]
fn unadmitted_candidate_cannot_enter_identity_matching() {
    runtime_authority_compile_fail(
        "tests/ui/runtime_authority/fail/unadmitted_candidate_cannot_enter_identity_matching.rs",
    );
}

#[test]
fn identity_match_graph_fields_are_not_publicly_mintable() {
    runtime_authority_compile_fail(
        "tests/ui/runtime_authority/fail/identity_match_graph_fields_not_public.rs",
    );
}

#[test]
fn identity_match_counters_are_not_publicly_mintable() {
    runtime_authority_compile_fail(
        "tests/ui/runtime_authority/fail/identity_match_counters_not_publicly_mintable.rs",
    );
}

#[test]
fn identity_seed_contribution_is_not_publicly_mintable() {
    runtime_authority_compile_fail(
        "tests/ui/runtime_authority/fail/identity_seed_contribution_not_publicly_mintable.rs",
    );
}

#[test]
fn node_replacement_plan_fields_are_not_publicly_mintable() {
    runtime_authority_compile_fail(
        "tests/ui/runtime_authority/fail/node_replacement_plan_fields_not_public.rs",
    );
}

#[test]
fn node_replacement_counters_are_not_publicly_mintable() {
    runtime_authority_compile_fail(
        "tests/ui/runtime_authority/fail/node_replacement_counters_not_publicly_mintable.rs",
    );
}
