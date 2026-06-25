mod common;

mod public_facade_compile_fail_contracts {
    include!("../src/certification/public_facade_contracts/compile_fail_contracts.rs");
}

#[test]
fn worth_topo_compile_fail_boundaries_hold() {
    common::configure_shared_trybuild_workspace();

    let test_cases = trybuild::TestCases::new();
    for target in public_facade_compile_fail_contracts::public_facade_compile_fail_targets() {
        test_cases.compile_fail(target);
    }
    for target in touched_graph_basis_compile_fail_targets() {
        test_cases.compile_fail(target);
    }
    for target in topology::facade::topology_query_runtime_phase_eight_compile_fail_targets() {
        test_cases.compile_fail(target.path());
    }
    for target in topology::facade::topology_query_runtime_phase_nine_compile_fail_targets() {
        test_cases.compile_fail(target.path());
    }
    for target in topology::facade::validation_authority_inventory_compile_fail_targets() {
        test_cases.compile_fail(target.path());
    }
    for target in topology::facade::worth_topology_legality_catalog_compile_fail_targets() {
        test_cases.compile_fail(target.path());
    }
}

#[test]
fn worth_topo_compile_fail_manifest_counts_hold() {
    assert_eq!(touched_graph_basis_compile_fail_targets().len(), 11);
    assert_eq!(
        topology::facade::topology_query_runtime_phase_eight_compile_fail_targets().len(),
        topology::facade::TOPOLOGY_QUERY_RUNTIME_PHASE_EIGHT_COMPILE_FAIL_TARGET_COUNT
    );
    assert_eq!(
        topology::facade::topology_query_runtime_phase_nine_compile_fail_targets().len(),
        topology::facade::TOPOLOGY_QUERY_RUNTIME_PHASE_NINE_COMPILE_FAIL_TARGET_COUNT
    );
    assert_eq!(
        topology::facade::validation_authority_inventory_compile_fail_targets().len(),
        topology::facade::VALIDATION_AUTHORITY_INVENTORY_COMPILE_FAIL_TARGET_COUNT
    );
    assert_eq!(
        topology::facade::worth_topology_legality_catalog_compile_fail_targets().len(),
        topology::facade::WORTH_TOPOLOGY_LEGALITY_CATALOG_COMPILE_FAIL_TARGET_COUNT
    );
}

fn touched_graph_basis_compile_fail_targets() -> Vec<String> {
    const COMPILE_FAIL_ROOT: &str = "tests/ui/touched_graph_basis";
    [
        "basis_struct_literal.rs",
        "entity_from_raw_id.rs",
        "geometry_evidence_from_copied_receipt_identity.rs",
        "hidden_spatial_admission_module_not_public.rs",
        "geometry_evidence_from_raw_digest.rs",
        "mutation_record_is_not_basis.rs",
        "operating_world_from_raw_string.rs",
        "query_descriptor_is_not_basis.rs",
        "raw_declaration_cannot_mint_basis.rs",
        "schema_admission_public_facade_cannot_mint_basis.rs",
        "serde_deserialization_is_not_authority.rs",
    ]
    .into_iter()
    .map(|file| format!("{COMPILE_FAIL_ROOT}/{file}"))
    .collect()
}
