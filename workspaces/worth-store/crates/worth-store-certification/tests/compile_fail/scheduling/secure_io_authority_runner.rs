use std::path::Path;

use worth_store_test_support::compiler_boundary::{
    cargo_dependency_manifest, run_cargo_ui_fixture_suite,
};

#[test]
fn io_qos_secure_io_authority_rejects_lower_authority_sources() {
    let root = store_workspace_root();
    let evidence = run_cargo_ui_fixture_suite(
        root,
        "secure-io-authority",
        cargo_dependency_manifest(
            &[
                (
                    "worth-store-aspect-native",
                    root.join("crates/worth-store-aspect-native").as_path(),
                    &[],
                ),
                (
                    "worth-store-contracts",
                    root.join("crates/worth-store-contracts").as_path(),
                    &[],
                ),
                (
                    "worth-store-io-scheduler",
                    root.join("crates/worth-store-io-scheduler").as_path(),
                    &[],
                ),
                (
                    "worth-store-security",
                    root.join("crates/worth-store-security").as_path(),
                    &[],
                ),
            ],
            &[],
        ),
        "production",
        "diagnostic-test",
        &root.join(
            "crates/worth-store-certification/tests/compile_fail/scheduling/secure_io_authority",
        ),
        FIXTURES,
    )
    .unwrap();
    assert_eq!(evidence.fixtures.len(), FIXTURES.len());
}

const FIXTURES: &[(&str, &[&str])] = &[
    (
        "identity_provider_claim_cannot_satisfy_secure_io_scope.rs",
        &["IoSchedulerSecurityScopeAdmission", "StoreJwtSubjectClaim"],
    ),
    (
        "kms_key_id_cannot_satisfy_secure_io_scope.rs",
        &["IoSchedulerSecurityScopeAdmission", "StoreKmsKeyIdentifier"],
    ),
    (
        "iam_role_cannot_satisfy_secure_io_scope.rs",
        &["IoSchedulerSecurityScopeAdmission", "StoreIamRoleClaim"],
    ),
    (
        "operator_identity_cannot_satisfy_secure_io_scope.rs",
        &[
            "IoSchedulerSecurityScopeAdmission",
            "StoreOperatorIdentityClaim",
        ],
    ),
    (
        "terminal_projection_cannot_satisfy_secure_io_scope.rs",
        &[
            "IoSchedulerSecurityScopeAdmission",
            "StoreTerminalProjectionText",
        ],
    ),
    (
        "security_scope_identity_cannot_satisfy_secure_io_scope.rs",
        &[
            "IoSchedulerSecurityScopeAdmission",
            "StoreSecurityScopeIdentity",
        ],
    ),
];

fn store_workspace_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .unwrap()
}
