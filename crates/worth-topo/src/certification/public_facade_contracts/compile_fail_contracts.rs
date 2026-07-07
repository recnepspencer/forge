#[path = "phase_fifteen_fixture_inventory.rs"]
mod phase_fifteen_fixture_inventory;
#[path = "phase_fourteen_fixture_inventory.rs"]
mod phase_fourteen_fixture_inventory;

use std::collections::BTreeSet;

pub(crate) fn public_facade_compile_fail_targets() -> Vec<String> {
    const COMPILE_FAIL_ROOT: &str = "src/certification/public_facade_contracts/compile_fail";
    public_facade_compile_fail_target_files()
        .iter()
        .map(|file| format!("{COMPILE_FAIL_ROOT}/{file}"))
        .collect()
}

pub(crate) const fn public_facade_compile_fail_target_files() -> &'static [&'static str] {
    &[
        "private_stage_helper.rs",
        "mint_materialized_topology_view.rs",
        "mint_boundary_envelope.rs",
        "mint_boundary_failure.rs",
        "public_runtime_adapters_not_forgeable.rs",
        "public_topology_mutation_application_runner_not_exported.rs",
        "public_milestone_one_read_view_cert_not_exported.rs",
        "public_milestone_two_read_view_cert_not_exported.rs",
        "public_runtime_constructor_requires_adapters.rs",
        "public_query_row_materializer_not_exported.rs",
        "public_query_row_helpers_not_exported.rs",
        "public_derived_diagnostics_builders_not_exported.rs",
        "public_projection_surface_entry_not_exported.rs",
        "public_topology_construction_authority_not_exported.rs",
        "public_topology_construction_stepwise_lowering_not_exported.rs",
        "public_topology_construction_stepwise_execution_not_exported.rs",
        "public_topology_construction_stepwise_certification_not_exported.rs",
        "public_topology_construction_fact_report_not_exported.rs",
        "public_topology_construction_boundary_not_exported.rs",
        "public_topology_construction_boundary_preparation_not_exported.rs",
        "public_topology_construction_birth_compose_execute_not_exported.rs",
        "topology_query_raw_handoff_not_admitted.rs",
        "topology_query_admitted_handoff_not_executed_authority.rs",
        "topology_query_receipt_not_executed_authority.rs",
        "topology_operator_adoption_not_executed_authority.rs",
        "public_topology_birth_graph_authority_proof_not_forgeable.rs",
        "public_runtime_read_family_support_row_not_forgeable.rs",
        "public_runtime_mutation_family_support_row_not_forgeable.rs",
        "public_runtime_mutation_lane_support_row_not_forgeable.rs",
        "public_runtime_posture_row_not_forgeable.rs",
        "public_runtime_removed_mutation_lane_string_support_not_exported.rs",
        "public_runtime_removed_current_head_live_reads_supported_not_exported.rs",
        "public_runtime_removed_historical_basis_supported_not_exported.rs",
        "public_runtime_closeout_row_not_forgeable.rs",
        "public_topology_read_request_report_not_forgeable.rs",
        "public_topology_read_closeout_report_not_forgeable.rs",
        "public_topology_read_closeout_row_not_forgeable.rs",
        "public_no_n_plus_one_contract_row_not_forgeable.rs",
        "public_milestone_three_topology_mutation_digest_row_not_forgeable.rs",
        "public_topology_query_domain_entry_constructors_not_exported.rs",
        "public_topology_query_domain_entry_not_exported_from_facade.rs",
        "public_topology_query_domain_entry_not_exported_from_projection.rs",
        "public_bridge_registration_entry_not_exported.rs",
        "public_runtime_builder_not_exported_from_facade.rs",
        "public_topology_read_workspace_methods_not_exported.rs",
        "public_topology_reads_not_exported_from_facade.rs",
        "public_removed_domain_query_read_names_not_exported.rs",
        "public_topology_committed_artifact_not_exported.rs",
        "public_nmt_topology_construction_receipt_not_forgeable.rs",
        "public_nmt_topology_scope_receipt_not_forgeable.rs",
        "public_verified_topology_commit_cert_not_exported.rs",
        "public_topology_declaration_entry_stop_class_not_exported.rs",
        "public_topology_declaration_entry_refusal_class_not_exported.rs",
        "public_edge_split_operator_row_not_forgeable.rs",
        "public_edge_split_validator_row_not_forgeable.rs",
        "public_loop_operator_row_not_forgeable.rs",
        "public_loop_validator_row_not_forgeable.rs",
        "public_loop_blueprint_registry_not_forgeable.rs",
        "public_loop_blueprint_matrix_plan_mutators_not_exported.rs",
        "public_topology_local_ceremony_guard_surfaces_not_exported.rs",
        "public_derived_invalidation_inventory_row_not_forgeable.rs",
        "public_derived_invalidation_inventory_report_not_forgeable.rs",
        "public_derived_invalidation_inventory_closeout_not_forgeable.rs",
        "public_derived_invalidation_family_record_not_forgeable.rs",
        "public_derived_invalidation_consumed_graph_facts_not_forgeable.rs",
        "public_derived_invalidation_family_identity_not_consumption_authority.rs",
        "public_derived_invalidation_family_catalog_not_forgeable.rs",
        "public_derived_invalidation_family_closeout_not_forgeable.rs",
        "public_derived_invalidation_phase_three_seed_not_forgeable.rs",
        "public_derived_invalidation_query_support_evidence_not_forgeable.rs",
        "public_derived_invalidation_legality_support_evidence_not_forgeable.rs",
        "public_derived_invalidation_selected_plan_not_forgeable.rs",
        "public_derived_invalidation_selected_row_not_forgeable.rs",
        "public_derived_invalidation_denial_row_not_forgeable.rs",
        "public_derived_invalidation_phase_four_seed_not_forgeable.rs",
        "public_derived_invalidation_execution_receipt_not_forgeable.rs",
        "public_derived_invalidation_operator_cutover_receipt_not_forgeable.rs",
        "public_derived_invalidation_projection_read_stage_receipt_not_forgeable.rs",
        "public_derived_invalidation_operator_cutover_closeout_not_forgeable.rs",
        "public_derived_invalidation_phase_eight_seed_not_forgeable.rs",
        "public_derived_invalidation_deletion_row_not_forgeable.rs",
        "public_derived_invalidation_residue_audit_row_not_forgeable.rs",
        "public_derived_invalidation_deletion_closeout_not_forgeable.rs",
        "public_derived_invalidation_phase_nine_seed_not_forgeable.rs",
        "public_derived_invalidation_milestone_ten_closeout_not_forgeable.rs",
        "public_derived_invalidation_milestone_eleven_seed_not_forgeable.rs",
        "public_loop_cycle_topology_read_source_not_exported.rs",
        "public_materialized_graph_topology_prefix_read_source_not_exported.rs",
        "public_materialized_graph_read_stage_receipt_not_forgeable.rs",
        "public_traversal_views_topology_read_source_not_exported.rs",
        "public_traversal_views_topology_prefix_read_source_not_exported.rs",
        "public_traversal_views_read_stage_receipt_not_forgeable.rs",
        "public_topology_compiled_product_family_declaration_constructor_not_exported.rs",
        "public_topology_compiled_product_admission_not_exported.rs",
        "public_topology_compiled_product_reuse_decision_not_exported.rs",
        "public_topology_compiled_product_rebuild_denial_not_exported.rs",
        "public_derived_read_diagnostic_support_not_exported.rs",
        "public_topology_selected_route_authority_not_exported.rs",
        "public_topology_selected_route_admission_not_exported.rs",
        "public_invalidation_route_input_not_mintable_from_milestone_ten_summary_row.rs",
        "public_invalidation_route_input_not_mintable_from_projection_read_stage_receipt.rs",
        "public_topology_compiled_product_family_proof_products_not_deserializable.rs",
    ]
}

#[test]
fn public_api_cannot_forge_compiled_product_or_reuse_products() {
    const COMPILE_FAIL_ROOT: &str = "src/certification/public_facade_contracts/compile_fail";
    let test_cases = trybuild::TestCases::new();
    for fence in phase_fifteen_fixture_inventory::phase_fifteen_topology_compile_fail_fences() {
        let file_name = fence
            .fixture_path()
            .strip_prefix("src/certification/public_facade_contracts/compile_fail/")
            .expect("phase 15 topology fixture must stay under compile_fail root");
        test_cases.compile_fail(format!("{COMPILE_FAIL_ROOT}/{file_name}"));
    }
}

#[test]
fn public_api_cannot_mint_invalidation_route_input_from_projection_or_summary_rows() {
    const COMPILE_FAIL_ROOT: &str = "src/certification/public_facade_contracts/compile_fail";
    let test_cases = trybuild::TestCases::new();
    for file_name in [
        "public_invalidation_route_input_not_mintable_from_milestone_ten_summary_row.rs",
        "public_invalidation_route_input_not_mintable_from_projection_read_stage_receipt.rs",
    ] {
        test_cases.compile_fail(format!("{COMPILE_FAIL_ROOT}/{file_name}"));
    }
}

#[test]
fn phase_fourteen_topology_reintroduction_and_raw_part_fixtures_hold() {
    let test_cases = trybuild::TestCases::new();
    for fence in phase_fourteen_fixture_inventory::phase_fourteen_topology_compile_fail_fences() {
        test_cases.compile_fail(fence.fixture_path());
    }
}

#[test]
fn phase_fifteen_topology_compile_fail_fixtures_are_unique_per_fence_class() {
    let unique_fixtures: BTreeSet<_> =
        phase_fifteen_fixture_inventory::phase_fifteen_topology_compile_fail_fences()
            .iter()
            .map(|fence| fence.fixture_path())
            .collect();
    assert_eq!(
        unique_fixtures.len(),
        phase_fifteen_fixture_inventory::phase_fifteen_topology_compile_fail_fences().len(),
        "each certified topology fence class must map to its own executed compile-fail fixture",
    );
}

#[test]
fn phase_fourteen_topology_compile_fail_fixtures_are_unique_per_path() {
    let unique_fixtures: BTreeSet<_> =
        phase_fourteen_fixture_inventory::phase_fourteen_topology_compile_fail_fences()
            .iter()
            .map(|fence| fence.fixture_path())
            .collect();
    assert_eq!(
        unique_fixtures.len(),
        phase_fourteen_fixture_inventory::phase_fourteen_topology_compile_fail_fences().len(),
        "each phase 14 topology fence fixture should be executed exactly once",
    );
}

#[test]
fn compile_fail_target_helper_inventory_stays_in_sync_with_fixture_root() {
    let root = "src/certification/public_facade_contracts/compile_fail/";

    assert_eq!(
        public_facade_compile_fail_targets().len(),
        public_facade_compile_fail_target_files().len(),
        "compile-fail target helpers should describe the same number of fixtures",
    );
    assert!(public_facade_compile_fail_targets()
        .iter()
        .all(|target| target.starts_with(root) && target.ends_with(".rs")));
}

#[test]
fn phase_fourteen_topology_fences_publish_expected_diagnostic_paths_and_classes() {
    let fences = phase_fourteen_fixture_inventory::phase_fourteen_topology_compile_fail_fences();

    assert!(fences.iter().all(|fence| fence.stderr_path().ends_with(".stderr")));
    assert!(fences.iter().all(|fence| !fence.fence_class().is_empty()));
}

#[test]
fn phase_fifteen_topology_fences_publish_expected_diagnostic_paths_and_classes() {
    let fences = phase_fifteen_fixture_inventory::phase_fifteen_topology_compile_fail_fences();

    assert!(fences.iter().all(|fence| fence.stderr_path().ends_with(".stderr")));
    assert!(fences.iter().all(|fence| !fence.fence_class().is_empty()));
}
