use std::io::Read;

use sha2::{Digest, Sha256};
use worth_store_physical_format::{
    BackupBundleArtifactFamily, BackupBundleFormatAuthority, BackupBundleFormatDenial,
};

use super::{ClosedNonCurrentStagingMedia, PhysicalRecoveryStagingOwner};

#[derive(Debug, Clone, Copy)]
pub struct ClosedStagingArtifactVerificationRequest<'a> {
    media: &'a ClosedNonCurrentStagingMedia,
    family: BackupBundleArtifactFamily,
    maximum_bytes: u64,
}

impl<'a> ClosedStagingArtifactVerificationRequest<'a> {
    pub const fn new(
        media: &'a ClosedNonCurrentStagingMedia,
        family: BackupBundleArtifactFamily,
        maximum_bytes: u64,
    ) -> Self {
        Self {
            media,
            family,
            maximum_bytes,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClosedStagingArtifactVerificationReceipt {
    family: BackupBundleArtifactFamily,
    verified_artifacts: u64,
    verified_bytes: u64,
}

impl ClosedStagingArtifactVerificationReceipt {
    pub const fn family(self) -> BackupBundleArtifactFamily {
        self.family
    }
    pub const fn verified_artifacts(self) -> u64 {
        self.verified_artifacts
    }
    pub const fn verified_bytes(self) -> u64 {
        self.verified_bytes
    }
}

#[derive(Debug)]
pub enum ClosedStagingArtifactVerificationDenial {
    InvalidBudget,
    MissingFamily,
    BudgetExceeded,
    Io,
    Manifest(BackupBundleFormatDenial),
    ArtifactMismatch { output_name: String },
}

impl PhysicalRecoveryStagingOwner {
    pub fn verify_closed_artifact_family(
        request: ClosedStagingArtifactVerificationRequest<'_>,
    ) -> Result<ClosedStagingArtifactVerificationReceipt, ClosedStagingArtifactVerificationDenial>
    {
        if request.maximum_bytes == 0 {
            return Err(ClosedStagingArtifactVerificationDenial::InvalidBudget);
        }
        let manifest_bytes = std::fs::read(request.media.root().join("backup.manifest"))
            .map_err(|_| ClosedStagingArtifactVerificationDenial::Io)?;
        let manifest = BackupBundleFormatAuthority::canonical()
            .decode_manifest(&manifest_bytes)
            .map_err(ClosedStagingArtifactVerificationDenial::Manifest)?;
        let mut count = 0_u64;
        let mut bytes = 0_u64;
        let mut buffer = [0_u8; 64 * 1024];
        for row in manifest
            .artifacts()
            .iter()
            .filter(|row| row.family() == request.family)
        {
            bytes = bytes
                .checked_add(row.bytes())
                .ok_or(ClosedStagingArtifactVerificationDenial::BudgetExceeded)?;
            if bytes > request.maximum_bytes {
                return Err(ClosedStagingArtifactVerificationDenial::BudgetExceeded);
            }
            let mut file = std::fs::File::open(request.media.root().join(row.output_name()))
                .map_err(|_| ClosedStagingArtifactVerificationDenial::Io)?;
            let mut digest = Sha256::new();
            let mut observed = 0_u64;
            loop {
                let read = file
                    .read(&mut buffer)
                    .map_err(|_| ClosedStagingArtifactVerificationDenial::Io)?;
                if read == 0 {
                    break;
                }
                observed = observed
                    .checked_add(read as u64)
                    .ok_or(ClosedStagingArtifactVerificationDenial::BudgetExceeded)?;
                digest.update(&buffer[..read]);
            }
            if observed != row.bytes() || digest.finalize()[..] != row.content_digest() {
                return Err(ClosedStagingArtifactVerificationDenial::ArtifactMismatch {
                    output_name: row.output_name().to_owned(),
                });
            }
            count = count
                .checked_add(1)
                .ok_or(ClosedStagingArtifactVerificationDenial::BudgetExceeded)?;
        }
        if count == 0 {
            return Err(ClosedStagingArtifactVerificationDenial::MissingFamily);
        }
        Ok(ClosedStagingArtifactVerificationReceipt {
            family: request.family,
            verified_artifacts: count,
            verified_bytes: bytes,
        })
    }
}
