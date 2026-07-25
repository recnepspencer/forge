use std::io::Read;
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

use super::{
    OfflineHostileArtifactObservation, OfflineHostilePhysicalTruthBudget,
    OfflineHostilePhysicalTruthDenial,
};

struct ObservedArtifactContents {
    byte_length: u64,
    digest: [u8; 32],
    prefix: Box<[u8]>,
}

pub(super) fn inventory(
    store_root: &Path,
    budget: OfflineHostilePhysicalTruthBudget,
) -> Result<Vec<OfflineHostileArtifactObservation>, OfflineHostilePhysicalTruthDenial> {
    let root = store_root
        .canonicalize()
        .map_err(|error| OfflineHostilePhysicalTruthDenial::RootUnavailable(error.kind()))?;
    let mut pending = vec![root.clone()];
    let mut artifacts = Vec::new();
    let mut observed_bytes = 0_u64;
    while let Some(directory) = pending.pop() {
        let mut entries = std::fs::read_dir(&directory)
            .map_err(|error| OfflineHostilePhysicalTruthDenial::ArtifactUnavailable(error.kind()))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| {
                OfflineHostilePhysicalTruthDenial::ArtifactUnavailable(error.kind())
            })?;
        entries.sort_by_key(std::fs::DirEntry::file_name);
        for entry in entries.into_iter().rev() {
            let file_type = entry.file_type().map_err(|error| {
                OfflineHostilePhysicalTruthDenial::ArtifactUnavailable(error.kind())
            })?;
            if file_type.is_symlink() {
                return Err(OfflineHostilePhysicalTruthDenial::SymbolicLinkEncountered);
            }
            if file_type.is_dir() {
                pending.push(entry.path());
                continue;
            }
            if !file_type.is_file() {
                return Err(OfflineHostilePhysicalTruthDenial::ArtifactUnavailable(
                    std::io::ErrorKind::InvalidData,
                ));
            }
            if artifacts.len() == budget.max_files() {
                return Err(OfflineHostilePhysicalTruthDenial::FileBudgetExceeded);
            }
            let observation = observe_file(&root, entry.path(), budget, &mut observed_bytes)?;
            artifacts.push(observation);
        }
    }
    artifacts.sort_by(|left, right| left.path().cmp(right.path()));
    Ok(artifacts)
}

fn observe_file(
    root: &Path,
    path: PathBuf,
    budget: OfflineHostilePhysicalTruthBudget,
    observed_bytes: &mut u64,
) -> Result<OfflineHostileArtifactObservation, OfflineHostilePhysicalTruthDenial> {
    let relative = path
        .strip_prefix(root)
        .map_err(|_| OfflineHostilePhysicalTruthDenial::ArtifactChangedDuringObservation)?;
    let relative = relative
        .to_str()
        .ok_or(OfflineHostilePhysicalTruthDenial::NonUnicodeArtifactPath)?
        .replace('\\', "/")
        .into_boxed_str();
    let declared_bytes = path
        .metadata()
        .map_err(|error| OfflineHostilePhysicalTruthDenial::ArtifactUnavailable(error.kind()))?
        .len();
    if observed_bytes.saturating_add(declared_bytes) > budget.max_total_bytes() {
        return Err(OfflineHostilePhysicalTruthDenial::ByteBudgetExceeded);
    }
    let contents = read_file(&path, budget.prefix_bytes())?;
    if contents.byte_length != declared_bytes {
        return Err(OfflineHostilePhysicalTruthDenial::ArtifactChangedDuringObservation);
    }
    *observed_bytes = observed_bytes.saturating_add(contents.byte_length);
    Ok(OfflineHostileArtifactObservation::new(
        relative,
        contents.byte_length,
        contents.digest,
        contents.prefix,
    ))
}

fn read_file(
    path: &Path,
    prefix_limit: usize,
) -> Result<ObservedArtifactContents, OfflineHostilePhysicalTruthDenial> {
    let mut file = std::fs::File::open(path)
        .map_err(|error| OfflineHostilePhysicalTruthDenial::ArtifactUnavailable(error.kind()))?;
    let mut digest = Sha256::new();
    let mut prefix = Vec::with_capacity(prefix_limit);
    let mut buffer = [0_u8; 64 * 1024];
    let mut bytes = 0_u64;
    loop {
        let count = file.read(&mut buffer).map_err(|error| {
            OfflineHostilePhysicalTruthDenial::ArtifactUnavailable(error.kind())
        })?;
        if count == 0 {
            break;
        }
        digest.update(&buffer[..count]);
        let retained = prefix_limit.saturating_sub(prefix.len()).min(count);
        prefix.extend_from_slice(&buffer[..retained]);
        bytes = bytes.saturating_add(count as u64);
    }
    Ok(ObservedArtifactContents {
        byte_length: bytes,
        digest: digest.finalize().into(),
        prefix: prefix.into_boxed_slice(),
    })
}
