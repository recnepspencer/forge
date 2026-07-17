use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

use crate::{
    OperationalOperationId, OperationalSecurityScope, ProductionRestoreAdmissibleBackupBundle,
};

#[derive(Debug)]
pub struct BackupRestoreIntent {
    operation_id: OperationalOperationId,
    backup: ProductionRestoreAdmissibleBackupBundle,
    target_parent: PathBuf,
    security_scope: OperationalSecurityScope,
    admitted_capacity_bytes: u64,
    copy_buffer_bytes: usize,
}

impl BackupRestoreIntent {
    pub fn from_verified_backup(
        operation_id: OperationalOperationId,
        backup: ProductionRestoreAdmissibleBackupBundle,
        target_parent: impl Into<PathBuf>,
        security_scope: OperationalSecurityScope,
        admitted_capacity_bytes: u64,
        copy_buffer_bytes: usize,
    ) -> Self {
        Self {
            operation_id,
            backup,
            target_parent: target_parent.into(),
            security_scope,
            admitted_capacity_bytes,
            copy_buffer_bytes,
        }
    }

    pub fn resolve(self) -> EvidenceBoundBackupRestorePlan {
        let source_identity = self
            .backup
            .custody()
            .structural()
            .materialized()
            .manifest_digest();
        let target_identity = path_identity(&self.target_parent);
        EvidenceBoundBackupRestorePlan {
            operation_id: self.operation_id,
            backup: self.backup,
            target_parent: self.target_parent,
            security_scope: self.security_scope,
            admitted_capacity_bytes: self.admitted_capacity_bytes,
            copy_buffer_bytes: self.copy_buffer_bytes,
            source_identity,
            target_identity,
        }
    }
}

#[derive(Debug)]
pub struct EvidenceBoundBackupRestorePlan {
    pub(super) operation_id: OperationalOperationId,
    pub(super) backup: ProductionRestoreAdmissibleBackupBundle,
    pub(super) target_parent: PathBuf,
    pub(super) security_scope: OperationalSecurityScope,
    pub(super) admitted_capacity_bytes: u64,
    pub(super) copy_buffer_bytes: usize,
    pub(super) source_identity: [u8; 32],
    pub(super) target_identity: [u8; 32],
}

impl EvidenceBoundBackupRestorePlan {
    pub const fn operation_id(&self) -> &OperationalOperationId {
        &self.operation_id
    }
    pub fn source_root(&self) -> &Path {
        self.backup.custody().structural().materialized().root()
    }
    pub fn target_parent(&self) -> &Path {
        &self.target_parent
    }
    pub const fn source_identity(&self) -> [u8; 32] {
        self.source_identity
    }
    pub const fn target_identity(&self) -> [u8; 32] {
        self.target_identity
    }
}

fn path_identity(path: &Path) -> [u8; 32] {
    let value = path.as_os_str().to_string_lossy();
    let mut digest = Sha256::new();
    digest.update(b"worth-store-recovery-target-path-v1");
    digest.update((value.len() as u64).to_be_bytes());
    digest.update(value.as_bytes());
    digest.finalize().into()
}
