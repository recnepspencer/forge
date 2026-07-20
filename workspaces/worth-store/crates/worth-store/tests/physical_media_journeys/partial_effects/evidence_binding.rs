use sha2::{Digest, Sha256};
use std::io::Read;
use std::path::Path;

use worth_store_physical_backend::{filesystem_media_build_identity, FilesystemBackendProfile};
use worth_store_physical_format::store_namespace::StoreNamespaceVersion;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct CourtroomBinding {
    source_manifest: String,
    harness_binary: String,
    observer_binary: String,
    environment: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct CaseEvidenceBinding {
    courtroom: CourtroomBinding,
    schedule: &'static str,
}

impl CourtroomBinding {
    pub(super) fn capture(profile: &FilesystemBackendProfile) -> Self {
        let harness = std::env::current_exe().expect("journey executable path");
        let observer = Path::new(env!("CARGO_BIN_EXE_physical_media_os_observer"));
        Self {
            source_manifest: hex(&filesystem_media_build_identity()),
            harness_binary: hash_file(&harness),
            observer_binary: hash_file(observer),
            environment: format!(
                "os={};arch={};filesystem={};volume={};granularity={};namespace={};full_seams={}",
                std::env::consts::OS,
                std::env::consts::ARCH,
                profile.filesystem_type(),
                hex(&profile.volume_identity()),
                profile.allocation_granularity(),
                StoreNamespaceVersion::CURRENT.value(),
                std::env::var_os("WORTH_STORE_C4_FULL_SEAMS").is_some(),
            ),
        }
    }

    pub(super) fn for_schedule(&self, schedule: &'static str) -> CaseEvidenceBinding {
        CaseEvidenceBinding {
            courtroom: self.clone(),
            schedule,
        }
    }
}

fn hash_file(path: &Path) -> String {
    let mut file = std::fs::File::open(path).expect("evidence binary must be readable");
    let mut digest = Sha256::new();
    let mut buffer = [0_u8; 64 * 1_024];
    loop {
        let count = file.read(&mut buffer).expect("evidence binary read");
        if count == 0 {
            break;
        }
        digest.update(&buffer[..count]);
    }
    hex(&digest.finalize())
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}
