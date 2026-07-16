use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

use crate::{OfflineMediaConsistencyBasis, OfflineMediaReadDenial, ReadOnlyOfflineMediaCapability};

#[derive(Debug)]
pub enum PhysicalBackupArtifactObservationDenial {
    InvalidBufferBudget,
    BufferAllocationFailed,
    EmptyArtifact { path: PathBuf },
    Media(OfflineMediaReadDenial),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalBackupArtifactObservation {
    path: PathBuf,
    bytes: u64,
    content_digest: [u8; 32],
    peak_buffer_bytes: u64,
    physical_identity: [u8; 32],
}

impl PhysicalBackupArtifactObservation {
    pub fn path(&self) -> &Path {
        &self.path
    }
    pub const fn bytes(&self) -> u64 {
        self.bytes
    }
    pub const fn content_digest(&self) -> [u8; 32] {
        self.content_digest
    }
    pub const fn peak_buffer_bytes(&self) -> u64 {
        self.peak_buffer_bytes
    }
    pub const fn physical_identity(&self) -> [u8; 32] {
        self.physical_identity
    }
}

pub fn observe_physical_backup_artifact(
    path: impl Into<PathBuf>,
    buffer_bytes: usize,
) -> Result<PhysicalBackupArtifactObservation, PhysicalBackupArtifactObservationDenial> {
    if buffer_bytes == 0 {
        return Err(PhysicalBackupArtifactObservationDenial::InvalidBufferBudget);
    }
    let requested_path = path.into();
    let path = std::fs::canonicalize(&requested_path).map_err(|source| {
        PhysicalBackupArtifactObservationDenial::Media(OfflineMediaReadDenial::Io {
            path: requested_path,
            source,
        })
    })?;
    let mut media = ReadOnlyOfflineMediaCapability::open(
        [path.clone()],
        OfflineMediaConsistencyBasis::single_artifact_mutation_detection(),
    )
    .map_err(PhysicalBackupArtifactObservationDenial::Media)?;
    let identity = media.file(0).cloned().ok_or_else(|| {
        PhysicalBackupArtifactObservationDenial::EmptyArtifact { path: path.clone() }
    })?;
    if identity.length() == 0 {
        return Err(PhysicalBackupArtifactObservationDenial::EmptyArtifact { path });
    }
    let mut offset = 0u64;
    let mut hasher = Sha256::new();
    let mut buffer = Vec::new();
    buffer
        .try_reserve_exact(buffer_bytes)
        .map_err(|_| PhysicalBackupArtifactObservationDenial::BufferAllocationFailed)?;
    buffer.resize(buffer_bytes, 0);
    while offset < identity.length() {
        let remaining = usize::try_from(identity.length() - offset).unwrap_or(usize::MAX);
        let requested = buffer_bytes.min(remaining);
        let observation = media
            .read_bounded_into(0, offset, &mut buffer[..requested])
            .map_err(PhysicalBackupArtifactObservationDenial::Media)?;
        hasher.update(&buffer[..observation.bytes_read()]);
        offset += observation.bytes_read() as u64;
    }
    media
        .revalidate_consistency()
        .map_err(PhysicalBackupArtifactObservationDenial::Media)?;
    Ok(PhysicalBackupArtifactObservation {
        path,
        bytes: identity.length(),
        content_digest: hasher.finalize().into(),
        peak_buffer_bytes: buffer_bytes as u64,
        physical_identity: identity.physical_key_fingerprint(),
    })
}
