use crate::{StoreCurrentAuthorityIdentity, StoreCurrentAuthorityWitness};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BackupRestoreAdmissionPolicy {
    require_same_current_authority: bool,
}

impl BackupRestoreAdmissionPolicy {
    pub const fn production_default() -> Self {
        Self {
            require_same_current_authority: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BackupRestoreAdmissionRequest {
    structural_verification_identity: [u8; 32],
    custody_scope_fingerprint: u64,
    cut_authority_identity: StoreCurrentAuthorityIdentity,
}

impl BackupRestoreAdmissionRequest {
    pub const fn new(
        structural_verification_identity: [u8; 32],
        custody_scope_fingerprint: u64,
        cut_authority_identity: StoreCurrentAuthorityIdentity,
    ) -> Self {
        Self {
            structural_verification_identity,
            custody_scope_fingerprint,
            cut_authority_identity,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackupRestoreAdmissionDenial {
    StaleCutAuthority,
    MissingCustodyScope,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BackupRestoreAdmissionReceipt {
    structural_verification_identity: [u8; 32],
    custody_scope_fingerprint: u64,
    admitting_authority: StoreCurrentAuthorityIdentity,
}

impl BackupRestoreAdmissionReceipt {
    pub const fn structural_verification_identity(self) -> [u8; 32] {
        self.structural_verification_identity
    }
    pub const fn custody_scope_fingerprint(self) -> u64 {
        self.custody_scope_fingerprint
    }
    pub const fn admitting_authority(self) -> StoreCurrentAuthorityIdentity {
        self.admitting_authority
    }
}

#[derive(Debug)]
pub struct BackupRestoreAdmissionAuthority<'a> {
    current_authority: &'a StoreCurrentAuthorityWitness,
}

impl<'a> BackupRestoreAdmissionAuthority<'a> {
    pub const fn for_current_store(current_authority: &'a StoreCurrentAuthorityWitness) -> Self {
        Self { current_authority }
    }
    pub fn admit(
        self,
        request: BackupRestoreAdmissionRequest,
        policy: BackupRestoreAdmissionPolicy,
    ) -> Result<BackupRestoreAdmissionReceipt, BackupRestoreAdmissionDenial> {
        let admitting_authority = self.current_authority.authority_identity();
        if policy.require_same_current_authority
            && request.cut_authority_identity != admitting_authority
        {
            return Err(BackupRestoreAdmissionDenial::StaleCutAuthority);
        }
        if request.custody_scope_fingerprint == 0 {
            return Err(BackupRestoreAdmissionDenial::MissingCustodyScope);
        }
        Ok(BackupRestoreAdmissionReceipt {
            structural_verification_identity: request.structural_verification_identity,
            custody_scope_fingerprint: request.custody_scope_fingerprint,
            admitting_authority,
        })
    }
}
