use worth_store_security::{
    StoreAuthenticityRequirement, StoreCustodyPosture, StoreKeyScope, StoreKeyVersionPosture,
    StoreSecurityScopeIdentity, StoreTenantScope,
};

use crate::backup_export_custody_declaration::backup_capsule_authenticity;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BackupImportSourceCustodyScope {
    identity: StoreSecurityScopeIdentity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackupImportSourceCustodyDenial {
    WrongKeyScope,
    WrongKeyVersion,
    WrongTenantScope,
    WrongAuthenticityRequirement,
    WrongCustodyPosture,
}

pub fn admit_backup_import_source_custody_scope(
    identity: StoreSecurityScopeIdentity,
) -> Result<BackupImportSourceCustodyScope, BackupImportSourceCustodyDenial> {
    reject_wrong_key_scope(identity)?;
    reject_wrong_key_version(identity)?;
    reject_wrong_tenant_scope(identity)?;
    reject_wrong_authenticity(identity)?;
    reject_wrong_custody(identity)?;

    Ok(BackupImportSourceCustodyScope { identity })
}

impl BackupImportSourceCustodyScope {
    pub const fn identity(self) -> StoreSecurityScopeIdentity {
        self.identity
    }
}

fn reject_wrong_key_scope(
    identity: StoreSecurityScopeIdentity,
) -> Result<(), BackupImportSourceCustodyDenial> {
    if identity.key_scope() == StoreKeyScope::BackupExportEnvelope {
        Ok(())
    } else {
        Err(BackupImportSourceCustodyDenial::WrongKeyScope)
    }
}

fn reject_wrong_key_version(
    identity: StoreSecurityScopeIdentity,
) -> Result<(), BackupImportSourceCustodyDenial> {
    if identity.key_version_posture() == StoreKeyVersionPosture::Current {
        Ok(())
    } else {
        Err(BackupImportSourceCustodyDenial::WrongKeyVersion)
    }
}

fn reject_wrong_tenant_scope(
    identity: StoreSecurityScopeIdentity,
) -> Result<(), BackupImportSourceCustodyDenial> {
    match identity.tenant_scope() {
        StoreTenantScope::BackupRestoreBoundary | StoreTenantScope::ImportReadmissionBoundary => {
            Ok(())
        }
        _ => Err(BackupImportSourceCustodyDenial::WrongTenantScope),
    }
}

fn reject_wrong_authenticity(
    identity: StoreSecurityScopeIdentity,
) -> Result<(), BackupImportSourceCustodyDenial> {
    let expected: StoreAuthenticityRequirement = backup_capsule_authenticity();
    if identity.authenticity_requirement() == expected {
        Ok(())
    } else {
        Err(BackupImportSourceCustodyDenial::WrongAuthenticityRequirement)
    }
}

fn reject_wrong_custody(
    identity: StoreSecurityScopeIdentity,
) -> Result<(), BackupImportSourceCustodyDenial> {
    match identity.custody_posture() {
        StoreCustodyPosture::ExportPrepared | StoreCustodyPosture::Readmitted => Ok(()),
        _ => Err(BackupImportSourceCustodyDenial::WrongCustodyPosture),
    }
}
