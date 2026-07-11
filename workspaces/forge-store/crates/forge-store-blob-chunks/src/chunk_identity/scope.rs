use forge_store_security::{
    StoreAdmittedSecurityScope, StoreAuthenticityRequirement, StoreCustodyPosture, StoreKeyScope,
    StoreKeyVersionPosture, StoreSecurityScopeAdmissionReceipt, StoreSecurityScopeIdentity,
    StoreTenantScope,
};

use crate::{
    AdmittedBlobChunkSecurity, BlobChunkScopeCounterSnapshot, BlobChunkSecurityMetadataWitness,
    BlobChunkSecurityScopeDenial,
};

#[derive(Debug, PartialEq, Eq)]
pub struct BlobChunkSecurityScope {
    metadata: BlobChunkSecurityMetadataWitness,
}

impl BlobChunkSecurityScope {
    pub(crate) fn from_admitted_security_scope(
        security_scope: StoreAdmittedSecurityScope,
    ) -> Result<Self, BlobChunkSecurityScopeDenial> {
        Ok(Self {
            metadata: BlobChunkSecurityMetadataWitness::from_admitted_security_scope(
                security_scope,
            )?,
        })
    }

    pub fn from_admitted_blob_security(handoff: AdmittedBlobChunkSecurity) -> Self {
        handoff.into_scope()
    }

    pub const fn metadata(&self) -> BlobChunkSecurityMetadataWitness {
        self.metadata
    }

    pub const fn identity(&self) -> StoreSecurityScopeIdentity {
        self.metadata.identity()
    }

    pub const fn key_scope(&self) -> StoreKeyScope {
        self.metadata.key_scope()
    }

    pub const fn key_version_posture(&self) -> StoreKeyVersionPosture {
        self.metadata.key_version_posture()
    }

    pub const fn tenant_scope(&self) -> StoreTenantScope {
        self.metadata.tenant_scope()
    }

    pub const fn authenticity_requirement(&self) -> StoreAuthenticityRequirement {
        self.metadata.authenticity_requirement()
    }

    pub const fn custody_posture(&self) -> StoreCustodyPosture {
        self.metadata.custody_posture()
    }

    pub const fn receipt(&self) -> StoreSecurityScopeAdmissionReceipt {
        self.metadata.receipt()
    }

    pub const fn counters(&self) -> BlobChunkScopeCounterSnapshot {
        self.metadata.counters()
    }
}
