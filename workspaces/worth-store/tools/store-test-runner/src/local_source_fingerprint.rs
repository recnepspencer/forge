use std::{
    io::Read,
    path::{Path, PathBuf},
};

use sha2::{Digest, Sha256};

#[cfg(feature = "physical-work-evidence")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct LocalSourceFingerprint {
    digest: [u8; 32],
    source_files: u64,
    source_bytes: u64,
}

#[cfg(feature = "physical-work-evidence")]
impl LocalSourceFingerprint {
    pub(crate) const fn digest(self) -> [u8; 32] {
        self.digest
    }

    pub(crate) const fn source_files(self) -> u64 {
        self.source_files
    }

    pub(crate) const fn source_bytes(self) -> u64 {
        self.source_bytes
    }
}

pub(crate) fn hash_sources(repository: &Path, sources: &[PathBuf]) -> Result<[u8; 32], String> {
    let workers = source_hashing_workers();
    hash_sources_with_workers(repository, sources, workers)
}

#[cfg(feature = "physical-work-evidence")]
pub(crate) fn fingerprint_sources(
    repository: &Path,
    sources: &[PathBuf],
) -> Result<LocalSourceFingerprint, String> {
    let (digest, source_files, source_bytes) =
        fingerprint_components_with_workers(repository, sources, source_hashing_workers())?;
    Ok(LocalSourceFingerprint {
        digest,
        source_files,
        source_bytes,
    })
}

fn hash_sources_with_workers(
    repository: &Path,
    sources: &[PathBuf],
    workers: usize,
) -> Result<[u8; 32], String> {
    fingerprint_components_with_workers(repository, sources, workers)
        .map(|fingerprint| fingerprint.0)
}

fn fingerprint_components_with_workers(
    repository: &Path,
    sources: &[PathBuf],
    workers: usize,
) -> Result<([u8; 32], u64, u64), String> {
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
    let source_files = u64::try_from(leaves.len())
        .map_err(|_| "source inventory file count overflowed u64".to_owned())?;
    let mut source_bytes = 0_u64;
    let mut digest = Sha256::new();
    for leaf in leaves {
        source_bytes = source_bytes
            .checked_add(leaf.length)
            .ok_or_else(|| "source inventory byte count overflowed u64".to_owned())?;
        digest.update((leaf.path.len() as u64).to_le_bytes());
        digest.update(leaf.path.as_bytes());
        digest.update(leaf.length.to_le_bytes());
        digest.update(leaf.digest);
    }
    Ok((digest.finalize().into(), source_files, source_bytes))
}

fn source_hashing_workers() -> usize {
    std::thread::available_parallelism()
        .map(std::num::NonZeroUsize::get)
        .unwrap_or(1)
        .min(8)
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
