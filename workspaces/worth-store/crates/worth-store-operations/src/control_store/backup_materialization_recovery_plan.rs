use std::path::{Path, PathBuf};

use super::operational_media_path::{
    encode_operational_media_path, resolve_operational_media_path, OperationalMediaPathDenial,
};

#[derive(Debug)]
pub enum BackupMaterializationRecoveryPlanDenial {
    InvalidBufferBudget,
    TargetPath(std::io::Error),
    TargetPathTooLarge,
    UnsupportedTargetPath,
    AllocationFailed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackupMaterializationRecoveryPlan {
    cut_identity: [u8; 32],
    target_parent: PathBuf,
    buffer_bytes: usize,
}

impl BackupMaterializationRecoveryPlan {
    pub(crate) fn prepare(
        cut_identity: [u8; 32],
        target_parent: &Path,
        buffer_bytes: usize,
    ) -> Result<Self, BackupMaterializationRecoveryPlanDenial> {
        if buffer_bytes == 0 {
            return Err(BackupMaterializationRecoveryPlanDenial::InvalidBufferBudget);
        }
        let target_parent = resolve_operational_media_path(target_parent)
            .map_err(BackupMaterializationRecoveryPlanDenial::TargetPath)?;
        encode_operational_media_path(&target_parent).map_err(map_path_denial)?;
        Ok(Self {
            cut_identity,
            target_parent,
            buffer_bytes,
        })
    }

    pub(crate) fn from_persisted(
        cut_identity: [u8; 32],
        target_parent: PathBuf,
        buffer_bytes: u64,
    ) -> Result<Self, BackupMaterializationRecoveryPlanDenial> {
        let buffer_bytes = usize::try_from(buffer_bytes)
            .map_err(|_| BackupMaterializationRecoveryPlanDenial::InvalidBufferBudget)?;
        if buffer_bytes == 0 {
            return Err(BackupMaterializationRecoveryPlanDenial::InvalidBufferBudget);
        }
        if !target_parent.is_absolute() {
            return Err(BackupMaterializationRecoveryPlanDenial::UnsupportedTargetPath);
        }
        encode_operational_media_path(&target_parent).map_err(map_path_denial)?;
        Ok(Self {
            cut_identity,
            target_parent,
            buffer_bytes,
        })
    }

    pub const fn cut_identity(&self) -> [u8; 32] {
        self.cut_identity
    }

    pub fn target_parent(&self) -> &Path {
        &self.target_parent
    }

    pub const fn buffer_bytes(&self) -> usize {
        self.buffer_bytes
    }

    pub(crate) fn persisted_path(&self) -> Result<(u8, Vec<u8>), OperationalMediaPathDenial> {
        encode_operational_media_path(&self.target_parent)
    }
}

const fn map_path_denial(
    denial: OperationalMediaPathDenial,
) -> BackupMaterializationRecoveryPlanDenial {
    match denial {
        OperationalMediaPathDenial::TooLarge => {
            BackupMaterializationRecoveryPlanDenial::TargetPathTooLarge
        }
        OperationalMediaPathDenial::AllocationFailed => {
            BackupMaterializationRecoveryPlanDenial::AllocationFailed
        }
        OperationalMediaPathDenial::Empty
        | OperationalMediaPathDenial::UnsupportedPlatform
        | OperationalMediaPathDenial::InvalidEncoding => {
            BackupMaterializationRecoveryPlanDenial::UnsupportedTargetPath
        }
    }
}
