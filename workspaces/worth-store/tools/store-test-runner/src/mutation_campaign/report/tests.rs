use sha2::{Digest, Sha256};

use super::{
    legacy_owner_marker, normalized_report, published_artifact_directory, MutationEvidenceReport,
    MutationEvidenceSession, ARTIFACT_OWNER_MARKER, MUTATION_EVIDENCE_REPORT_SCHEMA,
};
use crate::mutation_campaign::{
    evidence::MutationObservation, source_inventory::MutationSourceBinding,
};

#[test]
fn report_schema_is_versioned_and_c5_1_specific() {
    let encoded = serde_json::to_value(MutationEvidenceReport {
        schema: MUTATION_EVIDENCE_REPORT_SCHEMA,
        source: &source_binding(),
        observations: &[],
    })
    .unwrap();
    assert_eq!(encoded["schema"], "worth.store.c5_1.mutation-evidence.v2");
    assert_eq!(
        encoded["source"]["binding"],
        "worth.store.c5_1.mutation-source-closure.v1"
    );
    assert_eq!(encoded["observations"], serde_json::json!([]));
}

#[test]
fn session_retains_distinct_binaries_and_publishes_report_last() {
    let temporary = tempfile::tempdir().unwrap();
    let report = temporary.path().join("phase16.json");
    std::fs::write(&report, b"stale").unwrap();
    let first = temporary.path().join("first.exe");
    let second = temporary.path().join("second.exe");
    std::fs::write(&first, b"first mutant").unwrap();
    std::fs::write(&second, b"second mutant").unwrap();

    let source = source_binding();
    let mut session = MutationEvidenceSession::begin(&report, source.clone()).unwrap();
    assert!(!report.exists(), "old success report must be invalidated");
    let mut observations = vec![observation(15, &first), observation(16, &second)];
    for observation in &mut observations {
        session.retain_binary(observation).unwrap();
    }
    assert_ne!(
        observations[0].binary_binding,
        observations[1].binary_binding
    );
    assert!(observations
        .iter()
        .all(|observation| !std::path::Path::new(&observation.binary_binding).exists()));
    session.publish(&observations, &source).unwrap();

    let encoded: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&report).unwrap()).unwrap();
    assert_eq!(encoded["observations"].as_array().unwrap().len(), 2);
    assert!(observations
        .iter()
        .all(|observation| std::path::Path::new(&observation.binary_binding).is_file()));
}

#[test]
fn successful_rerun_replaces_the_owned_artifact_set() {
    let temporary = tempfile::tempdir().unwrap();
    let report = temporary.path().join("phase16.json");
    let binary = temporary.path().join("mutant.exe");
    std::fs::write(&binary, b"first mutant").unwrap();
    let first_binding = publish_one(&report, &binary);
    assert!(first_binding.is_file());

    std::fs::write(&binary, b"replacement mutant").unwrap();
    let second_binding = publish_one(&report, &binary);
    assert!(!first_binding.exists());
    assert!(second_binding.is_file());
    assert_eq!(
        published_artifact_directory(&report).unwrap(),
        second_binding.parent().unwrap()
    );
    let artifact_directories = std::fs::read_dir(temporary.path())
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| {
            path.is_dir()
                && path
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .starts_with("phase16.json.artifacts")
        })
        .count();
    assert_eq!(artifact_directories, 1);
}

#[test]
fn abandoned_session_leaves_neither_report_nor_staged_binaries() {
    let temporary = tempfile::tempdir().unwrap();
    let report = temporary.path().join("phase16.json");
    std::fs::write(&report, b"stale").unwrap();
    let binary = temporary.path().join("mutant.exe");
    std::fs::write(&binary, b"mutant").unwrap();
    let staging;
    let published;
    {
        let mut session = MutationEvidenceSession::begin(&report, source_binding()).unwrap();
        staging = session.staging.clone();
        published = session.published_artifacts.clone();
        let mut observation = observation(15, &binary);
        session.retain_binary(&mut observation).unwrap();
        assert!(staging.is_dir());
        assert!(!published.exists());
    }
    assert!(!report.exists());
    assert!(!staging.exists());
    assert!(!published.exists());
}

#[test]
fn source_drift_rejects_publication_and_removes_staged_evidence() {
    let temporary = tempfile::tempdir().unwrap();
    let report = temporary.path().join("phase16.json");
    let binary = temporary.path().join("mutant.exe");
    std::fs::write(&binary, b"mutant").unwrap();
    let source = source_binding();
    let mut session = MutationEvidenceSession::begin(&report, source.clone()).unwrap();
    let staging = session.staging.clone();
    let mut observations = vec![observation(15, &binary)];
    session.retain_binary(&mut observations[0]).unwrap();
    let mut changed = source;
    changed.sha256 = "55".repeat(32);

    let error = session.publish(&observations, &changed).unwrap_err();

    assert!(
        error.contains("source changed before publication"),
        "{error}"
    );
    assert!(!report.exists());
    assert!(!staging.exists());
}

#[test]
fn legacy_owned_artifacts_are_migrated_without_accepting_unmarked_data() {
    let temporary = tempfile::tempdir().unwrap();
    let report = temporary.path().join("phase16.json");
    let artifacts = published_artifact_directory(&report).unwrap();
    let normalized = normalized_report(&report).unwrap();
    std::fs::create_dir(&artifacts).unwrap();
    std::fs::write(
        artifacts.join(ARTIFACT_OWNER_MARKER),
        legacy_owner_marker(&normalized),
    )
    .unwrap();
    std::fs::write(artifacts.join("legacy-mutant.exe"), b"legacy").unwrap();

    let session = MutationEvidenceSession::begin(&report, source_binding()).unwrap();

    assert!(!artifacts.exists());
    drop(session);
}

#[test]
fn unmarked_artifact_directory_is_never_replaced() {
    let temporary = tempfile::tempdir().unwrap();
    let report = temporary.path().join("phase16.json");
    let artifacts = published_artifact_directory(&report).unwrap();
    std::fs::create_dir(&artifacts).unwrap();
    std::fs::write(artifacts.join("user-data"), b"retain").unwrap();

    assert!(MutationEvidenceSession::begin(&report, source_binding()).is_err());
    assert!(artifacts.join("user-data").is_file());
    assert!(!artifacts.join(ARTIFACT_OWNER_MARKER).exists());
}

fn publish_one(report: &std::path::Path, binary: &std::path::Path) -> std::path::PathBuf {
    let source = source_binding();
    let mut session = MutationEvidenceSession::begin(report, source.clone()).unwrap();
    let mut observations = vec![observation(15, binary)];
    session.retain_binary(&mut observations[0]).unwrap();
    let binding = std::path::PathBuf::from(&observations[0].binary_binding);
    session.publish(&observations, &source).unwrap();
    binding
}

fn source_binding() -> MutationSourceBinding {
    MutationSourceBinding {
        binding: "worth.store.c5_1.mutation-source-closure.v1".into(),
        sha256: "44".repeat(32),
    }
}

fn observation(id: u8, binary: &std::path::Path) -> MutationObservation {
    let bytes = std::fs::read(binary).unwrap();
    MutationObservation {
        id,
        source_binding: "source.rs".into(),
        source_sha256: "11".repeat(32),
        mutant_sha256: "22".repeat(32),
        binary_binding: binary.display().to_string(),
        binary_sha256: Sha256::digest(bytes)
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect(),
        profile_binding: "test".into(),
        scenario_binding: "scenario".into(),
        expected_failing_predicate: "predicate".into(),
        actual_failing_predicate: "predicate".into(),
        localization: "test.rs:1".into(),
    }
}
