use forge_store_security::{StoreKeyScope, StoreTenantScope};

use super::super::BaselineLsmExecutionAdmissionDenial;
use super::{BaselineLsmAdmittedKey, BlobWalRecordKind};

pub(super) fn decode_key(
    tenant: &str,
    key_scope: &str,
    bytes: &str,
) -> Result<BaselineLsmAdmittedKey, BaselineLsmExecutionAdmissionDenial> {
    let canonical_key_bytes = unhex(bytes)?
        .try_into()
        .map_err(|_| BaselineLsmExecutionAdmissionDenial::PersistedIndexIo)?;
    Ok(BaselineLsmAdmittedKey {
        tenant_scope: decode_tenant(number(tenant)?)?,
        key_scope: decode_key_scope(number(key_scope)?)?,
        canonical_key_bytes,
    })
}

pub(super) fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn unhex(value: &str) -> Result<Vec<u8>, BaselineLsmExecutionAdmissionDenial> {
    if !value.len().is_multiple_of(2) {
        return Err(BaselineLsmExecutionAdmissionDenial::PersistedIndexIo);
    }
    (0..value.len())
        .step_by(2)
        .map(|index| u8::from_str_radix(&value[index..index + 2], 16))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| BaselineLsmExecutionAdmissionDenial::PersistedIndexIo)
}

pub(super) fn text(value: &str) -> Result<String, BaselineLsmExecutionAdmissionDenial> {
    String::from_utf8(unhex(value)?)
        .map_err(|_| BaselineLsmExecutionAdmissionDenial::PersistedIndexIo)
}

pub(super) fn number<T: std::str::FromStr>(
    value: &str,
) -> Result<T, BaselineLsmExecutionAdmissionDenial> {
    value
        .parse()
        .map_err(|_| BaselineLsmExecutionAdmissionDenial::PersistedIndexIo)
}

pub(super) const fn tenant_code(value: StoreTenantScope) -> u8 {
    match value {
        StoreTenantScope::StoreInternal => 0,
        StoreTenantScope::TenantPhysicalBoundary => 1,
        StoreTenantScope::MultiTenantPhysicalBoundary => 2,
        StoreTenantScope::BackupRestoreBoundary => 3,
        StoreTenantScope::RepairBlastRadius => 4,
        StoreTenantScope::ImportReadmissionBoundary => 5,
        StoreTenantScope::SecurityLifecycleFoundation => 6,
    }
}

fn decode_tenant(value: u8) -> Result<StoreTenantScope, BaselineLsmExecutionAdmissionDenial> {
    match value {
        0 => Ok(StoreTenantScope::StoreInternal),
        1 => Ok(StoreTenantScope::TenantPhysicalBoundary),
        2 => Ok(StoreTenantScope::MultiTenantPhysicalBoundary),
        3 => Ok(StoreTenantScope::BackupRestoreBoundary),
        4 => Ok(StoreTenantScope::RepairBlastRadius),
        5 => Ok(StoreTenantScope::ImportReadmissionBoundary),
        6 => Ok(StoreTenantScope::SecurityLifecycleFoundation),
        _ => Err(BaselineLsmExecutionAdmissionDenial::PersistedIndexIo),
    }
}

pub(super) const fn key_scope_code(value: StoreKeyScope) -> u8 {
    match value {
        StoreKeyScope::StoreManagedRoot => 0,
        StoreKeyScope::TenantEnvelope => 1,
        StoreKeyScope::ArtifactEnvelope => 2,
        StoreKeyScope::PageEnvelope => 3,
        StoreKeyScope::WalCheckpointEnvelope => 4,
        StoreKeyScope::BlobChunkEnvelope => 5,
        StoreKeyScope::BackupExportEnvelope => 6,
        StoreKeyScope::RepairScopeEnvelope => 7,
        StoreKeyScope::SecurityLifecycleFoundation => 8,
    }
}

fn decode_key_scope(value: u8) -> Result<StoreKeyScope, BaselineLsmExecutionAdmissionDenial> {
    match value {
        0 => Ok(StoreKeyScope::StoreManagedRoot),
        1 => Ok(StoreKeyScope::TenantEnvelope),
        2 => Ok(StoreKeyScope::ArtifactEnvelope),
        3 => Ok(StoreKeyScope::PageEnvelope),
        4 => Ok(StoreKeyScope::WalCheckpointEnvelope),
        5 => Ok(StoreKeyScope::BlobChunkEnvelope),
        6 => Ok(StoreKeyScope::BackupExportEnvelope),
        7 => Ok(StoreKeyScope::RepairScopeEnvelope),
        8 => Ok(StoreKeyScope::SecurityLifecycleFoundation),
        _ => Err(BaselineLsmExecutionAdmissionDenial::PersistedIndexIo),
    }
}

pub(super) fn decode_kind(
    value: u8,
) -> Result<BlobWalRecordKind, BaselineLsmExecutionAdmissionDenial> {
    match value {
        0 => Ok(BlobWalRecordKind::ChunkAppend),
        1 => Ok(BlobWalRecordKind::LsmValue),
        2 => Ok(BlobWalRecordKind::LsmTombstone),
        3 => Ok(BlobWalRecordKind::RootCandidate),
        4 => Ok(BlobWalRecordKind::GenerationPublication),
        5 => Ok(BlobWalRecordKind::SessionCheckpoint),
        6 => Ok(BlobWalRecordKind::SessionCloseout),
        _ => Err(BaselineLsmExecutionAdmissionDenial::PersistedIndexIo),
    }
}
