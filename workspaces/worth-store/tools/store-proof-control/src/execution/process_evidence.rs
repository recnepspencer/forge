use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

pub(super) const PROCESS_PROBE_EVIDENCE_ROOT_ENV: &str =
    "WORTH_STORE_PROCESS_PROBE_EVIDENCE_ROOT";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProcessProbeEvidenceReference {
    pub unit_identity: String,
    pub scenario_identity: String,
    pub role: String,
    pub process_id: u32,
    pub executable_identity: String,
    pub termination_mode: String,
    pub evidence_identity: String,
    pub evidence_path: String,
}

#[derive(Deserialize)]
struct ProcessProbeEnvelope {
    schema_version: u32,
    declaration: DeclarationProjection,
    process: ProcessProjection,
    termination: TerminationProjection,
    evidence_identity: [u8; 32],
}

#[derive(Deserialize)]
struct DeclarationProjection {
    scenario_identity: String,
    role: String,
    executable_identity: [u8; 32],
}

#[derive(Deserialize)]
struct ProcessProjection {
    role: String,
    executable_identity: [u8; 32],
    process_id: u32,
}

#[derive(Deserialize)]
struct TerminationProjection {
    mode: String,
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
        .join("process-probes")
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
) -> Result<Vec<ProcessProbeEvidenceReference>, String> {
    let mut paths = if evidence_root.exists() {
        fs::read_dir(evidence_root)
            .map_err(|error| format!("could not inspect {}: {error}", evidence_root.display()))?
            .map(|entry| entry.map(|entry| entry.path()))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| format!("could not inspect {}: {error}", evidence_root.display()))?
    } else {
        Vec::new()
    };
    paths.sort();
    let mut references = Vec::with_capacity(paths.len());
    for path in paths {
        let bytes = fs::read(&path)
            .map_err(|error| format!("could not read {}: {error}", path.display()))?;
        let evidence: ProcessProbeEnvelope = serde_json::from_slice(&bytes)
            .map_err(|error| format!("could not decode {}: {error}", path.display()))?;
        let evidence_identity = hex(&evidence.evidence_identity);
        if evidence.schema_version != 1
            || evidence.declaration.role != evidence.process.role
            || evidence.declaration.executable_identity != evidence.process.executable_identity
            || evidence.process.process_id == 0
            || path.file_stem().and_then(|value| value.to_str())
                != Some(evidence_identity.as_str())
        {
            return Err(format!(
                "process probe artifact {} has inconsistent identity fields",
                path.display()
            ));
        }
        references.push(ProcessProbeEvidenceReference {
            unit_identity: unit_identity.to_owned(),
            scenario_identity: evidence.declaration.scenario_identity,
            role: evidence.declaration.role,
            process_id: evidence.process.process_id,
            executable_identity: hex(&evidence.process.executable_identity),
            termination_mode: evidence.termination.mode,
            evidence_identity,
            evidence_path: normalized_path(workspace_root, &path),
        });
    }
    if evidence_required && references.is_empty() {
        return Err(format!(
            "fresh-process unit {unit_identity} passed without ProcessProbeExecution evidence"
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

fn hex(bytes: &[u8; 32]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}
