use std::path::{Path, PathBuf};

use serde::Deserialize;
use worth_store::physical_runtime::PhysicalWorkMutantLocalization;

use super::{
    hash_file, published_artifact_directory, validate_owned_artifacts,
    MUTATION_EVIDENCE_REPORT_SCHEMA,
};
use crate::mutation_campaign::{
    catalog::{physical_work_mutations, ControlledMutation},
    evidence::{self, MutationObservation},
    source_inventory::{self, MutationSourceBinding},
};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct PublishedMutationEvidenceReport {
    schema: String,
    source: MutationSourceBinding,
    observations: Vec<MutationObservation>,
}

#[derive(Deserialize)]
struct PublishedMutationEvidenceReportHeader {
    schema: String,
}

pub(super) fn load_physical_work_evidence(
    report: &Path,
    workspace: &Path,
) -> Result<Vec<PhysicalWorkMutantLocalization>, String> {
    let report = report.canonicalize().map_err(|error| {
        format!(
            "cannot locate mutation report {}: {error}",
            report.display()
        )
    })?;
    let bytes = std::fs::read(&report)
        .map_err(|error| format!("cannot read mutation report {}: {error}", report.display()))?;
    let header: PublishedMutationEvidenceReportHeader = serde_json::from_slice(&bytes)
        .map_err(|error| format!("cannot decode mutation report header: {error}"))?;
    if header.schema != MUTATION_EVIDENCE_REPORT_SCHEMA {
        return Err(format!(
            "unsupported mutation report schema `{}`",
            header.schema
        ));
    }
    let published: PublishedMutationEvidenceReport = serde_json::from_slice(&bytes)
        .map_err(|error| format!("cannot decode mutation report: {error}"))?;
    debug_assert_eq!(published.schema, MUTATION_EVIDENCE_REPORT_SCHEMA);
    validate_shape(&published.observations)?;
    validate_campaign_source(&published.source, workspace)?;
    let artifacts = PublishedArtifactPolicy::for_report(&report)?;
    published
        .observations
        .into_iter()
        .zip(physical_work_mutations())
        .map(|(observation, expected)| {
            validate_observation(observation, expected, workspace, &artifacts)
        })
        .collect()
}

fn validate_campaign_source(
    expected: &MutationSourceBinding,
    workspace: &Path,
) -> Result<(), String> {
    let current = source_inventory::bind(workspace)?;
    require_campaign_source(expected, &current)
}

fn require_campaign_source(
    expected: &MutationSourceBinding,
    current: &MutationSourceBinding,
) -> Result<(), String> {
    if expected != current {
        return Err("mutation campaign source is stale".into());
    }
    Ok(())
}

fn validate_shape(observations: &[MutationObservation]) -> Result<(), String> {
    let expected = physical_work_mutations();
    if observations.len() != expected.len() {
        return Err(format!(
            "physical-work mutation report requires {} observations, found {}",
            expected.len(),
            observations.len()
        ));
    }
    for (observation, mutation) in observations.iter().zip(expected) {
        if observation.id != mutation.id {
            return Err(format!(
                "physical-work mutation report expected mutant {}, found {}",
                mutation.id, observation.id
            ));
        }
    }
    Ok(())
}

fn validate_observation(
    mut observation: MutationObservation,
    expected: &ControlledMutation,
    workspace: &Path,
    artifacts: &PublishedArtifactPolicy,
) -> Result<PhysicalWorkMutantLocalization, String> {
    validate_declared_binding(&observation, expected)?;
    let source = workspace.join(expected.source);
    let source_text = std::fs::read_to_string(&source)
        .map_err(|error| format!("cannot read mutant {} source: {error}", expected.id))?;
    if expected.source_occurrences(&source_text) != 1 {
        return Err(format!(
            "mutant {} no longer binds exactly once in current source",
            expected.id
        ));
    }
    if hash_file(&source)? != observation.source_sha256 {
        return Err(format!("mutant {} source is stale", expected.id));
    }
    if observation.source_sha256 == observation.mutant_sha256 {
        return Err(format!("mutant {} made no source change", expected.id));
    }
    let binary = artifacts.resolve(&observation.binary_binding)?;
    if hash_file(&binary)? != observation.binary_sha256 {
        return Err(format!("mutant {} binary is stale", expected.id));
    }
    observation.binary_binding = binary.display().to_string();
    let encoded = evidence::encode(&observation)?;
    evidence::decode_physical_work_localization(&encoded)
}

fn validate_declared_binding(
    observation: &MutationObservation,
    expected: &ControlledMutation,
) -> Result<(), String> {
    if observation.source_binding != expected.source {
        return Err(format!("mutant {} source binding changed", expected.id));
    }
    if observation.expected_failing_predicate != expected.predicate
        || observation.actual_failing_predicate != expected.predicate
    {
        return Err(format!("mutant {} predicate binding changed", expected.id));
    }
    if observation.profile_binding != "test" || observation.scenario_binding != expected.selector {
        return Err(format!("mutant {} execution binding changed", expected.id));
    }
    Ok(())
}

struct PublishedArtifactPolicy {
    directory: PathBuf,
}

impl PublishedArtifactPolicy {
    fn for_report(report: &Path) -> Result<Self, String> {
        let directory = published_artifact_directory(report)?;
        validate_owned_artifacts(&directory, report)?;
        let directory = directory.canonicalize().map_err(|error| {
            format!(
                "cannot canonicalize mutation artifact directory {}: {error}",
                directory.display()
            )
        })?;
        Ok(Self { directory })
    }

    fn resolve(&self, binding: &str) -> Result<PathBuf, String> {
        let claimed = PathBuf::from(binding);
        let resolved = if claimed.is_absolute() {
            claimed
        } else {
            self.directory.join(claimed)
        };
        let canonical = resolved.canonicalize().map_err(|error| {
            format!(
                "cannot locate retained mutation binary {}: {error}",
                resolved.display()
            )
        })?;
        if canonical.parent() != Some(self.directory.as_path()) || !canonical.is_file() {
            return Err(format!(
                "mutation binary {} escaped its published artifact set",
                canonical.display()
            ));
        }
        Ok(canonical)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        load_physical_work_evidence, require_campaign_source, MutationSourceBinding,
        PublishedArtifactPolicy,
    };

    #[test]
    fn report_requires_the_complete_physical_work_campaign() {
        let temporary = tempfile::tempdir().unwrap();
        let report = temporary.path().join("mutants.json");
        std::fs::write(
            &report,
            br#"{"schema":"worth.store.c5_1.mutation-evidence.v2","source":{"binding":"worth.store.c5_1.mutation-source-closure.v1","sha256":"1111111111111111111111111111111111111111111111111111111111111111"},"observations":[]}"#,
        )
        .unwrap();
        let error = load_physical_work_evidence(&report, temporary.path()).unwrap_err();
        assert!(error.contains("requires 29 observations"), "{error}");
    }

    #[test]
    fn legacy_schema_is_classified_before_v2_body_decoding() {
        let temporary = tempfile::tempdir().unwrap();
        let report = temporary.path().join("mutants.json");
        std::fs::write(
            &report,
            br#"{"schema":"worth.store.c5_1.mutation-evidence.v1","observations":[]}"#,
        )
        .unwrap();

        let error = load_physical_work_evidence(&report, temporary.path()).unwrap_err();

        assert!(
            error.contains("unsupported mutation report schema"),
            "{error}"
        );
        assert!(!error.contains("missing field"), "{error}");
    }

    #[test]
    fn artifact_policy_rejects_a_binary_outside_the_owned_report_directory() {
        let temporary = tempfile::tempdir().unwrap();
        let report = temporary.path().join("mutants.json");
        std::fs::write(&report, b"report").unwrap();
        let directory = temporary.path().join("mutants.json.artifacts.current");
        std::fs::create_dir(&directory).unwrap();
        std::fs::write(
            directory.join(".worth-store-mutation-evidence-owner"),
            format!(
                "{}\n{}\n",
                super::super::ARTIFACT_OWNER_SCHEMA,
                report.canonicalize().unwrap().display()
            ),
        )
        .unwrap();
        let outside = temporary.path().join("outside.exe");
        std::fs::write(&outside, b"outside").unwrap();
        let policy = PublishedArtifactPolicy::for_report(&report.canonicalize().unwrap()).unwrap();
        assert!(policy.resolve(outside.to_str().unwrap()).is_err());
    }

    #[test]
    fn report_reader_rejects_same_length_source_closure_drift() {
        let expected = MutationSourceBinding {
            binding: "worth.store.c5_1.mutation-source-closure.v1".into(),
            sha256: "11".repeat(32),
        };
        let current = MutationSourceBinding {
            binding: expected.binding.clone(),
            sha256: "22".repeat(32),
        };

        assert_eq!(
            require_campaign_source(&expected, &current).unwrap_err(),
            "mutation campaign source is stale"
        );
    }
}
