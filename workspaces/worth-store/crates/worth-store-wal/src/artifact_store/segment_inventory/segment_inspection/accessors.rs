use super::{
    InterruptedWalSegmentStart, InterruptedWalTail, VerifiedWalActiveTail, VerifiedWalFramePayload,
    VerifiedWalSegment, WalSegmentInspection,
};
use crate::WalLsnRange;

impl<'segment> VerifiedWalFramePayload<'segment> {
    pub const fn lsn_range(self) -> WalLsnRange {
        self.lsn_range
    }

    pub const fn payload(&self) -> &'segment [u8] {
        self.payload
    }

    pub fn to_owned_verified(self) -> super::VerifiedWalFrame {
        super::VerifiedWalFrame {
            lsn_range: self.lsn_range,
            payload: self.payload.into(),
            encoded_bytes: self.encoded_bytes,
        }
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

impl<'segment> VerifiedWalActiveTail<'segment> {
    pub fn into_verified_prefix(self) -> VerifiedWalSegment<'segment> {
        self.verified_prefix
    }

    pub const fn interrupted_tail(&self) -> Option<InterruptedWalTail> {
        self.interrupted_tail
    }
}

impl InterruptedWalTail {
    pub const fn valid_prefix_bytes(self) -> u64 {
        self.valid_prefix_bytes
    }

    pub const fn observed_bytes(self) -> u64 {
        self.observed_bytes
    }
}

impl InterruptedWalSegmentStart {
    pub const fn observed_bytes(self) -> u64 {
        self.observed_bytes
    }
}

impl WalSegmentInspection {
    pub const fn identity(self) -> super::WalSegmentArtifactIdentity {
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
