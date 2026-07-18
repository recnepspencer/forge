use std::path::Path;

use worth_store_test_support::compiler_boundary::{
    cargo_dependency_manifest, run_cargo_ui_fixture_suite,
};

#[test]
fn simulation_harness_entry_boundary_rejects_lower_authority_callers_at_compile_time() {
    let root = store_workspace_root();
    let evidence = run_cargo_ui_fixture_suite(
        root,
        "simulation-harness-entry-boundary",
        dependency_manifest(root),
        "production",
        "diagnostic-test",
        &root.join("crates/worth-store-certification/tests/compile_fail/recovery/entry_boundary"),
        FIXTURES,
    )
    .unwrap();
    assert_eq!(evidence.fixtures.len(), FIXTURES.len());
}

const FIXTURES: &[(&str, &[&str])] = &[
    (
        "copied_report_cannot_enter.rs",
        &["BoundedRecoveryReceipt", "&str"],
    ),
    (
        "raw_inventory_surface_cannot_be_minted.rs",
        &["ExistingSimulationHarnessSurface", "private"],
    ),
    (
        "entry_identity_cannot_be_minted.rs",
        &["SimulationHarnessEntryIdentity", "private"],
    ),
    (
        "entry_struct_literal_cannot_be_minted.rs",
        &["SimulationHarnessEntry", "private"],
    ),
];

fn dependency_manifest(root: &Path) -> String {
    cargo_dependency_manifest(
        &[(
            "worth-store-physical-certification",
            root.join("crates/worth-store-physical-certification")
                .as_path(),
            &[],
        )],
        &[],
    )
}

fn store_workspace_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("certification crate lives under the Store workspace")
}
