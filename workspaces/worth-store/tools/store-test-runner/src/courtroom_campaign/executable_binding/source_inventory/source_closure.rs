use std::path::{Path, PathBuf};
use worth_store::physical_runtime::{PhysicalWorkEvidenceDigest, PhysicalWorkSourceBinding};

use super::super::SourceClosureWorkload;
use super::BoundLocalSourceClosure;

pub(super) fn build_inputs(repository: &Path, workspace: &Path) -> Result<Vec<PathBuf>, String> {
    let mut inputs = vec![
        repository.join("Cargo.toml"),
        workspace.join("Cargo.toml"),
        workspace.join("Cargo.lock"),
    ];
    for configuration in [
        repository.join(".cargo/config.toml"),
        workspace.join(".cargo/config.toml"),
    ] {
        if configuration.is_file() {
            inputs.push(configuration);
        }
    }
    inputs
        .into_iter()
        .map(|path| {
            path.canonicalize()
                .map_err(|error| format!("cannot bind build input {}: {error}", path.display()))
        })
        .collect()
}

pub(super) fn bind(
    repository: &Path,
    workspace: &Path,
    package_roots: &[PathBuf],
    build_inputs: &[PathBuf],
) -> Result<BoundLocalSourceClosure, String> {
    let mut sources = build_inputs.to_vec();
    for root in package_roots {
        collect_package_sources(root, &mut sources)?;
    }
    sources.sort();
    sources.dedup();
    let fingerprint = crate::local_source_fingerprint::fingerprint_sources(repository, &sources)?;
    let digest = PhysicalWorkEvidenceDigest::new(fingerprint.digest())
        .ok_or_else(|| "source inventory produced an all-zero digest".to_owned())?;
    let binding = PhysicalWorkSourceBinding::new(
        format!("{}#c5-1-local-runtime-source-closure", workspace.display()),
        digest,
    )
    .map_err(|denial| format!("source evidence binding denied: {denial:?}"))?;
    Ok(BoundLocalSourceClosure::new(
        binding,
        SourceClosureWorkload::new(fingerprint.source_files(), fingerprint.source_bytes()),
    ))
}

fn collect_package_sources(root: &Path, files: &mut Vec<PathBuf>) -> Result<(), String> {
    let manifest = root.join("Cargo.toml");
    if !manifest.is_file() {
        return Err(format!(
            "local source package omitted manifest {}",
            manifest.display()
        ));
    }
    files.push(manifest);
    let source = root.join("src");
    if source.is_dir() {
        collect_files(&source, files)?;
    }
    let build = root.join("build.rs");
    if build.is_file() {
        files.push(build);
    }
    Ok(())
}

fn collect_files(root: &Path, files: &mut Vec<PathBuf>) -> Result<(), String> {
    let mut pending = vec![root.to_path_buf()];
    while let Some(directory) = pending.pop() {
        for entry in std::fs::read_dir(&directory)
            .map_err(|error| format!("cannot read source {}: {error}", directory.display()))?
        {
            let entry = entry.map_err(|error| format!("cannot read source entry: {error}"))?;
            let kind = entry
                .file_type()
                .map_err(|error| format!("cannot inspect source entry: {error}"))?;
            if kind.is_symlink() {
                return Err(format!(
                    "source inventory refuses symbolic link {}",
                    entry.path().display()
                ));
            }
            if kind.is_dir() {
                if !matches!(entry.file_name().to_str(), Some("target") | Some(".git")) {
                    pending.push(entry.path());
                }
            } else if kind.is_file() {
                files.push(entry.path());
            }
        }
    }
    Ok(())
}
