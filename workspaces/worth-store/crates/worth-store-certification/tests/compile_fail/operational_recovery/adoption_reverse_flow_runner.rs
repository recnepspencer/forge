use std::path::Path;

use worth_store_test_support::compiler_boundary::{
    cargo_dependency_manifest, run_cargo_ui_fixture_suite,
};

#[test]
fn descriptive_shared_artifacts_cannot_reenter_store_authority() {
    let root = store_workspace_root();
    let evidence = run_cargo_ui_fixture_suite(
        root,
        "s10-adoption-reverse-flow",
        cargo_dependency_manifest(
            &[
                ("worth-store-authority", root.join("crates/worth-store-authority").as_path(), &[]),
                ("worth-store-offline-verifier", root.join("crates/worth-store-offline-verifier").as_path(), &[]),
                ("worth-store-operations", root.join("crates/worth-store-operations").as_path(), &[]),
                ("worth-store-replication", root.join("crates/worth-store-replication").as_path(), &[]),
                ("worth-store-layout-indexes", root.join("crates/worth-store-layout-indexes").as_path(), &[]),
            ],
            &[],
        ),
        "production",
        "diagnostic-test",
        &root.join("crates/worth-store-certification/tests/compile_fail/operational_recovery/cases/adoption_reverse_flow/src/bin"),
        FIXTURES,
    ).unwrap();
    assert_eq!(evidence.fixtures.len(), FIXTURES.len());
}

const FIXTURES: &[(&str, &[&str])] = &[(
    "shared_audit_record_cannot_construct_control_record.rs",
    &["OperationalAuditRecord", "OperationalControlRecord"],
)];

fn store_workspace_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .unwrap()
}
