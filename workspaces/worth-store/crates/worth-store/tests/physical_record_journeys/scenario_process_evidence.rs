use std::path::Path;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use worth_store::physical_runtime::ServingPhysicalRuntime;

const PROCESS_PREFIX: &str = "C5_PROCESS_EVIDENCE ";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(super) struct ScenarioProcessEvidence {
    role: String,
    process_id: u32,
    runtime_identity: Option<u64>,
    store_identity: Option<String>,
    media_owner_identity: Option<String>,
    mutation_attempt_identity: Option<String>,
    backend_profile_identity: Option<String>,
    binary_identity: String,
}

impl ScenarioProcessEvidence {
    pub(super) fn current_runtime(role: &str, runtime: &ServingPhysicalRuntime) -> Self {
        let media = runtime.observer().media_snapshot().unwrap();
        let mutation = media.mutation_owner();
        Self {
            role: role.into(),
            process_id: std::process::id(),
            runtime_identity: Some(runtime.runtime_identity().get()),
            store_identity: Some(hex(&runtime.store_identity().bytes())),
            media_owner_identity: Some(hex(&mutation.owner().bytes())),
            mutation_attempt_identity: Some(hex(&mutation.attempt().bytes())),
            backend_profile_identity: Some(profile_identity(media.backend_profile())),
            binary_identity: current_binary_identity(),
        }
    }

    pub(super) fn offline_process(stdout: &str, binary: &Path) -> Self {
        let process_id = stdout
            .lines()
            .find_map(|line| line.strip_prefix("C5_OFFLINE_PROCESS "))
            .expect("offline observer must report its process identity")
            .parse()
            .unwrap();
        Self {
            role: "offline-observer".into(),
            process_id,
            runtime_identity: None,
            store_identity: None,
            media_owner_identity: None,
            mutation_attempt_identity: None,
            backend_profile_identity: None,
            binary_identity: binary_identity(binary),
        }
    }

    pub(super) fn parse_child(stdout: &str, role: &str) -> Self {
        stdout
            .lines()
            .filter_map(|line| line.strip_prefix(PROCESS_PREFIX))
            .map(|record| serde_json::from_str::<Self>(record).unwrap())
            .find(|record| record.role == role)
            .unwrap_or_else(|| panic!("child process did not emit evidence for role `{role}`"))
    }

    pub(super) fn binary_identity(&self) -> &str {
        &self.binary_identity
    }
}

pub(super) fn emit_process(role: &str, runtime: &ServingPhysicalRuntime) {
    let evidence = ScenarioProcessEvidence::current_runtime(role, runtime);
    println!(
        "{PROCESS_PREFIX}{}",
        serde_json::to_string(&evidence).unwrap()
    );
}

pub(super) fn binary_identity(path: &Path) -> String {
    hex(&Sha256::digest(std::fs::read(path).unwrap()))
}

pub(super) fn current_binary_identity() -> String {
    static IDENTITY: std::sync::OnceLock<String> = std::sync::OnceLock::new();
    IDENTITY
        .get_or_init(|| binary_identity(&std::env::current_exe().unwrap()))
        .clone()
}

fn profile_identity(profile: &worth_store_physical_backend::FilesystemBackendProfile) -> String {
    let mut digest = Sha256::new();
    digest.update(profile.root_identity());
    digest.update(profile.volume_identity());
    digest.update(profile.filesystem_type().as_bytes());
    digest.update(profile.allocation_granularity().get().to_le_bytes());
    digest.update([profile.location() as u8]);
    digest.update([
        u8::from(profile.is_removable()),
        u8::from(profile.is_read_only()),
    ]);
    hex(&digest.finalize())
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}
