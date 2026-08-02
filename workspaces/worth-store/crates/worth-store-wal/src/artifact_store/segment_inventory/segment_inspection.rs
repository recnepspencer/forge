use sha2::{Digest, Sha256};

use super::WalSegmentArtifactIdentity;
use crate::{LogSequenceNumber, WalArtifactStoreDenial, WalLsnRange};

use crate::artifact_store::frame_codec::{decode_header, FOOTER_BYTES, HEADER_BYTES};

/// Complete, digest-verified facts reconstructed from one bounded WAL segment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WalSegmentInspection {
    identity: WalSegmentArtifactIdentity,
    lsn_range: WalLsnRange,
    frame_count: u64,
    byte_count: u64,
    artifact_digest: [u8; 32],
}

/// One frame payload borrowed from the exact complete segment bytes whose
/// integrity and topology were admitted by [`inspect_verified_wal_segment`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VerifiedWalFramePayload<'segment> {
    lsn_range: WalLsnRange,
    payload: &'segment [u8],
}

/// Complete segment facts plus payload views from the same verification pass.
#[derive(Debug, PartialEq, Eq)]
pub struct VerifiedWalSegment<'segment> {
    inspection: WalSegmentInspection,
    frames: Vec<VerifiedWalFramePayload<'segment>>,
}

pub fn inspect_complete_wal_segment(
    identity: WalSegmentArtifactIdentity,
    bytes: &[u8],
) -> Result<WalSegmentInspection, WalArtifactStoreDenial> {
    Ok(inspect_verified_wal_segment(identity, bytes)?.inspection)
}

pub fn inspect_verified_wal_segment(
    identity: WalSegmentArtifactIdentity,
    bytes: &[u8],
) -> Result<VerifiedWalSegment<'_>, WalArtifactStoreDenial> {
    if bytes.is_empty() {
        return Err(WalArtifactStoreDenial::InvalidFrame);
    }
    let mut offset = 0usize;
    let mut first_lsn = None;
    let mut last_lsn_end = None;
    let mut frame_count = 0u64;
    let mut frames = Vec::new();
    while offset < bytes.len() {
        let header_end = offset
            .checked_add(HEADER_BYTES)
            .ok_or(WalArtifactStoreDenial::InvalidFrame)?;
        let header: &[u8; HEADER_BYTES] = bytes
            .get(offset..header_end)
            .and_then(|header| header.try_into().ok())
            .ok_or(WalArtifactStoreDenial::InvalidFrame)?;
        let fields = decode_header(header)?;
        if fields.segment_id != identity.segment().get()
            || fields.generation != identity.generation().get()
        {
            return Err(WalArtifactStoreDenial::StoreBindingMismatch);
        }
        if last_lsn_end.is_some_and(|prior| prior != fields.lsn_start) {
            return Err(WalArtifactStoreDenial::NonContiguousLsn);
        }
        let payload_bytes = usize::try_from(fields.payload_bytes)
            .map_err(|_| WalArtifactStoreDenial::InvalidFrame)?;
        let payload_end = header_end
            .checked_add(payload_bytes)
            .ok_or(WalArtifactStoreDenial::InvalidFrame)?;
        let frame_end = payload_end
            .checked_add(FOOTER_BYTES)
            .ok_or(WalArtifactStoreDenial::InvalidFrame)?;
        let payload = bytes
            .get(header_end..payload_end)
            .ok_or(WalArtifactStoreDenial::InvalidFrame)?;
        let footer = bytes
            .get(payload_end..frame_end)
            .ok_or(WalArtifactStoreDenial::InvalidFrame)?;
        if Sha256::digest(payload)[..] != header[84..116] {
            return Err(WalArtifactStoreDenial::DigestMismatch);
        }
        if Sha256::digest(&bytes[offset..payload_end])[..] != *footer {
            return Err(WalArtifactStoreDenial::DigestMismatch);
        }
        first_lsn.get_or_insert(fields.lsn_start);
        last_lsn_end = Some(fields.lsn_end);
        let lsn_range = WalLsnRange::new(
            LogSequenceNumber::new(fields.lsn_start),
            LogSequenceNumber::new(fields.lsn_end),
        )
        .map_err(|_| WalArtifactStoreDenial::InvalidFrame)?;
        frames.push(VerifiedWalFramePayload { lsn_range, payload });
        frame_count = frame_count
            .checked_add(1)
            .ok_or(WalArtifactStoreDenial::InvalidFrame)?;
        offset = frame_end;
    }
    let start = LogSequenceNumber::new(first_lsn.ok_or(WalArtifactStoreDenial::InvalidFrame)?);
    let end = LogSequenceNumber::new(last_lsn_end.ok_or(WalArtifactStoreDenial::InvalidFrame)?);
    let lsn_range =
        WalLsnRange::new(start, end).map_err(|_| WalArtifactStoreDenial::InvalidFrame)?;
    Ok(VerifiedWalSegment {
        inspection: WalSegmentInspection {
            identity,
            lsn_range,
            frame_count,
            byte_count: bytes.len() as u64,
            artifact_digest: Sha256::digest(bytes).into(),
        },
        frames,
    })
}

impl<'segment> VerifiedWalFramePayload<'segment> {
    pub const fn lsn_range(self) -> WalLsnRange {
        self.lsn_range
    }

    pub const fn payload(&self) -> &'segment [u8] {
        self.payload
    }
}

impl VerifiedWalSegment<'_> {
    pub const fn inspection(&self) -> WalSegmentInspection {
        self.inspection
    }

    pub fn frames(&self) -> &[VerifiedWalFramePayload<'_>] {
        &self.frames
    }
}

impl WalSegmentInspection {
    pub const fn identity(self) -> WalSegmentArtifactIdentity {
        self.identity
    }

    pub const fn lsn_range(self) -> WalLsnRange {
        self.lsn_range
    }

    pub const fn frame_count(self) -> u64 {
        self.frame_count
    }

    pub const fn byte_count(self) -> u64 {
        self.byte_count
    }

    pub const fn artifact_digest(self) -> [u8; 32] {
        self.artifact_digest
    }
}
