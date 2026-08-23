use std::path::Path;

#[cfg(feature = "physical-work-evidence")]
use serde::Deserialize;
use serde::Serialize;
#[cfg(feature = "physical-work-evidence")]
use sha2::{Digest, Sha256};

#[cfg(feature = "physical-work-evidence")]
use super::{catalog::ControlledMutation, source_inventory};
use super::{
    evidence::{MutationExecutionTranscript, MutationObservation},
    source_inventory::MutationSourceBinding,
};

pub(super) const C8_RETAINED_RECORD_SCHEMA: &str = "worth.store.c8-controlled-mutation-record.v1";
const C8_RETAINED_RECORD_ROLE: &str = "non-authoritative-live-execution-record";

#[derive(Serialize)]
struct BorrowedC8CampaignRecord<'evidence> {
    schema: &'static str,
    evidence_role: &'static str,
    source: &'evidence MutationSourceBinding,
    phase_eight_source_closure_sha256: &'evidence str,
    observations: &'evidence [MutationObservation],
}

#[cfg(feature = "physical-work-evidence")]
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct RetainedC8CampaignRecord {
    schema: String,
    evidence_role: String,
    source: MutationSourceBinding,
    phase_eight_source_closure_sha256: String,
    observations: Vec<MutationObservation>,
}

#[cfg(feature = "physical-work-evidence")]
impl RetainedC8CampaignRecord {
    #[cfg(test)]
    pub(crate) fn identities(&self) -> impl Iterator<Item = u8> + '_ {
        self.observations.iter().map(|observation| observation.id)
    }

    #[cfg(test)]
    pub(crate) fn observation_count(&self) -> usize {
        self.observations.len()
    }

    #[cfg(test)]
    pub(crate) fn source_closure_sha256(&self) -> &str {
        &self.phase_eight_source_closure_sha256
    }
}

pub(super) fn encode(
    source: &MutationSourceBinding,
    source_closure: &str,
    observations: &[MutationObservation],
) -> Result<Vec<u8>, String> {
    require_live_transcripts(observations)?;
    serde_json::to_vec_pretty(&BorrowedC8CampaignRecord {
        schema: C8_RETAINED_RECORD_SCHEMA,
        evidence_role: C8_RETAINED_RECORD_ROLE,
        source,
        phase_eight_source_closure_sha256: source_closure,
        observations,
    })
    .map_err(|error| format!("cannot encode retained C8 campaign record: {error}"))
}

#[cfg(feature = "physical-work-evidence")]
pub(super) fn load(
    report: &Path,
    workspace: &Path,
    expected: &[ControlledMutation],
) -> Result<RetainedC8CampaignRecord, String> {
    let bytes = std::fs::read(report).map_err(|error| {
        format!(
            "cannot read retained C8 record {}: {error}",
            report.display()
        )
    })?;
    let record: RetainedC8CampaignRecord = serde_json::from_slice(&bytes)
        .map_err(|error| format!("cannot decode retained C8 campaign record: {error}"))?;
    if record.schema != C8_RETAINED_RECORD_SCHEMA {
        return Err(format!("unsupported retained C8 schema {}", record.schema));
    }
    if record.evidence_role != C8_RETAINED_RECORD_ROLE {
        return Err("retained C8 record claimed an unsupported evidence role".into());
    }
    if record.source != source_inventory::bind(workspace)? {
        return Err("retained C8 campaign source is stale".into());
    }
    let current_closure = phase_eight_source_closure(workspace)?;
    if record.phase_eight_source_closure_sha256 != current_closure {
        return Err("retained C8 campaign source closure is stale".into());
    }
    validate_shape(&record.observations, expected)?;
    for (observation, mutation) in record.observations.iter().zip(expected) {
        validate_portable_observation(observation, mutation, workspace)?;
    }
    Ok(record)
}

pub(super) fn phase_eight_source_closure(workspace: &Path) -> Result<String, String> {
    let repository = workspace
        .parent()
        .and_then(Path::parent)
        .ok_or_else(|| "Store workspace omitted repository ancestors".to_owned())?;
    let ledger =
        repository.join("_docs/worth-store/physical-reconstruction-c8-phase-8-closure-ledger.md");
    let document = std::fs::read_to_string(&ledger)
        .map_err(|error| format!("cannot read Phase 8 ledger {}: {error}", ledger.display()))?;
    let digest = document
        .lines()
        .find_map(|line| line.strip_prefix("Source closure SHA-256: "))
        .ok_or_else(|| "Phase 8 ledger omitted its source-closure digest".to_owned())?;
    require_digest(digest, "Phase 8 source-closure")?;
    Ok(digest.to_owned())
}

fn require_live_transcripts(observations: &[MutationObservation]) -> Result<(), String> {
    for observation in observations {
        let transcript = observation.transcript.as_ref().ok_or_else(|| {
            format!(
                "mutant {} omitted its live execution transcript",
                observation.id
            )
        })?;
        validate_transcript(transcript, observation.id)?;
    }
    Ok(())
}

#[cfg(feature = "physical-work-evidence")]
fn validate_shape(
    observations: &[MutationObservation],
    expected: &[ControlledMutation],
) -> Result<(), String> {
    if observations.len() != expected.len() {
        return Err(format!(
            "retained C8 record requires {} observations, found {}",
            expected.len(),
            observations.len()
        ));
    }
    for (observation, mutation) in observations.iter().zip(expected) {
        if observation.id != mutation.id {
            return Err(format!(
                "retained C8 record expected mutant {}, found {}",
                mutation.id, observation.id
            ));
        }
    }
    Ok(())
}

#[cfg(feature = "physical-work-evidence")]
fn validate_portable_observation(
    observation: &MutationObservation,
    mutation: &ControlledMutation,
    workspace: &Path,
) -> Result<(), String> {
    if observation.source_binding != mutation.source
        || observation.scenario_binding != mutation.selector
        || observation.expected_failing_predicate != mutation.predicate
        || observation.actual_failing_predicate != mutation.predicate
    {
        return Err(format!("mutant {} retained binding changed", mutation.id));
    }
    let source = std::fs::read(workspace.join(mutation.source))
        .map_err(|error| format!("cannot read mutant {} source: {error}", mutation.id))?;
    if hash(&source) != observation.source_sha256 {
        return Err(format!("mutant {} retained source is stale", mutation.id));
    }
    let text = std::str::from_utf8(&source)
        .map_err(|_| format!("mutant {} source is not UTF-8", mutation.id))?;
    if mutation.source_occurrences(text) != 1 {
        return Err(format!(
            "mutant {} no longer binds exactly once",
            mutation.id
        ));
    }
    let mutant = text
        .replacen(
            mutation.source_needle(text).as_ref(),
            mutation.source_replacement(text).as_ref(),
            1,
        )
        .into_bytes();
    if hash(&mutant) != observation.mutant_sha256 {
        return Err(format!(
            "mutant {} retained mutant digest is forged",
            mutation.id
        ));
    }
    require_digest(&observation.binary_sha256, "ephemeral binary")?;
    validate_transcript(
        observation
            .transcript
            .as_ref()
            .ok_or_else(|| format!("mutant {} omitted its transcript", mutation.id))?,
        mutation.id,
    )
}

fn validate_transcript(transcript: &MutationExecutionTranscript, mutant: u8) -> Result<(), String> {
    if transcript.exit_code == Some(0) {
        return Err(format!("mutant {mutant} retained a successful exit status"));
    }
    require_digest(&transcript.stdout_sha256, "stdout")?;
    require_digest(&transcript.stderr_sha256, "stderr")?;
    if transcript.causal_lines.is_empty() || transcript.causal_lines.len() > 32 {
        return Err(format!(
            "mutant {mutant} retained invalid causal transcript bounds"
        ));
    }
    Ok(())
}

fn require_digest(value: &str, label: &str) -> Result<(), String> {
    if value.len() != 64 || !value.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(format!("{label} digest is not 64 hexadecimal characters"));
    }
    Ok(())
}

#[cfg(feature = "physical-work-evidence")]
fn hash(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

#[cfg(all(test, feature = "physical-work-evidence"))]
mod tests {
    use super::{encode, validate_shape, validate_transcript};
    use crate::mutation_campaign::{
        catalog,
        evidence::{
            MutationExecutionClass, MutationExecutionEvidence, MutationExecutionTranscript,
            MutationObservation,
        },
        source_inventory::MutationSourceBinding,
    };

    #[test]
    fn retained_record_requires_live_transcript_and_declares_no_authority() {
        let source = MutationSourceBinding {
            binding: "worth.store.controlled-mutation-source-closure.v3".into(),
            sha256: "11".repeat(32),
        };
        let mut observation = observation(134);
        observation.transcript = None;
        assert!(encode(&source, &"22".repeat(32), &[observation.clone()]).is_err());

        observation.transcript = Some(transcript(Some(1)));
        let encoded = encode(&source, &"22".repeat(32), &[observation]).unwrap();
        let value: serde_json::Value = serde_json::from_slice(&encoded).unwrap();
        assert_eq!(
            value["evidence_role"],
            "non-authoritative-live-execution-record"
        );
    }

    #[test]
    fn retained_record_rejects_successful_execution() {
        if validate_transcript(&transcript(Some(0)), 134).is_ok() {
            panic!("MUTANT_PREDICATE:c8-retained-success-status-accepted");
        }
        assert!(validate_transcript(&transcript(Some(1)), 134).is_ok());
    }

    #[test]
    fn retained_record_rejects_an_incomplete_c8_catalog() {
        let expected = catalog::c8_closure_mutations();
        let observations = expected[..expected.len() - 1]
            .iter()
            .map(|mutation| observation(mutation.id))
            .collect::<Vec<_>>();
        if validate_shape(&observations, expected).is_ok() {
            panic!("MUTANT_PREDICATE:c8-retained-incomplete-catalog-accepted");
        }
    }

    fn observation(id: u8) -> MutationObservation {
        MutationObservation {
            id,
            source_binding: "source.rs".into(),
            source_sha256: "22".repeat(32),
            mutant_sha256: "33".repeat(32),
            binary_binding: "ephemeral-test-binary".into(),
            binary_sha256: "44".repeat(32),
            profile_binding: "test".into(),
            scenario_binding: "scenario".into(),
            expected_failing_predicate: "predicate".into(),
            actual_failing_predicate: "predicate".into(),
            localization: "test.rs:1".into(),
            execution: MutationExecutionEvidence::bind(
                MutationExecutionClass::Ordinary,
                std::time::Duration::from_millis(1),
            )
            .unwrap(),
            transcript: Some(transcript(Some(1))),
        }
    }

    fn transcript(exit_code: Option<i32>) -> MutationExecutionTranscript {
        MutationExecutionTranscript {
            exit_code,
            stdout_sha256: "55".repeat(32),
            stdout_bytes: 1,
            stderr_sha256: "66".repeat(32),
            stderr_bytes: 1,
            causal_lines: vec!["MUTANT_PREDICATE:predicate".into()],
        }
    }
}
