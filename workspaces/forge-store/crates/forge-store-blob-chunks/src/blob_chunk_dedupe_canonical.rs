use forge_foundational::BoundaryArtifactId;

use crate::BlobChunkDedupeCandidate;

pub(crate) fn candidate_artifact_id(candidate: &BlobChunkDedupeCandidate) -> BoundaryArtifactId {
    BoundaryArtifactId::new(stable_candidate_hash(candidate))
}

fn stable_candidate_hash(candidate: &BlobChunkDedupeCandidate) -> u64 {
    let mut hash = 0xcbf29ce484222325_u64;
    hash = hash_bytes(
        hash,
        candidate.identity().chunk_digest().as_str().as_bytes(),
    );
    hash = hash_bytes(hash, candidate.content_digest().as_str().as_bytes());
    let metadata = candidate.security_metadata();
    hash = hash_u64(hash, key_scope_tag(metadata.key_scope()));
    hash = hash_u64(
        hash,
        key_version_posture_tag(metadata.key_version_posture()),
    );
    hash = hash_u64(hash, tenant_scope_tag(metadata.tenant_scope()));
    hash = hash_u64(
        hash,
        authenticity_requirement_tag(metadata.authenticity_requirement()),
    );
    hash_u64(hash, custody_posture_tag(metadata.custody_posture()))
}

fn hash_bytes(mut hash: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

const fn hash_u64(hash: u64, value: u64) -> u64 {
    (hash ^ value).wrapping_mul(0x100000001b3)
}

const fn key_scope_tag(scope: forge_store_security::StoreKeyScope) -> u64 {
    match scope {
        forge_store_security::StoreKeyScope::StoreManagedRoot => 1,
        forge_store_security::StoreKeyScope::TenantEnvelope => 2,
        forge_store_security::StoreKeyScope::ArtifactEnvelope => 3,
        forge_store_security::StoreKeyScope::PageEnvelope => 4,
        forge_store_security::StoreKeyScope::WalCheckpointEnvelope => 5,
        forge_store_security::StoreKeyScope::BlobChunkEnvelope => 6,
        forge_store_security::StoreKeyScope::BackupExportEnvelope => 7,
        forge_store_security::StoreKeyScope::RepairScopeEnvelope => 8,
        forge_store_security::StoreKeyScope::SecurityLifecycleFoundation => 9,
    }
}

const fn key_version_posture_tag(posture: forge_store_security::StoreKeyVersionPosture) -> u64 {
    match posture {
        forge_store_security::StoreKeyVersionPosture::Current => 101,
        forge_store_security::StoreKeyVersionPosture::Stale => 102,
        forge_store_security::StoreKeyVersionPosture::RebindRequired => 103,
        forge_store_security::StoreKeyVersionPosture::Unsupported => 104,
        forge_store_security::StoreKeyVersionPosture::Unavailable => 105,
        forge_store_security::StoreKeyVersionPosture::Denied => 106,
    }
}

const fn tenant_scope_tag(scope: forge_store_security::StoreTenantScope) -> u64 {
    match scope {
        forge_store_security::StoreTenantScope::StoreInternal => 11,
        forge_store_security::StoreTenantScope::TenantPhysicalBoundary => 12,
        forge_store_security::StoreTenantScope::MultiTenantPhysicalBoundary => 13,
        forge_store_security::StoreTenantScope::BackupRestoreBoundary => 14,
        forge_store_security::StoreTenantScope::RepairBlastRadius => 15,
        forge_store_security::StoreTenantScope::ImportReadmissionBoundary => 16,
        forge_store_security::StoreTenantScope::SecurityLifecycleFoundation => 17,
    }
}

const fn authenticity_requirement_tag(
    requirement: forge_store_security::StoreAuthenticityRequirement,
) -> u64 {
    match requirement.class() {
        None => 20,
        Some(forge_store_security::StoreAuthenticityRequirementClass::AuthenticatedFrame) => 21,
        Some(forge_store_security::StoreAuthenticityRequirementClass::AuthenticatedWalRecord) => 22,
        Some(forge_store_security::StoreAuthenticityRequirementClass::AuthenticatedManifest) => 23,
        Some(forge_store_security::StoreAuthenticityRequirementClass::AuthenticatedBlobChunk) => 24,
        Some(
            forge_store_security::StoreAuthenticityRequirementClass::AuthenticatedBackupCapsule,
        ) => 25,
        Some(forge_store_security::StoreAuthenticityRequirementClass::AuthenticatedRepairRead) => {
            26
        }
    }
}

const fn custody_posture_tag(posture: forge_store_security::StoreCustodyPosture) -> u64 {
    match posture {
        forge_store_security::StoreCustodyPosture::InternalStoreCustody => 31,
        forge_store_security::StoreCustodyPosture::ExportPrepared => 32,
        forge_store_security::StoreCustodyPosture::ExportedOutOfCustody => 33,
        forge_store_security::StoreCustodyPosture::ImportedUnreadmitted => 34,
        forge_store_security::StoreCustodyPosture::Readmitted => 35,
        forge_store_security::StoreCustodyPosture::CustodyUnavailable => 36,
        forge_store_security::StoreCustodyPosture::CustodyDenied => 37,
        forge_store_security::StoreCustodyPosture::CustodyUnsupported => 38,
    }
}
