use std::path::Path;

use worth_store_test_support::compiler_boundary::{
    cargo_dependency_manifest, run_cargo_ui_fixture_suite,
};

#[test]
fn copy_on_write_publication_misuse_does_not_compile() {
    let root = store_workspace_root();
    let evidence = run_cargo_ui_fixture_suite(
        root,
        "copy-on-write-publication",
        cargo_dependency_manifest(
            &[
                ("worth-store-physical-isolation", root.join("crates/worth-store-physical-isolation").as_path(), &[]),
                ("worth-store-recovery-physics", root.join("crates/worth-store-recovery-physics").as_path(), &[]),
            ],
            &[],
        ),
        "production",
        "diagnostic-test",
        &root.join("crates/worth-store-certification/tests/compile_fail/physical_isolation/copy_on_write_publication"),
        FIXTURES,
    ).unwrap();
    assert_eq!(evidence.fixtures.len(), FIXTURES.len());
}

const FIXTURES: &[(&str, &[&str])] = &[
    (
        "raw_intent_cannot_publish.rs",
        &["no function or associated item named `publish`"],
    ),
    (
        "lowered_plan_cannot_publish.rs",
        &["no function or associated item named `publish`"],
    ),
    (
        "checkpoint_receipt_cannot_publish.rs",
        &["no function or associated item named `publish`"],
    ),
    (
        "foundational_evidence_cannot_publish.rs",
        &["no function or associated item named `publish`"],
    ),
    (
        "raw_recovery_observation_cannot_publish.rs",
        &["S5PublicationRecoveryObservation"],
    ),
    (
        "publication_recovery_replay_requires_readiness.rs",
        &["execute", "method not found"],
    ),
];

fn store_workspace_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .unwrap()
}
