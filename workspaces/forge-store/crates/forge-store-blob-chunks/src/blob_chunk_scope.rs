use forge_store_readiness::{S51AdmittedSecurityScopeReadiness, S51SecurityScopeReadinessFamily};
use forge_store_security::{
    StoreAuthenticityRequirement, StoreAuthenticityRequirementClass, StoreCustodyPosture,
    StoreKeyScope, StoreSecurityScopeAdmissionReceipt, StoreSecurityScopeIdentity,
    StoreTenantScope,
};

use crate::{
    BlobChunkScopeCounterSnapshot, BlobChunkSecurityScopeDenial, S7BlobChunkSecurityHandoff,
};

#[derive(Debug, PartialEq, Eq)]
pub struct BlobChunkSecurityScope {
    identity: StoreSecurityScopeIdentity,
    receipt: StoreSecurityScopeAdmissionReceipt,
    counters: BlobChunkScopeCounterSnapshot,
}

impl BlobChunkSecurityScope {
    pub(crate) fn from_s5_1_readiness(
        readiness: S51AdmittedSecurityScopeReadiness,
    ) -> Result<Self, BlobChunkSecurityScopeDenial> {
        let counters = BlobChunkScopeCounterSnapshot::start();
        reject_non_blob_readiness_family(&readiness, counters)?;

        let identity = readiness.receipt().identity();
        reject_non_blob_key_scope(identity, counters)?;
        reject_non_blob_tenant_scope(identity, counters)?;
        reject_non_blob_authenticity_requirement(identity, counters)?;
        reject_unsupported_blob_custody(identity, counters)?;

        Ok(Self {
            identity,
            receipt: readiness.receipt(),
            counters: counters.admitted(),
        })
    }

    pub fn from_s7_handoff(handoff: S7BlobChunkSecurityHandoff) -> Self {
        handoff.into_scope()
    }

    pub const fn identity(&self) -> StoreSecurityScopeIdentity {
        self.identity
    }

    pub const fn key_scope(&self) -> StoreKeyScope {
        self.identity.key_scope()
    }

    pub const fn tenant_scope(&self) -> StoreTenantScope {
        self.identity.tenant_scope()
    }

    pub const fn authenticity_requirement(&self) -> StoreAuthenticityRequirement {
        self.identity.authenticity_requirement()
    }

    pub const fn custody_posture(&self) -> StoreCustodyPosture {
        self.identity.custody_posture()
    }

    pub const fn receipt(&self) -> StoreSecurityScopeAdmissionReceipt {
        self.receipt
    }

    pub const fn counters(&self) -> BlobChunkScopeCounterSnapshot {
        self.counters
    }
}

fn reject_non_blob_readiness_family(
    readiness: &S51AdmittedSecurityScopeReadiness,
    counters: BlobChunkScopeCounterSnapshot,
) -> Result<(), BlobChunkSecurityScopeDenial> {
    let family = readiness.reservation().family();
    if family == S51SecurityScopeReadinessFamily::BlobChunk {
        Ok(())
    } else {
        Err(BlobChunkSecurityScopeDenial::WrongReadinessFamily {
            actual: family,
            counters: counters.denied(),
        })
    }
}

fn reject_non_blob_key_scope(
    identity: StoreSecurityScopeIdentity,
    counters: BlobChunkScopeCounterSnapshot,
) -> Result<(), BlobChunkSecurityScopeDenial> {
    if identity.key_scope() == StoreKeyScope::BlobChunkEnvelope {
        Ok(())
    } else {
        Err(BlobChunkSecurityScopeDenial::WrongKeyScope {
            actual: identity.key_scope(),
            counters: counters.denied(),
        })
    }
}

fn reject_non_blob_tenant_scope(
    identity: StoreSecurityScopeIdentity,
    counters: BlobChunkScopeCounterSnapshot,
) -> Result<(), BlobChunkSecurityScopeDenial> {
    match identity.tenant_scope() {
        StoreTenantScope::TenantPhysicalBoundary
        | StoreTenantScope::MultiTenantPhysicalBoundary => Ok(()),
        actual => Err(BlobChunkSecurityScopeDenial::WrongTenantScope {
            actual,
            counters: counters.denied(),
        }),
    }
}

fn reject_non_blob_authenticity_requirement(
    identity: StoreSecurityScopeIdentity,
    counters: BlobChunkScopeCounterSnapshot,
) -> Result<(), BlobChunkSecurityScopeDenial> {
    let expected = StoreAuthenticityRequirement::required(
        StoreAuthenticityRequirementClass::AuthenticatedBlobChunk,
    );
    if identity.authenticity_requirement() == expected {
        Ok(())
    } else {
        Err(BlobChunkSecurityScopeDenial::WrongAuthenticityRequirement {
            actual: identity.authenticity_requirement(),
            counters: counters.denied(),
        })
    }
}

fn reject_unsupported_blob_custody(
    identity: StoreSecurityScopeIdentity,
    counters: BlobChunkScopeCounterSnapshot,
) -> Result<(), BlobChunkSecurityScopeDenial> {
    match identity.custody_posture() {
        StoreCustodyPosture::InternalStoreCustody
        | StoreCustodyPosture::ExportPrepared
        | StoreCustodyPosture::Readmitted => Ok(()),
        actual => Err(BlobChunkSecurityScopeDenial::UnsupportedCustodyPosture {
            actual,
            counters: counters.denied(),
        }),
    }
}
