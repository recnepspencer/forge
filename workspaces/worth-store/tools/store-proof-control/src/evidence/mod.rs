use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use serde::de::DeserializeOwned;
use serde::Serialize;
use sha2::{Digest, Sha256};

pub fn write_json<T: Serialize>(path: &Path, value: &T) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| format!("could not create {}: {error}", parent.display()))?;
    }
    let encoded = serde_json::to_vec_pretty(value)
        .map_err(|error| format!("could not encode {}: {error}", path.display()))?;
    let mut file = fs::File::create(path)
        .map_err(|error| format!("could not create {}: {error}", path.display()))?;
    file.write_all(&encoded)
        .map_err(|error| format!("could not write {}: {error}", path.display()))?;
    file.write_all(b"\n")
        .map_err(|error| format!("could not finish {}: {error}", path.display()))
}

pub fn write_new_json<T: Serialize>(path: &Path, value: &T) -> Result<(), String> {
    let mut encoded = serde_json::to_vec_pretty(value)
        .map_err(|error| format!("could not encode {}: {error}", path.display()))?;
    encoded.push(b'\n');
    if publish_immutable(path, &encoded)? {
        Ok(())
    } else {
        Err(format!("authority already exists: {}", path.display()))
    }
}

pub fn write_immutable_json<T: Serialize>(path: &Path, value: &T) -> Result<(), String> {
    let mut encoded = serde_json::to_vec_pretty(value)
        .map_err(|error| format!("could not encode {}: {error}", path.display()))?;
    encoded.push(b'\n');
    publish_immutable(path, &encoded).map(|_| ())
}

pub fn read_json<T: DeserializeOwned>(path: &Path) -> Result<T, String> {
    let bytes =
        fs::read(path).map_err(|error| format!("could not read {}: {error}", path.display()))?;
    serde_json::from_slice(&bytes)
        .map_err(|error| format!("could not decode {}: {error}", path.display()))
}

pub fn sha256_file(path: &Path) -> Result<String, String> {
    let mut file = fs::File::open(path)
        .map_err(|error| format!("could not read {}: {error}", path.display()))?;
    let mut digest = Sha256::new();
    let mut buffer = [0_u8; 64 * 1024];
    loop {
        let read = file
            .read(&mut buffer)
            .map_err(|error| format!("could not read {}: {error}", path.display()))?;
        if read == 0 {
            break;
        }
        digest.update(&buffer[..read]);
    }
    Ok(format!("{:x}", digest.finalize()))
}

pub fn sha256_serialized<T: Serialize>(value: &T) -> Result<String, String> {
    let bytes = serde_json::to_vec(value)
        .map_err(|error| format!("could not encode digest input: {error}"))?;
    Ok(sha256_bytes(&bytes))
}

pub fn evidence_plan_path(workspace_root: &Path, digest: &str) -> PathBuf {
    workspace_root
        .join(".store-proof")
        .join("evidence")
        .join("plans")
        .join(format!("{digest}.json"))
}

pub(crate) fn sha256_bytes(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}

fn publish_immutable(path: &Path, bytes: &[u8]) -> Result<bool, String> {
    let parent = path
        .parent()
        .ok_or_else(|| format!("{} has no parent", path.display()))?;
    fs::create_dir_all(parent)
        .map_err(|error| format!("could not create {}: {error}", parent.display()))?;
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| format!("{} has no UTF-8 file name", path.display()))?;
    let nonce = PUBLISH_NONCE.fetch_add(1, Ordering::Relaxed);
    let staging = parent.join(format!(
        ".{file_name}.publish-{}-{nonce}",
        std::process::id()
    ));
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&staging)
        .map_err(|error| format!("could not stage {}: {error}", staging.display()))?;
    let write = file.write_all(bytes).and_then(|()| file.sync_all());
    drop(file);
    if let Err(error) = write {
        let _ = fs::remove_file(&staging);
        return Err(format!("could not finish {}: {error}", staging.display()));
    }
    let linked = fs::hard_link(&staging, path);
    let _ = fs::remove_file(&staging);
    match linked {
        Ok(()) => Ok(true),
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
            let metadata = fs::symlink_metadata(path)
                .map_err(|error| format!("could not inspect {}: {error}", path.display()))?;
            if !metadata.is_file() || metadata.file_type().is_symlink() {
                return Err(format!(
                    "immutable evidence is not a file: {}",
                    path.display()
                ));
            }
            let existing = fs::read(path)
                .map_err(|error| format!("could not read {}: {error}", path.display()))?;
            if existing == bytes {
                Ok(false)
            } else {
                Err(format!(
                    "immutable evidence identity collision at {}",
                    path.display()
                ))
            }
        }
        Err(error) => Err(format!("could not publish {}: {error}", path.display())),
    }
}

static PUBLISH_NONCE: AtomicU64 = AtomicU64::new(0);
