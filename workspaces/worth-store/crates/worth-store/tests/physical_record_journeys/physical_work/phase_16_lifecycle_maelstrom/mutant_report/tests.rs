use std::path::PathBuf;

use sha2::{Digest, Sha256};

use super::{
    complete_mutant_count, expectation, validate_campaign_shape, validate_observation,
    ArtifactPolicy, MutationExecutionClass, MutationExecutionEvidence, MutationObservation,
    ARTIFACT_OWNER_MARKER, ARTIFACT_OWNER_SCHEMA, FIRST_MUTANT, LAST_MUTANT,
};

#[test]
fn campaign_shape_requires_every_phase_16_mutant_including_c_only_rows() {
    let mut observations = (FIRST_MUTANT..=LAST_MUTANT)
        .map(empty_observation)
        .collect::<Vec<_>>();
    assert!(validate_campaign_shape(&observations).is_ok());
    assert_eq!(observations.len(), complete_mutant_count());
    assert_eq!(complete_mutant_count(), 30);
    observations.swap(26, 27);
    assert!(validate_campaign_shape(&observations).is_err());
    observations.swap(26, 27);
    observations.pop();
    assert!(validate_campaign_shape(&observations).is_err());
}

#[test]
fn validation_rejects_stale_source_and_binary_bytes() {
    let fixture = Fixture::new();
    let baseline = fixture.validate(fixture.observation());
    assert!(baseline.is_ok(), "baseline denial: {baseline:?}");

    std::fs::write(&fixture.source, b"stale source").unwrap();
    assert!(fixture
        .validate(fixture.observation())
        .unwrap_err()
        .contains("source is stale"));
    std::fs::write(&fixture.source, Fixture::SOURCE).unwrap();

    std::fs::write(&fixture.binary, b"stale binary").unwrap();
    assert!(fixture
        .validate(fixture.observation())
        .unwrap_err()
        .contains("binary is stale"));
}

#[test]
fn validation_rejects_binary_path_laundering() {
    let fixture = Fixture::new();
    let escaped = fixture.temporary.path().join("escaped.exe");
    std::fs::write(&escaped, Fixture::BINARY).unwrap();
    let mut observation = fixture.observation();
    observation.binary_binding = escaped.display().to_string();
    assert!(fixture
        .validate(observation)
        .unwrap_err()
        .contains("escaped its report artifact directory"));
}

struct Fixture {
    temporary: tempfile::TempDir,
    workspace: PathBuf,
    source: PathBuf,
    binary: PathBuf,
    policy: ArtifactPolicy,
}

impl Fixture {
    const SOURCE: &'static [u8] = b"current source";
    const BINARY: &'static [u8] = b"retained mutant executable";

    fn new() -> Self {
        let temporary = tempfile::tempdir().unwrap();
        let workspace = temporary.path().join("workspace");
        let expected = expectation::for_id(FIRST_MUTANT);
        let source = workspace.join(expected.source);
        std::fs::create_dir_all(source.parent().unwrap()).unwrap();
        std::fs::write(&source, Self::SOURCE).unwrap();
        let report = temporary.path().join("phase16.json");
        let artifacts = temporary.path().join("phase16.json.artifacts.1.1");
        std::fs::create_dir(&artifacts).unwrap();
        std::fs::write(
            artifacts.join(ARTIFACT_OWNER_MARKER),
            format!("{ARTIFACT_OWNER_SCHEMA}\n{}\n", report.display()),
        )
        .unwrap();
        let binary = artifacts.join("mutant-15.exe");
        std::fs::write(&binary, Self::BINARY).unwrap();
        let policy = ArtifactPolicy::for_report(&report).unwrap();
        Self {
            temporary,
            workspace,
            source,
            binary,
            policy,
        }
    }

    fn observation(&self) -> MutationObservation {
        let expected = expectation::for_id(FIRST_MUTANT);
        MutationObservation {
            id: FIRST_MUTANT,
            source_binding: expected.source.into(),
            source_sha256: hash(Self::SOURCE),
            mutant_sha256: "22".repeat(32),
            binary_binding: self.binary.display().to_string(),
            binary_sha256: hash(Self::BINARY),
            profile_binding: "test".into(),
            scenario_binding: expected.scenario.into(),
            expected_failing_predicate: expected.predicate.into(),
            actual_failing_predicate: expected.predicate.into(),
            localization: "courtroom.rs:1".into(),
            execution: MutationExecutionEvidence {
                class: MutationExecutionClass::Ordinary,
                elapsed_ms: 1,
                budget_ms: 180_000,
            },
        }
    }

    fn validate(
        &self,
        observation: MutationObservation,
    ) -> Result<worth_store::physical_runtime::PhysicalWorkMutantLocalization, String> {
        validate_observation(
            observation,
            expectation::for_id(FIRST_MUTANT),
            &self.workspace,
            &self.policy,
        )
    }
}

fn hash(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn empty_observation(id: u8) -> MutationObservation {
    MutationObservation {
        id,
        source_binding: String::new(),
        source_sha256: String::new(),
        mutant_sha256: String::new(),
        binary_binding: String::new(),
        binary_sha256: String::new(),
        profile_binding: String::new(),
        scenario_binding: String::new(),
        expected_failing_predicate: String::new(),
        actual_failing_predicate: String::new(),
        localization: String::new(),
        execution: MutationExecutionEvidence {
            class: MutationExecutionClass::Ordinary,
            elapsed_ms: 0,
            budget_ms: 180_000,
        },
    }
}
