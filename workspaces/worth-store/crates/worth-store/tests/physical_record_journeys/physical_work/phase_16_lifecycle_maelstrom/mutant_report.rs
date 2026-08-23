use std::path::{Path, PathBuf};

use serde::Deserialize;
use sha2::{Digest, Sha256};
use worth_store::physical_runtime::{
    PhysicalWorkEvidenceBindingDenial, PhysicalWorkEvidenceDigest, PhysicalWorkMutantBinding,
    PhysicalWorkMutantExecutionContext, PhysicalWorkMutantLocalization, PhysicalWorkMutantOutcome,
    PhysicalWorkMutantSubject, PhysicalWorkSourceBinding,
};

mod campaign_source;
mod decoding;
mod expectation;

const REPORT_ENV: &str = "WORTH_STORE_C5_1_MUTANT_REPORT";
const REPORT_SCHEMA: &str = "worth.store.controlled-mutation-evidence.v5";
const ARTIFACT_OWNER_SCHEMA: &str = "worth.store.c5_1.mutation-artifacts.v1";
const ARTIFACT_OWNER_MARKER: &str = ".worth-store-mutation-evidence-owner";
const FIRST_MUTANT: u8 = 15;
const LAST_MUTANT: u8 = 44;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct MutationReport {
    schema: String,
    scope: MutationReportScope,
    source: campaign_source::MutationSourceBinding,
    observations: Vec<MutationObservation>,
}

#[derive(Clone, Copy, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
enum MutationReportScope {
    PhysicalWork,
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct MutationObservation {
    id: u8,
    source_binding: String,
    source_sha256: String,
    mutant_sha256: String,
    binary_binding: String,
    binary_sha256: String,
    profile_binding: String,
    scenario_binding: String,
    expected_failing_predicate: String,
    actual_failing_predicate: String,
    localization: String,
    execution: MutationExecutionEvidence,
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct MutationExecutionEvidence {
    class: MutationExecutionClass,
    elapsed_ms: u64,
    budget_ms: u64,
}

#[derive(Clone, Copy, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
enum MutationExecutionClass {
    Ordinary,
    NestedExecutableCold,
}

struct ArtifactPolicy {
    report: PathBuf,
    parent: PathBuf,
    directory_prefix: String,
}

pub(super) fn load() -> Option<Vec<PhysicalWorkMutantLocalization>> {
    let path = PathBuf::from(std::env::var_os(REPORT_ENV)?);
    Some(
        decode_file(&path, &workspace_root())
            .unwrap_or_else(|error| panic!("invalid {REPORT_ENV} evidence: {error}")),
    )
}

fn decode_file(
    path: &Path,
    workspace: &Path,
) -> Result<Vec<PhysicalWorkMutantLocalization>, String> {
    let canonical = path
        .canonicalize()
        .map_err(|error| format!("cannot locate report {}: {error}", path.display()))?;
    let bytes = std::fs::read(&canonical)
        .map_err(|error| format!("cannot read report {}: {error}", canonical.display()))?;
    decoding::require_supported_schema(&bytes, REPORT_SCHEMA)?;
    let report: MutationReport = serde_json::from_slice(&bytes)
        .map_err(|error| format!("cannot decode mutation report: {error}"))?;
    if report.scope != MutationReportScope::PhysicalWork {
        return Err("mutation report scope does not satisfy Courtroom A".into());
    }
    debug_assert_eq!(report.schema, REPORT_SCHEMA);
    validate_campaign_shape(&report.observations)?;
    campaign_source::validate(&report.source, workspace)?;
    let policy = ArtifactPolicy::for_report(&canonical)?;
    let mut localizations = Vec::with_capacity(complete_mutant_count());
    for observation in report.observations {
        let id = observation.id;
        let expected = expectation::for_id(id);
        let localization = validate_observation(observation, expected, workspace, &policy)?;
        localizations.push(localization);
    }
    Ok(localizations)
}

fn validate_campaign_shape(observations: &[MutationObservation]) -> Result<(), String> {
    let expected_count = complete_mutant_count();
    if observations.len() != expected_count {
        return Err(format!(
            "mutation report requires {expected_count} observations, found {}",
            observations.len()
        ));
    }
    for (expected, observation) in (FIRST_MUTANT..=LAST_MUTANT).zip(observations) {
        if observation.id != expected {
            return Err(format!(
                "mutation report expected mutant {expected}, found {}",
                observation.id
            ));
        }
    }
    Ok(())
}

pub(super) fn complete_mutant_count() -> usize {
    usize::from(LAST_MUTANT - FIRST_MUTANT + 1)
}

fn validate_observation(
    observation: MutationObservation,
    expected: expectation::MutantExpectation,
    workspace: &Path,
    artifacts: &ArtifactPolicy,
) -> Result<PhysicalWorkMutantLocalization, String> {
    if observation.source_binding != expected.source {
        return Err(format!("mutant {} source binding changed", observation.id));
    }
    if observation.expected_failing_predicate != expected.predicate
        || observation.actual_failing_predicate != expected.predicate
    {
        return Err(format!(
            "mutant {} predicate binding changed",
            observation.id
        ));
    }
    if observation.profile_binding != "test" || observation.scenario_binding != expected.scenario {
        return Err(format!(
            "mutant {} execution binding changed",
            observation.id
        ));
    }
    if observation.execution.budget_ms
        != match observation.execution.class {
            MutationExecutionClass::Ordinary => 180_000,
            MutationExecutionClass::NestedExecutableCold => 300_000,
        }
        || observation.execution.elapsed_ms > observation.execution.budget_ms
    {
        return Err(format!(
            "mutant {} execution evidence violates its cost budget",
            observation.id
        ));
    }
    let source_digest = parse_digest(&observation.source_sha256)?;
    let current_source = hash_file(&workspace.join(expected.source))?;
    if source_digest != current_source {
        return Err(format!("mutant {} source is stale", observation.id));
    }
    let mutant_digest = parse_digest(&observation.mutant_sha256)?;
    if mutant_digest == source_digest {
        return Err(format!("mutant {} made no source change", observation.id));
    }
    let binary_path = artifacts.resolve(&observation.binary_binding)?;
    let binary_digest = parse_digest(&observation.binary_sha256)?;
    if hash_file(&binary_path)? != binary_digest {
        return Err(format!("mutant {} binary is stale", observation.id));
    }
    bind_localization(
        observation,
        expected,
        source_digest,
        mutant_digest,
        binary_path,
        binary_digest,
    )
}

fn bind_localization(
    observation: MutationObservation,
    expected: expectation::MutantExpectation,
    source_digest: PhysicalWorkEvidenceDigest,
    mutant_digest: PhysicalWorkEvidenceDigest,
    binary_path: PathBuf,
    binary_digest: PhysicalWorkEvidenceDigest,
) -> Result<PhysicalWorkMutantLocalization, String> {
    let subject = PhysicalWorkMutantSubject::new(
        u16::from(observation.id),
        expected.predicate,
        expected.source,
    )
    .map_err(binding_denial)?;
    let execution =
        PhysicalWorkMutantExecutionContext::new(observation.profile_binding, expected.scenario)
            .map_err(binding_denial)?;
    let binary = PhysicalWorkSourceBinding::new(binary_path.display().to_string(), binary_digest)
        .map_err(binding_denial)?;
    let binding =
        PhysicalWorkMutantBinding::new(subject, source_digest, mutant_digest, binary, execution);
    PhysicalWorkMutantLocalization::new(
        binding,
        PhysicalWorkMutantOutcome::new(true, observation.localization),
    )
    .map_err(binding_denial)
}

impl ArtifactPolicy {
    fn for_report(report: &Path) -> Result<Self, String> {
        let parent = report
            .parent()
            .ok_or_else(|| "mutation report has no parent".to_owned())?
            .canonicalize()
            .map_err(|error| format!("cannot canonicalize mutation report parent: {error}"))?;
        let name = report
            .file_name()
            .and_then(|value| value.to_str())
            .ok_or_else(|| "mutation report filename must be Unicode".to_owned())?;
        Ok(Self {
            report: report.to_path_buf(),
            parent,
            directory_prefix: format!("{name}.artifacts."),
        })
    }

    fn resolve(&self, binding: &str) -> Result<PathBuf, String> {
        let claimed = PathBuf::from(binding);
        let resolved = if claimed.is_absolute() {
            claimed
        } else {
            self.parent.join(claimed)
        };
        let canonical = resolved.canonicalize().map_err(|error| {
            format!(
                "cannot locate mutant binary {}: {error}",
                resolved.display()
            )
        })?;
        let directory = canonical
            .parent()
            .ok_or_else(|| "mutant binary has no parent".to_owned())?;
        let directory_name = directory
            .file_name()
            .and_then(|value| value.to_str())
            .unwrap_or_default();
        if !directory.starts_with(&self.parent)
            || !directory_name.starts_with(&self.directory_prefix)
            || !canonical.is_file()
        {
            return Err("mutant binary escaped its report artifact directory".into());
        }
        let marker = std::fs::read_to_string(directory.join(ARTIFACT_OWNER_MARKER))
            .map_err(|_| "mutant binary artifact directory omitted its owner marker".to_owned())?;
        let expected = format!("{ARTIFACT_OWNER_SCHEMA}\n{}\n", self.report.display());
        if marker != expected {
            return Err("mutant binary artifact directory has foreign ownership".into());
        }
        Ok(canonical)
    }
}

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .unwrap()
        .to_path_buf()
}

fn hash_file(path: &Path) -> Result<PhysicalWorkEvidenceDigest, String> {
    let bytes =
        std::fs::read(path).map_err(|error| format!("cannot hash {}: {error}", path.display()))?;
    PhysicalWorkEvidenceDigest::new(Sha256::digest(bytes).into())
        .ok_or_else(|| format!("{} has an all-zero digest", path.display()))
}

fn parse_digest(encoded: &str) -> Result<PhysicalWorkEvidenceDigest, String> {
    if encoded.len() != 64 || !encoded.is_ascii() {
        return Err("mutation digest must be 64 hexadecimal characters".into());
    }
    let mut bytes = [0_u8; 32];
    for (index, slot) in bytes.iter_mut().enumerate() {
        let offset = index * 2;
        *slot = u8::from_str_radix(&encoded[offset..offset + 2], 16)
            .map_err(|_| "mutation digest contains non-hexadecimal data".to_owned())?;
    }
    PhysicalWorkEvidenceDigest::new(bytes)
        .ok_or_else(|| "mutation digest cannot be all zero".to_owned())
}

fn binding_denial(denial: PhysicalWorkEvidenceBindingDenial) -> String {
    format!("mutation evidence binding denied: {denial:?}")
}

#[cfg(test)]
mod tests;
