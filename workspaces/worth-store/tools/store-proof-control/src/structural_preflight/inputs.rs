use std::fs;
use std::path::{Path, PathBuf};

use worth_store_test_support::structural_preflight::PreflightInputScope;

use crate::evidence::sha256_bytes;

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
    let mut basis = Vec::new();
    for path in files {
        let relative = path.strip_prefix(forge_root).unwrap_or(&path);
        let relative = relative.to_string_lossy().replace('\\', "/");
        let bytes = fs::read(&path)
            .map_err(|error| format!("could not read preflight input {}: {error}", path.display()))?;
        basis.extend_from_slice(&(relative.len() as u64).to_be_bytes());
        basis.extend_from_slice(relative.as_bytes());
        basis.extend_from_slice(&(bytes.len() as u64).to_be_bytes());
        basis.extend_from_slice(&bytes);
    }
    if basis.is_empty() {
        return Err(format!(
            "preflight input scope {scope_identity} selected no files"
        ));
    }
    Ok(PreflightInputScope {
        scope_identity: scope_identity.to_owned(),
        source_paths: source_paths.iter().map(|path| (*path).to_owned()).collect(),
        included_extensions: included_extensions
            .iter()
            .map(|extension| (*extension).to_owned())
            .collect(),
        input_identity: sha256_bytes(&basis),
    })
}

fn collect(
    path: &Path,
    extensions: &[&str],
    files: &mut Vec<PathBuf>,
) -> Result<(), String> {
    if path.is_file() {
        if admitted_file(path, extensions) {
            files.push(path.to_path_buf());
        }
        return Ok(());
    }
    if !path.exists() {
        return Err(format!("preflight input is missing: {}", path.display()));
    }
    for entry in fs::read_dir(path)
        .map_err(|error| format!("could not inspect {}: {error}", path.display()))?
    {
        let entry = entry.map_err(|error| format!("could not inspect {}: {error}", path.display()))?;
        let child = entry.path();
        if child.is_dir() && excluded_directory(&child) {
            continue;
        }
        if child.is_dir() || admitted_file(&child, extensions) {
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
