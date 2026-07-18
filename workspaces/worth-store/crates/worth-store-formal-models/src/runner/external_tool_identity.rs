use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExternalToolIdentity {
    adapter_name: String,
    adapter_version: String,
    provenance: String,
    executable_path: PathBuf,
    executable_sha256: [u8; 32],
    executable_version: String,
    tool_artifact_path: PathBuf,
    tool_artifact_sha256: [u8; 32],
    timeout_millis: u64,
    resource_posture: String,
}

pub(super) struct ExternalToolObservation {
    pub(super) adapter_name: String,
    pub(super) adapter_version: String,
    pub(super) provenance: String,
    pub(super) executable_path: PathBuf,
    pub(super) executable_sha256: [u8; 32],
    pub(super) executable_version: String,
    pub(super) tool_artifact_path: PathBuf,
    pub(super) tool_artifact_sha256: [u8; 32],
    pub(super) timeout_millis: u64,
    pub(super) resource_posture: String,
}

impl ExternalToolIdentity {
    pub(super) fn observed(observation: ExternalToolObservation) -> Self {
        Self {
            adapter_name: observation.adapter_name,
            adapter_version: observation.adapter_version,
            provenance: observation.provenance,
            executable_path: observation.executable_path,
            executable_sha256: observation.executable_sha256,
            executable_version: observation.executable_version,
            tool_artifact_path: observation.tool_artifact_path,
            tool_artifact_sha256: observation.tool_artifact_sha256,
            timeout_millis: observation.timeout_millis,
            resource_posture: observation.resource_posture,
        }
    }

    pub fn adapter_name(&self) -> &str {
        &self.adapter_name
    }
    pub fn adapter_version(&self) -> &str {
        &self.adapter_version
    }
    pub fn provenance(&self) -> &str {
        &self.provenance
    }
    pub fn executable_path(&self) -> &Path {
        &self.executable_path
    }
    pub const fn executable_sha256(&self) -> &[u8; 32] {
        &self.executable_sha256
    }
    pub fn executable_version(&self) -> &str {
        &self.executable_version
    }
    pub fn tool_artifact_path(&self) -> &Path {
        &self.tool_artifact_path
    }
    pub const fn tool_artifact_sha256(&self) -> &[u8; 32] {
        &self.tool_artifact_sha256
    }
    pub const fn timeout_millis(&self) -> u64 {
        self.timeout_millis
    }
    pub fn resource_posture(&self) -> &str {
        &self.resource_posture
    }
}
