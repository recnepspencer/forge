use std::io::Read;
use std::path::Path;

use sha2::{Digest, Sha256};

use super::bundle_manifest::BackupBundleManifestAdmissionDenial;
use super::manifest_binary_codec::{decode_manifest, encode_manifest};
use super::{BackupBundleManifest, MaterializedBackupBundle};

const MAX_MANIFEST_READ_BUFFER_BYTES: usize = 8 * 1024 * 1024;

#[derive(Debug)]
pub enum BackupBundleFormatDenial {
    Read(std::io::Error),
    MissingPublishedManifest,
    SymbolicLinkUnsupported,
    ManifestReadLimitExceeded {
        encoded_bytes: u64,
        maximum_bytes: u64,
    },
    ManifestArtifactLimitExceeded {
        artifacts: u64,
        maximum_artifacts: u64,
    },
    ManifestAllocationFailed,
    ManifestAllocationCountOverflow,
    ManifestOwnedAllocationLimitExceeded {
        observed_bytes: u64,
        maximum_bytes: u64,
    },
    InvalidManifest,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BackupBundleManifestReadLimits {
    maximum_encoded_bytes: u64,
    maximum_artifacts: u64,
    read_buffer_bytes: usize,
    maximum_owned_allocation_bytes: u64,
}

impl BackupBundleManifestReadLimits {
    pub const fn new(
        maximum_encoded_bytes: u64,
        maximum_artifacts: u64,
        read_buffer_bytes: usize,
        maximum_owned_allocation_bytes: u64,
    ) -> Option<Self> {
        if maximum_encoded_bytes == 0
            || maximum_artifacts == 0
            || read_buffer_bytes == 0
            || read_buffer_bytes > MAX_MANIFEST_READ_BUFFER_BYTES
            || read_buffer_bytes as u64 > maximum_encoded_bytes
            || maximum_owned_allocation_bytes == 0
            || read_buffer_bytes as u64 > maximum_owned_allocation_bytes
        {
            None
        } else {
            Some(Self {
                maximum_encoded_bytes,
                maximum_artifacts,
                read_buffer_bytes,
                maximum_owned_allocation_bytes,
            })
        }
    }

    pub const fn canonical() -> Self {
        Self {
            maximum_encoded_bytes: 64 * 1024 * 1024,
            maximum_artifacts: 262_144,
            read_buffer_bytes: 64 * 1024,
            maximum_owned_allocation_bytes: 128 * 1024 * 1024,
        }
    }

    pub const fn maximum_encoded_bytes(self) -> u64 {
        self.maximum_encoded_bytes
    }

    pub const fn maximum_artifacts(self) -> u64 {
        self.maximum_artifacts
    }

    pub const fn read_buffer_bytes(self) -> usize {
        self.read_buffer_bytes
    }

    pub const fn maximum_owned_allocation_bytes(self) -> u64 {
        self.maximum_owned_allocation_bytes
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BackupBundleManifestReadObservation {
    encoded_bytes: u64,
    read_buffer_bytes: u64,
    owned_allocation_bytes: u64,
}

impl BackupBundleManifestReadObservation {
    pub const fn encoded_bytes(self) -> u64 {
        self.encoded_bytes
    }

    pub const fn read_buffer_bytes(self) -> u64 {
        self.read_buffer_bytes
    }

    pub const fn owned_allocation_bytes(self) -> u64 {
        self.owned_allocation_bytes
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BackupBundleFormatAuthority;

impl BackupBundleFormatAuthority {
    pub const fn canonical() -> Self {
        Self
    }
    pub fn encode_manifest(
        self,
        manifest: &BackupBundleManifest,
    ) -> Result<Vec<u8>, BackupBundleFormatDenial> {
        encode_manifest(manifest)
    }
    pub fn decode_manifest(
        self,
        bytes: &[u8],
    ) -> Result<BackupBundleManifest, BackupBundleFormatDenial> {
        self.decode_manifest_with_limits(bytes, BackupBundleManifestReadLimits::canonical())
    }

    pub fn decode_manifest_with_limits(
        self,
        bytes: &[u8],
        limits: BackupBundleManifestReadLimits,
    ) -> Result<BackupBundleManifest, BackupBundleFormatDenial> {
        enforce_encoded_limit(bytes.len() as u64, limits)?;
        let decoded = decode_manifest(
            std::io::Cursor::new(bytes),
            limits.maximum_artifacts,
            bytes.len() as u64,
            limits.maximum_owned_allocation_bytes,
        )?;
        enforce_artifact_limit(&decoded, limits)?;
        admit_decoded_manifest(decoded, limits.maximum_owned_allocation_bytes)
            .map(|(manifest, _)| manifest)
    }
    pub fn admit_materialized(
        self,
        root: impl AsRef<Path>,
    ) -> Result<MaterializedBackupBundle, BackupBundleFormatDenial> {
        self.admit_materialized_with_limits(root, BackupBundleManifestReadLimits::canonical())
    }

    pub fn admit_materialized_with_limits(
        self,
        root: impl AsRef<Path>,
        limits: BackupBundleManifestReadLimits,
    ) -> Result<MaterializedBackupBundle, BackupBundleFormatDenial> {
        let root = root.as_ref();
        let path = root.join("backup.manifest");
        if is_symbolic_link(root) || is_symbolic_link(&path) {
            return Err(BackupBundleFormatDenial::SymbolicLinkUnsupported);
        }
        if !path.is_file() {
            return Err(BackupBundleFormatDenial::MissingPublishedManifest);
        }
        let file = std::fs::File::open(&path).map_err(BackupBundleFormatDenial::Read)?;
        let encoded_bytes = file
            .metadata()
            .map_err(BackupBundleFormatDenial::Read)?
            .len();
        enforce_encoded_limit(encoded_bytes, limits)?;
        let digesting = DigestingReader::new(file.take(encoded_bytes));
        let mut buffered =
            FalliblyAllocatedBufferedReader::new(digesting, limits.read_buffer_bytes)?;
        let decoded = decode_manifest(
            &mut buffered,
            limits.maximum_artifacts,
            encoded_bytes,
            limits.maximum_owned_allocation_bytes,
        )?;
        enforce_artifact_limit(&decoded, limits)?;
        let (manifest, peak_owned_allocation_bytes) =
            admit_decoded_manifest(decoded, limits.maximum_owned_allocation_bytes)?;
        let digesting = buffered.into_inner();
        if digesting.bytes_read != encoded_bytes {
            return Err(BackupBundleFormatDenial::InvalidManifest);
        }
        let observation = BackupBundleManifestReadObservation {
            encoded_bytes,
            read_buffer_bytes: limits.read_buffer_bytes as u64,
            owned_allocation_bytes: peak_owned_allocation_bytes,
        };
        Ok(MaterializedBackupBundle::new(
            root.to_path_buf(),
            manifest,
            digesting.digest.finalize().into(),
            observation,
        ))
    }
}

fn admit_decoded_manifest(
    decoded: BackupBundleManifest,
    maximum_owned_allocation_bytes: u64,
) -> Result<(BackupBundleManifest, u64), BackupBundleFormatDenial> {
    decoded
        .admit_decoded(maximum_owned_allocation_bytes)
        .map_err(|denial| match denial {
            BackupBundleManifestAdmissionDenial::InvalidManifest => {
                BackupBundleFormatDenial::InvalidManifest
            }
            BackupBundleManifestAdmissionDenial::AllocationFailed => {
                BackupBundleFormatDenial::ManifestAllocationFailed
            }
            BackupBundleManifestAdmissionDenial::AllocationCountOverflow => {
                BackupBundleFormatDenial::ManifestAllocationCountOverflow
            }
            BackupBundleManifestAdmissionDenial::AllocationLimitExceeded {
                observed_bytes,
                maximum_bytes,
            } => BackupBundleFormatDenial::ManifestOwnedAllocationLimitExceeded {
                observed_bytes,
                maximum_bytes,
            },
        })
}

fn is_symbolic_link(path: &Path) -> bool {
    std::fs::symlink_metadata(path)
        .map(|metadata| metadata.file_type().is_symlink())
        .unwrap_or(false)
}

fn enforce_encoded_limit(
    encoded_bytes: u64,
    limits: BackupBundleManifestReadLimits,
) -> Result<(), BackupBundleFormatDenial> {
    if encoded_bytes > limits.maximum_encoded_bytes {
        Err(BackupBundleFormatDenial::ManifestReadLimitExceeded {
            encoded_bytes,
            maximum_bytes: limits.maximum_encoded_bytes,
        })
    } else {
        Ok(())
    }
}

fn enforce_artifact_limit(
    manifest: &BackupBundleManifest,
    limits: BackupBundleManifestReadLimits,
) -> Result<(), BackupBundleFormatDenial> {
    let artifacts = manifest.artifacts().len() as u64;
    if artifacts > limits.maximum_artifacts {
        Err(BackupBundleFormatDenial::ManifestArtifactLimitExceeded {
            artifacts,
            maximum_artifacts: limits.maximum_artifacts,
        })
    } else {
        Ok(())
    }
}

struct DigestingReader<R> {
    inner: R,
    digest: Sha256,
    bytes_read: u64,
}

impl<R> DigestingReader<R> {
    fn new(inner: R) -> Self {
        Self {
            inner,
            digest: Sha256::new(),
            bytes_read: 0,
        }
    }
}

impl<R: Read> Read for DigestingReader<R> {
    fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
        let read = self.inner.read(buffer)?;
        self.digest.update(&buffer[..read]);
        self.bytes_read = self
            .bytes_read
            .checked_add(read as u64)
            .ok_or_else(|| std::io::Error::other("manifest read counter overflow"))?;
        Ok(read)
    }
}

struct FalliblyAllocatedBufferedReader<R> {
    inner: R,
    buffer: Vec<u8>,
    position: usize,
    filled: usize,
}

impl<R> FalliblyAllocatedBufferedReader<R> {
    fn new(inner: R, buffer_bytes: usize) -> Result<Self, BackupBundleFormatDenial> {
        let mut buffer = Vec::new();
        buffer
            .try_reserve_exact(buffer_bytes)
            .map_err(|_| BackupBundleFormatDenial::ManifestAllocationFailed)?;
        buffer.resize(buffer_bytes, 0);
        Ok(Self {
            inner,
            buffer,
            position: 0,
            filled: 0,
        })
    }

    fn into_inner(self) -> R {
        self.inner
    }
}

impl<R: Read> Read for FalliblyAllocatedBufferedReader<R> {
    fn read(&mut self, output: &mut [u8]) -> std::io::Result<usize> {
        if output.is_empty() {
            return Ok(0);
        }
        if self.position == self.filled {
            self.filled = self.inner.read(&mut self.buffer)?;
            self.position = 0;
            if self.filled == 0 {
                return Ok(0);
            }
        }
        let copied = output.len().min(self.filled - self.position);
        output[..copied].copy_from_slice(&self.buffer[self.position..self.position + copied]);
        self.position += copied;
        Ok(copied)
    }
}
