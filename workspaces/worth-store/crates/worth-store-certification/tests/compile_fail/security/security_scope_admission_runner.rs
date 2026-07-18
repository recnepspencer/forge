use std::path::Path;

use worth_store_test_support::compiler_boundary::{
    cargo_dependency_manifest, run_cargo_ui_fixture_suite,
};

#[test]
fn an_admission_receipt_cannot_substitute_for_current_scope_authority() {
    let root = store_workspace_root();
    let evidence = run_cargo_ui_fixture_suite(
        root,
        "security-scope-admission",
        cargo_dependency_manifest(
            &[(
                "worth-store-security",
                root.join("crates/worth-store-security").as_path(),
                &[],
            )],
            &[],
        ),
        "production",
        "diagnostic-test",
        &root.join(
            "crates/worth-store-certification/tests/compile_fail/security/security_scope_admission",
        ),
        &[(
            "admission_receipt_cannot_satisfy_key_witness.rs",
            &[
                "StoreCurrentKeyScopeWitness",
                "StoreSecurityScopeAdmissionReceipt",
            ],
        )],
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
