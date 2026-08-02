use std::path::{Path, PathBuf};

use super::WalArtifactStoreDenial;
use crate::{CheckpointPublicationScope, WalFramePublicationScope};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalFrameArtifactObservation {
    scope: WalFramePublicationScope,
    path: PathBuf,
    frame_offset: u64,
    payload_offset: u64,
    payload_bytes: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckpointArtifactObservation {
    scope: CheckpointPublicationScope,
    path: PathBuf,
    bytes: u64,
}

pub fn observe_wal_frame_artifact(
    path: &Path,
    frame_offset: u64,
    frame_bytes: u64,
    scope: &WalFramePublicationScope,
) -> Result<WalFrameArtifactObservation, WalArtifactStoreDenial> {
    let path = std::fs::canonicalize(path).map_err(|_| WalArtifactStoreDenial::Io)?;
    let (payload_offset, payload_bytes) =
        super::frame_codec::validate_persisted_frame(&path, frame_offset, frame_bytes, scope)?;
    Ok(WalFrameArtifactObservation {
        scope: scope.clone(),
        path,
        frame_offset,
        payload_offset,
        payload_bytes,
    })
}

pub fn observe_checkpoint_artifact(
    path: &Path,
    scope: &CheckpointPublicationScope,
    expected: &[u8],
) -> Result<CheckpointArtifactObservation, WalArtifactStoreDenial> {
    let path = std::fs::canonicalize(path).map_err(|_| WalArtifactStoreDenial::Io)?;
    let observed = std::fs::read(&path).map_err(|_| WalArtifactStoreDenial::Io)?;
    if observed != expected {
        return Err(WalArtifactStoreDenial::DigestMismatch);
    }
    Ok(CheckpointArtifactObservation {
        scope: scope.clone(),
        path,
        bytes: observed.len() as u64,
    })
}

impl WalFrameArtifactObservation {
    pub const fn scope(&self) -> &WalFramePublicationScope {
        &self.scope
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub const fn frame_offset(&self) -> u64 {
        self.frame_offset
    }

    pub const fn payload_offset(&self) -> u64 {
        self.payload_offset
    }

    pub const fn payload_bytes(&self) -> u64 {
        self.payload_bytes
    }

    pub fn payload_matches(&self, expected: &[u8]) -> bool {
        use std::io::{Read, Seek, SeekFrom};

        if self.payload_bytes != expected.len() as u64 {
            return false;
        }
        let mut observed = vec![0; expected.len()];
        std::fs::File::open(&self.path)
            .and_then(|mut file| {
                file.seek(SeekFrom::Start(self.payload_offset))?;
                file.read_exact(&mut observed)
            })
            .is_ok_and(|()| observed == expected)
    }
}

impl CheckpointArtifactObservation {
    pub const fn scope(&self) -> &CheckpointPublicationScope {
        &self.scope
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub const fn bytes(&self) -> u64 {
        self.bytes
    }
}
