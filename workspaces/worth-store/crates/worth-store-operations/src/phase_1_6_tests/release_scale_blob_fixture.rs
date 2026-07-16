use std::io::{Seek, SeekFrom, Write};
use std::path::Path;

use sha2::{Digest, Sha256};
use worth_store_security::{
    StoreAuthenticityRequirementClass, StoreCustodyPosture, StoreKeyScope, StoreKeyVersionPosture,
    StoreTenantScope,
};

const MAGIC: &[u8; 8] = b"WORTHBLB";
const VERSION: u16 = 1;
const HEADER_BYTES: usize = 57;
const RULE_VERSION: &str = "s7.fixed-size.raw-chunk.v1";
const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;

pub(super) fn write_sparse_zero_blob(
    path: &Path,
    payload_bytes: u64,
) -> Result<String, std::io::Error> {
    let ordinal = 1_u64;
    let range_start = 0_u64;
    let security = security_codes();
    let checksum_state = fnv_zeros(FNV_OFFSET, payload_bytes);
    let checksum = format!("fnv64:{checksum_state:016x}");
    let content_prefix = content_hash_prefix(ordinal, range_start, payload_bytes);
    let content = format!(
        "s7:content:{:016x}",
        fnv_zeros(content_prefix, payload_bytes)
    );
    let stored = stable_digest("stored", ordinal, range_start, payload_bytes, &checksum);
    let evidence = format!(
        "{}:{}:{}:{}:{}:{}",
        stored, security[0], security[1], security[2], security[3], security[4]
    );
    let identity = stable_digest("chunk", ordinal, range_start, payload_bytes, &evidence);
    let fields = [
        RULE_VERSION,
        checksum.as_str(),
        stored.as_str(),
        content.as_str(),
        identity.as_str(),
    ];
    let header = encode_header(ordinal, range_start, payload_bytes, security, &fields)?;

    let mut internal = Sha256::new();
    internal.update(header);
    for field in fields {
        internal.update(field.as_bytes());
    }
    hash_zero_payload(&mut internal, payload_bytes);
    let footer = internal.finalize();

    let mut file = std::fs::File::create(path)?;
    file.write_all(&header)?;
    for field in fields {
        file.write_all(field.as_bytes())?;
    }
    file.seek(SeekFrom::Current(i64::try_from(payload_bytes).map_err(
        |_| std::io::Error::other("release-scale blob exceeds seek range"),
    )?))?;
    file.write_all(&footer)?;
    file.sync_all()?;
    Ok(identity)
}

fn encode_header(
    ordinal: u64,
    range_start: u64,
    payload_bytes: u64,
    security: [u8; 5],
    fields: &[&str; 5],
) -> Result<[u8; HEADER_BYTES], std::io::Error> {
    let mut header = [0; HEADER_BYTES];
    header[0..8].copy_from_slice(MAGIC);
    header[8..10].copy_from_slice(&VERSION.to_le_bytes());
    header[10..18].copy_from_slice(&ordinal.to_le_bytes());
    header[18..26].copy_from_slice(&range_start.to_le_bytes());
    header[26..34].copy_from_slice(&payload_bytes.to_le_bytes());
    header[34..42].copy_from_slice(&payload_bytes.to_le_bytes());
    for (index, field) in fields.iter().enumerate() {
        let length = u16::try_from(field.len())
            .map_err(|_| std::io::Error::other("blob metadata field too large"))?;
        let offset = 42 + index * 2;
        header[offset..offset + 2].copy_from_slice(&length.to_le_bytes());
    }
    header[52..57].copy_from_slice(&security);
    Ok(header)
}

fn security_codes() -> [u8; 5] {
    [
        StoreKeyScope::BlobChunkEnvelope as u8,
        StoreKeyVersionPosture::Current as u8,
        StoreTenantScope::TenantPhysicalBoundary as u8,
        StoreAuthenticityRequirementClass::AuthenticatedBlobChunk as u8,
        StoreCustodyPosture::InternalStoreCustody as u8,
    ]
}

fn content_hash_prefix(ordinal: u64, range_start: u64, range_len: u64) -> u64 {
    let mut hash = fnv(FNV_OFFSET, b"content");
    hash = fnv(hash, RULE_VERSION.as_bytes());
    hash = fnv(hash, &ordinal.to_le_bytes());
    hash = fnv(hash, &range_start.to_le_bytes());
    fnv(hash, &range_len.to_le_bytes())
}

fn stable_digest(
    lane: &str,
    ordinal: u64,
    range_start: u64,
    range_len: u64,
    evidence: &str,
) -> String {
    let mut hash = fnv(FNV_OFFSET, lane.as_bytes());
    hash = fnv(hash, RULE_VERSION.as_bytes());
    hash = fnv(hash, &ordinal.to_le_bytes());
    hash = fnv(hash, &range_start.to_le_bytes());
    hash = fnv(hash, &range_len.to_le_bytes());
    hash = fnv(hash, evidence.as_bytes());
    format!("s7:{lane}:{hash:016x}")
}

fn fnv(mut state: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        state ^= u64::from(*byte);
        state = state.wrapping_mul(FNV_PRIME);
    }
    state
}

fn fnv_zeros(state: u64, bytes: u64) -> u64 {
    state.wrapping_mul(wrapping_pow(FNV_PRIME, bytes))
}

fn wrapping_pow(mut base: u64, mut exponent: u64) -> u64 {
    let mut result = 1_u64;
    while exponent > 0 {
        if exponent & 1 == 1 {
            result = result.wrapping_mul(base);
        }
        base = base.wrapping_mul(base);
        exponent >>= 1;
    }
    result
}

fn hash_zero_payload(digest: &mut Sha256, mut bytes: u64) {
    let zeros = [0_u8; 64 * 1024];
    while bytes > 0 {
        let take = usize::try_from(bytes.min(zeros.len() as u64)).expect("bounded zero chunk");
        digest.update(&zeros[..take]);
        bytes -= take as u64;
    }
}

#[test]
fn sparse_release_fixture_is_a_real_owner_decodable_blob_artifact() {
    let directory = tempfile::tempdir().expect("fixture directory");
    let path = directory.path().join("blob.media");
    let payload_bytes = 1024 * 1024;
    let identity = write_sparse_zero_blob(&path, payload_bytes).expect("sparse fixture");
    let observation = worth_store_physical_backend::observe_physical_backup_artifact(&path, 4096)
        .expect("fixture observation");
    let decoded = worth_store_blob_chunks::verify_bounded_blob_backup_artifact(
        &path,
        worth_store_blob_chunks::BoundedBlobBackupVerificationRequest {
            expected_identity: &identity,
            expected_bytes: observation.bytes(),
            expected_digest: observation.content_digest(),
            max_buffer_bytes: 128 * 1024,
        },
    )
    .expect("fixture must pass the production owner decoder");
    assert_eq!(decoded.payload_bytes(), payload_bytes);
    assert_eq!(decoded.bytes_read(), observation.bytes());
}
