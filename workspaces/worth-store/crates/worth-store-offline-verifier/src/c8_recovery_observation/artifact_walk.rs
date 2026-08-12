use std::io::Read;
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

use super::report_protocol::RecoveryObserverDecodeDenial;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryObserverLimitsDenial {
    ZeroArtifactLimit,
    ZeroByteLimit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecoveryObserverLimits {
    maximum_artifacts: usize,
    maximum_bytes: u64,
}

impl RecoveryObserverLimits {
    pub const fn new(
        maximum_artifacts: usize,
        maximum_bytes: u64,
    ) -> Result<Self, RecoveryObserverLimitsDenial> {
        if maximum_artifacts == 0 {
            return Err(RecoveryObserverLimitsDenial::ZeroArtifactLimit);
        }
        if maximum_bytes == 0 {
            return Err(RecoveryObserverLimitsDenial::ZeroByteLimit);
        }
        Ok(Self {
            maximum_artifacts,
            maximum_bytes,
        })
    }

    const fn maximum_artifacts(self) -> usize {
        self.maximum_artifacts
    }

    const fn maximum_bytes(self) -> u64 {
        self.maximum_bytes
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ObservedRecoveryArtifact {
    path: Box<str>,
    byte_length: u64,
    digest: [u8; 32],
}

impl ObservedRecoveryArtifact {
    pub(super) fn path(&self) -> &str {
        &self.path
    }

    pub(super) const fn byte_length(&self) -> u64 {
        self.byte_length
    }

    pub(super) const fn digest(&self) -> [u8; 32] {
        self.digest
    }
}

pub(super) fn walk(
    store_root: &Path,
    limits: RecoveryObserverLimits,
) -> Result<(Vec<ObservedRecoveryArtifact>, u64), RecoveryObserverDecodeDenial> {
    let root = store_root
        .canonicalize()
        .map_err(|error| RecoveryObserverDecodeDenial::Media(error.kind()))?;
    let mut pending = vec![root.clone()];
    let mut artifacts = Vec::new();
    let mut bytes_read = 0_u64;
    while let Some(directory) = pending.pop() {
        let mut entries = std::fs::read_dir(&directory)
            .map_err(|error| RecoveryObserverDecodeDenial::Media(error.kind()))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| RecoveryObserverDecodeDenial::Media(error.kind()))?;
        entries.sort_by_key(std::fs::DirEntry::file_name);
        for entry in entries.into_iter().rev() {
            let file_type = entry
                .file_type()
                .map_err(|error| RecoveryObserverDecodeDenial::Media(error.kind()))?;
            if file_type.is_symlink() {
                return Err(RecoveryObserverDecodeDenial::SymbolicLink);
            }
            if file_type.is_dir() {
                pending.push(entry.path());
            } else if file_type.is_file() {
                if artifacts.len() == limits.maximum_artifacts() {
                    return Err(RecoveryObserverDecodeDenial::ArtifactLimit);
                }
                let artifact = observe_file(&root, entry.path(), limits, &mut bytes_read)?;
                artifacts.push(artifact);
            } else {
                return Err(RecoveryObserverDecodeDenial::UnsupportedFileType);
            }
        }
    }
    artifacts.sort_by(|left, right| left.path.cmp(&right.path));
    Ok((artifacts, bytes_read))
}

fn observe_file(
    root: &Path,
    path: PathBuf,
    limits: RecoveryObserverLimits,
    bytes_read: &mut u64,
) -> Result<ObservedRecoveryArtifact, RecoveryObserverDecodeDenial> {
    let declared = path
        .metadata()
        .map_err(|error| RecoveryObserverDecodeDenial::Media(error.kind()))?
        .len();
    let projected = bytes_read
        .checked_add(declared)
        .ok_or(RecoveryObserverDecodeDenial::ByteLimit)?;
    if projected > limits.maximum_bytes() {
        return Err(RecoveryObserverDecodeDenial::ByteLimit);
    }
    let mut file = std::fs::File::open(&path)
        .map_err(|error| RecoveryObserverDecodeDenial::Media(error.kind()))?;
    let mut digest = Sha256::new();
    let mut observed = 0_u64;
    let mut buffer = [0_u8; 64 * 1024];
    loop {
        let count = file
            .read(&mut buffer)
            .map_err(|error| RecoveryObserverDecodeDenial::Media(error.kind()))?;
        if count == 0 {
            break;
        }
        digest.update(&buffer[..count]);
        observed = observed
            .checked_add(count as u64)
            .ok_or(RecoveryObserverDecodeDenial::ByteLimit)?;
    }
    if observed != declared {
        return Err(RecoveryObserverDecodeDenial::ArtifactChanged);
    }
    *bytes_read = projected;
    let relative = path
        .strip_prefix(root)
        .map_err(|_| RecoveryObserverDecodeDenial::ArtifactChanged)?
        .to_str()
        .ok_or(RecoveryObserverDecodeDenial::NonUnicodePath)?
        .replace('\\', "/")
        .into_boxed_str();
    Ok(ObservedRecoveryArtifact {
        path: relative,
        byte_length: observed,
        digest: digest.finalize().into(),
    })
}
