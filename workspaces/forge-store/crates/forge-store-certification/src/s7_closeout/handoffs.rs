use forge_store_readiness::{
    S10BackupRepairReadinessNonClaim, S11KeyLifecycleReadinessNonClaim,
    S12FullCertificationNonClaim, S8LayoutReadinessNonClaim,
};

use super::S7NativeBlobStoreCloseout;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S7S8LayoutReadinessHandoff {
    binding_tag: String,
    declared_chunk_count: u64,
    declared_bytes: u64,
    non_claims: [S8LayoutReadinessNonClaim; 1],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S10BlobBackupRepairNonClaimHandoff {
    binding_tag: String,
    non_claims: [S10BackupRepairReadinessNonClaim; 2],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S11KeyLifecycleNonClaimHandoff {
    binding_tag: String,
    non_claims: [S11KeyLifecycleReadinessNonClaim; 2],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S12FullCertificationNonClaimHandoff {
    binding_tag: String,
    non_claims: [S12FullCertificationNonClaim; 1],
}

pub fn admit_s7_layout_readiness_handoff(
    closeout: &S7NativeBlobStoreCloseout,
) -> Result<S7S8LayoutReadinessHandoff, super::S7CloseoutDenial> {
    Ok(S7S8LayoutReadinessHandoff {
        binding_tag: closeout.binding_tag().to_owned(),
        declared_chunk_count: closeout.declared_chunk_count(),
        declared_bytes: closeout.declared_bytes(),
        non_claims: S8LayoutReadinessNonClaim::required(),
    })
}

pub fn admit_s7_backup_non_claim_handoff(
    closeout: &S7NativeBlobStoreCloseout,
) -> Result<S10BlobBackupRepairNonClaimHandoff, super::S7CloseoutDenial> {
    Ok(S10BlobBackupRepairNonClaimHandoff {
        binding_tag: closeout.binding_tag().to_owned(),
        non_claims: S10BackupRepairReadinessNonClaim::required(),
    })
}

pub fn admit_s7_key_lifecycle_non_claim_handoff(
    closeout: &S7NativeBlobStoreCloseout,
) -> Result<S11KeyLifecycleNonClaimHandoff, super::S7CloseoutDenial> {
    Ok(S11KeyLifecycleNonClaimHandoff {
        binding_tag: closeout.binding_tag().to_owned(),
        non_claims: S11KeyLifecycleReadinessNonClaim::required(),
    })
}

pub fn admit_s7_full_certification_non_claim_handoff(
    closeout: &S7NativeBlobStoreCloseout,
) -> Result<S12FullCertificationNonClaimHandoff, super::S7CloseoutDenial> {
    Ok(S12FullCertificationNonClaimHandoff {
        binding_tag: closeout.binding_tag().to_owned(),
        non_claims: S12FullCertificationNonClaim::required(),
    })
}

impl S7S8LayoutReadinessHandoff {
    pub fn binding_tag(&self) -> &str {
        &self.binding_tag
    }
    pub const fn declared_chunk_count(&self) -> u64 {
        self.declared_chunk_count
    }
    pub const fn declared_bytes(&self) -> u64 {
        self.declared_bytes
    }
    pub const fn non_claims(&self) -> &[S8LayoutReadinessNonClaim; 1] {
        &self.non_claims
    }
}

impl S10BlobBackupRepairNonClaimHandoff {
    pub fn binding_tag(&self) -> &str {
        &self.binding_tag
    }
    pub const fn non_claims(&self) -> &[S10BackupRepairReadinessNonClaim; 2] {
        &self.non_claims
    }
}

impl S11KeyLifecycleNonClaimHandoff {
    pub fn binding_tag(&self) -> &str {
        &self.binding_tag
    }
    pub const fn non_claims(&self) -> &[S11KeyLifecycleReadinessNonClaim; 2] {
        &self.non_claims
    }
}

impl S12FullCertificationNonClaimHandoff {
    pub fn binding_tag(&self) -> &str {
        &self.binding_tag
    }
    pub const fn non_claims(&self) -> &[S12FullCertificationNonClaim; 1] {
        &self.non_claims
    }
}
