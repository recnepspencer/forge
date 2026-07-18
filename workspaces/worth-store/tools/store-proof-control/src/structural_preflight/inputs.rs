use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};
use worth_store_test_support::structural_preflight::PreflightInputScope;

pub(super) fn scope(
    forge_root: &Path,
    scope_identity: &str,
    source_paths: &[&str],
    included_extensions: &[&str],
) -> Result<PreflightInputScope, String> {
    let mut files = Vec::new();
    for source in source_paths {
        collect(
            &forge_root.join(source),
            included_extensions,
            &mut files,
        )?;
    }
    files.sort();
    files.dedup();
    if files.is_empty() {
        return Err(format!(
            "preflight input scope {scope_identity} selected no files"
        ));
    }
    let mut basis = Sha256::new();
    basis.update(b"worth-store-structural-preflight-input-scope-v1");
    for path in files {
        let relative = path.strip_prefix(forge_root).unwrap_or(&path);
        let relative = relative.to_string_lossy().replace('\\', "/");
        let mut file = fs::File::open(&path).map_err(|error| {
            format!("could not read preflight input {}: {error}", path.display())
        })?;
        let length = file
            .metadata()
            .map_err(|error| format!("could not inspect {}: {error}", path.display()))?
            .len();
        basis.update((relative.len() as u64).to_be_bytes());
        basis.update(relative.as_bytes());
        basis.update(length.to_be_bytes());
        let mut buffer = [0_u8; 64 * 1024];
        loop {
            let read = file.read(&mut buffer).map_err(|error| {
                format!("could not read preflight input {}: {error}", path.display())
            })?;
            if read == 0 {
                break;
            }
            basis.update(&buffer[..read]);
        }
    }
    Ok(PreflightInputScope {
        scope_identity: scope_identity.to_owned(),
        source_paths: source_paths.iter().map(|path| (*path).to_owned()).collect(),
        included_extensions: included_extensions
            .iter()
            .map(|extension| (*extension).to_owned())
            .collect(),
        input_identity: format!("{:x}", basis.finalize()),
    })
}

fn collect(
    path: &Path,
    extensions: &[&str],
    files: &mut Vec<PathBuf>,
) -> Result<(), String> {
    let metadata = fs::symlink_metadata(path)
        .map_err(|error| format!("could not inspect {}: {error}", path.display()))?;
    if metadata.file_type().is_symlink() {
        return Err(format!(
            "preflight input scope may not follow symlink {}",
            path.display()
        ));
    }
    if metadata.is_file() {
        if admitted_file(path, extensions) {
            files.push(path.to_path_buf());
        }
        return Ok(());
    }
    if !metadata.is_dir() {
        return Err(format!(
            "preflight input is neither a file nor directory: {}",
            path.display()
        ));
    }
    let mut entries = fs::read_dir(path)
        .map_err(|error| format!("could not inspect {}: {error}", path.display()))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("could not inspect {}: {error}", path.display()))?;
    entries.sort_by_key(|entry| entry.path());
    for entry in entries {
        let child = entry.path();
        let file_type = entry
            .file_type()
            .map_err(|error| format!("could not inspect {}: {error}", child.display()))?;
        if file_type.is_symlink() {
            return Err(format!(
                "preflight input scope may not follow symlink {}",
                child.display()
            ));
        }
        if file_type.is_dir() && excluded_directory(&child) {
            continue;
        }
        if file_type.is_dir() || (file_type.is_file() && admitted_file(&child, extensions)) {
            collect(&child, extensions, files)?;
        }
    }
    Ok(())
}

fn admitted_file(path: &Path, extensions: &[&str]) -> bool {
    extensions.is_empty()
        || path
            .extension()
            .and_then(|extension| extension.to_str())
            .is_some_and(|extension| extensions.contains(&extension))
}

fn excluded_directory(path: &Path) -> bool {
    path.file_name().is_some_and(|name| {
        matches!(
            name.to_str(),
            Some(".git" | ".store-proof" | "target" | "_tmp")
        )
    })
}
