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

impl ExternalToolIdentity {
    #[allow(clippy::too_many_arguments)]
    pub(super) fn observed(
        adapter_name: impl Into<String>,
        adapter_version: impl Into<String>,
        provenance: impl Into<String>,
        executable_path: PathBuf,
        executable_sha256: [u8; 32],
        executable_version: String,
        tool_artifact_path: PathBuf,
        tool_artifact_sha256: [u8; 32],
        timeout_millis: u64,
        resource_posture: impl Into<String>,
    ) -> Self {
        Self {
            adapter_name: adapter_name.into(),
            adapter_version: adapter_version.into(),
            provenance: provenance.into(),
            executable_path,
            executable_sha256,
            executable_version,
            tool_artifact_path,
            tool_artifact_sha256,
            timeout_millis,
            resource_posture: resource_posture.into(),
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
