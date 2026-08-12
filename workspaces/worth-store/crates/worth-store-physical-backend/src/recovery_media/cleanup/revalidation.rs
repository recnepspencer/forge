use sha2::{Digest, Sha256};

use crate::filesystem_media::{ArtifactTreeFailure, ArtifactTreeFile};

use super::super::AdmittedRecoveryFilesystemMedia;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RecoveryCleanupArtifactRevalidationProgress {
    reads_attempted: u64,
    reads_completed: u64,
    bytes_read: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryCleanupArtifactRevalidationDenial {
    Read(ArtifactTreeFailure),
    LengthMismatch {
        expected_bytes: u64,
        observed_bytes: u64,
    },
    DigestMismatch {
        expected_digest: [u8; 32],
        observed_digest: [u8; 32],
    },
}

pub(super) struct RecoveryCleanupArtifactRevalidationFailure {
    denial: RecoveryCleanupArtifactRevalidationDenial,
    progress: RecoveryCleanupArtifactRevalidationProgress,
}

pub(super) fn verify(
    media: &AdmittedRecoveryFilesystemMedia,
    artifact: &ArtifactTreeFile,
    expected_bytes: u64,
    expected_digest: [u8; 32],
) -> Result<RecoveryCleanupArtifactRevalidationProgress, RecoveryCleanupArtifactRevalidationFailure>
{
    let attempted = RecoveryCleanupArtifactRevalidationProgress {
        reads_attempted: 1,
        reads_completed: 0,
        bytes_read: 0,
    };
    let observed = media
        .parts
        .artifact_tree()
        .read_bounded(artifact, expected_bytes)
        .map_err(|failure| RecoveryCleanupArtifactRevalidationFailure {
            denial: RecoveryCleanupArtifactRevalidationDenial::Read(failure),
            progress: attempted,
        })?;
    let progress = RecoveryCleanupArtifactRevalidationProgress {
        reads_attempted: 1,
        reads_completed: 1,
        bytes_read: observed.len() as u64,
    };
    if observed.len() as u64 != expected_bytes {
        return Err(RecoveryCleanupArtifactRevalidationFailure {
            denial: RecoveryCleanupArtifactRevalidationDenial::LengthMismatch {
                expected_bytes,
                observed_bytes: observed.len() as u64,
            },
            progress,
        });
    }
    let observed_digest = <[u8; 32]>::from(Sha256::digest(&observed));
    if observed_digest != expected_digest {
        return Err(RecoveryCleanupArtifactRevalidationFailure {
            denial: RecoveryCleanupArtifactRevalidationDenial::DigestMismatch {
                expected_digest,
                observed_digest,
            },
            progress,
        });
    }
    Ok(progress)
}

impl RecoveryCleanupArtifactRevalidationProgress {
    pub const fn reads_attempted(self) -> u64 {
        self.reads_attempted
    }

    pub const fn reads_completed(self) -> u64 {
        self.reads_completed
    }

    pub const fn bytes_read(self) -> u64 {
        self.bytes_read
    }
}

impl RecoveryCleanupArtifactRevalidationFailure {
    pub(super) const fn denial(&self) -> RecoveryCleanupArtifactRevalidationDenial {
        self.denial
    }

    pub(super) const fn progress(&self) -> RecoveryCleanupArtifactRevalidationProgress {
        self.progress
    }
}
