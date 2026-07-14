use super::{
    admit_replay_cursor_segments, ReplayCursor, WalLsnRange, WalSegmentGeneration, WalSegmentId,
    WalTopologyDenial,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WalSegmentScanLifecycle {
    Current,
    Stale,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WalSegmentScanRecord {
    segment_id: WalSegmentId,
    generation: WalSegmentGeneration,
    lsn_range: WalLsnRange,
    lifecycle_posture: WalSegmentScanLifecycle,
}

impl WalSegmentScanRecord {
    pub const fn current(
        segment_id: WalSegmentId,
        generation: WalSegmentGeneration,
        lsn_range: WalLsnRange,
    ) -> Self {
        Self {
            segment_id,
            generation,
            lsn_range,
            lifecycle_posture: WalSegmentScanLifecycle::Current,
        }
    }

    pub const fn stale(
        segment_id: WalSegmentId,
        generation: WalSegmentGeneration,
        lsn_range: WalLsnRange,
    ) -> Self {
        Self {
            segment_id,
            generation,
            lsn_range,
            lifecycle_posture: WalSegmentScanLifecycle::Stale,
        }
    }

    pub const fn segment_id(self) -> WalSegmentId {
        self.segment_id
    }

    pub const fn generation(self) -> WalSegmentGeneration {
        self.generation
    }

    pub const fn lsn_range(self) -> WalLsnRange {
        self.lsn_range
    }

    pub(super) const fn lifecycle_posture(self) -> WalSegmentScanLifecycle {
        self.lifecycle_posture
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalTopologyScan {
    segment_records: Vec<WalSegmentScanRecord>,
}

impl WalTopologyScan {
    pub fn from_segment_scan(records: impl IntoIterator<Item = WalSegmentScanRecord>) -> Self {
        Self {
            segment_records: records.into_iter().collect(),
        }
    }

    pub fn segment_records(&self) -> &[WalSegmentScanRecord] {
        &self.segment_records
    }

    pub fn admit_replay_cursor(
        self,
        expected_generation: WalSegmentGeneration,
    ) -> Result<ReplayCursor, WalTopologyDenial> {
        let admitted = admit_replay_cursor_segments(self.segment_records, expected_generation)?;
        Ok(ReplayCursor::from_admitted_segments(
            admitted.segments,
            admitted.ordering_proof,
        ))
    }
}
