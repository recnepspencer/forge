use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

pub(super) fn allocate(parent: &Path) -> Result<PathBuf, String> {
    let parent = parent
        .canonicalize()
        .or_else(|_| std::fs::create_dir_all(parent).and_then(|()| parent.canonicalize()))
        .map_err(|error| format!("prepare finalized process-bundle parent: {error}"))?;
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| format!("clock before finalized process-bundle allocation: {error}"))?
        .as_nanos();
    for attempt in 0..64_u32 {
        let candidate = parent.join(format!(
            ".worth-store-finalized-process-bundle-{}-{nonce}-{attempt}",
            std::process::id()
        ));
        match std::fs::create_dir(&candidate) {
            Ok(()) => return Ok(candidate),
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(error) => {
                return Err(format!(
                    "allocate finalized process-bundle directory {}: {error}",
                    candidate.display()
                ))
            }
        }
    }
    Err(format!(
        "could not allocate finalized process-bundle directory beneath {}",
        parent.display()
    ))
}

pub(super) fn seal(directory: &Path) -> Result<(), String> {
    set_children_read_only(directory, true)?;
    let mut permissions = std::fs::metadata(directory)
        .map_err(|error| format!("inspect finalized process-bundle directory: {error}"))?
        .permissions();
    permissions.set_readonly(true);
    std::fs::set_permissions(directory, permissions)
        .map_err(|error| format!("seal finalized process-bundle directory: {error}"))
}

pub(super) fn remove(directory: &Path) -> Result<(), String> {
    if !directory.exists() {
        return Ok(());
    }
    make_writable(directory)?;
    std::fs::remove_dir_all(directory).map_err(|error| {
        format!(
            "remove finalized process-bundle {}: {error}",
            directory.display()
        )
    })?;
    if directory.exists() {
        return Err(format!(
            "finalized process-bundle survived removal: {}",
            directory.display()
        ));
    }
    Ok(())
}

pub(super) fn make_writable(directory: &Path) -> Result<(), String> {
    set_children_read_only(directory, false)?;
    let mut permissions = std::fs::metadata(directory)
        .map_err(|error| format!("inspect finalized process-bundle directory: {error}"))?
        .permissions();
    permissions.set_readonly(false);
    std::fs::set_permissions(directory, permissions)
        .map_err(|error| format!("unseal finalized process-bundle directory: {error}"))
}

fn set_children_read_only(directory: &Path, read_only: bool) -> Result<(), String> {
    for entry in std::fs::read_dir(directory)
        .map_err(|error| format!("read finalized process-bundle directory: {error}"))?
    {
        let path = entry
            .map_err(|error| format!("inspect finalized process-bundle entry: {error}"))?
            .path();
        let mut permissions = std::fs::metadata(&path)
            .map_err(|error| format!("inspect finalized process-bundle file: {error}"))?
            .permissions();
        permissions.set_readonly(read_only);
        std::fs::set_permissions(&path, permissions).map_err(|error| {
            format!(
                "change finalized process-bundle file permissions {}: {error}",
                path.display()
            )
        })?;
    }
    Ok(())
}
