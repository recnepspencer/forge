use std::path::Path;

use worth_store_test_support::compiler_boundary::{
    cargo_dependency_manifest, run_cargo_ui_fixture_suite,
};

#[test]
fn io_qos_readiness_handoff_denies_public_raw_materialization_paths() {
    let root = store_workspace_root();
    let evidence = run_cargo_ui_fixture_suite(
        root,
        "io-qos-readiness-handoff",
        dependency_manifest(root),
        "certification-authority",
        "diagnostic-test",
        &root.join(
            "crates/worth-store-certification/tests/compile_fail/scheduling/io_qos_readiness_handoff",
        ),
        FIXTURES,
    )
    .unwrap();
    assert_eq!(evidence.fixtures.len(), FIXTURES.len());
}

const FIXTURES: &[(&str, &[&str])] = &[
    (
        "raw_counts_cannot_materialize_readiness.rs",
        &["S5CertifiedStoreExecutionCloseout"],
    ),
    (
        "raw_source_cannot_publish_readiness.rs",
        &["publish_scheduler_isolation_capability"],
    ),
    (
        "raw_request_cannot_be_built_without_handoff_evidence.rs",
        &["SchedulerIsolationCapabilityRequest"],
    ),
];

fn dependency_manifest(root: &Path) -> String {
    cargo_dependency_manifest(
        &[
            (
                "worth-store-authority",
                root.join("crates/worth-store-authority").as_path(),
                &[],
            ),
            (
                "worth-store-physical-isolation",
                root.join("crates/worth-store-physical-isolation").as_path(),
                &["certification-authority"],
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
