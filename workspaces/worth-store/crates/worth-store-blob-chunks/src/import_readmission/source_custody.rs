use worth_store_security::{
    StoreAuthenticityRequirement, StoreAuthenticityRequirementClass, StoreCustodyPosture,
    StoreKeyScope, StoreKeyVersionPosture, StoreSecurityScopeIdentity, StoreTenantScope,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdmittedBlobImportSourceCustody {
    identity: StoreSecurityScopeIdentity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobImportSourceCustodyDenial {
    WrongKeyScope,
    WrongKeyVersion,
    WrongTenantScope,
    WrongAuthenticityRequirement,
    WrongCustodyPosture,
}

pub fn admit_blob_import_source_custody(
    identity: StoreSecurityScopeIdentity,
) -> Result<AdmittedBlobImportSourceCustody, BlobImportSourceCustodyDenial> {
    if identity.key_scope() != StoreKeyScope::BackupExportEnvelope {
        return Err(BlobImportSourceCustodyDenial::WrongKeyScope);
    }
    if identity.key_version_posture() != StoreKeyVersionPosture::Current {
        return Err(BlobImportSourceCustodyDenial::WrongKeyVersion);
    }
    if !matches!(
        identity.tenant_scope(),
        StoreTenantScope::BackupRestoreBoundary | StoreTenantScope::ImportReadmissionBoundary
    ) {
        return Err(BlobImportSourceCustodyDenial::WrongTenantScope);
    }
    let expected = StoreAuthenticityRequirement::required(
        StoreAuthenticityRequirementClass::AuthenticatedBackupCapsule,
    );
    if identity.authenticity_requirement() != expected {
        return Err(BlobImportSourceCustodyDenial::WrongAuthenticityRequirement);
    }
    if !matches!(
        identity.custody_posture(),
        StoreCustodyPosture::ExportPrepared | StoreCustodyPosture::Readmitted
    ) {
        return Err(BlobImportSourceCustodyDenial::WrongCustodyPosture);
    }
    Ok(AdmittedBlobImportSourceCustody { identity })
}

impl AdmittedBlobImportSourceCustody {
    pub const fn identity(self) -> StoreSecurityScopeIdentity {
        self.identity
    }
}
