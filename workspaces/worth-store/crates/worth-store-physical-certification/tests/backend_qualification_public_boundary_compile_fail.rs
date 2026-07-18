use std::path::Path;

use worth_store_test_support::compiler_boundary::{
    cargo_dependency_manifest, run_cargo_ui_fixture_suite,
};

#[test]
fn copied_rows_and_proof_authority_cannot_enter_public_publication_boundary() {
    let root = store_workspace_root();
    let evidence = run_cargo_ui_fixture_suite(
        root,
        "backend-qualification-public-boundary",
        cargo_dependency_manifest(
            &[
                ("worth-store-physical-backend", root.join("crates/worth-store-physical-backend").as_path(), &["certification-test-authority"]),
                ("worth-store-physical-certification", root.join("crates/worth-store-physical-certification").as_path(), &[]),
            ],
            &[],
        ),
        "backend-certification-authority",
        "diagnostic-test",
        &root.join("crates/worth-store-physical-certification/tests/ui/qualification/backend_public_boundary"),
        FIXTURES,
    ).unwrap();
    assert_eq!(evidence.fixtures.len(), FIXTURES.len());
}

const FIXTURES: &[(&str, &[&str])] = &[
    ("copied_row_cannot_publish.rs", &["with_row", "private"]),
    (
        "proof_authority_constructor_is_private.rs",
        &["private", "_private"],
    ),
    (
        "proof_authority_factory_is_private.rs",
        &["from_executed_store_evidence", "private"],
    ),
    (
        "row_proof_constructor_is_private.rs",
        &["from_admitted_backend_witness_with_proof", "private"],
    ),
];

fn store_workspace_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .unwrap()
}
