use worth_store_security::{
    StoreAuthenticityRequirement, StoreAuthenticityRequirementClass, StoreCustodyPosture,
    StoreKeyScope, StoreKeyVersionPosture, StoreTenantScope,
};

use super::DisasterRecoveryBundleDenial;

pub(super) const fn key_scope_tag(value: StoreKeyScope) -> u8 {
    match value {
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

pub(super) fn key_scope(tag: u8) -> Result<StoreKeyScope, DisasterRecoveryBundleDenial> {
    match tag {
        1 => Ok(StoreKeyScope::StoreManagedRoot),
        2 => Ok(StoreKeyScope::TenantEnvelope),
        3 => Ok(StoreKeyScope::ArtifactEnvelope),
        4 => Ok(StoreKeyScope::PageEnvelope),
        5 => Ok(StoreKeyScope::WalCheckpointEnvelope),
        6 => Ok(StoreKeyScope::BlobChunkEnvelope),
        7 => Ok(StoreKeyScope::BackupExportEnvelope),
        8 => Ok(StoreKeyScope::RepairScopeEnvelope),
        9 => Ok(StoreKeyScope::SecurityLifecycleFoundation),
        _ => Err(DisasterRecoveryBundleDenial::ManifestMalformed),
    }
}

pub(super) const fn key_version_tag(value: StoreKeyVersionPosture) -> u8 {
    match value {
        StoreKeyVersionPosture::Current => 1,
        StoreKeyVersionPosture::Stale => 2,
        StoreKeyVersionPosture::RebindRequired => 3,
        StoreKeyVersionPosture::Unsupported => 4,
        StoreKeyVersionPosture::Unavailable => 5,
        StoreKeyVersionPosture::Denied => 6,
    }
}

pub(super) fn key_version(tag: u8) -> Result<StoreKeyVersionPosture, DisasterRecoveryBundleDenial> {
    match tag {
        1 => Ok(StoreKeyVersionPosture::Current),
        2 => Ok(StoreKeyVersionPosture::Stale),
        3 => Ok(StoreKeyVersionPosture::RebindRequired),
        4 => Ok(StoreKeyVersionPosture::Unsupported),
        5 => Ok(StoreKeyVersionPosture::Unavailable),
        6 => Ok(StoreKeyVersionPosture::Denied),
        _ => Err(DisasterRecoveryBundleDenial::ManifestMalformed),
    }
}

pub(super) const fn tenant_scope_tag(value: StoreTenantScope) -> u8 {
    match value {
        StoreTenantScope::StoreInternal => 1,
        StoreTenantScope::TenantPhysicalBoundary => 2,
        StoreTenantScope::MultiTenantPhysicalBoundary => 3,
        StoreTenantScope::BackupRestoreBoundary => 4,
        StoreTenantScope::RepairBlastRadius => 5,
        StoreTenantScope::ImportReadmissionBoundary => 6,
        StoreTenantScope::SecurityLifecycleFoundation => 7,
    }
}

pub(super) fn tenant_scope(tag: u8) -> Result<StoreTenantScope, DisasterRecoveryBundleDenial> {
    match tag {
        1 => Ok(StoreTenantScope::StoreInternal),
        2 => Ok(StoreTenantScope::TenantPhysicalBoundary),
        3 => Ok(StoreTenantScope::MultiTenantPhysicalBoundary),
        4 => Ok(StoreTenantScope::BackupRestoreBoundary),
        5 => Ok(StoreTenantScope::RepairBlastRadius),
        6 => Ok(StoreTenantScope::ImportReadmissionBoundary),
        7 => Ok(StoreTenantScope::SecurityLifecycleFoundation),
        _ => Err(DisasterRecoveryBundleDenial::ManifestMalformed),
    }
}

pub(super) const fn authenticity_tag(value: StoreAuthenticityRequirement) -> u8 {
    match value {
        StoreAuthenticityRequirement::NotRequired => 0,
        StoreAuthenticityRequirement::Required(
            StoreAuthenticityRequirementClass::AuthenticatedFrame,
        ) => 1,
        StoreAuthenticityRequirement::Required(
            StoreAuthenticityRequirementClass::AuthenticatedWalRecord,
        ) => 2,
        StoreAuthenticityRequirement::Required(
            StoreAuthenticityRequirementClass::AuthenticatedManifest,
        ) => 3,
        StoreAuthenticityRequirement::Required(
            StoreAuthenticityRequirementClass::AuthenticatedBlobChunk,
        ) => 4,
        StoreAuthenticityRequirement::Required(
            StoreAuthenticityRequirementClass::AuthenticatedBackupCapsule,
        ) => 5,
        StoreAuthenticityRequirement::Required(
            StoreAuthenticityRequirementClass::AuthenticatedRepairRead,
        ) => 6,
    }
}

pub(super) fn authenticity(
    tag: u8,
) -> Result<StoreAuthenticityRequirement, DisasterRecoveryBundleDenial> {
    let value = match tag {
        0 => StoreAuthenticityRequirement::NotRequired,
        1 => StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedFrame,
        ),
        2 => StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedWalRecord,
        ),
        3 => StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedManifest,
        ),
        4 => StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedBlobChunk,
        ),
        5 => StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedBackupCapsule,
        ),
        6 => StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedRepairRead,
        ),
        _ => return Err(DisasterRecoveryBundleDenial::ManifestMalformed),
    };
    Ok(value)
}

pub(super) const fn custody_tag(value: StoreCustodyPosture) -> u8 {
    match value {
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

pub(super) fn custody(tag: u8) -> Result<StoreCustodyPosture, DisasterRecoveryBundleDenial> {
    match tag {
        1 => Ok(StoreCustodyPosture::InternalStoreCustody),
        2 => Ok(StoreCustodyPosture::ExportPrepared),
        3 => Ok(StoreCustodyPosture::ExportedOutOfCustody),
        4 => Ok(StoreCustodyPosture::ImportedUnreadmitted),
        5 => Ok(StoreCustodyPosture::Readmitted),
        6 => Ok(StoreCustodyPosture::CustodyUnavailable),
        7 => Ok(StoreCustodyPosture::CustodyDenied),
        8 => Ok(StoreCustodyPosture::CustodyUnsupported),
        _ => Err(DisasterRecoveryBundleDenial::ManifestMalformed),
    }
}
