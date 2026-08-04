use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

use super::prefix_scan::{scan_segment_path, WalPrefixScan};
use super::{WalArtifactStoreDenial, WalFrameAppendPlan};

const MAGIC: &[u8; 8] = b"WORTHWAL";
const VERSION: u16 = 1;
pub(super) const HEADER_BYTES: usize = 116;
pub(super) const FOOTER_BYTES: usize = 32;

pub(super) fn validate_persisted_frame(
    path: &Path,
    encoded_offset: u64,
    encoded_bytes: u64,
    scope: &crate::WalFramePublicationScope,
) -> Result<(u64, u64), WalArtifactStoreDenial> {
    let mut file = std::fs::File::open(path).map_err(|_| WalArtifactStoreDenial::Io)?;
    let end = encoded_offset
        .checked_add(encoded_bytes)
        .ok_or(WalArtifactStoreDenial::InvalidFrame)?;
    if file
        .metadata()
        .map_err(|_| WalArtifactStoreDenial::Io)?
        .len()
        < end
        || encoded_bytes < (HEADER_BYTES + FOOTER_BYTES) as u64
    {
        return Err(WalArtifactStoreDenial::InvalidFrame);
    }
    file.seek(SeekFrom::Start(encoded_offset))
        .map_err(|_| WalArtifactStoreDenial::Io)?;
    let mut header = [0u8; HEADER_BYTES];
    file.read_exact(&mut header)
        .map_err(|_| WalArtifactStoreDenial::Io)?;
    let frame = decode_header(&header)?;
    let expected_encoded = ((HEADER_BYTES + FOOTER_BYTES) as u64)
        .checked_add(frame.payload_bytes)
        .ok_or(WalArtifactStoreDenial::InvalidFrame)?;
    let expected_identity = Sha256::digest(scope.frame_digest().as_bytes());
    if expected_encoded != encoded_bytes
        || frame.segment_id != scope.segment_id()
        || frame.generation != scope.generation()
        || frame.lsn_start != scope.lsn_start()
        || frame.lsn_end != scope.lsn_end()
        || header[52..84] != expected_identity[..]
        || frame.payload_bytes != scope.expected_bytes()
    {
        return Err(WalArtifactStoreDenial::StoreBindingMismatch);
    }
    let mut payload_digest = Sha256::new();
    let mut frame_digest = Sha256::new();
    frame_digest.update(header);
    let mut buffer = vec![0u8; super::prefix_scan::WAL_SCAN_BUFFER_BYTES];
    let mut remaining = frame.payload_bytes;
    while remaining > 0 {
        let take = usize::try_from(remaining.min(buffer.len() as u64))
            .expect("bounded WAL validation chunk fits usize");
        file.read_exact(&mut buffer[..take])
            .map_err(|_| WalArtifactStoreDenial::Io)?;
        payload_digest.update(&buffer[..take]);
        frame_digest.update(&buffer[..take]);
        remaining -= take as u64;
    }
    if payload_digest.finalize()[..] != header[84..116] {
        return Err(WalArtifactStoreDenial::DigestMismatch);
    }
    let mut footer = [0u8; FOOTER_BYTES];
    file.read_exact(&mut footer)
        .map_err(|_| WalArtifactStoreDenial::Io)?;
    if frame_digest.finalize()[..] != footer {
        return Err(WalArtifactStoreDenial::DigestMismatch);
    }
    Ok((encoded_offset + HEADER_BYTES as u64, frame.payload_bytes))
}

pub(super) fn prepare_append(
    root: &Path,
    segment_id: u64,
    generation: u64,
    lsn_start: u64,
    lsn_end: u64,
    declared_digest: &str,
    payload: &[u8],
) -> Result<WalFrameAppendPlan, WalArtifactStoreDenial> {
    let prefix = scan_segment_path(root, segment_id, generation)?;
    encode_append(
        segment_id,
        generation,
        lsn_start,
        lsn_end,
        declared_digest,
        payload,
        prefix,
    )
}

pub(super) fn encode_append(
    segment_id: u64,
    generation: u64,
    lsn_start: u64,
    lsn_end: u64,
    declared_digest: &str,
    payload: &[u8],
    prefix: WalPrefixScan,
) -> Result<WalFrameAppendPlan, WalArtifactStoreDenial> {
    if segment_id == 0 || generation == 0 || lsn_start >= lsn_end || payload.is_empty() {
        return Err(WalArtifactStoreDenial::InvalidFrame);
    }
    if prefix.last_lsn_end.is_some_and(|last| last != lsn_start) {
        return Err(WalArtifactStoreDenial::NonContiguousLsn);
    }
    let payload_digest = Sha256::digest(payload);
    let identity_digest = Sha256::digest(declared_digest.as_bytes());
    let relative_path = segment_relative_path(segment_id, generation);
    Ok(WalFrameAppendPlan {
        relative_path,
        encoded_frame: encode_frame(
            segment_id,
            generation,
            lsn_start,
            lsn_end,
            identity_digest.as_ref(),
            payload_digest.as_ref(),
            payload,
        ),
        valid_prefix_bytes: prefix.valid_prefix_bytes,
        observed_file_bytes: prefix.observed_file_bytes,
        prefix_bytes_scanned: prefix.bytes_scanned,
    })
}

#[derive(Debug, Clone, Copy)]
pub(super) struct WalFrameHeader {
    pub segment_id: u64,
    pub generation: u64,
    pub lsn_start: u64,
    pub lsn_end: u64,
    pub payload_bytes: u64,
}

pub(super) fn decode_header(
    header: &[u8; HEADER_BYTES],
) -> Result<WalFrameHeader, WalArtifactStoreDenial> {
    if &header[..8] != MAGIC
        || read_u16(header, 8)? != VERSION
        || read_u16(header, 10)? as usize != HEADER_BYTES
    {
        return Err(WalArtifactStoreDenial::InvalidFrame);
    }
    let value = WalFrameHeader {
        segment_id: read_u64(header, 12)?,
        generation: read_u64(header, 20)?,
        lsn_start: read_u64(header, 28)?,
        lsn_end: read_u64(header, 36)?,
        payload_bytes: read_u64(header, 44)?,
    };
    if value.segment_id == 0
        || value.generation == 0
        || value.lsn_start >= value.lsn_end
        || value.payload_bytes == 0
    {
        return Err(WalArtifactStoreDenial::InvalidFrame);
    }
    Ok(value)
}

pub(super) fn segment_relative_path(segment_id: u64, generation: u64) -> PathBuf {
    PathBuf::from("wal").join(format!("segment-{segment_id}-generation-{generation}.wal"))
}

fn encode_frame(
    segment_id: u64,
    generation: u64,
    lsn_start: u64,
    lsn_end: u64,
    identity_digest: &[u8],
    payload_digest: &[u8],
    payload: &[u8],
) -> Vec<u8> {
    let mut frame = Vec::with_capacity(HEADER_BYTES + payload.len() + FOOTER_BYTES);
    frame.extend_from_slice(MAGIC);
    frame.extend_from_slice(&VERSION.to_le_bytes());
    frame.extend_from_slice(&(HEADER_BYTES as u16).to_le_bytes());
    frame.extend_from_slice(&segment_id.to_le_bytes());
    frame.extend_from_slice(&generation.to_le_bytes());
    frame.extend_from_slice(&lsn_start.to_le_bytes());
    frame.extend_from_slice(&lsn_end.to_le_bytes());
    frame.extend_from_slice(&(payload.len() as u64).to_le_bytes());
    frame.extend_from_slice(identity_digest);
    frame.extend_from_slice(payload_digest);
    frame.extend_from_slice(payload);
    let frame_digest = Sha256::digest(&frame);
    frame.extend_from_slice(&frame_digest);
    frame
}

fn read_u16(bytes: &[u8], offset: usize) -> Result<u16, WalArtifactStoreDenial> {
    let raw = bytes
        .get(offset..offset + 2)
        .ok_or(WalArtifactStoreDenial::InvalidFrame)?;
    Ok(u16::from_le_bytes(raw.try_into().expect("length checked")))
}

fn read_u64(bytes: &[u8], offset: usize) -> Result<u64, WalArtifactStoreDenial> {
    let raw = bytes
        .get(offset..offset + 8)
        .ok_or(WalArtifactStoreDenial::InvalidFrame)?;
    Ok(u64::from_le_bytes(raw.try_into().expect("length checked")))
}
