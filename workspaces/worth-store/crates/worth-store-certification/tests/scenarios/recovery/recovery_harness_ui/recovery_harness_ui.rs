use std::path::Path;

use worth_store_test_support::compiler_boundary::{
    cargo_dependency_manifest, run_cargo_ui_fixture_suite,
};

#[test]
fn recovery_harness_public_facade_rejects_shortcut_authority() {
    let root = store_workspace_root();
    let evidence = run_cargo_ui_fixture_suite(
        root,
        "recovery-harness-public-authority",
        cargo_dependency_manifest(
            &[
                (
                    "worth-store-certification",
                    root.join("crates/worth-store-certification").as_path(),
                    &[],
                ),
                (
                    "worth-store-recovery-physics",
                    root.join("crates/worth-store-recovery-physics").as_path(),
                    &[],
                ),
                (
                    "worth-store-physical-backend",
                    root.join("crates/worth-store-physical-backend").as_path(),
                    &[],
                ),
            ],
            &[],
        ),
        "production",
        "diagnostic-test",
        &root.join("crates/worth-store-certification/tests/compile_fail/recovery/recovery_harness"),
        FIXTURES,
    )
    .unwrap();
    assert_eq!(evidence.fixtures.len(), FIXTURES.len());
}

const FIXTURES: &[(&str, &[&str])] = &[
    (
        "direct_private_mutation_cannot_certify.rs",
        &["RecoveryPhysicsMutationSuiteLaneEvidence", "private"],
    ),
    (
        "same_run_self_comparison_cannot_certify.rs",
        &["denied", "private"],
    ),
    (
        "foundational_bundle_cannot_satisfy_recovered_state.rs",
        &[
            "RecoveredPhysicalState",
            "FoundationalRecoveryEvidenceBundle",
        ],
    ),
    (
        "proof_trace_cannot_satisfy_redo_plan.rs",
        &["RecoveryRedoPlan", "ProofProgressionRecoveryTrace"],
    ),
    (
        "performance_receipt_cannot_satisfy_durable_ack.rs",
        &["DurableAckReceipt", "RecoveryCounterPerformanceReceipt"],
    ),
];

fn store_workspace_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("certification crate lives under the Store workspace")
}
