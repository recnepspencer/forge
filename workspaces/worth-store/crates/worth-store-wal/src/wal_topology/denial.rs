use crate::{LogSequenceNumber, WalLsnRange, WalSegmentGeneration, WalSegmentId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WalTopologyDenialKind {
    EmptyTopology,
    EmptySegmentId,
    InvalidSegmentGeneration,
    EmptyRange,
    InvertedRange,
    WrongGeneration,
    StaleSegment,
    DuplicateSegment,
    NonContiguousSegment,
    DuplicateLsn,
    OverlappingRange,
    Gap,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalTopologyDenial {
    kind: WalTopologyDenialKind,
    segment_id: Option<WalSegmentId>,
    expected_generation: Option<WalSegmentGeneration>,
    observed_generation: Option<WalSegmentGeneration>,
    previous_range: Option<WalLsnRange>,
    observed_range: Option<WalLsnRange>,
    missing_from: Option<LogSequenceNumber>,
    missing_to: Option<LogSequenceNumber>,
}

impl WalTopologyDenial {
    pub(crate) const fn new(kind: WalTopologyDenialKind) -> Self {
        Self {
            kind,
            segment_id: None,
            expected_generation: None,
            observed_generation: None,
            previous_range: None,
            observed_range: None,
            missing_from: None,
            missing_to: None,
        }
    }

    pub(crate) const fn for_segment(kind: WalTopologyDenialKind, segment_id: WalSegmentId) -> Self {
        Self {
            segment_id: Some(segment_id),
            ..Self::new(kind)
        }
    }

    pub(crate) const fn generation_mismatch(
        segment_id: WalSegmentId,
        expected_generation: WalSegmentGeneration,
        observed_generation: WalSegmentGeneration,
    ) -> Self {
        Self {
            kind: WalTopologyDenialKind::WrongGeneration,
            segment_id: Some(segment_id),
            expected_generation: Some(expected_generation),
            observed_generation: Some(observed_generation),
            previous_range: None,
            observed_range: None,
            missing_from: None,
            missing_to: None,
        }
    }

    pub(crate) const fn duplicate_lsn(
        previous_range: WalLsnRange,
        observed_range: WalLsnRange,
    ) -> Self {
        Self {
            kind: WalTopologyDenialKind::DuplicateLsn,
            previous_range: Some(previous_range),
            observed_range: Some(observed_range),
            ..Self::new(WalTopologyDenialKind::DuplicateLsn)
        }
    }

    pub(crate) const fn overlapping_range(
        previous_range: WalLsnRange,
        observed_range: WalLsnRange,
    ) -> Self {
        Self {
            kind: WalTopologyDenialKind::OverlappingRange,
            previous_range: Some(previous_range),
            observed_range: Some(observed_range),
            ..Self::new(WalTopologyDenialKind::OverlappingRange)
        }
    }

    pub(crate) const fn gap(
        missing_from: LogSequenceNumber,
        missing_to: LogSequenceNumber,
    ) -> Self {
        Self {
            kind: WalTopologyDenialKind::Gap,
            missing_from: Some(missing_from),
            missing_to: Some(missing_to),
            ..Self::new(WalTopologyDenialKind::Gap)
        }
    }

    pub const fn kind(&self) -> WalTopologyDenialKind {
        self.kind
    }

    pub const fn segment_id(&self) -> Option<WalSegmentId> {
        self.segment_id
    }

    pub const fn expected_generation(&self) -> Option<WalSegmentGeneration> {
        self.expected_generation
    }

    pub const fn observed_generation(&self) -> Option<WalSegmentGeneration> {
        self.observed_generation
    }

    pub const fn previous_range(&self) -> Option<WalLsnRange> {
        self.previous_range
    }

    pub const fn observed_range(&self) -> Option<WalLsnRange> {
        self.observed_range
    }

    pub const fn missing_from(&self) -> Option<LogSequenceNumber> {
        self.missing_from
    }

    pub const fn missing_to(&self) -> Option<LogSequenceNumber> {
        self.missing_to
    }
}
