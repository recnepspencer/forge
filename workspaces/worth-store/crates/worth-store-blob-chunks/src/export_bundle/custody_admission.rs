use worth_store_security::{
    StoreAuthenticityRequirement, StoreAuthenticityRequirementClass, StoreCustodyPosture,
    StoreKeyScope, StoreSecurityScopeAdmissionReceipt, StoreSecurityScopeIdentity,
    StoreTenantScope,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobCustodyPurpose {
    Backup,
    PointInTimeRecovery,
    Export,
}

impl BlobCustodyPurpose {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Backup => "backup",
            Self::PointInTimeRecovery => "pitr",
            Self::Export => "export",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobCustodyAdmissionDenial {
    WrongKeyScope,
    WrongTenantScope,
    WrongAuthenticityRequirement,
    WrongCustodyPosture,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdmittedBlobCustody {
    purpose: BlobCustodyPurpose,
    receipt: StoreSecurityScopeAdmissionReceipt,
}

impl AdmittedBlobCustody {
    pub fn from_security_receipt(
        purpose: BlobCustodyPurpose,
        receipt: StoreSecurityScopeAdmissionReceipt,
    ) -> Result<Self, BlobCustodyAdmissionDenial> {
        validate_identity(receipt.identity())?;
        Ok(Self { purpose, receipt })
    }

    pub const fn purpose(self) -> BlobCustodyPurpose {
        self.purpose
    }

    pub const fn identity(self) -> StoreSecurityScopeIdentity {
        self.receipt.identity()
    }

    pub const fn receipt(self) -> StoreSecurityScopeAdmissionReceipt {
        self.receipt
    }
}

fn validate_identity(
    identity: StoreSecurityScopeIdentity,
) -> Result<(), BlobCustodyAdmissionDenial> {
    if identity.key_scope() != StoreKeyScope::BackupExportEnvelope {
        return Err(BlobCustodyAdmissionDenial::WrongKeyScope);
    }
    if !matches!(
        identity.tenant_scope(),
        StoreTenantScope::BackupRestoreBoundary | StoreTenantScope::ImportReadmissionBoundary
    ) {
        return Err(BlobCustodyAdmissionDenial::WrongTenantScope);
    }
    if identity.authenticity_requirement()
        != StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedBackupCapsule,
        )
    {
        return Err(BlobCustodyAdmissionDenial::WrongAuthenticityRequirement);
    }
    if !matches!(
        identity.custody_posture(),
        StoreCustodyPosture::ExportPrepared | StoreCustodyPosture::Readmitted
    ) {
        return Err(BlobCustodyAdmissionDenial::WrongCustodyPosture);
    }
    Ok(())
}
