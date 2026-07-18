use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitStatus;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use super::{
    ProcessIdentityEvidence, ProcessProbeDeclaration, ProcessTerminationRequirement,
};

pub const PROCESS_PROBE_EVIDENCE_ROOT_ENV: &str = "WORTH_STORE_PROCESS_PROBE_EVIDENCE_ROOT";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "mode", rename_all = "snake_case")]
pub enum ProcessTermination {
    GracefulExit { code: Option<i32> },
    PanicUnwind { code: Option<i32> },
    Abort { platform_status: String },
    ParentKill { platform_status: String },
    OsTermination { platform_status: String },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProcessProbeExecution {
    pub schema_version: u32,
    pub declaration: ProcessProbeDeclaration,
    pub process: ProcessIdentityEvidence,
    pub termination: ProcessTermination,
    pub output_artifact_identity: [u8; 32],
    pub evidence_identity: [u8; 32],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProcessProbeEvidenceDenial {
    UnadmittedEnvironment,
    InvalidChildObservation,
    MissingChildObservation,
    RoleMismatch,
    ProcessRelationshipMismatch,
    ExecutableMismatch,
    InputIdentityMismatch,
    WorkingDirectoryMismatch,
    EnvironmentMismatch,
    TerminationMismatch,
    OutputArtifactUnavailable,
    EvidenceWrite,
}

impl ProcessProbeExecution {
    pub fn observed(
        declaration: ProcessProbeDeclaration,
        process: ProcessIdentityEvidence,
        termination: ProcessTermination,
        output_artifact: &Path,
    ) -> Result<Self, ProcessProbeEvidenceDenial> {
        if !termination_satisfies(declaration.required_termination(), &termination) {
            return Err(ProcessProbeEvidenceDenial::TerminationMismatch);
        }
        let output_artifact_identity = fs::read(output_artifact)
            .map(|bytes| Sha256::digest(bytes).into())
            .map_err(|_| ProcessProbeEvidenceDenial::OutputArtifactUnavailable)?;
        let mut evidence = Self {
            schema_version: 1,
            declaration,
            process,
            termination,
            output_artifact_identity,
            evidence_identity: [0; 32],
        };
        evidence.evidence_identity = serde_json::to_vec(&evidence)
            .map(|bytes| Sha256::digest(bytes).into())
            .map_err(|_| ProcessProbeEvidenceDenial::EvidenceWrite)?;
        Ok(evidence)
    }
}

pub(crate) fn persist_execution(
    fallback_root: &Path,
    execution: &ProcessProbeExecution,
) -> Result<PathBuf, ProcessProbeEvidenceDenial> {
    let root = std::env::var_os(PROCESS_PROBE_EVIDENCE_ROOT_ENV)
        .map(PathBuf::from)
        .unwrap_or_else(|| fallback_root.join("process-probes"));
    fs::create_dir_all(&root).map_err(|_| ProcessProbeEvidenceDenial::EvidenceWrite)?;
    let path = root.join(format!("{}.json", hex(&execution.evidence_identity)));
    let mut bytes = serde_json::to_vec_pretty(execution)
        .map_err(|_| ProcessProbeEvidenceDenial::EvidenceWrite)?;
    bytes.push(b'\n');
    if path.exists() {
        return if fs::read(&path).ok().as_deref() == Some(bytes.as_slice()) {
            Ok(path)
        } else {
            Err(ProcessProbeEvidenceDenial::EvidenceWrite)
        };
    }
    fs::write(&path, bytes).map_err(|_| ProcessProbeEvidenceDenial::EvidenceWrite)?;
    Ok(path)
}

pub(crate) fn classify_exit(status: ExitStatus) -> ProcessTermination {
    ProcessTermination::GracefulExit {
        code: status.code(),
    }
}

fn termination_satisfies(
    required: ProcessTerminationRequirement,
    observed: &ProcessTermination,
) -> bool {
    matches!(
        (required, observed),
        (
            ProcessTerminationRequirement::GracefulExit,
            ProcessTermination::GracefulExit { .. }
        ) | (
            ProcessTerminationRequirement::PanicUnwind,
            ProcessTermination::PanicUnwind { .. }
        ) | (
            ProcessTerminationRequirement::Abort,
            ProcessTermination::Abort { .. }
        ) | (
            ProcessTerminationRequirement::ParentKill,
            ProcessTermination::ParentKill { .. }
        ) | (
            ProcessTerminationRequirement::OsTermination,
            ProcessTermination::OsTermination { .. }
        )
    )
}

fn hex(bytes: &[u8; 32]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}
