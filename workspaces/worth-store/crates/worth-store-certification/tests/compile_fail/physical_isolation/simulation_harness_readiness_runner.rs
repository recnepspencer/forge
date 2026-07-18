use std::path::Path;

use worth_store_test_support::compiler_boundary::{
    cargo_dependency_manifest, run_cargo_ui_fixture_suite,
};

#[test]
fn physical_isolation_readiness_handoff_authority_cannot_be_forged_at_compile_time() {
    let root = store_workspace_root();
    let evidence = run_cargo_ui_fixture_suite(
        root,
        "physical-isolation-harness-readiness",
        dependency_manifest(root),
        "production",
        "diagnostic-test",
        &root.join(
            "crates/worth-store-certification/tests/compile_fail/physical_isolation/readiness",
        ),
        FIXTURES,
    )
    .unwrap();
    assert_eq!(evidence.fixtures.len(), FIXTURES.len());
}

const FIXTURES: &[(&str, &[&str])] = &[
    (
        "handoff_receipt_cannot_be_struct_literal.rs",
        &["PhysicalIsolationHarnessReadinessReceipt", "private"],
    ),
    (
        "physical_isolation_cannot_accept_generic_runner.rs",
        &["accept_store_owned_s5_harness_readiness", "no"],
    ),
    (
        "generic_runner_cannot_implement_readiness_contract.rs",
        &["PhysicalIsolationHarnessReadinessContract", "not found"],
    ),
    (
        "future_slot_cannot_be_readiness.rs",
        &[
            "accept_store_owned_s5_harness_readiness",
            "s5_simulation_harness_readiness_requirement",
        ],
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
                "worth-store-physical-isolation",
                root.join("crates/worth-store-physical-isolation").as_path(),
                &[],
            ),
            (
                "worth-store-readiness",
                root.join("crates/worth-store-readiness").as_path(),
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
