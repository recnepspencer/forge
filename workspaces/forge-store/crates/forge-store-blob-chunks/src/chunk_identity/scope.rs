use forge_store_readiness::S51AdmittedSecurityScopeReadiness;
use forge_store_security::{
    StoreAuthenticityRequirement, StoreCustodyPosture, StoreKeyScope, StoreKeyVersionPosture,
    StoreSecurityScopeAdmissionReceipt, StoreSecurityScopeIdentity, StoreTenantScope,
};

use crate::{
    BlobChunkScopeCounterSnapshot, BlobChunkSecurityMetadataWitness, BlobChunkSecurityScopeDenial,
    S7BlobChunkSecurityHandoff,
};

#[derive(Debug, PartialEq, Eq)]
pub struct BlobChunkSecurityScope {
    metadata: BlobChunkSecurityMetadataWitness,
}

impl BlobChunkSecurityScope {
    pub(crate) fn from_s5_1_readiness(
        readiness: S51AdmittedSecurityScopeReadiness,
    ) -> Result<Self, BlobChunkSecurityScopeDenial> {
        Ok(Self {
            metadata: BlobChunkSecurityMetadataWitness::from_s5_1_readiness(readiness)?,
        })
    }

    pub fn from_s7_handoff(handoff: S7BlobChunkSecurityHandoff) -> Self {
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
