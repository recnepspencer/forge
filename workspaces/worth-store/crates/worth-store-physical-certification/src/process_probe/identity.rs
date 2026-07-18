use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use super::{ProcessProbeDeclaration, ProcessProbeEvidenceDenial, ProcessRole};
use crate::certification_child_process::{decode_hex_32, encode_hex_32};

const ROLE_ENV: &str = "WORTH_STORE_PROCESS_PROBE_ROLE";
const PARENT_ENV: &str = "WORTH_STORE_PROCESS_PROBE_PARENT_PID";
const INPUT_ENV: &str = "WORTH_STORE_PROCESS_PROBE_INPUT_IDENTITY";
const OBSERVATION_ENV: &str = "WORTH_STORE_PROCESS_PROBE_OBSERVATION";
const ENVIRONMENT_KEYS_ENV: &str = "WORTH_STORE_PROCESS_PROBE_ENVIRONMENT_KEYS";
const ENVIRONMENT_IDENTITY_ENV: &str = "WORTH_STORE_PROCESS_PROBE_ENVIRONMENT_IDENTITY";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProcessEnvironmentBindingEvidence {
    pub name: String,
    pub value_sha256: [u8; 32],
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProcessIdentityEvidence {
    pub role: ProcessRole,
    pub executable_identity: [u8; 32],
    pub process_id: u32,
    pub launch_parent_process_id: u32,
    pub working_directory: String,
    pub working_directory_identity: [u8; 32],
    pub environment: Vec<ProcessEnvironmentBindingEvidence>,
    pub environment_identity: [u8; 32],
    pub input_artifact_identity: [u8; 32],
    pub runtime_identity: Option<[u8; 32]>,
}

pub(crate) fn configure_process_probe(
    command: &mut Command,
    declaration: &ProcessProbeDeclaration,
    observation_path: &Path,
    additional_environment_keys: &[&str],
) -> Result<(), ProcessProbeEvidenceDenial> {
    let mut keys = vec![ROLE_ENV, PARENT_ENV, INPUT_ENV, OBSERVATION_ENV];
    keys.extend(additional_environment_keys.iter().copied());
    keys.sort_unstable();
    keys.dedup();
    if keys
        .iter()
        .any(|key| !key.starts_with("WORTH_STORE_") || key.contains(';'))
    {
        return Err(ProcessProbeEvidenceDenial::UnadmittedEnvironment);
    }
    command
        .env(ROLE_ENV, role_token(declaration.role()))
        .env(PARENT_ENV, std::process::id().to_string())
        .env(INPUT_ENV, encode_hex_32(&declaration.input_identity()))
        .env(OBSERVATION_ENV, observation_path)
        .env(ENVIRONMENT_KEYS_ENV, keys.join(";"));
    let bindings = command_bindings(command, &keys)?;
    let identity = environment_identity(&bindings)?;
    command.env(ENVIRONMENT_IDENTITY_ENV, encode_hex_32(&identity));
    Ok(())
}

pub(crate) fn write_current_process_observation(
    expected_role: ProcessRole,
    runtime_identity: Option<[u8; 32]>,
) -> Result<(), ProcessProbeEvidenceDenial> {
    let observation_path = required_path(OBSERVATION_ENV)?;
    let role = parse_role(&required_string(ROLE_ENV)?)
        .ok_or(ProcessProbeEvidenceDenial::InvalidChildObservation)?;
    if role != expected_role {
        return Err(ProcessProbeEvidenceDenial::RoleMismatch);
    }
    let launch_parent_process_id = required_string(PARENT_ENV)?
        .parse()
        .map_err(|_| ProcessProbeEvidenceDenial::InvalidChildObservation)?;
    let input_artifact_identity = decode_hex_32(&required_string(INPUT_ENV)?)
        .ok_or(ProcessProbeEvidenceDenial::InvalidChildObservation)?;
    let declared_environment_identity = decode_hex_32(&required_string(ENVIRONMENT_IDENTITY_ENV)?)
        .ok_or(ProcessProbeEvidenceDenial::InvalidChildObservation)?;
    let keys = required_string(ENVIRONMENT_KEYS_ENV)?
        .split(';')
        .map(str::to_owned)
        .collect::<Vec<_>>();
    let environment = current_bindings(&keys)?;
    let environment_identity = environment_identity(&environment)?;
    if environment_identity != declared_environment_identity {
        return Err(ProcessProbeEvidenceDenial::EnvironmentMismatch);
    }
    let executable = std::env::current_exe()
        .and_then(fs::canonicalize)
        .map_err(|_| ProcessProbeEvidenceDenial::InvalidChildObservation)?;
    let executable_identity = file_digest(&executable)?;
    let working_directory = std::env::current_dir()
        .and_then(fs::canonicalize)
        .map_err(|_| ProcessProbeEvidenceDenial::InvalidChildObservation)?
        .to_string_lossy()
        .replace('\\', "/");
    let evidence = ProcessIdentityEvidence {
        role,
        executable_identity,
        process_id: std::process::id(),
        launch_parent_process_id,
        working_directory_identity: Sha256::digest(working_directory.as_bytes()).into(),
        working_directory,
        environment,
        environment_identity,
        input_artifact_identity,
        runtime_identity,
    };
    write_new_json(&observation_path, &evidence)
}

pub(crate) fn read_process_observation(
    path: &Path,
    declaration: &ProcessProbeDeclaration,
    spawned_process_id: u32,
) -> Result<ProcessIdentityEvidence, ProcessProbeEvidenceDenial> {
    let bytes = fs::read(path).map_err(|_| ProcessProbeEvidenceDenial::MissingChildObservation)?;
    let evidence: ProcessIdentityEvidence = serde_json::from_slice(&bytes)
        .map_err(|_| ProcessProbeEvidenceDenial::InvalidChildObservation)?;
    if evidence.role != declaration.role() {
        return Err(ProcessProbeEvidenceDenial::RoleMismatch);
    }
    if evidence.process_id != spawned_process_id
        || evidence.process_id == std::process::id()
        || evidence.launch_parent_process_id != std::process::id()
    {
        return Err(ProcessProbeEvidenceDenial::ProcessRelationshipMismatch);
    }
    if evidence.executable_identity != declaration.executable_identity() {
        return Err(ProcessProbeEvidenceDenial::ExecutableMismatch);
    }
    if evidence.input_artifact_identity != declaration.input_identity() {
        return Err(ProcessProbeEvidenceDenial::InputIdentityMismatch);
    }
    if evidence.working_directory != declaration.working_directory() {
        return Err(ProcessProbeEvidenceDenial::WorkingDirectoryMismatch);
    }
    if environment_identity(&evidence.environment)? != evidence.environment_identity {
        return Err(ProcessProbeEvidenceDenial::EnvironmentMismatch);
    }
    Ok(evidence)
}

fn command_bindings(
    command: &Command,
    keys: &[&str],
) -> Result<Vec<ProcessEnvironmentBindingEvidence>, ProcessProbeEvidenceDenial> {
    keys.iter()
        .map(|key| {
            let value = command
                .get_envs()
                .find(|(name, _)| name == std::ffi::OsStr::new(key))
                .and_then(|(_, value)| value)
                .ok_or(ProcessProbeEvidenceDenial::UnadmittedEnvironment)?;
            Ok(ProcessEnvironmentBindingEvidence {
                name: (*key).to_owned(),
                value_sha256: Sha256::digest(value.to_string_lossy().as_bytes()).into(),
            })
        })
        .collect()
}

fn current_bindings(
    keys: &[String],
) -> Result<Vec<ProcessEnvironmentBindingEvidence>, ProcessProbeEvidenceDenial> {
    if keys.iter().any(|key| !key.starts_with("WORTH_STORE_")) {
        return Err(ProcessProbeEvidenceDenial::UnadmittedEnvironment);
    }
    keys.iter()
        .map(|key| {
            let value = std::env::var_os(key)
                .ok_or(ProcessProbeEvidenceDenial::InvalidChildObservation)?;
            Ok(ProcessEnvironmentBindingEvidence {
                name: key.clone(),
                value_sha256: Sha256::digest(value.to_string_lossy().as_bytes()).into(),
            })
        })
        .collect()
}

fn environment_identity(
    bindings: &[ProcessEnvironmentBindingEvidence],
) -> Result<[u8; 32], ProcessProbeEvidenceDenial> {
    serde_json::to_vec(bindings)
        .map(|bytes| Sha256::digest(bytes).into())
        .map_err(|_| ProcessProbeEvidenceDenial::InvalidChildObservation)
}

fn write_new_json(
    path: &Path,
    evidence: &ProcessIdentityEvidence,
) -> Result<(), ProcessProbeEvidenceDenial> {
    let parent = path
        .parent()
        .ok_or(ProcessProbeEvidenceDenial::EvidenceWrite)?;
    fs::create_dir_all(parent).map_err(|_| ProcessProbeEvidenceDenial::EvidenceWrite)?;
    let mut bytes = serde_json::to_vec_pretty(evidence)
        .map_err(|_| ProcessProbeEvidenceDenial::EvidenceWrite)?;
    bytes.push(b'\n');
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|_| ProcessProbeEvidenceDenial::EvidenceWrite)?;
    use std::io::Write;
    file.write_all(&bytes)
        .and_then(|()| file.sync_all())
        .map_err(|_| ProcessProbeEvidenceDenial::EvidenceWrite)
}

fn file_digest(path: &Path) -> Result<[u8; 32], ProcessProbeEvidenceDenial> {
    fs::read(path)
        .map(|bytes| Sha256::digest(bytes).into())
        .map_err(|_| ProcessProbeEvidenceDenial::InvalidChildObservation)
}

fn required_string(name: &str) -> Result<String, ProcessProbeEvidenceDenial> {
    std::env::var(name).map_err(|_| ProcessProbeEvidenceDenial::InvalidChildObservation)
}

fn required_path(name: &str) -> Result<PathBuf, ProcessProbeEvidenceDenial> {
    std::env::var_os(name)
        .map(PathBuf::from)
        .ok_or(ProcessProbeEvidenceDenial::InvalidChildObservation)
}

const fn role_token(role: ProcessRole) -> &'static str {
    match role {
        ProcessRole::Writer => "writer",
        ProcessRole::CrashTarget => "crash-target",
        ProcessRole::RecoveredRuntime => "recovered-runtime",
        ProcessRole::OfflineVerifier => "offline-verifier",
        ProcessRole::FormalCheckerAdapter => "formal-checker-adapter",
        ProcessRole::AllocatorIsolatedProbe => "allocator-isolated-probe",
    }
}

fn parse_role(value: &str) -> Option<ProcessRole> {
    Some(match value {
        "writer" => ProcessRole::Writer,
        "crash-target" => ProcessRole::CrashTarget,
        "recovered-runtime" => ProcessRole::RecoveredRuntime,
        "offline-verifier" => ProcessRole::OfflineVerifier,
        "formal-checker-adapter" => ProcessRole::FormalCheckerAdapter,
        "allocator-isolated-probe" => ProcessRole::AllocatorIsolatedProbe,
        _ => return None,
    })
}
