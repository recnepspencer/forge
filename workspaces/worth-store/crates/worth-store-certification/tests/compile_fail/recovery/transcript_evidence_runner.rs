use std::path::Path;

use worth_store_test_support::compiler_boundary::{
    cargo_dependency_manifest, run_cargo_ui_fixture_suite,
};

#[test]
fn transcript_evidence_boundaries_reject_shortcuts_at_compile_time() {
    let root = store_workspace_root();
    let evidence = run_cargo_ui_fixture_suite(
        root,
        "physical-transcript-evidence",
        dependency_manifest(root),
        "production",
        "diagnostic-test",
        &root.join(
            "crates/worth-store-certification/tests/compile_fail/recovery/transcript_evidence",
        ),
        FIXTURES,
    )
    .unwrap();
    assert_eq!(evidence.fixtures.len(), FIXTURES.len());
}

const FIXTURES: &[(&str, &[&str])] = &[
    (
        "transcript_cannot_be_struct_literal.rs",
        &["PhysicalSimulationTranscript", "private"],
    ),
    (
        "copied_digest_cannot_be_transcript_identity.rs",
        &["PhysicalSimulationTranscriptIdentity", "[u8; 32]"],
    ),
    (
        "terminal_json_cannot_satisfy_replay_bundle.rs",
        &["SimulationReplayBundle", "Value"],
    ),
    (
        "loose_log_cannot_satisfy_replay_bundle.rs",
        &["SimulationReplayBundle", "String"],
    ),
    (
        "terminal_json_cannot_satisfy_evidence_bundle.rs",
        &["PhysicalCertificationEvidenceBundle", "Value"],
    ),
    (
        "same_run_comparison_cannot_satisfy_evidence_bundle.rs",
        &["PhysicalCertificationEvidenceBundle", "SameRunComparison"],
    ),
    (
        "copied_field_bag_cannot_construct_detached_replay_parts.rs",
        &["DetachedSimulationReplayParts", "private"],
    ),
    (
        "foundational_bundle_cannot_satisfy_store_evidence.rs",
        &[
            "PhysicalCertificationEvidenceBundle",
            "FoundationalPhysicalCertificationEvidenceBundle",
        ],
    ),
];

fn dependency_manifest(root: &Path) -> String {
    cargo_dependency_manifest(
        &[(
            "worth-store-physical-certification",
            root.join("crates/worth-store-physical-certification")
                .as_path(),
            &[],
        )],
        &[("serde_json", "1")],
    )
}

fn store_workspace_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("certification crate lives under the Store workspace")
}
