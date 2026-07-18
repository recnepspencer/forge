use std::path::Path;

use worth_store_test_support::compiler_boundary::{
    cargo_dependency_manifest, run_cargo_ui_fixture_suite,
};

#[test]
fn materialized_authority_boundaries_reject_public_forgery() {
    let root = store_workspace_root();
    let forge_root = root.ancestors().nth(2).unwrap();
    let evidence = run_cargo_ui_fixture_suite(
        root,
        "s6-evidence-materialization",
        cargo_dependency_manifest(
            &[
                ("worth-foundational", forge_root.join("crates/worth-foundational").as_path(), &[]),
                ("worth-store-certification", root.join("crates/worth-store-certification").as_path(), &[]),
                ("worth-store-readiness", root.join("crates/worth-store-readiness").as_path(), &[]),
            ],
            &[],
        ),
        "production",
        "diagnostic-test",
        &root.join("crates/worth-store-certification/tests/compile_fail/scheduling/evidence_materialization"),
        FIXTURES,
    ).unwrap();
    assert_eq!(evidence.fixtures.len(), FIXTURES.len());
}

const FIXTURES: &[(&str, &[&str])] = &[
    ("bundle_fields_cannot_be_minted.rs", &["private"]),
    ("source_fields_cannot_be_minted.rs", &["private"]),
    (
        "closeout_rejects_foundational_receipt.rs",
        &["mismatched types"],
    ),
    (
        "closeout_rejects_profile_evidence.rs",
        &["mismatched types"],
    ),
    (
        "closeout_rejects_foundational_boundary.rs",
        &["mismatched types"],
    ),
    ("closeout_rejects_canonical_basis.rs", &["mismatched types"]),
    ("closeout_rejects_proof_trace.rs", &["mismatched types"]),
    (
        "legacy_scalar_closeout_evidence_is_unavailable.rs",
        &["S6MaterializedCertificationCloseoutEvidence"],
    ),
];

fn store_workspace_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .unwrap()
}
