use std::fs;
use std::io::Write;
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};

use super::UiRunFailure;

pub(super) fn write(path: &Path, bytes: &[u8]) -> Result<bool, UiRunFailure> {
    let parent = path.parent().ok_or_else(|| {
        UiRunFailure::EnvironmentObservation(format!("{} has no parent", path.display()))
    })?;
    fs::create_dir_all(parent)
        .map_err(|error| UiRunFailure::EnvironmentObservation(error.to_string()))?;
    publish(path, bytes).map_err(UiRunFailure::EnvironmentObservation)
}

fn publish(path: &Path, bytes: &[u8]) -> Result<bool, String> {
    let parent = path
        .parent()
        .ok_or_else(|| format!("{} has no parent", path.display()))?;
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
        .map_err(|error| format!("create {}: {error}", staging.display()))?;
    let write = file.write_all(bytes).and_then(|()| file.sync_all());
    drop(file);
    if let Err(error) = write {
        let _ = fs::remove_file(&staging);
        return Err(format!("write {}: {error}", staging.display()));
    }
    let linked = fs::hard_link(&staging, path);
    let _ = fs::remove_file(&staging);
    match linked {
        Ok(()) => Ok(true),
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
            let metadata = fs::symlink_metadata(path)
                .map_err(|error| format!("inspect {}: {error}", path.display()))?;
            if !metadata.is_file() || metadata.file_type().is_symlink() {
                return Err(format!("immutable file is not regular: {}", path.display()));
            }
            let existing =
                fs::read(path).map_err(|error| format!("read {}: {error}", path.display()))?;
            if existing == bytes {
                Ok(false)
            } else {
                Err(format!("immutable file collision at {}", path.display()))
            }
        }
        Err(error) => Err(format!("publish {}: {error}", path.display())),
    }
}

static PUBLISH_NONCE: AtomicU64 = AtomicU64::new(0);
