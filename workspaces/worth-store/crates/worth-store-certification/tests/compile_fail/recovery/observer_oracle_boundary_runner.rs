use std::path::Path;

use worth_store_test_support::compiler_boundary::{
    cargo_dependency_manifest, run_cargo_ui_fixture_suite,
};

#[test]
fn observer_oracle_boundary_rejects_forbidden_sources_at_compile_time() {
    let root = store_workspace_root();
    let evidence = run_cargo_ui_fixture_suite(
        root,
        "observer-oracle-boundary",
        dependency_manifest(root),
        "production",
        "diagnostic-test",
        &root.join(
            "crates/worth-store-certification/tests/compile_fail/recovery/observer_oracle_boundary",
        ),
        FIXTURES,
    )
    .unwrap();
    assert_eq!(evidence.fixtures.len(), FIXTURES.len());
}

const FIXTURES: &[(&str, &[&str])] = &[
    (
        "test_support_oracle_cannot_implement_physical_oracle.rs",
        &["CertificationOwnedOracle", "TestSupportOracle"],
    ),
    (
        "log_text_cannot_be_oracle.rs",
        &["PhysicalProofOracle", "&str"],
    ),
    (
        "expected_error_text_cannot_be_verdict.rs",
        &["PhysicalProofOracleVerdict", "&str"],
    ),
    (
        "same_run_self_comparison_cannot_be_oracle.rs",
        &["PhysicalProofOracle", "SameRunSelfComparison"],
    ),
    (
        "fixture_label_cannot_be_oracle.rs",
        &["PhysicalProofOracle", "FixtureLabel"],
    ),
    (
        "oracle_verdict_basis_cannot_be_struct_literal.rs",
        &["OracleVerdictBasis", "private"],
    ),
    (
        "oracle_verdict_cannot_be_struct_literal.rs",
        &["PhysicalProofOracleVerdict", "private"],
    ),
];

fn dependency_manifest(root: &Path) -> String {
    cargo_dependency_manifest(
        &[
            (
                "worth-store-physical-certification",
                root.join("crates/worth-store-physical-certification")
                    .as_path(),
                &[],
            ),
            (
                "worth-store-test-support",
                root.join("crates/worth-store-test-support").as_path(),
                &[],
            ),
        ],
        &[],
    )
}

fn store_workspace_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("certification crate lives under the Store workspace")
}
