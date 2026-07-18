use std::path::Path;

use worth_store_test_support::compiler_boundary::{
    cargo_dependency_manifest, run_cargo_ui_fixture_suite,
};

#[test]
fn later_milestone_consumers_compile_but_cannot_reinterpret_handoffs_as_authority() {
    let _ = consume_s11_handoff
        as fn(&worth_store_certification::courtroom::operational_recovery::S11StructuredAuditHardeningHandoff);
    let _ = consume_s12_handoff
        as fn(&worth_store_certification::courtroom::operational_recovery::S12PhysicalQualificationHandoff);

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

fn consume_s11_handoff(
    handoff: &worth_store_certification::courtroom::operational_recovery::S11StructuredAuditHardeningHandoff,
) {
    let _ = handoff.closeout_identity();
    let _ = handoff.structured_audit_schema();
    let _ = handoff.scenario_evidence_identities();
    let _ = handoff.unimplemented_strengthening();
}

fn consume_s12_handoff(
    handoff: &worth_store_certification::courtroom::operational_recovery::S12PhysicalQualificationHandoff,
) {
    let _ = handoff.closeout_identity();
    let _ = handoff.scenario_evidence_identities();
    let _ = handoff.complexity_contracts();
    let _ = handoff.unqualified_dimensions();
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
