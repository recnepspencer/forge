use std::io::Read;
use std::path::Path;

use sha2::{Digest, Sha256};
use worth_store_security::{
    StoreAuthenticityRequirementClass, StoreCustodyPosture, StoreKeyScope, StoreKeyVersionPosture,
    StoreTenantScope,
};

use crate::{BlobChunkByteRange, BlobChunkOrdinal};

use super::artifact::{
    fnv64, identity_evidence, FOOTER_BYTES, HEADER_BYTES, MAGIC, RULE_VERSION, VERSION,
};

#[derive(Debug, Clone, Copy)]
pub struct BoundedBlobBackupVerificationRequest<'a> {
    pub expected_identity: &'a str,
    pub expected_bytes: u64,
    pub expected_digest: [u8; 32],
    pub max_buffer_bytes: usize,
}

#[derive(Debug)]
pub enum BoundedBlobBackupDenial {
    Io(std::io::Error),
    BufferTooSmall,
    AllocationFailed,
    LengthMismatch { expected: u64, actual: u64 },
    InvalidHeader,
    InvalidMetadata,
    IdentityMismatch,
    ChecksumMismatch,
    ContentDigestMismatch,
    StoredDigestMismatch,
    InternalDigestMismatch,
    ArtifactDigestMismatch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoundedBlobBackupObservation {
    payload_bytes: u64,
    bytes_read: u64,
    decoder_allocation_bytes: u64,
    peak_buffer_bytes: u64,
}

impl BoundedBlobBackupObservation {
    pub const fn payload_bytes(self) -> u64 {
        self.payload_bytes
    }
    pub const fn bytes_read(self) -> u64 {
        self.bytes_read
    }
    pub const fn decoder_allocation_bytes(self) -> u64 {
        self.decoder_allocation_bytes
    }
    pub const fn peak_buffer_bytes(self) -> u64 {
        self.peak_buffer_bytes
    }
}

pub fn verify_bounded_blob_backup_artifact(
    path: &Path,
    request: BoundedBlobBackupVerificationRequest<'_>,
) -> Result<BoundedBlobBackupObservation, BoundedBlobBackupDenial> {
    let mut file = std::fs::File::open(path).map_err(BoundedBlobBackupDenial::Io)?;
    let actual = file.metadata().map_err(BoundedBlobBackupDenial::Io)?.len();
    verify_bounded_blob_backup_artifact_from_reader(&mut file, actual, request)
}

pub fn verify_bounded_blob_backup_artifact_from_reader(
    reader: &mut impl Read,
    actual: u64,
    request: BoundedBlobBackupVerificationRequest<'_>,
) -> Result<BoundedBlobBackupObservation, BoundedBlobBackupDenial> {
    if request.max_buffer_bytes <= HEADER_BYTES + FOOTER_BYTES {
        return Err(BoundedBlobBackupDenial::BufferTooSmall);
    }
    if actual != request.expected_bytes {
        return Err(BoundedBlobBackupDenial::LengthMismatch {
            expected: request.expected_bytes,
            actual,
        });
    }
    let mut header = [0_u8; HEADER_BYTES];
    reader
        .read_exact(&mut header)
        .map_err(BoundedBlobBackupDenial::Io)?;
    let fields = decode_header(&header)?;
    let metadata_bytes = fields
        .lengths
        .iter()
        .map(|value| *value as u64)
        .sum::<u64>();
    let encoded_bytes =
        HEADER_BYTES as u64 + metadata_bytes + fields.payload_bytes + FOOTER_BYTES as u64;
    if encoded_bytes != actual {
        return Err(BoundedBlobBackupDenial::InvalidHeader);
    }
    let fixed_bytes = HEADER_BYTES + FOOTER_BYTES;
    let working_bytes = request.max_buffer_bytes - fixed_bytes;
    let metadata_len =
        usize::try_from(metadata_bytes).map_err(|_| BoundedBlobBackupDenial::InvalidMetadata)?;
    if metadata_len >= working_bytes {
        return Err(BoundedBlobBackupDenial::BufferTooSmall);
    }
    let mut metadata = Vec::new();
    metadata
        .try_reserve_exact(metadata_len)
        .map_err(|_| BoundedBlobBackupDenial::AllocationFailed)?;
    metadata.resize(metadata_len, 0);
    reader
        .read_exact(&mut metadata)
        .map_err(BoundedBlobBackupDenial::Io)?;
    let decoded = decode_metadata(&metadata, fields.lengths)?;
    if decoded.rule != RULE_VERSION
        || decoded.identity != request.expected_identity
        || !valid_security(fields.security)
    {
        return Err(BoundedBlobBackupDenial::IdentityMismatch);
    }
    let ordinal = BlobChunkOrdinal::from_raw(fields.ordinal);
    let range = BlobChunkByteRange::new(fields.range_start, fields.range_len)
        .map_err(|_| BoundedBlobBackupDenial::InvalidMetadata)?;

    let chunk_bytes = (working_bytes - metadata_len).min(64 * 1024);
    let mut buffer = Vec::new();
    buffer
        .try_reserve_exact(chunk_bytes)
        .map_err(|_| BoundedBlobBackupDenial::AllocationFailed)?;
    buffer.resize(chunk_bytes, 0);
    let mut checksum_state = 0xcbf2_9ce4_8422_2325;
    let mut content_state = content_hash_prefix(ordinal, range);
    let mut internal_digest = Sha256::new();
    let mut artifact_digest = Sha256::new();
    internal_digest.update(header);
    internal_digest.update(&metadata);
    artifact_digest.update(header);
    artifact_digest.update(&metadata);
    let mut remaining = fields.payload_bytes;
    while remaining > 0 {
        let take = usize::try_from(remaining.min(chunk_bytes as u64))
            .expect("bounded blob chunk fits usize");
        reader
            .read_exact(&mut buffer[..take])
            .map_err(BoundedBlobBackupDenial::Io)?;
        checksum_state = fnv64(checksum_state, &buffer[..take]);
        content_state = fnv64(content_state, &buffer[..take]);
        internal_digest.update(&buffer[..take]);
        artifact_digest.update(&buffer[..take]);
        remaining -= take as u64;
    }
    if decoded.checksum != format!("fnv64:{checksum_state:016x}") {
        return Err(BoundedBlobBackupDenial::ChecksumMismatch);
    }
    if decoded.content != format!("s7:content:{content_state:016x}") {
        return Err(BoundedBlobBackupDenial::ContentDigestMismatch);
    }
    let expected_stored = stable_digest_text("stored", ordinal, range, decoded.checksum);
    if decoded.stored != expected_stored {
        return Err(BoundedBlobBackupDenial::StoredDigestMismatch);
    }
    let expected_identity = stable_digest_text(
        "chunk",
        ordinal,
        range,
        &identity_evidence(decoded.stored, fields.security),
    );
    if decoded.identity != expected_identity {
        return Err(BoundedBlobBackupDenial::IdentityMismatch);
    }
    let mut footer = [0_u8; FOOTER_BYTES];
    reader
        .read_exact(&mut footer)
        .map_err(BoundedBlobBackupDenial::Io)?;
    if internal_digest.finalize()[..] != footer {
        return Err(BoundedBlobBackupDenial::InternalDigestMismatch);
    }
    artifact_digest.update(footer);
    if <[u8; 32]>::from(artifact_digest.finalize()) != request.expected_digest {
        return Err(BoundedBlobBackupDenial::ArtifactDigestMismatch);
    }
    Ok(BoundedBlobBackupObservation {
        payload_bytes: fields.payload_bytes,
        bytes_read: actual,
        decoder_allocation_bytes: (metadata_len + chunk_bytes) as u64,
        peak_buffer_bytes: (fixed_bytes + metadata_len + chunk_bytes) as u64,
    })
}

#[derive(Debug, Clone, Copy)]
struct HeaderFields {
    ordinal: u64,
    range_start: u64,
    range_len: u64,
    payload_bytes: u64,
    lengths: [usize; 5],
    security: [u8; 5],
}

fn decode_header(header: &[u8; HEADER_BYTES]) -> Result<HeaderFields, BoundedBlobBackupDenial> {
    if &header[0..8] != MAGIC || read_u16(header, 8) != VERSION {
        return Err(BoundedBlobBackupDenial::InvalidHeader);
    }
    let fields = HeaderFields {
        ordinal: read_u64(header, 10),
        range_start: read_u64(header, 18),
        range_len: read_u64(header, 26),
        payload_bytes: read_u64(header, 34),
        lengths: std::array::from_fn(|index| usize::from(read_u16(header, 42 + index * 2))),
        security: header[52..57].try_into().expect("fixed security fields"),
    };
    if fields.range_len == 0
        || fields.range_len != fields.payload_bytes
        || fields.range_start.checked_add(fields.range_len).is_none()
        || fields.lengths.contains(&0)
        || fields.lengths.iter().any(|length| *length > 1024)
    {
        return Err(BoundedBlobBackupDenial::InvalidHeader);
    }
    Ok(fields)
}

struct DecodedMetadata<'a> {
    rule: &'a str,
    checksum: &'a str,
    stored: &'a str,
    content: &'a str,
    identity: &'a str,
}

fn decode_metadata(
    bytes: &[u8],
    lengths: [usize; 5],
) -> Result<DecodedMetadata<'_>, BoundedBlobBackupDenial> {
    let mut fields = [""; 5];
    let mut offset = 0;
    for (index, length) in lengths.into_iter().enumerate() {
        fields[index] = std::str::from_utf8(&bytes[offset..offset + length])
            .map_err(|_| BoundedBlobBackupDenial::InvalidMetadata)?;
        offset += length;
    }
    Ok(DecodedMetadata {
        rule: fields[0],
        checksum: fields[1],
        stored: fields[2],
        content: fields[3],
        identity: fields[4],
    })
}

fn valid_security(codes: [u8; 5]) -> bool {
    codes[0] == StoreKeyScope::BlobChunkEnvelope as u8
        && codes[1] == StoreKeyVersionPosture::Current as u8
        && (codes[2] == StoreTenantScope::TenantPhysicalBoundary as u8
            || codes[2] == StoreTenantScope::MultiTenantPhysicalBoundary as u8)
        && codes[3] == StoreAuthenticityRequirementClass::AuthenticatedBlobChunk as u8
        && (codes[4] == StoreCustodyPosture::InternalStoreCustody as u8
            || codes[4] == StoreCustodyPosture::ExportPrepared as u8
            || codes[4] == StoreCustodyPosture::Readmitted as u8)
}

fn content_hash_prefix(ordinal: BlobChunkOrdinal, range: BlobChunkByteRange) -> u64 {
    let mut hash = fnv64(0xcbf2_9ce4_8422_2325, b"content");
    hash = fnv64(hash, RULE_VERSION.as_bytes());
    hash = fnv64(hash, &ordinal.get().to_le_bytes());
    hash = fnv64(hash, &range.start().to_le_bytes());
    fnv64(hash, &range.len().to_le_bytes())
}

fn stable_digest_text(
    lane: &str,
    ordinal: BlobChunkOrdinal,
    range: BlobChunkByteRange,
    evidence: &str,
) -> String {
    let mut hash = fnv64(0xcbf2_9ce4_8422_2325, lane.as_bytes());
    hash = fnv64(hash, RULE_VERSION.as_bytes());
    hash = fnv64(hash, &ordinal.get().to_le_bytes());
    hash = fnv64(hash, &range.start().to_le_bytes());
    hash = fnv64(hash, &range.len().to_le_bytes());
    hash = fnv64(hash, evidence.as_bytes());
    format!("s7:{lane}:{hash:016x}")
}

fn read_u16(bytes: &[u8], offset: usize) -> u16 {
    u16::from_le_bytes(bytes[offset..offset + 2].try_into().expect("fixed field"))
}
fn read_u64(bytes: &[u8], offset: usize) -> u64 {
    u64::from_le_bytes(bytes[offset..offset + 8].try_into().expect("fixed field"))
}
