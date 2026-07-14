use worth_store_security::{
    StoreAdmittedSecurityScope, StoreAuthenticityRequirement, StoreAuthenticityRequirementClass,
    StoreCustodyPosture, StoreKeyScope, StoreKeyVersionPosture, StoreSecurityScopeAdmissionReceipt,
    StoreSecurityScopeIdentity, StoreTenantScope,
};

use crate::{BlobChunkScopeCounterSnapshot, BlobChunkSecurityScopeDenial};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlobChunkSecurityMetadataWitness {
    identity: StoreSecurityScopeIdentity,
    key_scope: StoreKeyScope,
    key_version_posture: StoreKeyVersionPosture,
    tenant_scope: StoreTenantScope,
    authenticity_requirement: StoreAuthenticityRequirement,
    custody_posture: StoreCustodyPosture,
    receipt: StoreSecurityScopeAdmissionReceipt,
    counters: BlobChunkScopeCounterSnapshot,
}

impl BlobChunkSecurityMetadataWitness {
    pub(crate) fn from_admitted_security_scope(
        security_scope: StoreAdmittedSecurityScope,
    ) -> Result<Self, BlobChunkSecurityScopeDenial> {
        let counters = BlobChunkScopeCounterSnapshot::start();
        let witnesses = security_scope.witnesses();
        let identity = security_scope.identity();
        reject_non_blob_key_scope(witnesses.key_scope().key_scope(), counters)?;
        reject_non_current_key_version(
            witnesses.key_version_scope().key_version_posture(),
            counters,
        )?;
        reject_non_blob_tenant_scope(witnesses.tenant_scope().tenant_scope(), counters)?;
        reject_non_blob_authenticity_requirement(
            witnesses.authenticity_scope().requirement(),
            counters,
        )?;
        reject_unsupported_blob_custody(witnesses.custody_scope().custody_posture(), counters)?;

        Ok(Self {
            identity,
            key_scope: witnesses.key_scope().key_scope(),
            key_version_posture: witnesses.key_version_scope().key_version_posture(),
            tenant_scope: witnesses.tenant_scope().tenant_scope(),
            authenticity_requirement: witnesses.authenticity_scope().requirement(),
            custody_posture: witnesses.custody_scope().custody_posture(),
            receipt: security_scope.receipt(),
            counters: counters
                .preserve_key_scope()
                .preserve_key_version()
                .preserve_tenant_scope()
                .preserve_authenticity()
                .preserve_custody()
                .issue_metadata_witness()
                .admitted(),
        })
    }

    pub const fn identity(&self) -> StoreSecurityScopeIdentity {
        self.identity
    }

    pub const fn key_scope(&self) -> StoreKeyScope {
        self.key_scope
    }

    pub const fn key_version_posture(&self) -> StoreKeyVersionPosture {
        self.key_version_posture
    }

    pub const fn tenant_scope(&self) -> StoreTenantScope {
        self.tenant_scope
    }

    pub const fn authenticity_requirement(&self) -> StoreAuthenticityRequirement {
        self.authenticity_requirement
    }

    pub const fn custody_posture(&self) -> StoreCustodyPosture {
        self.custody_posture
    }

    pub const fn receipt(&self) -> StoreSecurityScopeAdmissionReceipt {
        self.receipt
    }

    pub const fn counters(&self) -> BlobChunkScopeCounterSnapshot {
        self.counters
    }
}

fn reject_non_blob_key_scope(
    actual: StoreKeyScope,
    counters: BlobChunkScopeCounterSnapshot,
) -> Result<(), BlobChunkSecurityScopeDenial> {
    if actual == StoreKeyScope::BlobChunkEnvelope {
        Ok(())
    } else {
        Err(BlobChunkSecurityScopeDenial::WrongKeyScope {
            actual,
            counters: counters.denied(),
        })
    }
}

fn reject_non_current_key_version(
    actual: StoreKeyVersionPosture,
    counters: BlobChunkScopeCounterSnapshot,
) -> Result<(), BlobChunkSecurityScopeDenial> {
    if actual == StoreKeyVersionPosture::Current {
        Ok(())
    } else {
        Err(BlobChunkSecurityScopeDenial::StaleKeyVersionPosture {
            actual,
            counters: counters.denied(),
        })
    }
}

fn reject_non_blob_tenant_scope(
    actual: StoreTenantScope,
    counters: BlobChunkScopeCounterSnapshot,
) -> Result<(), BlobChunkSecurityScopeDenial> {
    match actual {
        StoreTenantScope::TenantPhysicalBoundary
        | StoreTenantScope::MultiTenantPhysicalBoundary => Ok(()),
        actual => Err(BlobChunkSecurityScopeDenial::WrongTenantScope {
            actual,
            counters: counters.denied(),
        }),
    }
}

fn reject_non_blob_authenticity_requirement(
    actual: StoreAuthenticityRequirement,
    counters: BlobChunkScopeCounterSnapshot,
) -> Result<(), BlobChunkSecurityScopeDenial> {
    let expected = StoreAuthenticityRequirement::required(
        StoreAuthenticityRequirementClass::AuthenticatedBlobChunk,
    );
    if actual == expected {
        Ok(())
    } else {
        Err(BlobChunkSecurityScopeDenial::WrongAuthenticityRequirement {
            actual,
            counters: counters.denied(),
        })
    }
}

fn reject_unsupported_blob_custody(
    actual: StoreCustodyPosture,
    counters: BlobChunkScopeCounterSnapshot,
) -> Result<(), BlobChunkSecurityScopeDenial> {
    match actual {
        StoreCustodyPosture::InternalStoreCustody
        | StoreCustodyPosture::ExportPrepared
        | StoreCustodyPosture::Readmitted => Ok(()),
        actual => Err(BlobChunkSecurityScopeDenial::UnsupportedCustodyPosture {
            actual,
            counters: counters.denied(),
        }),
    }
}
