use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use sha2::{Digest, Sha256};

use super::{artifact_store, bounded_process, UiCompilerToolchainIdentity, UiProofRunFailure};

pub(super) struct SealedEnvironmentLock {
    pub path: PathBuf,
    pub sha256: String,
    pub created: bool,
}

pub(super) fn seal(
    workspace_root: &Path,
    environment_root: &Path,
    manifest_path: &Path,
    manifest: &str,
    toolchain: &UiCompilerToolchainIdentity,
) -> Result<SealedEnvironmentLock, UiProofRunFailure> {
    let lock_path = environment_root.join("Cargo.lock");
    if lock_path.is_file() {
        return observed(lock_path, false);
    }
    let staging = LockGenerationStaging::create(workspace_root)?;
    let staging_manifest = staging.path.join("Cargo.toml");
    artifact_store::write_immutable_file(&staging_manifest, manifest.as_bytes())?;
    artifact_store::write_immutable_file(&staging.path.join("src/lib.rs"), b"#![no_std]\n")?;
    let mut command = Command::new(&toolchain.cargo.executable_path);
    command
        .args(["generate-lockfile", "--offline", "--manifest-path"])
        .arg(&staging_manifest)
        .env("RUSTC", &toolchain.rustc.executable_path)
        .env_remove("RUSTC_WRAPPER")
        .env_remove("RUSTC_WORKSPACE_WRAPPER")
        .env_remove("RUSTC_BOOTSTRAP")
        .env_remove("RUSTFLAGS")
        .env_remove("CARGO_ENCODED_RUSTFLAGS")
        .current_dir(workspace_root);
    let output = bounded_process::run(
        &mut command,
        Duration::from_millis(toolchain.compile_timeout_millis),
        toolchain.output_cap_bytes_per_stream,
    )
    .map_err(UiProofRunFailure::CompilerLaunch)?;
    if output.timed_out {
        return Err(UiProofRunFailure::CompilerTimedOut(
            "environment-lock-generation".to_owned(),
        ));
    }
    if !output.status.success() {
        return Err(UiProofRunFailure::EnvironmentObservation(format!(
            "could not seal UI dependency lock for {}: {}{}",
            manifest_path.display(),
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        )));
    }
    let generated = std::fs::read(staging.path.join("Cargo.lock"))
        .map_err(|error| UiProofRunFailure::EnvironmentObservation(error.to_string()))?;
    let created = artifact_store::write_immutable_file(&lock_path, &generated)?;
    observed(lock_path, created)
}

fn observed(path: PathBuf, created: bool) -> Result<SealedEnvironmentLock, UiProofRunFailure> {
    let metadata = std::fs::symlink_metadata(&path)
        .map_err(|error| UiProofRunFailure::EnvironmentObservation(error.to_string()))?;
    if !metadata.is_file() || metadata.file_type().is_symlink() {
        return Err(UiProofRunFailure::EnvironmentObservation(format!(
            "UI dependency lock is not a regular file: {}",
            path.display()
        )));
    }
    let bytes = std::fs::read(&path)
        .map_err(|error| UiProofRunFailure::EnvironmentObservation(error.to_string()))?;
    Ok(SealedEnvironmentLock {
        path,
        sha256: format!("{:x}", Sha256::digest(bytes)),
        created,
    })
}

struct LockGenerationStaging {
    path: PathBuf,
}

impl LockGenerationStaging {
    fn create(workspace_root: &Path) -> Result<Self, UiProofRunFailure> {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|error| UiProofRunFailure::EnvironmentObservation(error.to_string()))?
            .as_nanos();
        let parent = workspace_root.join(".store-proof/ui/lock-generation");
        std::fs::create_dir_all(&parent)
            .map_err(|error| UiProofRunFailure::EnvironmentObservation(error.to_string()))?;
        let path = parent.join(format!("{}-{nonce}", std::process::id()));
        std::fs::create_dir(&path)
            .map_err(|error| UiProofRunFailure::EnvironmentObservation(error.to_string()))?;
        Ok(Self { path })
    }
}

impl Drop for LockGenerationStaging {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.path);
    }
}
