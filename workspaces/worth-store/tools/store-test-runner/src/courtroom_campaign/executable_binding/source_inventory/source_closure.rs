use std::{
    io::Read,
    path::{Path, PathBuf},
};

use sha2::{Digest, Sha256};
use worth_store::physical_runtime::{PhysicalWorkEvidenceDigest, PhysicalWorkSourceBinding};

pub(super) fn build_inputs(repository: &Path, workspace: &Path) -> Result<Vec<PathBuf>, String> {
    [
        repository.join("Cargo.toml"),
        workspace.join("Cargo.toml"),
        workspace.join("Cargo.lock"),
    ]
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
) -> Result<PhysicalWorkSourceBinding, String> {
    let mut sources = build_inputs.to_vec();
    for root in package_roots {
        collect_package_sources(root, &mut sources)?;
    }
    sources.sort();
    sources.dedup();
    let digest = hash_sources(repository, &sources)?;
    PhysicalWorkSourceBinding::new(
        format!("{}#c5-1-local-runtime-source-closure", workspace.display()),
        digest,
    )
    .map_err(|denial| format!("source evidence binding denied: {denial:?}"))
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

fn hash_sources(
    repository: &Path,
    sources: &[PathBuf],
) -> Result<PhysicalWorkEvidenceDigest, String> {
    let workers = std::thread::available_parallelism()
        .map(std::num::NonZeroUsize::get)
        .unwrap_or(1)
        .min(8);
    hash_sources_with_workers(repository, sources, workers)
}

fn hash_sources_with_workers(
    repository: &Path,
    sources: &[PathBuf],
    workers: usize,
) -> Result<PhysicalWorkEvidenceDigest, String> {
    if sources.is_empty() {
        return Err("source inventory produced an empty closure".into());
    }
    let workers = workers.clamp(1, sources.len());
    let chunk_size = sources.len().div_ceil(workers);
    let leaves = std::thread::scope(|scope| {
        let handles = sources
            .chunks(chunk_size)
            .map(|chunk| {
                scope.spawn(move || {
                    chunk
                        .iter()
                        .map(|source| hash_source(repository, source))
                        .collect::<Result<Vec<_>, _>>()
                })
            })
            .collect::<Vec<_>>();
        let mut leaves = Vec::with_capacity(sources.len());
        for handle in handles {
            leaves.extend(
                handle
                    .join()
                    .map_err(|_| "source hashing worker panicked".to_owned())??,
            );
        }
        Ok::<_, String>(leaves)
    })?;
    let mut digest = Sha256::new();
    for leaf in leaves {
        digest.update((leaf.path.len() as u64).to_le_bytes());
        digest.update(leaf.path.as_bytes());
        digest.update(leaf.length.to_le_bytes());
        digest.update(leaf.digest);
    }
    PhysicalWorkEvidenceDigest::new(digest.finalize().into())
        .ok_or_else(|| "source inventory produced an all-zero digest".to_owned())
}

struct SourceLeaf {
    path: String,
    length: u64,
    digest: [u8; 32],
}

fn hash_source(repository: &Path, source: &Path) -> Result<SourceLeaf, String> {
    let relative = source
        .strip_prefix(repository)
        .map_err(|_| format!("source {} escaped repository", source.display()))?;
    let path = relative.to_string_lossy().replace('\\', "/");
    let length = source
        .metadata()
        .map_err(|error| format!("cannot inspect source {}: {error}", source.display()))?
        .len();
    let mut file = std::fs::File::open(source)
        .map_err(|error| format!("cannot read source {}: {error}", source.display()))?;
    let mut buffer = [0_u8; 64 * 1024];
    let mut digest = Sha256::new();
    loop {
        let count = file
            .read(&mut buffer)
            .map_err(|error| format!("cannot hash source {}: {error}", source.display()))?;
        if count == 0 {
            return Ok(SourceLeaf {
                path,
                length,
                digest: digest.finalize().into(),
            });
        }
        digest.update(&buffer[..count]);
    }
}

#[cfg(test)]
mod tests {
    use super::hash_sources_with_workers;

    #[test]
    fn closure_digest_is_deterministic_across_worker_counts() {
        let repository = tempfile::tempdir().unwrap();
        let first = repository.path().join("first.rs");
        let second = repository.path().join("second.rs");
        std::fs::write(&first, b"first source").unwrap();
        std::fs::write(&second, b"second source").unwrap();
        let sources = vec![first, second];

        assert_eq!(
            hash_sources_with_workers(repository.path(), &sources, 1).unwrap(),
            hash_sources_with_workers(repository.path(), &sources, 8).unwrap(),
        );
    }

    #[test]
    fn same_length_source_mutation_changes_the_closure_digest() {
        let repository = tempfile::tempdir().unwrap();
        let source = repository.path().join("source.rs");
        std::fs::write(&source, b"original").unwrap();
        let sources = vec![source.clone()];
        let before = hash_sources_with_workers(repository.path(), &sources, 2).unwrap();

        std::fs::write(source, b"mutation").unwrap();
        let after = hash_sources_with_workers(repository.path(), &sources, 2).unwrap();

        assert_ne!(before, after);
    }
}
