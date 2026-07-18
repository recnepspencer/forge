use std::path::Path;

use worth_store_test_support::compiler_boundary::{
    cargo_dependency_manifest, run_cargo_ui_fixture_suite,
};

#[test]
fn coverage_and_readiness_authority_cannot_be_hand_filled() {
    let root = store_workspace_root();
    let evidence = run_cargo_ui_fixture_suite(
        root,
        "recovery-coverage-authority",
        dependency_manifest(root),
        "production",
        "diagnostic-test",
        &root.join("crates/worth-store-certification/tests/compile_fail/recovery/coverage"),
        FIXTURES,
    )
    .unwrap();
    assert_eq!(evidence.fixtures.len(), FIXTURES.len());
}

const FIXTURES: &[(&str, &[&str])] = &[
    (
        "coverage_row_cannot_be_struct_literal.rs",
        &["PhysicalCoverageMatrixRow", "private"],
    ),
    (
        "generated_matrix_cannot_be_struct_literal.rs",
        &["GeneratedCoverageMatrix", "private"],
    ),
    (
        "terminal_json_cannot_satisfy_coverage.rs",
        &["GeneratedCoverageMatrix", "Value"],
    ),
    (
        "mutation_coverage_cannot_be_label_minted.rs",
        &["admitted_expected_failure"],
    ),
    (
        "mutation_coverage_cannot_be_plan_only.rs",
        &["from_private_mutation_denial"],
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
                "worth-store-readiness",
                root.join("crates/worth-store-readiness").as_path(),
                &[],
            ),
        ],
        &[("serde_json", "1")],
    )
}

fn store_workspace_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("certification crate lives under the Store workspace")
}
