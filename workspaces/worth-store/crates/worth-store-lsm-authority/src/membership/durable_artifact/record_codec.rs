use super::super::model::LsmMembershipKey;
use crate::{
    BlobWalRecordEnvelope, BlobWalRecordKind, CheckpointDurablePublicationScope,
    WalFrameDurablePublicationScope,
};

/// Non-authoritative bytes to hand to a durability backend.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LsmMembershipArtifactDeclaration {
    bytes: Vec<u8>,
}

impl LsmMembershipArtifactDeclaration {
    pub(crate) fn from_owner_bytes(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }

    pub fn record(envelope: &BlobWalRecordEnvelope, key: LsmMembershipKey) -> Self {
        Self {
            bytes: lsm_membership_record_bytes(envelope, key),
        }
    }

    pub fn manifest(scope: &CheckpointDurablePublicationScope) -> Self {
        Self {
            bytes: lsm_membership_manifest_bytes(scope),
        }
    }

    pub fn compaction_output(scope: &WalFrameDurablePublicationScope) -> Self {
        Self {
            bytes: lsm_membership_output_bytes(scope),
        }
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

pub(crate) fn lsm_membership_record_bytes(
    envelope: &BlobWalRecordEnvelope,
    key: LsmMembershipKey,
) -> Vec<u8> {
    let crate::DurablePublicationScope::WalFrame(scope) = envelope.durable_publication().scope()
    else {
        return Vec::new();
    };
    let body = format!(
        "worth-store:wal-lsm-membership:v1 {} {} {} {} {} {} {} {} {} {} {} {}",
        tenant_code(key.tenant_scope()),
        key_scope_code(key.key_scope()),
        hex(key.canonical()),
        envelope.identity().sequence(),
        record_kind_code(envelope.identity().kind()),
        scope.segment_id(),
        scope.generation(),
        scope.lsn_start(),
        scope.lsn_end(),
        hex(scope.frame_digest().as_bytes()),
        scope.expected_bytes(),
        hex(envelope.payload_digest().as_bytes()),
    );
    let canonical = format!("{body} {:016x}", checksum(body.as_bytes()));
    let mut bytes = Vec::with_capacity(4096);
    bytes.extend_from_slice(canonical.as_bytes());
    bytes.resize(4096, 0);
    bytes
}

pub(crate) fn lsm_membership_manifest_bytes(scope: &CheckpointDurablePublicationScope) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(64 + scope.manifest_digest().len());
    bytes.extend_from_slice(b"worth-store:wal-lsm-manifest:v1\0");
    bytes.extend_from_slice(&scope.checkpoint().checkpoint_epoch().to_le_bytes());
    bytes.extend_from_slice(&scope.covered_lsn_start().to_le_bytes());
    bytes.extend_from_slice(&scope.covered_lsn_end().to_le_bytes());
    bytes.extend_from_slice(scope.manifest_digest().as_bytes());
    bytes
}

pub(crate) fn lsm_membership_output_bytes(scope: &WalFrameDurablePublicationScope) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(4096);
    bytes.extend_from_slice(b"worth-store:wal-lsm-output:v1\0");
    bytes.extend_from_slice(scope.frame_digest().as_bytes());
    bytes.resize(4096, 0);
    bytes
}

pub(crate) fn lsm_membership_activation_digest_prefix(
    key: LsmMembershipKey,
    records: [crate::BlobWalRecordIdentity; 3],
    base: Option<crate::BlobWalRecordIdentity>,
    output: crate::BlobWalRecordIdentity,
    store_binding: &str,
    output_scope: &WalFrameDurablePublicationScope,
) -> String {
    format!(
        "{}:output-scope={}:{}:{}:{}:{}:{}:physical=",
        lsm_membership_replacement_digest(key, records, base, output, store_binding),
        output_scope.segment_id(),
        output_scope.generation(),
        output_scope.lsn_start(),
        output_scope.lsn_end(),
        hex(output_scope.frame_digest().as_bytes()),
        output_scope.expected_bytes(),
    )
}

pub(crate) fn lsm_membership_digest(
    key: LsmMembershipKey,
    records: [crate::BlobWalRecordIdentity; 3],
    base: Option<crate::BlobWalRecordIdentity>,
    store_binding: &str,
) -> String {
    format!(
        "wal-lsm-membership:{store_binding}:{}:{}:{}:{:?}:{:?}:{:?}:base={base:?}",
        tenant_code(key.tenant_scope()),
        key_scope_code(key.key_scope()),
        hex(key.canonical()),
        records[0],
        records[1],
        records[2],
    )
}

pub(crate) fn lsm_membership_replacement_digest(
    key: LsmMembershipKey,
    records: [crate::BlobWalRecordIdentity; 3],
    base: Option<crate::BlobWalRecordIdentity>,
    output: crate::BlobWalRecordIdentity,
    store_binding: &str,
) -> String {
    format!(
        "wal-lsm-replacement:{store_binding}:{}:{}:{}:{:?}:{:?}:{:?}:base={base:?}:{output:?}",
        tenant_code(key.tenant_scope()),
        key_scope_code(key.key_scope()),
        hex(key.canonical()),
        records[0],
        records[1],
        records[2],
    )
}

pub(crate) fn persisted_artifact_matches(
    path: &std::path::Path,
    bytes: u64,
    expected: &[u8],
) -> bool {
    bytes == expected.len() as u64
        && std::fs::read(path)
            .map(|persisted| persisted == expected)
            .unwrap_or(false)
}

pub(crate) fn checksum(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf2_9ce4_8422_2325, |hash, byte| {
        (hash ^ u64::from(*byte)).wrapping_mul(0x0000_0100_0000_01b3)
    })
}

pub(crate) fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

pub(crate) fn unhex(value: &str) -> Option<Vec<u8>> {
    if !value.len().is_multiple_of(2) {
        return None;
    }
    (0..value.len())
        .step_by(2)
        .map(|index| u8::from_str_radix(&value[index..index + 2], 16))
        .collect::<Result<Vec<_>, _>>()
        .ok()
}

pub(crate) const fn record_kind_code(kind: BlobWalRecordKind) -> u8 {
    match kind {
        BlobWalRecordKind::ChunkAppend => 0,
        BlobWalRecordKind::LsmValue => 1,
        BlobWalRecordKind::LsmTombstone => 2,
        BlobWalRecordKind::RootCandidate => 3,
        BlobWalRecordKind::GenerationPublication => 4,
        BlobWalRecordKind::SessionCheckpoint => 5,
        BlobWalRecordKind::SessionCloseout => 6,
    }
}

pub(crate) const fn decode_kind(code: u8) -> Option<BlobWalRecordKind> {
    match code {
        0 => Some(BlobWalRecordKind::ChunkAppend),
        1 => Some(BlobWalRecordKind::LsmValue),
        2 => Some(BlobWalRecordKind::LsmTombstone),
        3 => Some(BlobWalRecordKind::RootCandidate),
        4 => Some(BlobWalRecordKind::GenerationPublication),
        5 => Some(BlobWalRecordKind::SessionCheckpoint),
        6 => Some(BlobWalRecordKind::SessionCloseout),
        _ => None,
    }
}

pub(crate) const fn tenant_code(value: worth_store_security::StoreTenantScope) -> u8 {
    use worth_store_security::StoreTenantScope;
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

pub(crate) const fn key_scope_code(value: worth_store_security::StoreKeyScope) -> u8 {
    use worth_store_security::StoreKeyScope;
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

pub(crate) const fn decode_tenant(code: u8) -> Option<worth_store_security::StoreTenantScope> {
    use worth_store_security::StoreTenantScope;
    match code {
        0 => Some(StoreTenantScope::StoreInternal),
        1 => Some(StoreTenantScope::TenantPhysicalBoundary),
        2 => Some(StoreTenantScope::MultiTenantPhysicalBoundary),
        3 => Some(StoreTenantScope::BackupRestoreBoundary),
        4 => Some(StoreTenantScope::RepairBlastRadius),
        5 => Some(StoreTenantScope::ImportReadmissionBoundary),
        6 => Some(StoreTenantScope::SecurityLifecycleFoundation),
        _ => None,
    }
}

pub(crate) const fn decode_key_scope(code: u8) -> Option<worth_store_security::StoreKeyScope> {
    use worth_store_security::StoreKeyScope;
    match code {
        0 => Some(StoreKeyScope::StoreManagedRoot),
        1 => Some(StoreKeyScope::TenantEnvelope),
        2 => Some(StoreKeyScope::ArtifactEnvelope),
        3 => Some(StoreKeyScope::PageEnvelope),
        4 => Some(StoreKeyScope::WalCheckpointEnvelope),
        5 => Some(StoreKeyScope::BlobChunkEnvelope),
        6 => Some(StoreKeyScope::BackupExportEnvelope),
        7 => Some(StoreKeyScope::RepairScopeEnvelope),
        8 => Some(StoreKeyScope::SecurityLifecycleFoundation),
        _ => None,
    }
}
