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
    resource_posture: ExternalToolResourcePosture,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExternalToolResourcePosture {
    worker_strategy: String,
    available_parallelism: usize,
    deadlock_check: bool,
    state_directory: PathBuf,
    state_directory_fresh_exclusive: bool,
    version_probe_timeout_millis: u64,
    output_cap_bytes_per_stream: usize,
}

pub(crate) struct ExternalToolObservation {
    pub(crate) adapter_name: String,
    pub(crate) adapter_version: String,
    pub(crate) provenance: String,
    pub(crate) executable_path: PathBuf,
    pub(crate) executable_sha256: [u8; 32],
    pub(crate) executable_version: String,
    pub(crate) tool_artifact_path: PathBuf,
    pub(crate) tool_artifact_sha256: [u8; 32],
    pub(crate) timeout_millis: u64,
    pub(crate) resource_posture: ExternalToolResourcePosture,
}

impl ExternalToolResourcePosture {
    pub(super) fn tlc(state_directory: &Path) -> Self {
        Self {
            worker_strategy: "auto".to_owned(),
            available_parallelism: std::thread::available_parallelism()
                .map_or(1, std::num::NonZeroUsize::get),
            deadlock_check: true,
            state_directory: state_directory.to_path_buf(),
            state_directory_fresh_exclusive: true,
            version_probe_timeout_millis: 10_000,
            output_cap_bytes_per_stream: 64 * 1024 * 1024,
        }
    }

    pub fn worker_strategy(&self) -> &str {
        &self.worker_strategy
    }
    pub const fn available_parallelism(&self) -> usize {
        self.available_parallelism
    }
    pub const fn deadlock_check(&self) -> bool {
        self.deadlock_check
    }
    pub fn state_directory(&self) -> &Path {
        &self.state_directory
    }
    pub const fn state_directory_fresh_exclusive(&self) -> bool {
        self.state_directory_fresh_exclusive
    }
    pub const fn version_probe_timeout_millis(&self) -> u64 {
        self.version_probe_timeout_millis
    }
    pub const fn output_cap_bytes_per_stream(&self) -> usize {
        self.output_cap_bytes_per_stream
    }
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
    pub const fn resource_posture(&self) -> &ExternalToolResourcePosture {
        &self.resource_posture
    }
}
