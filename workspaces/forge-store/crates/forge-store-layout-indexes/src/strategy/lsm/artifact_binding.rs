use super::{BaselineLsmAdmittedKey, BlobWalRecordEnvelope, BlobWalRecordIdentity};
use forge_store_wal::{
    BlobWalRecordKind, CheckpointDurablePublicationScope, WalFrameDurablePublicationScope,
};

pub fn baseline_lsm_record_artifact_bytes(
    envelope: &BlobWalRecordEnvelope,
    key: BaselineLsmAdmittedKey,
) -> Vec<u8> {
    let forge_store_wal::DurablePublicationScope::WalFrame(scope) =
        envelope.durable_publication().scope()
    else {
        return Vec::new();
    };
    let body = format!(
        "forge-store:baseline-lsm-record:v2 {} {} {} {} {} {} {} {} {} {} {} {}",
        super::persisted_codec::tenant_code(key.tenant_scope()),
        super::persisted_codec::key_scope_code(key.key_scope()),
        super::persisted_codec::hex(&key.canonical_key_bytes()),
        envelope.identity().sequence(),
        record_kind_code(envelope.identity().kind()),
        scope.segment_id(),
        scope.generation(),
        scope.lsn_start(),
        scope.lsn_end(),
        super::persisted_codec::hex(scope.frame_digest().as_bytes()),
        scope.expected_bytes(),
        super::persisted_codec::hex(envelope.payload_digest().as_bytes()),
    );
    let canonical = format!("{body} {:016x}", record_artifact_checksum(body.as_bytes()));
    let mut bytes = Vec::with_capacity(4096);
    bytes.extend_from_slice(canonical.as_bytes());
    bytes.resize(4096, 0);
    bytes
}

pub(super) fn record_artifact_checksum(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf2_9ce4_8422_2325, |hash, byte| {
        (hash ^ u64::from(*byte)).wrapping_mul(0x0000_0100_0000_01b3)
    })
}

pub fn baseline_lsm_output_artifact_bytes(scope: &WalFrameDurablePublicationScope) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(4096);
    bytes.extend_from_slice(b"forge-store:baseline-lsm-output:v1\0");
    bytes.extend_from_slice(scope.frame_digest().as_bytes());
    bytes.resize(4096, 0);
    bytes
}

pub fn baseline_lsm_manifest_artifact_bytes(scope: &CheckpointDurablePublicationScope) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(64 + scope.manifest_digest().len());
    bytes.extend_from_slice(b"forge-store:baseline-lsm-manifest:v1\0");
    bytes.extend_from_slice(&scope.checkpoint().checkpoint_epoch().to_le_bytes());
    bytes.extend_from_slice(&scope.covered_lsn_start().to_le_bytes());
    bytes.extend_from_slice(&scope.covered_lsn_end().to_le_bytes());
    bytes.extend_from_slice(scope.manifest_digest().as_bytes());
    bytes
}

pub(super) fn persisted_artifact_matches(
    path: &std::path::Path,
    bytes: u64,
    expected: &[u8],
) -> bool {
    bytes == expected.len() as u64
        && std::fs::read(path)
            .map(|persisted| persisted == expected)
            .unwrap_or(false)
}

pub fn baseline_lsm_manifest_membership_digest(
    key: BaselineLsmAdmittedKey,
    records: [BlobWalRecordIdentity; 3],
    store_binding: &str,
) -> String {
    let tenant_scope = super::persisted_codec::tenant_code(key.tenant_scope());
    let key_scope = super::persisted_codec::key_scope_code(key.key_scope());
    let key = key.canonical_key_bytes();
    format!(
        "lsm-manifest:{store_binding}:{}:{}:{key:02x?}:{:?}:{:?}:{:?}",
        tenant_scope, key_scope, records[0], records[1], records[2]
    )
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
