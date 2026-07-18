use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

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

#[derive(Clone, Serialize, Deserialize)]
struct ProcessProbeEnvelope {
    schema_version: u32,
    declaration: DeclarationProjection,
    process: ProcessProjection,
    termination: TerminationProjection,
    output_artifact_identity: [u8; 32],
    evidence_identity: [u8; 32],
}

#[derive(Clone, Serialize, Deserialize)]
struct DeclarationProjection {
    scenario_identity: String,
    role: ProcessRoleProjection,
    isolation: ProcessIsolationProjection,
    required_termination: ProcessTerminationRequirementProjection,
    input_identity: [u8; 32],
    executable_identity: [u8; 32],
    working_directory: String,
    declaration_identity: [u8; 32],
}

#[derive(Clone, Serialize, Deserialize)]
struct ProcessProjection {
    role: ProcessRoleProjection,
    executable_identity: [u8; 32],
    process_id: u32,
    launch_parent_process_id: u32,
    working_directory: String,
    working_directory_identity: [u8; 32],
    environment: Vec<EnvironmentBindingProjection>,
    environment_identity: [u8; 32],
    input_artifact_identity: [u8; 32],
    runtime_identity: Option<[u8; 32]>,
}

#[derive(Clone, Serialize, Deserialize)]
struct EnvironmentBindingProjection {
    name: String,
    value_sha256: [u8; 32],
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "mode", rename_all = "snake_case")]
enum TerminationProjection {
    GracefulExit { code: Option<i32> },
    PanicUnwind { code: Option<i32> },
    Abort { platform_status: String },
    ParentKill { platform_status: String },
    OsTermination { platform_status: String },
}

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
enum ProcessRoleProjection {
    Writer,
    CrashTarget,
    RecoveredRuntime,
    OfflineVerifier,
    FormalCheckerAdapter,
    AllocatorIsolatedProbe,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum ProcessIsolationProjection {
    FreshProcess,
    ParentTerminated,
    IndependentObserver,
    IsolatedAllocator,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum ProcessTerminationRequirementProjection {
    GracefulExit,
    PanicUnwind,
    Abort,
    ParentKill,
    OsTermination,
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
        if path.extension().and_then(|value| value.to_str()) != Some("json")
            || evidence.schema_version != 1
            || evidence.declaration.role != evidence.process.role
            || evidence.declaration.executable_identity != evidence.process.executable_identity
            || evidence.declaration.input_identity != evidence.process.input_artifact_identity
            || evidence.declaration.working_directory != evidence.process.working_directory
            || evidence.process.process_id == 0
            || evidence.process.launch_parent_process_id == 0
            || evidence.process.process_id == evidence.process.launch_parent_process_id
            || evidence.declaration.input_identity == [0; 32]
            || evidence.declaration.executable_identity == [0; 32]
            || evidence.declaration.declaration_identity == [0; 32]
            || evidence.process.working_directory_identity
                != Sha256::digest(evidence.process.working_directory.as_bytes()).into()
            || !environment_is_admitted(&evidence.process.environment)
            || environment_identity(&evidence.process.environment)
                != evidence.process.environment_identity
            || !termination_matches(
                &evidence.declaration.required_termination,
                &evidence.termination,
            )
            || !declaration_identity_matches(&evidence.declaration)
            || !execution_identity_matches(&evidence)
            || evidence.output_artifact_identity == [0; 32]
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
            role: role_token(evidence.declaration.role).to_owned(),
            process_id: evidence.process.process_id,
            executable_identity: hex(&evidence.process.executable_identity),
            termination_mode: termination_mode(&evidence.termination).to_owned(),
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

fn declaration_identity_matches(declaration: &DeclarationProjection) -> bool {
    serde_json::to_vec(&(
        "worth-store-process-probe-declaration-v1",
        &declaration.scenario_identity,
        &declaration.role,
        &declaration.isolation,
        &declaration.required_termination,
        declaration.input_identity,
        declaration.executable_identity,
        &declaration.working_directory,
    ))
    .is_ok_and(|bytes| {
        let identity: [u8; 32] = Sha256::digest(bytes).into();
        identity == declaration.declaration_identity
    })
}

fn execution_identity_matches(evidence: &ProcessProbeEnvelope) -> bool {
    let claimed = evidence.evidence_identity;
    let mut unsigned = evidence.clone();
    unsigned.evidence_identity = [0; 32];
    serde_json::to_vec(&unsigned).is_ok_and(|bytes| {
        let identity: [u8; 32] = Sha256::digest(bytes).into();
        identity == claimed
    })
}

fn environment_identity(environment: &[EnvironmentBindingProjection]) -> [u8; 32] {
    serde_json::to_vec(environment)
        .map(|bytes| Sha256::digest(bytes).into())
        .unwrap_or([0; 32])
}

fn environment_is_admitted(environment: &[EnvironmentBindingProjection]) -> bool {
    !environment.is_empty()
        && environment.iter().all(|binding| {
            binding.name.starts_with("WORTH_STORE_") && binding.value_sha256 != [0; 32]
        })
        && environment
            .windows(2)
            .all(|pair| pair[0].name < pair[1].name)
}

fn termination_matches(
    required: &ProcessTerminationRequirementProjection,
    observed: &TerminationProjection,
) -> bool {
    matches!(
        (required, observed),
        (
            ProcessTerminationRequirementProjection::GracefulExit,
            TerminationProjection::GracefulExit { .. }
        ) | (
            ProcessTerminationRequirementProjection::PanicUnwind,
            TerminationProjection::PanicUnwind { .. }
        ) | (
            ProcessTerminationRequirementProjection::Abort,
            TerminationProjection::Abort { .. }
        ) | (
            ProcessTerminationRequirementProjection::ParentKill,
            TerminationProjection::ParentKill { .. }
        ) | (
            ProcessTerminationRequirementProjection::OsTermination,
            TerminationProjection::OsTermination { .. }
        )
    )
}

const fn role_token(role: ProcessRoleProjection) -> &'static str {
    match role {
        ProcessRoleProjection::Writer => "writer",
        ProcessRoleProjection::CrashTarget => "crash_target",
        ProcessRoleProjection::RecoveredRuntime => "recovered_runtime",
        ProcessRoleProjection::OfflineVerifier => "offline_verifier",
        ProcessRoleProjection::FormalCheckerAdapter => "formal_checker_adapter",
        ProcessRoleProjection::AllocatorIsolatedProbe => "allocator_isolated_probe",
    }
}

const fn termination_mode(termination: &TerminationProjection) -> &'static str {
    match termination {
        TerminationProjection::GracefulExit { .. } => "graceful_exit",
        TerminationProjection::PanicUnwind { .. } => "panic_unwind",
        TerminationProjection::Abort { .. } => "abort",
        TerminationProjection::ParentKill { .. } => "parent_kill",
        TerminationProjection::OsTermination { .. } => "os_termination",
    }
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
