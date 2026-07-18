use std::path::Path;

use worth_store_test_support::compiler_boundary::{
    cargo_dependency_manifest, run_cargo_ui_fixture_suite,
};

#[test]
fn stable_read_execution_misuse_does_not_compile() {
    run(FIXTURES, "stable-read-execution", "stable_read_execution");
}

const FIXTURES: &[(&str, &[&str])] = &[
    (
        "plan_cannot_start_execution.rs",
        &["from_plan", "StablePhysicalReadExecution"],
    ),
    (
        "raw_bytes_cannot_be_read_by_execution.rs",
        &["read_guarded_bytes", "PhysicalByteGuard"],
    ),
    (
        "reachability_barrier_is_not_byte_guard.rs",
        &["PhysicalByteGuard", "PhysicalReadReachabilityBarrier"],
    ),
    (
        "byte_guard_does_not_expose_raw_bytes.rs",
        &["no method named", "as_bytes"],
    ),
    (
        "raw_vec_cannot_mint_owned_read_buffer_guard.rs",
        &["from_owned_read_buffer"],
    ),
    (
        "guarded_bytes_cannot_outlive_execution_completion.rs",
        &["cannot move out of", "borrowed"],
    ),
    (
        "root_witness_cannot_satisfy_logical_decode_scope.rs",
        &["LogicalDecodeSecurityScopeEntry", "CurrentPhysicalRoot"],
    ),
    (
        "raw_bytes_cannot_enter_scoped_logical_decode.rs",
        &["PhysicalByteGuard", "[u8"],
    ),
    (
        "logical_decode_scope_entry_cannot_be_constructed.rs",
        &["from_observed_scope", "private"],
    ),
];

fn run(fixtures: &[(&str, &[&str])], suite: &str, directory: &str) {
    let root = store_workspace_root();
    let evidence = run_cargo_ui_fixture_suite(
        root,
        suite,
        cargo_dependency_manifest(
            &[(
                "worth-store-physical-isolation",
                root.join("crates/worth-store-physical-isolation").as_path(),
                &[],
            )],
            &[],
        ),
        "production",
        "diagnostic-test",
        &root
            .join("crates/worth-store-certification/tests/compile_fail/physical_isolation")
            .join(directory),
        fixtures,
    )
    .unwrap();
    assert_eq!(evidence.fixtures.len(), fixtures.len());
}

fn store_workspace_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .unwrap()
}
