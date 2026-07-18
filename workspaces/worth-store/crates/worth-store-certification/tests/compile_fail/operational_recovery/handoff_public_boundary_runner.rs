use std::path::Path;
use std::process::Command;

use worth_store_test_support::compiler_boundary::{
    cargo_dependency_manifest, run_cargo_ui_fixture_suite,
};

#[test]
fn later_milestone_consumers_compile_but_cannot_reinterpret_handoffs_as_authority() {
    for binary in ["s11_public_consumer", "s12_public_consumer"] {
        let output = Command::new(env!("CARGO"))
            .args(["check", "--offline", "--quiet", "--bin", binary])
            .current_dir(static_case_root())
            .env(
                "CARGO_TARGET_DIR",
                store_workspace_root().join(".store-proof/cache/ui/handoff-positive-api"),
            )
            .output()
            .expect("handoff positive API fixture invokes Cargo");
        assert!(
            output.status.success(),
            "{binary} public consumer failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let root = store_workspace_root();
    let evidence = run_cargo_ui_fixture_suite(
        root,
        "s10-handoff-public-boundary",
        cargo_dependency_manifest(
            &[
                (
                    "worth-store-authority",
                    root.join("crates/worth-store-authority").as_path(),
                    &[],
                ),
                (
                    "worth-store-certification",
                    root.join("crates/worth-store-certification").as_path(),
                    &[],
                ),
                (
                    "worth-store-operations",
                    root.join("crates/worth-store-operations").as_path(),
                    &[],
                ),
            ],
            &[],
        ),
        "production",
        "diagnostic-test",
        &static_case_root().join("src/bin"),
        FIXTURES,
    )
    .unwrap();
    assert_eq!(evidence.fixtures.len(), FIXTURES.len());
}

const FIXTURES: &[(&str, &[&str])] = &[
    (
        "s11_handoff_cannot_mint_current_authority.rs",
        &[
            "S11StructuredAuditHardeningHandoff",
            "StoreCurrentAuthorityWitness",
        ],
    ),
    (
        "s12_handoff_cannot_mint_control_state.rs",
        &[
            "S12PhysicalQualificationHandoff",
            "SelectedOperationalControlState",
        ],
    ),
    (
        "s11_handoff_fields_are_not_reinterpretable.rs",
        &["S11StructuredAuditHardeningHandoff", "private"],
    ),
    (
        "s12_handoff_fields_are_not_reinterpretable.rs",
        &["S12PhysicalQualificationHandoff", "private"],
    ),
];

fn static_case_root() -> std::path::PathBuf {
    store_workspace_root().join("crates/worth-store-certification/tests/compile_fail/operational_recovery/cases/handoff_public_boundary")
}

fn store_workspace_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .unwrap()
}
