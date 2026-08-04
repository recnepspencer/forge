use super::{MutationEvidenceReport, MutationEvidenceSession, MUTATION_EVIDENCE_REPORT_SCHEMA};
use crate::mutation_campaign::{
    evidence::{MutationExecutionClass, MutationExecutionEvidence, MutationObservation},
    source_inventory::MutationSourceBinding,
    MutationCampaignScope,
};

#[test]
fn report_schema_is_semantic_and_versioned() {
    let encoded = serde_json::to_value(MutationEvidenceReport {
        schema: MUTATION_EVIDENCE_REPORT_SCHEMA,
        scope: MutationCampaignScope::PhysicalWork,
        source: &source_binding(),
        observations: &[],
    })
    .unwrap();
    assert_eq!(
        encoded["schema"],
        "worth.store.controlled-mutation-evidence.v5"
    );
    assert_eq!(encoded["scope"], "physical-work");
    assert_eq!(
        encoded["source"]["binding"],
        "worth.store.controlled-mutation-source-closure.v3"
    );
    assert_eq!(encoded["observations"], serde_json::json!([]));
}

#[test]
fn session_publishes_one_report_without_a_companion_artifact_directory() {
    let temporary = tempfile::tempdir().unwrap();
    let report = temporary.path().join("mutants.json");
    let source = source_binding();
    let session = MutationEvidenceSession::begin(
        &report,
        source.clone(),
        MutationCampaignScope::BoundedResidency,
    )
    .unwrap();
    session.publish(&[observation(15)], &source).unwrap();

    let encoded: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&report).unwrap()).unwrap();
    assert_eq!(encoded["observations"].as_array().unwrap().len(), 1);
    assert_eq!(encoded["scope"], "bounded-residency");
    assert_eq!(
        std::fs::read_dir(temporary.path()).unwrap().count(),
        1,
        "the JSON report must be the only retained mutation artifact"
    );
}

#[test]
fn abandoned_session_leaves_neither_report_nor_pending_file() {
    let temporary = tempfile::tempdir().unwrap();
    let report = temporary.path().join("mutants.json");
    std::fs::write(&report, b"stale").unwrap();
    let pending;
    {
        let session = MutationEvidenceSession::begin(
            &report,
            source_binding(),
            MutationCampaignScope::PhysicalWork,
        )
        .unwrap();
        pending = session.pending.clone();
        std::fs::write(&pending, b"partial").unwrap();
        assert!(!report.exists());
    }
    assert!(!report.exists());
    assert!(!pending.exists());
}

#[test]
fn source_drift_rejects_publication_and_removes_pending_evidence() {
    let temporary = tempfile::tempdir().unwrap();
    let report = temporary.path().join("mutants.json");
    let source = source_binding();
    let session = MutationEvidenceSession::begin(
        &report,
        source.clone(),
        MutationCampaignScope::PhysicalWork,
    )
    .unwrap();
    let pending = session.pending.clone();
    let mut changed = source;
    changed.sha256 = "55".repeat(32);

    let error = session.publish(&[observation(15)], &changed).unwrap_err();

    assert!(
        error.contains("source changed before publication"),
        "{error}"
    );
    assert!(!report.exists());
    assert!(!pending.exists());
}

fn source_binding() -> MutationSourceBinding {
    MutationSourceBinding {
        binding: "worth.store.controlled-mutation-source-closure.v3".into(),
        sha256: "44".repeat(32),
    }
}

fn observation(id: u8) -> MutationObservation {
    MutationObservation {
        id,
        source_binding: "source.rs".into(),
        source_sha256: "11".repeat(32),
        mutant_sha256: "22".repeat(32),
        binary_binding: "target/proof.exe".into(),
        binary_sha256: "33".repeat(32),
        profile_binding: "test".into(),
        scenario_binding: "scenario".into(),
        expected_failing_predicate: "predicate".into(),
        actual_failing_predicate: "predicate".into(),
        localization: "test.rs:1".into(),
        execution: MutationExecutionEvidence::bind(
            MutationExecutionClass::Ordinary,
            std::time::Duration::from_millis(12),
        )
        .unwrap(),
    }
}
