use sha2::{Digest, Sha256};
use worth_store_aspect_native::StorePhysicalBoundaryWitness;

use crate::{
    StoreAuthenticityRequirement, StoreCustodyPosture, StoreKeyScope, StoreKeyVersionPosture,
    StoreTenantScope,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StoreSecurityScopeIdentity {
    physical_witness: StorePhysicalBoundaryWitness,
    key_scope: StoreKeyScope,
    key_version_posture: StoreKeyVersionPosture,
    tenant_scope: StoreTenantScope,
    authenticity_requirement: StoreAuthenticityRequirement,
    custody_posture: StoreCustodyPosture,
}

impl StoreSecurityScopeIdentity {
    pub const fn from_physical_security_scope(
        physical_witness: StorePhysicalBoundaryWitness,
        key_scope: StoreKeyScope,
        key_version_posture: StoreKeyVersionPosture,
        tenant_scope: StoreTenantScope,
        authenticity_requirement: StoreAuthenticityRequirement,
        custody_posture: StoreCustodyPosture,
    ) -> Self {
        Self {
            physical_witness,
            key_scope,
            key_version_posture,
            tenant_scope,
            authenticity_requirement,
            custody_posture,
        }
    }

    pub const fn physical_witness(&self) -> StorePhysicalBoundaryWitness {
        self.physical_witness
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

    pub fn stable_fingerprint(self) -> [u8; 32] {
        let authority = self.physical_witness.authority();
        let mut digest = Sha256::new();
        digest.update(b"worth-store:security-scope-identity:v1");
        update_text(&mut digest, authority.roadmap_scope().roadmap());
        update_text(&mut digest, authority.roadmap_scope().sequence());
        digest.update([physical_authority_scope_code(authority.authority_scope())]);
        update_text(&mut digest, authority.boundary_instance().label());
        digest.update([
            key_scope_code(self.key_scope),
            key_version_code(self.key_version_posture),
            tenant_scope_code(self.tenant_scope),
            authenticity_code(self.authenticity_requirement),
            custody_code(self.custody_posture),
        ]);
        digest.finalize().into()
    }
}

fn update_text(digest: &mut Sha256, value: &str) {
    digest.update((value.len() as u64).to_be_bytes());
    digest.update(value.as_bytes());
}

const fn physical_authority_scope_code(scope: worth_store_contracts::PhysicalAuthorityScope) -> u8 {
    use worth_store_contracts::PhysicalAuthorityScope;
    match scope {
        PhysicalAuthorityScope::AspectNativeBoundaryVocabulary => 1,
        PhysicalAuthorityScope::PhysicalFoundationVocabulary => 2,
        PhysicalAuthorityScope::PhysicalEvidenceExport => 3,
        PhysicalAuthorityScope::PhysicalSubstrateReadiness => 4,
    }
}

const fn key_scope_code(scope: StoreKeyScope) -> u8 {
    match scope {
        StoreKeyScope::StoreManagedRoot => 1,
        StoreKeyScope::TenantEnvelope => 2,
        StoreKeyScope::ArtifactEnvelope => 3,
        StoreKeyScope::PageEnvelope => 4,
        StoreKeyScope::WalCheckpointEnvelope => 5,
        StoreKeyScope::BlobChunkEnvelope => 6,
        StoreKeyScope::BackupExportEnvelope => 7,
        StoreKeyScope::RepairScopeEnvelope => 8,
        StoreKeyScope::SecurityLifecycleFoundation => 9,
    }
}

const fn key_version_code(posture: StoreKeyVersionPosture) -> u8 {
    match posture {
        StoreKeyVersionPosture::Current => 1,
        StoreKeyVersionPosture::Stale => 2,
        StoreKeyVersionPosture::RebindRequired => 3,
        StoreKeyVersionPosture::Unsupported => 4,
        StoreKeyVersionPosture::Unavailable => 5,
        StoreKeyVersionPosture::Denied => 6,
    }
}

const fn tenant_scope_code(scope: StoreTenantScope) -> u8 {
    match scope {
        StoreTenantScope::StoreInternal => 1,
        StoreTenantScope::TenantPhysicalBoundary => 2,
        StoreTenantScope::MultiTenantPhysicalBoundary => 3,
        StoreTenantScope::BackupRestoreBoundary => 4,
        StoreTenantScope::RepairBlastRadius => 5,
        StoreTenantScope::ImportReadmissionBoundary => 6,
        StoreTenantScope::SecurityLifecycleFoundation => 7,
    }
}

const fn authenticity_code(requirement: StoreAuthenticityRequirement) -> u8 {
    use crate::StoreAuthenticityRequirementClass as Class;
    match requirement {
        StoreAuthenticityRequirement::NotRequired => 0,
        StoreAuthenticityRequirement::Required(Class::AuthenticatedFrame) => 1,
        StoreAuthenticityRequirement::Required(Class::AuthenticatedWalRecord) => 2,
        StoreAuthenticityRequirement::Required(Class::AuthenticatedManifest) => 3,
        StoreAuthenticityRequirement::Required(Class::AuthenticatedBlobChunk) => 4,
        StoreAuthenticityRequirement::Required(Class::AuthenticatedBackupCapsule) => 5,
        StoreAuthenticityRequirement::Required(Class::AuthenticatedRepairRead) => 6,
    }
}

const fn custody_code(posture: StoreCustodyPosture) -> u8 {
    match posture {
        StoreCustodyPosture::InternalStoreCustody => 1,
        StoreCustodyPosture::ExportPrepared => 2,
        StoreCustodyPosture::ExportedOutOfCustody => 3,
        StoreCustodyPosture::ImportedUnreadmitted => 4,
        StoreCustodyPosture::Readmitted => 5,
        StoreCustodyPosture::CustodyUnavailable => 6,
        StoreCustodyPosture::CustodyDenied => 7,
        StoreCustodyPosture::CustodyUnsupported => 8,
    }
}
