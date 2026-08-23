use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

use super::metadata_graph::{dependency_ids, Metadata};
use super::targets::TargetSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SourceWorkload {
    source_files: u64,
    source_bytes: u64,
}

impl SourceWorkload {
    pub const fn source_files(self) -> u64 {
        self.source_files
    }

    pub const fn source_bytes(self) -> u64 {
        self.source_bytes
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BoundSource {
    repository: PathBuf,
    digest: [u8; 32],
    workload: SourceWorkload,
    entries: Box<[PathBuf]>,
}

impl BoundSource {
    pub fn repository(&self) -> &Path {
        &self.repository
    }

    pub const fn digest(&self) -> [u8; 32] {
        self.digest
    }

    pub const fn workload(&self) -> SourceWorkload {
        self.workload
    }

    pub fn entries(&self) -> &[PathBuf] {
        &self.entries
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct SourceSnapshot {
    pub(crate) bound: BoundSource,
}

pub(crate) fn capture(
    metadata: &Metadata,
    repository: &Path,
    targets: &TargetSet,
    source_packages: &[&str],
) -> Result<SourceSnapshot, String> {
    let mut files = BTreeSet::new();
    add_required(&mut files, &repository.join("Cargo.toml"), repository)?;
    add_required(
        &mut files,
        &metadata.workspace_root().join("Cargo.toml"),
        repository,
    )?;
    add_required(
        &mut files,
        &metadata.workspace_root().join("Cargo.lock"),
        repository,
    )?;
    for config in [
        repository.join(".cargo"),
        metadata.workspace_root().join(".cargo"),
    ] {
        if config.is_dir() {
            collect_directory(&mut files, &config, repository)?;
        }
    }
    let package_ids = dependency_ids(metadata, targets, source_packages)?;
    for id in package_ids {
        let package = metadata
            .package(&id)
            .ok_or_else(|| format!("metadata omitted package {id}"))?;
        add_required(&mut files, package.manifest(), repository)?;
        let source = package.root().join("src");
        if source.is_dir() {
            collect_directory(&mut files, &source, repository)?;
        }
        add_optional(&mut files, &package.root().join("build.rs"), repository)?;
    }
    let digest = digest_files(&files, repository)?;
    let source_bytes = files
        .iter()
        .map(|file| std::fs::metadata(file).map(|metadata| metadata.len()))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("read source closure metadata: {error}"))?
        .into_iter()
        .sum();
    Ok(SourceSnapshot {
        bound: BoundSource {
            repository: repository.to_owned(),
            digest,
            workload: SourceWorkload {
                source_files: files.len() as u64,
                source_bytes,
            },
            entries: files.into_iter().collect::<Vec<_>>().into_boxed_slice(),
        },
    })
}

fn digest_files(files: &BTreeSet<PathBuf>, repository: &Path) -> Result<[u8; 32], String> {
    let mut hasher = Sha256::new();
    for file in files {
        let relative = file
            .strip_prefix(repository)
            .map_err(|_| format!("source closure escaped repository: {file:?}"))?
            .to_string_lossy()
            .replace('\\', "/");
        hasher.update(relative.as_bytes());
        hasher.update([0]);
        hasher.update(
            std::fs::read(file)
                .map_err(|error| format!("read source closure file {file:?}: {error}"))?,
        );
        hasher.update([0xff]);
    }
    Ok(hasher.finalize().into())
}

fn add_required(
    files: &mut BTreeSet<PathBuf>,
    path: &Path,
    repository: &Path,
) -> Result<(), String> {
    if !path.is_file() {
        return Err(format!("source closure omitted required file {path:?}"));
    }
    add_optional(files, path, repository)
}

fn add_optional(
    files: &mut BTreeSet<PathBuf>,
    path: &Path,
    repository: &Path,
) -> Result<(), String> {
    if !path.exists() {
        return Ok(());
    }
    let canonical = path
        .canonicalize()
        .map_err(|error| format!("canonicalize source closure input {path:?}: {error}"))?;
    if !canonical.starts_with(repository) {
        return Err(format!(
            "source closure input escaped repository: {canonical:?}"
        ));
    }
    if canonical.is_file() {
        files.insert(canonical);
    }
    Ok(())
}

fn collect_directory(
    files: &mut BTreeSet<PathBuf>,
    directory: &Path,
    repository: &Path,
) -> Result<(), String> {
    for entry in std::fs::read_dir(directory)
        .map_err(|error| format!("read source closure directory {directory:?}: {error}"))?
    {
        let path = entry
            .map_err(|error| format!("read source closure entry: {error}"))?
            .path();
        let file_type = path
            .symlink_metadata()
            .map_err(|error| format!("inspect source closure entry {path:?}: {error}"))?;
        if file_type.file_type().is_symlink() {
            return Err(format!("source closure refuses symlink {path:?}"));
        }
        if path.is_dir() {
            collect_directory(files, &path, repository)?;
        } else {
            add_optional(files, &path, repository)?;
        }
    }
    Ok(())
}
