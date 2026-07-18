use std::path::Path;

use worth_store_test_support::compiler_boundary::{
    cargo_dependency_manifest, run_cargo_ui_fixture_suite,
};

#[test]
fn closeout_rejects_certification_owned_authority_shortcuts() {
    let root = store_workspace_root();
    let evidence = run_cargo_ui_fixture_suite(
        root,
        "security-certification-closeout",
        cargo_dependency_manifest(
            &[
                (
                    "worth-store-certification",
                    root.join("crates/worth-store-certification").as_path(),
                    &[],
                ),
                (
                    "worth-store-physical-certification",
                    root.join("crates/worth-store-physical-certification")
                        .as_path(),
                    &[],
                ),
                (
                    "worth-store-readiness",
                    root.join("crates/worth-store-readiness").as_path(),
                    &[],
                ),
            ],
            &[],
        ),
        "production",
        "diagnostic-test",
        &root.join(
            "crates/worth-store-certification/tests/compile_fail/security/certification_closeout",
        ),
        FIXTURES,
    )
    .unwrap();
    assert_eq!(evidence.fixtures.len(), FIXTURES.len());
}

const FIXTURES: &[(&str, &[&str])] = &[
    (
        "closeout_input_cannot_be_struct_literal.rs",
        &["S51CertificationCloseoutInput", "private"],
    ),
    (
        "performance_rows_cannot_satisfy_closeout_input.rs",
        &[
            "from_replay_and_security_scope",
            "SecurityScopeHarnessEvidence",
            "S51CloseoutPerformanceRows",
        ],
    ),
    (
        "closeout_evidence_cannot_satisfy_readiness.rs",
        &[
            "S51AdmittedSecurityScopeReadiness",
            "S51CertificationCloseoutEvidence",
        ],
    ),
];

fn store_workspace_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .unwrap()
}
