use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use worth_store_test_support::compiler_boundary::UiProofRunEvidence;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UiProofEvidenceReference {
    pub unit_identity: String,
    pub suite_identity: String,
    pub evidence_identity: String,
    pub environment_identity: String,
    pub fixture_count: usize,
    pub evidence_path: String,
}

pub(super) fn attempt_root(
    workspace_root: &Path,
    attempt_identity: &str,
    unit_index: usize,
    unit_identity: &str,
) -> PathBuf {
    workspace_root
        .join(".store-proof/evidence/runs")
        .join(attempt_identity)
        .join("ui")
        .join(format!(
            "{unit_index:04}-{}",
            filesystem_identity(unit_identity)
        ))
}

pub(super) fn collect(
    workspace_root: &Path,
    evidence_root: &Path,
    unit_identity: &str,
    evidence_required: bool,
) -> Result<Vec<UiProofEvidenceReference>, String> {
    let run_root = evidence_root.join("runs");
    let mut paths = if run_root.exists() {
        fs::read_dir(&run_root)
            .map_err(|error| format!("could not inspect {}: {error}", run_root.display()))?
            .map(|entry| entry.map(|entry| entry.path()))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| format!("could not inspect {}: {error}", run_root.display()))?
    } else {
        Vec::new()
    };
    paths.sort();
    let mut references = Vec::with_capacity(paths.len());
    for path in paths {
        if path.extension().and_then(|value| value.to_str()) != Some("json") {
            return Err(format!(
                "UI evidence root contains undeclared artifact {}",
                path.display()
            ));
        }
        let bytes = fs::read(&path)
            .map_err(|error| format!("could not read {}: {error}", path.display()))?;
        let evidence: UiProofRunEvidence = serde_json::from_slice(&bytes)
            .map_err(|error| format!("could not decode {}: {error}", path.display()))?;
        if evidence.fixtures.is_empty()
            || evidence
                .fixtures
                .iter()
                .any(|fixture| !fixture.semantic_denial_matched)
        {
            return Err(format!(
                "UI evidence {} does not carry checked semantic denials",
                evidence.evidence_identity
            ));
        }
        references.push(UiProofEvidenceReference {
            unit_identity: unit_identity.to_owned(),
            suite_identity: evidence.suite_identity,
            evidence_identity: evidence.evidence_identity,
            environment_identity: evidence.environment_identity,
            fixture_count: evidence.fixtures.len(),
            evidence_path: normalized_path(workspace_root, &path),
        });
    }
    if evidence_required && references.is_empty() {
        return Err(format!(
            "compiler-boundary unit {unit_identity} passed without UiProofRunEvidence"
        ));
    }
    Ok(references)
}

fn filesystem_identity(value: &str) -> String {
    value
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() {
                character.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect::<String>()
        .trim_matches('-')
        .to_owned()
}

fn normalized_path(workspace_root: &Path, path: &Path) -> String {
    path.strip_prefix(workspace_root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}
