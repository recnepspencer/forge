use std::path::Path;

use worth_store_test_support::compiler_boundary::{
    cargo_dependency_manifest, run_cargo_ui_fixture_suite,
};

#[test]
fn forbidden_shortcut_authority_cannot_be_forged_at_compile_time() {
    for fixture in fixtures() {
        run_fixture(fixture);
    }
}

#[derive(Clone, Copy)]
struct Fixture {
    directory: &'static str,
    name: &'static str,
    expected: &'static [&'static str],
}

fn fixtures() -> [Fixture; 13] {
    [
        fixture(
            "forbidden_shortcuts",
            "shortcut_report_cannot_be_struct_literal.rs",
            &["SyntheticHarnessShortcutRejectionReport", "private"],
        ),
        fixture(
            "forbidden_shortcuts",
            "shortcut_receipt_cannot_be_struct_literal.rs",
            &["SyntheticHarnessShortcutDenialReceipt", "private"],
        ),
        fixture(
            "forbidden_shortcuts",
            "raw_json_cannot_satisfy_certified_scenario.rs",
            &["CertifiedPhysicalScenario", "u8"],
        ),
        fixture(
            "forbidden_shortcuts",
            "terminal_text_cannot_satisfy_evidence_bundle.rs",
            &["PhysicalCertificationEvidenceBundle", "String"],
        ),
        fixture(
            "forbidden_shortcuts",
            "foundational_bundle_cannot_satisfy_store_shortcut_evidence.rs",
            &[
                "PhysicalCertificationEvidenceBundle",
                "FoundationalPhysicalCertificationEvidenceBundle",
            ],
        ),
        fixture(
            "forbidden_shortcuts",
            "proof_recipe_cannot_satisfy_lowered_plan.rs",
            &["PhysicalSimulationPlan", "Recipe"],
        ),
        fixture(
            "forbidden_shortcuts",
            "schedule_cannot_be_struct_literal.rs",
            &["PhysicalInterleavingSchedule", "private"],
        ),
        fixture(
            "forbidden_shortcuts",
            "executed_transcript_parts_cannot_be_struct_literal.rs",
            &["ExecutedTranscriptParts", "private"],
        ),
        fixture(
            "transcript_evidence",
            "transcript_cannot_be_struct_literal.rs",
            &["PhysicalSimulationTranscript", "private"],
        ),
        fixture(
            "observer_oracle_boundary",
            "oracle_verdict_cannot_be_struct_literal.rs",
            &["PhysicalProofOracleVerdict", "private"],
        ),
        fixture(
            "transcript_evidence",
            "copied_field_bag_cannot_construct_detached_replay_parts.rs",
            &["DetachedSimulationReplayParts", "private"],
        ),
        fixture(
            "forbidden_shortcuts",
            "closeout_report_constructor_is_private.rs",
            &["new", "private"],
        ),
        fixture(
            "forbidden_shortcuts",
            "executed_acceptance_suite_constructor_is_private.rs",
            &[
                "entry_boundary_suite_run",
                "ExecutedSimulationHarnessAcceptanceSuiteEvidence",
            ],
        ),
    ]
}

const fn fixture(
    directory: &'static str,
    name: &'static str,
    expected: &'static [&'static str],
) -> Fixture {
    Fixture {
        directory,
        name,
        expected,
    }
}

fn run_fixture(fixture: Fixture) {
    let root = store_workspace_root();
    let forge_root = root.ancestors().nth(2).unwrap();
    let evidence = run_cargo_ui_fixture_suite(
        root,
        "recovery-forbidden-shortcuts",
        cargo_dependency_manifest(
            &[
                (
                    "worth-proof",
                    forge_root.join("crates/worth-proof").as_path(),
                    &[],
                ),
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
            &[],
        ),
        "production",
        "diagnostic-test",
        &root
            .join("crates/worth-store-certification/tests/compile_fail/recovery")
            .join(fixture.directory),
        &[(fixture.name, fixture.expected)],
    )
    .unwrap();
    assert_eq!(evidence.fixtures.len(), 1);
}

fn store_workspace_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .unwrap()
}
