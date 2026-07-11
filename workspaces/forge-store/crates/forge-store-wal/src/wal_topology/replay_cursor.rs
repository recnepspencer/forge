use crate::{
    LogSequenceNumber, WalFrameOrderingProof, WalLsnRange, WalSegmentGeneration, WalSegmentId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReplayCursorSegment {
    segment_id: WalSegmentId,
    segment_generation: WalSegmentGeneration,
    lsn_range: WalLsnRange,
}

impl ReplayCursorSegment {
    pub(crate) const fn new(
        segment_id: WalSegmentId,
        segment_generation: WalSegmentGeneration,
        lsn_range: WalLsnRange,
    ) -> Self {
        Self {
            segment_id,
            segment_generation,
            lsn_range,
        }
    }

    pub const fn segment_id(self) -> WalSegmentId {
        self.segment_id
    }

    pub const fn segment_generation(self) -> WalSegmentGeneration {
        self.segment_generation
    }

    pub const fn lsn_range(self) -> WalLsnRange {
        self.lsn_range
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayCursor {
    segments: Vec<ReplayCursorSegment>,
    ordering_proof: WalFrameOrderingProof,
}

impl ReplayCursor {
    pub(crate) fn from_admitted_segments(
        segments: Vec<ReplayCursorSegment>,
        ordering_proof: WalFrameOrderingProof,
    ) -> Self {
        Self {
            segments,
            ordering_proof,
        }
    }

    pub fn segments(&self) -> &[ReplayCursorSegment] {
        &self.segments
    }

    pub const fn ordering_proof(&self) -> &WalFrameOrderingProof {
        &self.ordering_proof
    }

    pub fn first_lsn(&self) -> LogSequenceNumber {
        self.ordering_proof.first_lsn()
    }

    pub fn end_lsn(&self) -> LogSequenceNumber {
        self.ordering_proof.end_lsn()
    }
}
