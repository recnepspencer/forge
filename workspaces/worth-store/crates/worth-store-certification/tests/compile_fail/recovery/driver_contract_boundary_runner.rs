use std::path::Path;

use worth_store_test_support::compiler_boundary::{
    cargo_dependency_manifest, run_cargo_ui_fixture_suite,
};

#[test]
fn driver_contract_boundary_rejects_receipt_minting_at_compile_time() {
    run_fixture(
        "yieldpoint_pause_receipt_cannot_be_minted.rs",
        &["YieldpointPauseReceipt", "private"],
    );
}

#[test]
fn driver_contract_boundary_rejects_arbitrary_named_yieldpoint_authority() {
    run_fixture(
        "named_yieldpoint_authority_cannot_be_minted.rs",
        &["named_production_boundary"],
    );
}

#[test]
fn driver_contract_boundary_rejects_yieldpoint_struct_literal_minting() {
    run_fixture(
        "physical_boundary_yieldpoint_fields_cannot_be_minted.rs",
        &["private"],
    );
}

#[test]
fn driver_contract_boundary_rejects_declaration_struct_literal_minting() {
    run_fixture("yieldpoint_declaration_cannot_be_minted.rs", &["private"]);
}

#[test]
fn driver_contract_boundary_rejects_schedule_binding_struct_literal_minting() {
    run_fixture(
        "yieldpoint_schedule_binding_cannot_be_minted.rs",
        &["private"],
    );
}

#[test]
fn driver_contract_boundary_rejects_driver_struct_literal_minting() {
    run_fixture(
        "physical_simulation_driver_cannot_be_minted.rs",
        &["private"],
    );
}

fn run_fixture(name: &str, expected: &[&str]) {
    let root = store_workspace_root();
    let evidence = run_cargo_ui_fixture_suite(
        root,
        "physical-driver-contract-boundary",
        cargo_dependency_manifest(
            &[(
                "worth-store-physical-certification",
                root.join("crates/worth-store-physical-certification")
                    .as_path(),
                &[],
            )],
            &[],
        ),
        "production",
        "diagnostic-test",
        &root.join(
            "crates/worth-store-certification/tests/compile_fail/recovery/driver_contract_boundary",
        ),
        &[(name, expected)],
    )
    .unwrap();
    assert_eq!(evidence.fixtures.len(), 1);
}

fn store_workspace_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("certification crate lives under the Store workspace")
}
